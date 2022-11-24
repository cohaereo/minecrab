use wgpu::include_spirv;



pub struct FogRenderer;
impl FogRenderer {
    pub fn render<'a>(rpass: &mut wgpu::RenderPass<'a>) {
        rpass.draw(0..4 as u32, 0..1);
    }

    pub fn create_pipeline(
        device: &wgpu::Device,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        screen_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let shader_vs = device.create_shader_module(include_spirv!("../shaders/post/fog.vs.spv"));
        let shader_fs = device.create_shader_module(include_spirv!("../shaders/post/fog.fs.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Fog Render Pipeline Layout"),
                bind_group_layouts: &[texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Fog Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_vs,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader_fs,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: screen_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        })
    }
}
