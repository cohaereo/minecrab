use glam::Vec3;
use wgpu::{include_spirv, util::DeviceExt};

use super::texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DebugCubeVertex {
    pub pos: [f32; 3],
}

impl DebugCubeVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct DebugCubeRenderer {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: usize,
}

impl DebugCubeRenderer {
    pub fn render<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, position: Vec3) {
        rpass.set_push_constants(
            wgpu::ShaderStages::VERTEX,
            0,
            bytemuck::cast_slice(&[position.x, position.y, position.z]),
        );
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.draw(0..self.vertex_count as u32, 0..1);
    }

    pub fn create_pipeline(
        device: &wgpu::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        screen_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let shader_vs = device.create_shader_module(include_spirv!("../shaders/debug_cube.vs.spv"));
        let shader_fs = device.create_shader_module(include_spirv!("../shaders/debug_cube.fs.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Debugcube Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, texture_bind_group_layout],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX,
                    range: 0..12,
                }],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Debugcube Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_vs,
                entry_point: "vs_main",              // 1.
                buffers: &[DebugCubeVertex::desc()], // 2.
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
                topology: wgpu::PrimitiveTopology::TriangleStrip,
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

    pub fn new(device: &wgpu::Device) -> Self {
        const CUBE_VERTS: [[f32; 3]; 16] = [
            [-0.5, -0.5, 0.5],  // 1  left    First Strip
            [-0.5, 0.5, 0.5],   // 3
            [-0.5, -0.5, -0.5], // 0
            [-0.5, 0.5, -0.5],  // 2
            [0.5, -0.5, -0.5],  // 4  back
            [0.5, 0.5, -0.5],   // 6
            [0.5, -0.5, 0.5],   // 5  right
            [0.5, 0.5, 0.5],    // 7
            [0.5, 0.5, -0.5],   // 6  top     Second Strip
            [-0.5, 0.5, -0.5],  // 2
            [0.5, 0.5, 0.5],    // 7
            [-0.5, 0.5, 0.5],   // 3
            [0.5, -0.5, 0.5],   // 5  front
            [-0.5, -0.5, 0.5],  // 1
            [0.5, -0.5, -0.5],  // 4  bottom
            [-0.5, -0.5, -0.5], // 0
        ];

        Self {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunklines vertex buffer"),
                contents: bytemuck::cast_slice(&CUBE_VERTS),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            vertex_count: CUBE_VERTS.len(),
        }
    }
}
