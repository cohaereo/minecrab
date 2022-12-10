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

/// Struct representing the section blocks to be meshed and the blocks around it (used for face culling and AO)
#[derive(Default)]
pub struct ChunkSectionContext {
    blocks: [[[u8; 18]; 18]; 18],
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

    pub fn get_neighbors(&self, x: i32, y: i32, z: i32) -> (bool, bool, bool, bool, bool, bool) {
        macro_rules! nb {
            ($x:expr, $y:expr, $z:expr) => {{
                let (tx, ty, tz) = (x + $x, y + $y, z + $z);

                self.get_block(tx, ty, tz) != 0
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

    macro_rules! calculate_ao {
        ($side1:expr, $corner:expr, $side2:expr) => {
            vertex_ao(
                c.get_block($side1.0, $side1.1, $side1.2) != 0,
                c.get_block($corner.0, $corner.1, $corner.2) != 0,
                c.get_block($side2.0, $side2.1, $side2.2) != 0,
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
                let block = c.get_block(x, y, z);
                if block != 0 {
                    let (nup, ndown, nleft, nright, nfront, nback) = c.get_neighbors(x, y, z);

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
