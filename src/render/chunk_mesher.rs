use std::sync::Arc;

use cgmath::Point3;
use tokio::sync::mpsc;
use wgpu::util::DeviceExt;

use crate::world::ChunkManager;

use super::chunk::ChunkRenderData;

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

fn is_opaque(bid: u8) -> bool {
    match bid {
        0 | 6 | 8 | 9 | 10 | 11 | 18 | 20 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 36 | 37
        | 38 | 39 | 40 | 43 | 44 | 50 | 51 | 52 | 53 | 54 | 55 | 59 | 60 | 61 | 62 | 63 | 64
        | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 81 | 83
        | 85 | 86 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 101 | 102 | 103 | 104 | 105 | 106
        | 107 | 108 | 109 | 111 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 122 | 123
        | 124 | 125 | 126 | 127 | 128 | 130 | 131 | 132 | 134 | 135 | 136 | 138 | 139 | 141
        | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 154 | 156 | 157
        | 160 | 161 | 163 | 164 | 166 | 167 | 171 | 175 => false,
        _ => true,
    }
}

/// Struct representing the section blocks to be meshed and the blocks around it (used for face culling and AO)
#[derive(Default)]
pub struct ChunkSectionContext {
    blocks: [[[u8; 18]; 18]; 18],
    light: [[[u8; 18]; 18]; 18],
}

impl ChunkSectionContext {
    pub fn new(cm: &ChunkManager, position: Point3<i32>) -> Self {
        let base = position * 16 - Point3::new(1, 1, 1);

        let mut r = Self::default();

        // TODO: Can probably be optimized a bit?
        for y in 0..18 {
            for x in 0..18 {
                for z in 0..18 {
                    r.blocks[y][x][z] =
                        cm.get_block(base.x + x as i32, base.y + y as i32, base.z + z as i32);
                    let (light, skylight) =
                        cm.get_block_light(base.x + x as i32, base.y + y as i32, base.z + z as i32);
                    r.light[y][x][z] = ((skylight & 0xf) << 4) | (light & 0xf);
                }
            }
        }

        r
    }

    /// Returns minecraft:air for out of bounds coordinates
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u8 {
        if x < -1 || x > 17 || y < -1 || y > 17 || z < -1 || z > 17 {
            return 0;
        }

        self.blocks[1 + y as usize][1 + x as usize][1 + z as usize]
    }

    pub fn get_block_light(&self, x: i32, y: i32, z: i32) -> (u8, u8) {
        if x < -1 || x > 17 || y < -1 || y > 17 || z < -1 || z > 17 {
            return (0, 0);
        }

        let d = self.light[1 + y as usize][1 + x as usize][1 + z as usize];

        (d & 0x0f, (d >> 4) & 0x0f)
    }

    pub fn get_neighbors_merged_opaques(
        &self,
        x: i32,
        y: i32,
        z: i32,
    ) -> (bool, bool, bool, bool, bool, bool) {
        let center_block = self.get_block(x, y, z);

        macro_rules! nb {
            ($x:expr, $y:expr, $z:expr) => {{
                let (tx, ty, tz) = (x + $x, y + $y, z + $z);

                let block = self.get_block(tx, ty, tz);
                is_opaque(block) || (block == center_block && block != 18)
            }};
        }

        (
            nb!(0, 1, 0),  // Up
            nb!(0, -1, 0), // Down
            nb!(-1, 0, 0), // Left
            nb!(1, 0, 0),  // Right
            nb!(0, 0, -1), // Front
            nb!(0, 0, 1),  // Back
        )
    }
}

