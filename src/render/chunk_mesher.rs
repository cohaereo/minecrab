use cgmath::Vector3;

use crate::world::{ChunkManager};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkVertex {
    pub data: u32,
}

impl ChunkVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Uint32, 1 => Uint32];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

fn vertex_ao(side1: bool, side2: bool, corner: bool) -> u8 {
    if side1 && side2 {
        return 0;
    }

    3 - (side1 as u8 + side2 as u8 + corner as u8)
}

// TODO: This currently uses the chunk manager for every block, we can check all the blocks inside the chunks without it.
pub fn mesh_chunk(
    coords: (i32, i32, i32),
    cm: &ChunkManager,
    // c: &ChunkSectionData,
) -> (Vec<ChunkVertex>, Vec<u16>) {
    let base = Vector3::<i32>::new(coords.0 * 16, coords.1 as i32 * 16, coords.2 * 16);
    let (mut vertices, mut indices) = (vec![], vec![]);
    let mut ic = 0;

    macro_rules! get_block {
        ($x:expr, $y:expr, $z:expr) => {
            // if $x >= 0 && $x < 16 && $y >= 0 && $y < 16 && $z >= 0 && $z < 16 {
            // if $x >= base.x
            //     && $x < (base.x + 16)
            //     && $y >= base.y
            //     && $y < (base.y + 16)
            //     && $z >= base.z
            //     && $z < (base.z + 16)
            // {
            //     c.get_block($x, $y, $z)
            // } else {
            //     0
            cm.get_block($x, $y, $z)
            // }
        };
    }

    macro_rules! get_neighbors {
        ($x:expr, $y:expr, $z:expr) => {
            // if $x >= 0 && $x < 16 && $y >= 0 && $y < 16 && $z >= 0 && $z < 16 {
            // if $x >= base.x
            //     && $x < (base.x + 16)
            //     && $y >= base.y
            //     && $y < (base.y + 16)
            //     && $z >= base.z
            //     && $z < (base.z + 16)
            // {
            c.get_neighbors($x, $y, $z)
            // } else {
            //     (false, false, false, false, false, false)
            //     // cm.get_neighbors($x, $y, $z)
            // }
        };
    }

    macro_rules! vert {
        ($side:expr, $block:expr, $x:expr, $y:expr, $z:expr, $ao:expr, $light:expr) => {{
            {
                let pos = (($z << 10) | ($y << 5) | $x) as u32;
                let block = $block as u32
                    | (($side as u32 & 0b1111) << 8)
                    | (($ao as u32 & 0b11) << 12)
                    | (($light as u32 & 0b1111) << 14);

                vertices.push(ChunkVertex {
                    data: pos | block << 15,
                });

                ic += 1;
                (ic - 1)
                // }
            }
        }};
    }

    macro_rules! face {
        ($v1:expr, $v2:expr, $v3:expr, $v4:expr) => {
            indices.push($v1); // Bottom left
            indices.push($v2); // Top left
            indices.push($v3); // Top right

            indices.push($v3); // Top right
            indices.push($v4); // Bottom right
            indices.push($v1); // Bottom left
        };
    }

    macro_rules! calculate_ao {
        ($side1:expr, $corner:expr, $side2:expr) => {
            vertex_ao(
                get_block!(base.x + $side1.0, base.y + $side1.1, base.z + $side1.2) != 0,
                get_block!(base.x + $corner.0, base.y + $corner.1, base.z + $corner.2) != 0,
                get_block!(base.x + $side2.0, base.y + $side2.1, base.z + $side2.2) != 0,
            )
        };
    }

    let mut ao0;
    let mut ao1;
    let mut ao2;
    let mut ao3;

    macro_rules! independent_face {
        ($side:expr, $block:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $light:expr) => {
            let i0 = vert!($side, $block, $v1.0, $v1.1, $v1.2, ao0, $light); // 0, front bottom left
            let i1 = vert!($side, $block, $v2.0, $v2.1, $v2.2, ao1, $light); // 1, front bottom right
            let i2 = vert!($side, $block, $v3.0, $v3.1, $v3.2, ao2, $light); // 2, back bottom left
            let i3 = vert!($side, $block, $v4.0, $v4.1, $v4.2, ao3, $light); // 3, back bottom right

            indices.push(i0); // Bottom left
            indices.push(i1); // Top left
            indices.push(i2); // Top right

            indices.push(i2); // Top right
            indices.push(i3); // Bottom right
            indices.push(i0); // Bottom left
        };
    }

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                let block = cm.get_block(base.x + x, base.y + y, base.z + z);
                if block != 0 {
                    let (nup, ndown, nleft, nright, nfront, nback) =
                        cm.get_neighbors(base.x + x, base.y + y, base.z + z);

                    let light = 0xf;

                    if !nup {
                        ao0 = calculate_ao!(
                            (x - 1, y + 1, z + 0),
                            (x - 1, y + 1, z - 1),
                            (x + 0, y + 1, z - 1)
                        );
                        ao1 = calculate_ao!(
                            (x - 1, y + 1, z + 0),
                            (x - 1, y + 1, z + 1),
                            (x + 0, y + 1, z + 1)
                        );
                        ao2 = calculate_ao!(
                            (x + 1, y + 1, z + 0),
                            (x + 1, y + 1, z + 1),
                            (x + 0, y + 1, z + 1)
                        );
                        ao3 = calculate_ao!(
                            (x + 1, y + 1, z + 0),
                            (x + 1, y + 1, z - 1),
                            (x + 0, y + 1, z - 1)
                        );

                        independent_face!(
                            0,
                            block,
                            (x + 0, y + 1, z + 0), // Bottom left
                            (x + 0, y + 1, z + 1), // Top left
                            (x + 1, y + 1, z + 1), // Top right
                            (x + 1, y + 1, z + 0), // Bottom right
                            light
                        );
                    }

                    if !ndown {
                        ao0 = calculate_ao!(
                            (x + 1, y - 1, z + 0),
                            (x + 1, y - 1, z - 1),
                            (x + 0, y - 1, z - 1)
                        );
                        ao1 = calculate_ao!(
                            (x + 1, y - 1, z + 0),
                            (x + 1, y - 1, z + 1),
                            (x + 0, y - 1, z + 1)
                        );
                        ao2 = calculate_ao!(
                            (x - 1, y - 1, z + 0),
                            (x - 1, y - 1, z + 1),
                            (x + 0, y - 1, z + 1)
                        );
                        ao3 = calculate_ao!(
                            (x - 1, y - 1, z + 0),
                            (x - 1, y - 1, z - 1),
                            (x + 0, y - 1, z - 1)
                        );

                        independent_face!(
                            1,
                            block,
                            (x + 1, y + 0, z + 0),
                            (x + 1, y + 0, z + 1),
                            (x + 0, y + 0, z + 1),
                            (x + 0, y + 0, z + 0),
                            light
                        );
                    }

                    if !nleft {
                        ao0 = calculate_ao!(
                            (x - 1, y + 0, z + 1),
                            (x - 1, y - 1, z + 1),
                            (x - 1, y - 1, z + 0)
                        );
                        ao1 = calculate_ao!(
                            (x - 1, y + 0, z + 1),
                            (x - 1, y + 1, z + 1),
                            (x - 1, y + 1, z + 0)
                        );
                        ao2 = calculate_ao!(
                            (x - 1, y + 0, z - 1),
                            (x - 1, y + 1, z - 1),
                            (x - 1, y + 1, z + 0)
                        );
                        ao3 = calculate_ao!(
                            (x - 1, y + 0, z - 1),
                            (x - 1, y - 1, z - 1),
                            (x - 1, y - 1, z + 0)
                        );

                        independent_face!(
                            2,
                            block,
                            (x + 0, y + 0, z + 1), // Bottom left
                            (x + 0, y + 1, z + 1), // Top left
                            (x + 0, y + 1, z + 0), // Top right
                            (x + 0, y + 0, z + 0), // Bottom right
                            light
                        );
                    }

                    if !nright {
                        ao0 = calculate_ao!(
                            (x + 1, y + 0, z - 1),
                            (x + 1, y - 1, z - 1),
                            (x + 1, y - 1, z + 0)
                        );
                        ao1 = calculate_ao!(
                            (x + 1, y + 0, z - 1),
                            (x + 1, y + 1, z - 1),
                            (x + 1, y + 1, z + 0)
                        );
                        ao2 = calculate_ao!(
                            (x + 1, y + 0, z + 1),
                            (x + 1, y + 1, z + 1),
                            (x + 1, y + 1, z + 0)
                        );
                        ao3 = calculate_ao!(
                            (x + 1, y + 0, z + 1),
                            (x + 1, y - 1, z + 1),
                            (x + 1, y - 1, z + 0)
                        );

                        independent_face!(
                            3,
                            block,
                            (x + 1, y + 0, z + 0), // Bottom left
                            (x + 1, y + 1, z + 0), // Top left
                            (x + 1, y + 1, z + 1), // Top right
                            (x + 1, y + 0, z + 1), // Bottom right
                            light
                        );
                    }

                    if !nfront {
                        ao0 = calculate_ao!(
                            (x - 1, y + 0, z - 1),
                            (x - 1, y - 1, z - 1),
                            (x + 0, y - 1, z - 1)
                        );
                        ao1 = calculate_ao!(
                            (x - 1, y + 0, z - 1),
                            (x - 1, y + 1, z - 1),
                            (x + 0, y + 1, z - 1)
                        );
                        ao2 = calculate_ao!(
                            (x + 1, y + 0, z - 1),
                            (x + 1, y + 1, z - 1),
                            (x + 0, y + 1, z - 1)
                        );
                        ao3 = calculate_ao!(
                            (x + 1, y + 0, z - 1),
                            (x + 1, y - 1, z - 1),
                            (x + 0, y - 1, z - 1)
                        );

                        independent_face!(
                            4,
                            block,
                            (x + 0, y + 0, z + 0), // Bottom left
                            (x + 0, y + 1, z + 0), // Top left
                            (x + 1, y + 1, z + 0), // Top right
                            (x + 1, y + 0, z + 0), // Bottom right
                            light
                        );
                    }

                    if !nback {
                        ao0 = calculate_ao!(
                            (x + 1, y + 0, z + 1),
                            (x + 1, y - 1, z + 1),
                            (x + 0, y - 1, z + 1)
                        );
                        ao1 = calculate_ao!(
                            (x + 1, y + 0, z + 1),
                            (x + 1, y + 1, z + 1),
                            (x + 0, y + 1, z + 1)
                        );
                        ao2 = calculate_ao!(
                            (x - 1, y + 0, z + 1),
                            (x - 1, y + 1, z + 1),
                            (x + 0, y + 1, z + 1)
                        );
                        ao3 = calculate_ao!(
                            (x - 1, y + 0, z + 1),
                            (x - 1, y - 1, z + 1),
                            (x + 0, y - 1, z + 1)
                        );

                        independent_face!(
                            5,
                            block,
                            (x + 1, y + 0, z + 1), // Bottom left
                            (x + 1, y + 1, z + 1), // Top left
                            (x + 0, y + 1, z + 1), // Top right
                            (x + 0, y + 0, z + 1), // Bottom right
                            light
                        );
                    }
                }
            }
        }
    }

    (vertices, indices)
}
