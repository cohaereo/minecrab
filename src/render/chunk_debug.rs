use wgpu::{include_spirv, util::DeviceExt};

use super::texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DebugLineVertex {
    pub pos: [f32; 3],
    pub col: [f32; 3],
}

impl DebugLineVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct DebugLineRenderer {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: usize,
}

impl DebugLineRenderer {
    pub fn render<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, coordinates: (i32, i32)) {
        rpass.set_push_constants(
            wgpu::ShaderStages::VERTEX,
            0,
            bytemuck::cast_slice(&[coordinates.0, coordinates.1]),
        );
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.draw(0..self.vertex_count as u32, 0..1);
    }

    pub fn create_pipeline(
        device: &wgpu::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        screen_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let shader_vs =
            device.create_shader_module(include_spirv!("../shaders/debug_lines.vs.spv"));
        let shader_fs =
            device.create_shader_module(include_spirv!("../shaders/debug_lines.fs.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Debugline Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX,
                    range: 0..8,
                }],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Debugline Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_vs,
                entry_point: "vs_main",              // 1.
                buffers: &[DebugLineVertex::desc()], // 2.
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
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Line,
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

    pub fn new_chunklines(device: &wgpu::Device) -> Self {
        let mut vertex_data = vec![];

        const YELLOW: [f32; 3] = [1., 1., 0.];
        const RED: [f32; 3] = [1., 0., 0.];
        const BLUE: [f32; 3] = [0.247, 0.247, 1.];
        const CYAN: [f32; 3] = [0., 0.6, 0.6];

        macro_rules! line {
            ($a:expr, $b:expr, $col:expr) => {
                vertex_data.push(DebugLineVertex { pos: $a, col: $col });

                vertex_data.push(DebugLineVertex { pos: $b, col: $col });
            };
        }

        // Outside chunk markers
        line!([-16., -64., -16.], [-16., 320., -16.], RED);
        line!([0., -64., -16.], [0., 320., -16.], RED);
        line!([16., -64., -16.], [16., 320., -16.], RED);
        line!([32., -64., -16.], [32., 320., -16.], RED);

        line!([-16., -64., 0.], [-16., 320., 0.], RED);
        line!([32., -64., 0.], [32., 320., 0.], RED);
        line!([-16., -64., 16.], [-16., 320., 16.], RED);
        line!([32., -64., 16.], [32., 320., 16.], RED);

        line!([-16., -64., 32.], [-16., 320., 32.], RED);
        line!([0., -64., 32.], [0., 320., 32.], RED);
        line!([16., -64., 32.], [16., 320., 32.], RED);
        line!([32., -64., 32.], [32., 320., 32.], RED);

        // Horizontal lines
        for i in (-64..320).step_by(2) {
            let c = if i % 16 == 0 {
                BLUE
            } else if i % 8 == 0 {
                CYAN
            } else {
                YELLOW
            };
            line!([0., i as f32, 0.], [16., i as f32, 0.], c);
            line!([16., i as f32, 0.], [16., i as f32, 16.], c);
            line!([0., i as f32, 0.], [0., i as f32, 16.], c);
            line!([0., i as f32, 16.], [16., i as f32, 16.], c);
        }

        // Vertical lines
        for i in (0..16).step_by(2) {
            let c = if i % 16 == 0 {
                BLUE
            } else if i % 4 == 0 {
                CYAN
            } else {
                YELLOW
            };

            line!([i as f32, -64., 0.], [i as f32, 320., 0.], c);
            line!([16., -64., i as f32], [16., 320., i as f32], c);
            line!([0., -64., 16. - i as f32], [0., 320., 16. - i as f32], c);
            line!([16. - i as f32, -64., 16.], [16. - i as f32, 320., 16.], c);
        }

        Self {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunklines vertex buffer"),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            vertex_count: vertex_data.len(),
        }
    }
}
