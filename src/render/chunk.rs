use wgpu::{include_spirv, util::DeviceExt, RenderPass};

use crate::world::{ChunkManager, ChunkSectionData};

use super::{
    chunk_mesher::{self, ChunkVertex},
    texture,
};

pub struct ChunkRenderData {
    // Position is in units of 16 blocks, xyz respectively
    pub position: (i32, i32, i32),
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: usize,
    pub memory_usage: usize,
}

impl ChunkRenderData {
    pub fn new_from_chunk(
        // cm: &ChunkManager,
        device: &wgpu::Device,
        coords: (i32, i32, i32),
        c: &ChunkSectionData,
    ) -> Self {
        let (vertex_data, index_data) = chunk_mesher::mesh_chunk(coords, c);
        Self {
            position: coords,
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
            memory_usage: bytemuck::cast_slice::<ChunkVertex, u8>(&vertex_data).len()
                + bytemuck::cast_slice::<u16, u8>(&index_data).len(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ChunkRenderDataPushConstants {
    chunk_coords: [i32; 3],
}

pub struct ChunkRenderer;
impl ChunkRenderer {
    pub fn render<'a>(rpass: &mut RenderPass<'a>, cr: &'a ChunkRenderData) {
        let pc = ChunkRenderDataPushConstants {
            chunk_coords: [cr.position.0, cr.position.1 as i32, cr.position.2],
        };

        rpass.set_push_constants(wgpu::ShaderStages::VERTEX, 0, bytemuck::cast_slice(&[pc]));
        rpass.set_vertex_buffer(0, cr.vertex_buffer.slice(..));
        rpass.set_index_buffer(cr.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.draw_indexed(0..cr.index_count as u32, 0, 0..1);
    }

    pub fn create_pipeline(
        device: &wgpu::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        screen_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        // let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let shader_vs = device.create_shader_module(include_spirv!("../shaders/chunk.vs.spv"));
        let shader_fs = device.create_shader_module(include_spirv!("../shaders/chunk.fs.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[camera_bind_group_layout, texture_bind_group_layout],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX,
                    range: 0..12,
                }],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_vs,
                entry_point: "vs_main",          // 1.
                buffers: &[ChunkVertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader_fs,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: screen_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // polygon_mode: wgpu::PolygonMode::Line,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        })
    }
}