pub fn mesh_chunk(c: &ChunkSectionContext) -> (Vec<ChunkVertex>, Vec<u16>) {
    let (mut vertices, mut indices) = (vec![], vec![]);
    let mut ic = 0;

    macro_rules! vert {
        ($side:expr, $block:expr, $x:expr, $y:expr, $z:expr, $ao:expr, $light:expr) => {{
            {
                let pos = (($z << 10) | ($y << 5) | $x) as u32;
                let block = $block as u32
                    // | (($side as u32 & 0b1111) << 8)
                    | (($ao as u32 & 0b11) << 8)
                    | (($light as u32 & 0b1111) << 10);

                vertices.push(ChunkVertex {
                    data: pos | block << 15,
                });

                ic += 1;
                (ic - 1)
                // }
            }
        }};
    }

    macro_rules! calculate_ao {
        ($side1:expr, $corner:expr, $side2:expr) => {
            vertex_ao(
                is_opaque(c.get_block($side1.0, $side1.1, $side1.2)),
                is_opaque(c.get_block($corner.0, $corner.1, $corner.2)),
                is_opaque(c.get_block($side2.0, $side2.1, $side2.2)),
            )
        };
    }

    let mut ao0;
    let mut ao1;
    let mut ao2;
    let mut ao3;

    macro_rules! independent_face {
        ($side:expr, $block:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $neighbor:expr) => {
            let (_light, _skylight) = c.get_block_light($neighbor.0, $neighbor.1, $neighbor.2);

            let i0 = vert!(
                $side,
                $block,
                $v1.0,
                $v1.1,
                $v1.2,
                ao0,
                _light.max(_skylight)
            ); // 0, front bottom left
            let i1 = vert!(
                $side,
                $block,
                $v2.0,
                $v2.1,
                $v2.2,
                ao1,
                _light.max(_skylight)
            ); // 1, front bottom right
            let i2 = vert!(
                $side,
                $block,
                $v3.0,
                $v3.1,
                $v3.2,
                ao2,
                _light.max(_skylight)
            ); // 2, back bottom left
            let i3 = vert!(
                $side,
                $block,
                $v4.0,
                $v4.1,
                $v4.2,
                ao3,
                _light.max(_skylight)
            ); // 3, back bottom right

            if ao0 + ao2 > ao1 + ao3 {
                // 1-----2
                // | \   |
                // |   \ |
                // 0-----3
                indices.push(i0); // Bottom left
                indices.push(i1); // Top left
                indices.push(i3); // Bottom right

                indices.push(i1); // Top left
                indices.push(i2); // Top right
                indices.push(i3); // Bottom right
            } else {
                // 1-----2
                // |   / |
                // | /   |
                // 0-----3
                indices.push(i0); // Bottom left
                indices.push(i1); // Top left
                indices.push(i2); // Top right

                indices.push(i2); // Top right
                indices.push(i3); // Bottom right
                indices.push(i0); // Bottom left
            }
        };
    }

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                let block = c.get_block(x, y, z);
                if block != 0 {
                    let (nup, ndown, nleft, nright, nfront, nback) =
                        c.get_neighbors_merged_opaques(x, y, z);

                    // let light = 0xf;

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
                            (x + 0, y + 1, z + 0)  // light
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
                            (x + 0, y - 1, z + 0) // light
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
                            (x - 1, y + 0, z + 0)  // light
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
                            (x + 1, y + 0, z + 0)  // light
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
                            (x + 0, y + 0, z - 1)  // light
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
                            (x + 0, y + 0, z + 1)  // light
                        );
                    }
                }
            }
        }
    }

    (vertices, indices)
}

pub struct ChunkMeshingRequest {
    pub chunk_pos: Point3<i32>,
    pub data: ChunkSectionContext,

    /// If a buffer already exists it can be used instead of creating a brand new one
    pub buffers: Option<ChunkRenderData>,
}

pub fn chunk_mesher_thread(
    // TODO: Swap these for an IAD object
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
) -> (
    mpsc::Sender<ChunkMeshingRequest>,
    mpsc::UnboundedReceiver<ChunkRenderData>,
) {
    let (chunk_send, mut chunk_recv) = mpsc::channel::<ChunkMeshingRequest>(512);
    let (rdata_send, rdata_recv) = mpsc::unbounded_channel::<ChunkRenderData>();

    tokio::spawn(async move {
        while let Some(cd) = chunk_recv.recv().await {
            let (vertex_data, index_data) = mesh_chunk(&cd.data);

            let render_data = if let Some(b) = cd.buffers {
                queue.write_buffer(&b.vertex_buffer, 0, bytemuck::cast_slice(&vertex_data));
                queue.write_buffer(&b.index_buffer, 0, bytemuck::cast_slice(&index_data));

                ChunkRenderData {
                    index_count: index_data.len(),
                    ..b
                }
            } else {
                ChunkRenderData {
                    position: cd.chunk_pos,
                    vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Chunk vertex buffer"),
                        contents: bytemuck::cast_slice(&vertex_data),
                        usage: wgpu::BufferUsages::VERTEX,
                    }),
                    index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Chunk index buffer"),
                        contents: bytemuck::cast_slice(&index_data),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                    index_count: index_data.len(),
                }
            };

            rdata_send.send(render_data).ok();
        }
    });

    (chunk_send, rdata_recv)
}
