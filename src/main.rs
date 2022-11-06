#![feature(int_roundings)]

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

use tokio::{net::TcpStream, sync::mpsc};

use std::time::Instant;

use clap::Parser;
use glam::{DVec3, Mat4, Vec2, Vec3, Vec3Swizzles};
use imgui::FontGlyphRanges;
use wgpu::util::DeviceExt;
use world::ChunkManager;

use crate::{
    ecs::{update_velocity, Position, Velocity},
    net::{codec::MinecraftCodec, packets::decode_packet},
    render::{
        chunk::{ChunkRenderData, ChunkRenderer},
        chunk_debug::DebugLineRenderer,
        debug_cube::DebugCubeRenderer,
        texture,
        util::{Camera, CameraController, CameraUniform, AABB},
    },
};

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod ecs;
mod fixed_point;
mod net;
mod nibble_slice;
mod render;
mod varint;
mod world;

const ICON_MIN_FA: u32 = 0xe005;
const ICON_MAX_FA: u32 = 0xf8ff;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Address of the server to connect to
    #[arg(short, long, default_value = "localhost")]
    address: String,

    /// Port to use
    #[arg(short, long, default_value_t = 25565)]
    port: u16,

    #[arg(short, long, default_value = "Nautilus")]
    username: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // GLOBAL.reset();
    pretty_env_logger::init();

    let args = CliArgs::parse();

    let _client = tracy_client::Client::start();

    let mut chunks = ChunkManager::new();

    let (mut conn_read, mut conn_write) =
        TcpStream::connect(format!("{}:{}", args.address, args.port))
            .await?
            .into_split();

    let (write_tx, mut write_rx) = tokio::sync::mpsc::channel::<net::packets::Packet>(128);
    tokio::spawn(async move {
        loop {
            if let Some(p) = write_rx.recv().await {
                if let Ok(rp) = net::packets::encode_packet(&p) {
                    MinecraftCodec::write(&mut conn_write, &rp).await.unwrap();
                }
            } else {
                // Channel is closed, exit thread
                break;
            }
        }
    });

    write_tx
        .send(net::packets::Packet::Handshake {
            protocol_version: 5,
            server_address: args.address,
            server_port: args.port,
            next_state: 2,
        })
        .await?;

    write_tx
        .send(net::packets::Packet::LoginStart(args.username))
        .await?;

    // Wait for login success
    while MinecraftCodec::read(&mut conn_read).await?.id != 2 {}

    let mut camera = Camera::new();
    camera.aspect = 1600 as f32 / 900 as f32;

    // Wait for player pos

    loop {
        let p = MinecraftCodec::read(&mut conn_read).await?;
        match decode_packet(&p) {
            Ok(net::packets::Packet::PlayerPositionLookServer {
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
            }) => {
                camera.position = Vec3::new(x as f32, y as f32, z as f32);
                camera.orientation = Vec2::new(pitch, yaw);
                // println!("Writing response");
                write_tx
                    .send(net::packets::Packet::PlayerPositionLookClient {
                        x,
                        feet_y: y - 1.62,
                        head_y: y,
                        z,
                        yaw,
                        pitch,
                        on_ground,
                    })
                    .await?;

                write_tx.send(net::packets::Packet::ClientStatus(0)).await?;
                break;
            }
            _ => {}
        }
    }

    // Switch packet handling to a new thread
    // ! Unholy.
    let (main_tx, mut main_rx) = mpsc::channel::<net::packets::Packet>(256);
    let write_tx_net = write_tx.clone();
    tokio::spawn(async move {
        'game: loop {
            let p = MinecraftCodec::read(&mut conn_read).await.unwrap();
            match decode_packet(&p) {
                Ok(p) => match p {
                    net::packets::Packet::KeepAlive(t) => {
                        write_tx_net
                            .send(net::packets::Packet::KeepAlive(t))
                            .await
                            .unwrap();
                    }
                    net::packets::Packet::PlayerPositionLookServer {
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                        on_ground,
                    } => {
                        main_tx.send(p).await.unwrap();
                        write_tx_net
                            .send(net::packets::Packet::PlayerPositionLookClient {
                                x,
                                feet_y: y - 1.62,
                                head_y: y,
                                z,
                                yaw,
                                pitch,
                                on_ground,
                            })
                            .await
                            .unwrap();
                    }
                    net::packets::Packet::Respawn { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::MapChunkBulk { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::ChunkData { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::BlockChange { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::DestroyEntities { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::EntityLookRelativeMove { .. } => {
                        main_tx.send(p).await.unwrap()
                    }
                    net::packets::Packet::EntityRelativeMove { .. } => {
                        main_tx.send(p).await.unwrap()
                    }
                    net::packets::Packet::EntityVelocity { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::SpawnMob { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::SpawnObject { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::SpawnPlayer { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::MultiBlockChange { .. } => main_tx.send(p).await.unwrap(),
                    net::packets::Packet::ChatMessage(s) => {
                        info!("Chat message: {}", s);
                    }
                    net::packets::Packet::Disconnect(reason) => {
                        warn!("Disconnected: {}", reason);
                        break 'game;
                    }
                    _ => {}
                },
                Err(e) => error!("Error decoding packet: {}", e),
            }
        }
    });

    #[cfg(target_os = "linux")]
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1600, 900))
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    info!("Available devices:");
    for b in instance.enumerate_adapters(wgpu::Backends::all()) {
        info!(
            "\t- {} on {:?} (features {:b})",
            b.get_info().name,
            b.get_info().backend,
            b.features()
        )
    }

    let surface = unsafe { instance.create_surface(&window) };
    let adapter =
        futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

    let (device, queue) = futures::executor::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::PUSH_CONSTANTS | wgpu::Features::POLYGON_MODE_LINE,
            limits: wgpu::Limits {
                max_push_constant_size: 32,
                ..Default::default()
            },
            label: None,
        },
        None,
    ))
    .unwrap();
    info!(
        "Supported formats: {:?}",
        surface.get_supported_formats(&adapter)
    );
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: *surface.get_supported_formats(&adapter).first().unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
    };
    surface.configure(&device, &surface_config);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&mut camera);
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    });

    let dcube_texture = texture::Texture::load_png(&device, &queue, "block_debug.png");
    let atlas_texture = texture::Texture::load_png(&device, &queue, "atlas.png");
    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&atlas_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&atlas_texture.sampler),
            },
        ],
        label: Some("texture_bind_group"),
    });

    let texture_bind_group_debugcube = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&dcube_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&dcube_texture.sampler),
            },
        ],
        label: Some("texture_bind_group"),
    });

    let mut depth_texture =
        texture::Texture::create_depth_texture(&device, &surface_config, "depth_texture");

    let mut camera_controller = CameraController::new(6.0);

    let chunk_pipeline = ChunkRenderer::create_pipeline(
        &device,
        &camera_bind_group_layout,
        &texture_bind_group_layout,
        surface_config.format,
    );

    const CHUNK_AABB: AABB = AABB {
        min: Vec3::splat(0.),
        max: Vec3::splat(16.),
    };

    let mut imgui_ctx = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui_ctx);
    platform.attach_window(
        imgui_ctx.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Rounded,
    );
    let hidpi_factor = window.scale_factor();
    imgui_ctx.fonts().add_font(&[
        imgui::FontSource::TtfData {
            data: include_bytes!("../DroidSans.ttf"),
            size_pixels: (15. * hidpi_factor).round() as f32,
            config: Some(imgui::FontConfig {
                name: Some("DroidSans.ttf".to_string()),
                glyph_ranges: FontGlyphRanges::from_slice(&[
                    0x0020, 0x00FF, // Basic Latin + Latin Supplement
                    0x03BC, 0x03BC, // micro
                    0x03C3, 0x03C3, // small sigma
                    0x2013, 0x2013, // en dash
                    0x2264, 0x2264, // less-than or equal to
                    0,
                ]),
                ..Default::default()
            }),
        },
        imgui::FontSource::TtfData {
            data: include_bytes!("../FontAwesomeSolid.ttf"),
            size_pixels: (15. * hidpi_factor).round() as f32,
            config: Some(imgui::FontConfig {
                name: Some("FontAwesomeSolid.ttf".to_string()),
                glyph_ranges: FontGlyphRanges::from_slice(&[ICON_MIN_FA, ICON_MAX_FA, 0]),
                ..Default::default()
            }),
        },
    ]);

    {
        let style = imgui_ctx.style_mut();
        style.frame_rounding = 3.;
        style.window_rounding = 3.;
        style.tab_rounding = 3.;
        style.child_rounding = 3.;
        style.popup_rounding = 3.;
        style.scrollbar_rounding = 3.;
    }

    let mut imgui_renderer = imgui_wgpu::Renderer::new(
        &mut imgui_ctx,
        &device,
        &queue,
        imgui_wgpu::RendererConfig {
            texture_format: surface_config.format,
            ..Default::default()
        },
    );

    let mut world = hecs::World::new();

    let debugcube_pipeline = DebugCubeRenderer::create_pipeline(
        &device,
        &camera_bind_group_layout,
        &texture_bind_group_layout,
        surface_config.format,
    );

    let debuglines_pipeline = DebugLineRenderer::create_pipeline(
        &device,
        &camera_bind_group_layout,
        surface_config.format,
    );
    let debuglines = DebugLineRenderer::new_chunklines(&device);
    let debugcube = DebugCubeRenderer::new(&device);

    let adapter_info = adapter.get_info();
    let cpu_brand = raw_cpuid::CpuId::new()
        .get_processor_brand_string()
        .and_then(|b| Some(b.as_str().to_string()))
        .unwrap_or("Unknown CPU".to_string());

    let mut actual_player_pos = Vec3::default();
    let mut frame_count = 0;
    let mut last_frame = Instant::now();
    let mut chunks_rendered = 0;
    let mut total_chunks = 0;
    let mut demo_open = true;
    let mut render_distance = 16;
    let mut chunklines_shown = false;
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                camera_controller.process_events(event);
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(kc) = input.virtual_keycode {
                            match kc {
                                VirtualKeyCode::F4 => {
                                    if input.state == ElementState::Pressed {
                                        chunklines_shown = !chunklines_shown;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::Resized(_) => {
                        let size = window.inner_size();

                        surface_config.width = size.width;
                        surface_config.height = size.height;

                        surface.configure(&device, &surface_config);
                        depth_texture = texture::Texture::create_depth_texture(
                            &device,
                            &surface_config,
                            "depth texture",
                        );
                        camera.aspect = size.width as f32 / size.height as f32;
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let frame_delta = last_frame.elapsed().as_secs_f32();
                imgui_ctx.io_mut().update_delta_time(last_frame.elapsed());

                update_velocity(&mut world, frame_delta);

                // * Receive chunks
                // ! Yes i know doing this just before rendering a frame isn't great but it doesnt need to be yet.
                let mut packet_quota = 64;
                loop {
                    if let Ok(p) = main_rx.try_recv() {
                        match p {
                            net::packets::Packet::MapChunkBulk {
                                columns,
                                has_sky_light,
                                data,
                                meta,
                            } => {
                                println!("Received {} chunks", columns);
                                let mut data_offset = 0;
                                for (_i, cm) in meta.iter().enumerate() {
                                    let bytes_read = chunks
                                        .load_chunk(
                                            (cm.chunk_x, cm.chunk_z),
                                            cm.primary_bitmap,
                                            cm.add_bitmap,
                                            has_sky_light,
                                            true,
                                            &data[data_offset..],
                                        )
                                        .unwrap();
                                    data_offset += bytes_read as usize;
                                }

                                if data_offset < data.len() {
                                    warn!("Trailing data in chunk batch!");
                                }
                            }
                            net::packets::Packet::ChunkData {
                                chunk_x,
                                chunk_z,
                                ground_up_continuous,
                                primary_bitmap,
                                add_bitmap,
                                data,
                            } => {
                                chunks
                                    .load_chunk(
                                        (chunk_x, chunk_z),
                                        primary_bitmap,
                                        add_bitmap,
                                        false,
                                        ground_up_continuous,
                                        &data,
                                    )
                                    .unwrap();
                            }
                            net::packets::Packet::Respawn { .. } => {
                                chunks.chunks.clear();

                                // Shrink to reclaim memory
                                chunks.chunks.shrink_to_fit();
                            }
                            net::packets::Packet::BlockChange {
                                x, y, z, block_id, ..
                            } => {
                                chunks.set_block(x, y as i32, z, block_id as u8);
                            }
                            net::packets::Packet::MultiBlockChange {
                                chunk_x,
                                chunk_z,
                                records,
                            } => {
                                for r in records {
                                    chunks.set_block(
                                        chunk_x * 16 + r.x as i32,
                                        r.y as i32,
                                        chunk_z * 16 + r.z as i32,
                                        r.block_id as u8,
                                    );
                                }
                            }

                            net::packets::Packet::EntityLookRelativeMove {
                                eid,
                                dx,
                                dy,
                                dz,
                                ..
                            } => {
                                let ent = ecs::get_or_insert(&mut world, eid);
                                if let Ok((p, _v)) =
                                    world.query_one_mut::<(&mut Position, &mut Velocity)>(ent)
                                {
                                    p.0 += Vec3::new(dx as f32, dy as f32, dz as f32);
                                }
                            }
                            net::packets::Packet::EntityRelativeMove { eid, dx, dy, dz } => {
                                let ent = ecs::get_or_insert(&mut world, eid);
                                if let Ok((p, _v)) =
                                    world.query_one_mut::<(&mut Position, &mut Velocity)>(ent)
                                {
                                    p.0 += Vec3::new(dx as f32, dy as f32, dz as f32);
                                }
                            }
                            net::packets::Packet::EntityVelocity { eid, x, y, z } => {
                                let ent = ecs::get_or_insert(&mut world, eid);
                                if let Ok((_p, v)) =
                                    world.query_one_mut::<(&mut Position, &mut Velocity)>(ent)
                                {
                                    v.0 = Vec3::new(x as f32, y as f32, z as f32)
                                        * ecs::VELOCITY_UNIT;
                                }
                            }
                            net::packets::Packet::SpawnMob {
                                eid,
                                x,
                                y,
                                z,
                                velocity_x,
                                velocity_y,
                                velocity_z,
                                ..
                            } => {
                                let ent = ecs::get_or_insert(&mut world, eid);
                                if let Ok((p, v)) =
                                    world.query_one_mut::<(&mut Position, &mut Velocity)>(ent)
                                {
                                    p.0 = Vec3::new(x as f32, y as f32, z as f32);
                                    v.0 = Vec3::new(
                                        velocity_x as f32,
                                        velocity_y as f32,
                                        velocity_z as f32,
                                    ) * ecs::VELOCITY_UNIT;
                                }
                            }
                            net::packets::Packet::SpawnObject { eid, x, y, z, .. } => {
                                let ent = ecs::get_or_insert(&mut world, eid);
                                if let Ok(p) = world.query_one_mut::<&mut Position>(ent) {
                                    p.0 = Vec3::new(x as f32, y as f32, z as f32);
                                }
                            }
                            net::packets::Packet::SpawnPlayer { eid, x, y, z, .. } => {
                                let ent = ecs::get_or_insert(&mut world, eid);
                                if let Ok(p) = world.query_one_mut::<&mut Position>(ent) {
                                    p.0 = Vec3::new(x as f32, y as f32, z as f32);
                                }
                            }
                            net::packets::Packet::DestroyEntities(entities) => {
                                for e in entities {
                                    let eid = ecs::get_or_insert(&mut world, e);
                                    world.despawn(eid).ok();
                                }
                            }
                            net::packets::Packet::PlayerPositionLookServer {
                                x,
                                y,
                                z,
                                yaw,
                                pitch,
                                on_ground,
                            } => {
                                actual_player_pos = Vec3::new(x as f32, y as f32, z as f32);
                                camera.position = actual_player_pos;
                            }
                            _ => {}
                        }
                    } else {
                        break;
                    }

                    packet_quota -= 1;
                    if packet_quota == 0 {
                        warn!("Packet quota reached!");
                        break;
                    }
                }

                let dirty_chunk_count = chunks
                    .chunks
                    .iter()
                    .map(|c| {
                        let mut count = 0;
                        for s in c.1.sections.iter() {
                            if let Some(s) = s {
                                if s.dirty {
                                    count += 1
                                }
                            }
                        }
                        count
                    })
                    .sum::<usize>();

                if dirty_chunk_count != 0 {
                    let mut chunk_meshing_quota = 8;
                    for (coord, chunk) in chunks.chunks.iter_mut() {
                        for cy in 0..16 {
                            if let Some(cd) = chunk.get_section_mut(cy) {
                                if cd.dirty {
                                    cd.renderdata = Some(ChunkRenderData::new_from_chunk(
                                        // &chunks,
                                        &device,
                                        (coord.0, cy as i32, coord.1),
                                        cd,
                                    ));
                                    cd.dirty = false;

                                    chunk_meshing_quota -= 1;
                                }
                            }
                        }

                        if chunk_meshing_quota == 0 {
                            break;
                        }
                    }
                }

                // Send player position every tick
                // FIXME: If you dont have vsync enabled (or a 60hz monitor) this is going to hurt
                if frame_count % 3 == 0 {
                    write_tx
                        .try_send(net::packets::Packet::PlayerPositionLookClient {
                            x: camera.position.x as f64,
                            feet_y: camera.position.y as f64 - 1.62,
                            head_y: camera.position.y as f64,
                            z: camera.position.z as f64,
                            yaw: camera.orientation.y,
                            pitch: camera.orientation.x,
                            on_ground: false,
                        })
                        .ok();
                }

                last_frame = Instant::now();
                camera_controller.update_camera(&mut camera, frame_delta);
                camera_uniform.update_view_proj(&mut camera);
                queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));

                let output = surface.get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                platform
                    .prepare_frame(imgui_ctx.io_mut(), &window)
                    .expect("Failed to prepare imgui frame");

                let ui = imgui_ctx.frame();

                imgui::Window::new("Debug information")
                    .collapsible(false)
                    .resizable(false)
                    .movable(false)
                    .title_bar(false)
                    .position([0., 0.], imgui::Condition::Always)
                    .size([300., 200.], imgui::Condition::Always)
                    .build(&ui, || {
                        ui.text(format!("Nautilus {}", build_info::CRATE_VERSION));
                        ui.text(format!(
                            "XYZ: {:.3} / {:.5} / {:.3}",
                            camera.position.x, camera.position.y, camera.position.z
                        ));
                        ui.text(format!(
                            "Chunk: {} / {} / {}",
                            (camera.position.x / 16.0) as i32,
                            (camera.position.y / 16.0) as i32,
                            (camera.position.z / 16.0) as i32
                        ));
                        ui.separator();
                        ui.label_text(
                            "Chunks rendered",
                            format!("{}/{}", chunks_rendered, total_chunks),
                        );
                        ui.text(format!("{} chunks waiting for meshing", dirty_chunk_count))
                    });

                imgui::Window::new("Settings").build(&ui, || {
                    imgui::Slider::new("Render distance", 2, 64).build(&ui, &mut render_distance);
                    imgui::Slider::new("FOV", 30., 110.).build(&ui, &mut camera.fovy);
                });

                imgui::Window::new("System information").build(&ui, || {
                    ui.text(format!("Rust: {}", build_info::RUSTC_VERSION));
                    ui.separator();
                    ui.text(format!("CPU: {}", cpu_brand));
                    ui.separator();
                    ui.text(format!(
                        "Display: {}x{} ({:04x})",
                        surface_config.width, surface_config.height, adapter_info.vendor
                    ));
                    ui.text(format!(
                        "{} on {:?}",
                        &adapter_info.name, adapter_info.backend
                    ));
                    // TODO: Upgrade to wgpu 0.14 so we can use this, the imgui integration currently depends on 0.13
                    // ui.text(&adapter_info.driver_info);
                });

                imgui::Window::new("Entities").build(&ui, || {
                    for (e, pos) in world.query::<&Position>().iter() {
                        ui.text(format!("{:?} - {:?}", e, pos));
                    }
                });

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.541,
                                    g: 0.675,
                                    b: 1.000,
                                    // r: 0.20,
                                    // g: 0.031,
                                    // b: 0.031,
                                    a: 1.000,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &depth_texture.view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: None,
                        }),
                    });

                    render_pass.set_pipeline(&chunk_pipeline);
                    render_pass.set_bind_group(0, &camera_bind_group, &[]);
                    render_pass.set_bind_group(1, &texture_bind_group, &[]);

                    chunks_rendered = 0;
                    total_chunks = 0;
                    for (_, c) in chunks.chunks.iter() {
                        for section in &c.sections {
                            if let Some(s) = section {
                                if let Some(cr) = &s.renderdata {
                                    let _center = Vec3::new(
                                        (cr.position.0 * 16 + 8) as f32,
                                        (cr.position.1 * 16) as f32 + 8.,
                                        (cr.position.2 * 16 + 8) as f32,
                                    );

                                    if cr.position.1 >= 16 {
                                        warn!("chunk section {:?} has invalid Y", cr.position);
                                        continue;
                                    }

                                    let chunkpos_real = Vec3::new(
                                        (cr.position.0 * 16) as f32,
                                        (cr.position.1 * 16) as f32,
                                        (cr.position.2 * 16) as f32,
                                    );

                                    let chunk_transform = Mat4::from_translation(chunkpos_real);

                                    if chunkpos_real.distance(camera.position)
                                        < (render_distance as f32 * 2. * 16.)
                                    {
                                        if camera.is_in_frustrum(CHUNK_AABB, chunk_transform) {
                                            ChunkRenderer::render(
                                                &mut render_pass,
                                                cr,
                                                camera.position,
                                                render_distance as u32,
                                            );
                                            chunks_rendered += 1;
                                        }
                                    }
                                    total_chunks += 1;
                                }
                            }
                        }
                    }

                    render_pass.set_pipeline(&debugcube_pipeline);
                    render_pass.set_bind_group(0, &camera_bind_group, &[]);
                    render_pass.set_bind_group(1, &texture_bind_group_debugcube, &[]);
                    for (_, position) in world.query::<&Position>().iter() {
                        debugcube.render(&mut render_pass, position.0);
                    }

                    if chunklines_shown {
                        render_pass.set_pipeline(&debuglines_pipeline);
                        render_pass.set_bind_group(0, &camera_bind_group, &[]);
                        let camera_chunk = (
                            (camera.position.x / 16.).floor() as i32,
                            (camera.position.z / 16.).floor() as i32,
                        );
                        debuglines.render(&mut render_pass, camera_chunk);
                    }
                }

                {
                    let mut imgui_render_pass =
                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("dear imgui Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });

                    imgui_renderer
                        .render(ui.render(), &queue, &device, &mut imgui_render_pass)
                        .expect("Rendering failed");
                }

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
                frame_count += 1;
                profiling::finish_frame!();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }

        platform.handle_event(imgui_ctx.io_mut(), &window, &event);
    })
}
