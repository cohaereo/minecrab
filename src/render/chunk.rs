use glam::Vec3;
use wgpu::{include_spirv, util::DeviceExt, RenderPass};

use crate::world::ChunkSectionData;

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

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ChunkRenderDataPushConstants {
    chunk_coords: [i32; 3],
    render_distance: u32,
    camera_pos: [f32; 3],
}

pub struct ChunkRenderer;
impl ChunkRenderer {
    pub fn render<'a>(
        rpass: &mut RenderPass<'a>,
        cr: &'a ChunkRenderData,
        camera_pos: Vec3,
        render_distance: u32,
    ) {
        let pc = ChunkRenderDataPushConstants {
            chunk_coords: [cr.position.0, cr.position.1 as i32, cr.position.2],
            render_distance,
            camera_pos: [camera_pos.x, camera_pos.y, camera_pos.z],
        };

        rpass.set_push_constants(wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT, 0, bytemuck::cast_slice(&[pc]));
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
        let shader_vs = device.create_shader_module(include_spirv!("../shaders/chunk.vs.spv"));
        let shader_fs = device.create_shader_module(include_spirv!("../shaders/chunk.fs.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[camera_bind_group_layout, texture_bind_group_layout],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    range: 0..32,
                }],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_vs,
                entry_point: "vs_main",
                buffers: &[ChunkVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_fs,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: screen_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None, // 5.
        })
    }
}
