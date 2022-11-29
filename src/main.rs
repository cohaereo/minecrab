extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

use cgmath::{Euler, MetricSpace, Point3, Quaternion, Vector2, Vector3};
use flate2::read::ZlibDecoder;
use rand::Rng;
use tokio::net::TcpStream;

use std::{
    io::{Cursor, Read},
    sync::Arc,
    time::Instant,
};

use clap::Parser;
use imgui::FontGlyphRanges;
use wgpu::util::DeviceExt;
use world::ChunkManager;

use crate::{
    audio::AudioManager,
    ecs::{update_interpolation, update_velocity, InterpolatedPosition, Position, Velocity},
    net::{connection::ClientConnection, ConnectionState, ProtocolVersion},
    render::{
        chunk::ChunkRenderer,
        chunk_debug::DebugLineRenderer,
        chunk_mesher::{chunk_mesher_thread, ChunkMeshingRequest, ChunkSectionContext},
        debug_cube::DebugCubeRenderer,
        texture,
        util::{Camera, CameraController, CameraUniform},
    },
};

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod audio;
mod ecs;
mod fixed_point;
mod net;
mod render;
mod varint;
mod world;

const ICON_MIN_FA: u32 = 0xe005;
const ICON_MAX_FA: u32 = 0xf8ff;

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
    pretty_env_logger::init();

    let args = CliArgs::parse();

    let _client = tracy_client::Client::start();

    let mut chunks = ChunkManager::new();

    let stream = TcpStream::connect(format!("{}:{}", args.address, args.port)).await?;
    let mut connection = ClientConnection::from_stream(stream, ProtocolVersion::Proto1_7_6);

    connection.write(&net::packets::Packet::SetProtocol(
        net::packets::handshaking::serverbound::SetProtocol {
            protocol_version: varint::VarInt(5),
            server_host: args.address,
            server_port: args.port,
            next_state: varint::VarInt(2),
        },
    ))?;

    connection.write(&net::packets::Packet::LoginStart(
        net::packets::login::serverbound::LoginStart {
            username: args.username,
        },
    ))?;

    // Wait for login success
    while connection.state != ConnectionState::Play {
        connection.read();
    }

    let mut camera = Camera::new();
    camera.aspect = 1600 as f32 / 900 as f32;

    // Wait for player pos

    'w: loop {
        match connection.read() {
            Some(net::packets::Packet::PositionClientbound(p)) => {
                camera.position = Point3::new(p.x as f32, p.y as f32, p.z as f32);
                camera.orientation = Vector2::new(p.pitch, p.yaw);
                connection.write(&net::packets::Packet::PositionLook(
                    net::packets::play::serverbound::PositionLook {
                        x: p.x,
                        stance: p.y - 1.62,
                        y: p.y,
                        z: p.z,
                        yaw: p.yaw,
                        pitch: p.pitch,
                        on_ground: p.on_ground,
                    },
                ))?;

                connection.write(&net::packets::Packet::ClientCommand(
                    net::packets::play::serverbound::ClientCommand { payload: 0 },
                ))?;
                break 'w;
            }
            _ => {}
        }
    }

    #[cfg(target_os = "linux")]
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1600, 900))
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    info!("Available devices:");
    for b in instance.enumerate_adapters(wgpu::Backends::PRIMARY) {
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
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

    let (device, queue) = futures::executor::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::PUSH_CONSTANTS,
            limits: wgpu::Limits {
                max_push_constant_size: 32,
                ..Default::default()
            },
            label: None,
        },
        None,
    ))
    .unwrap();

    let device = Arc::new(device);
    let queue = Arc::new(queue);

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

    // const CHUNK_AABB: AABB = AABB::new(Vector3::splat(0.), Vector3::splat(16.));

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

    let (chunkmesher_send, mut chunkmesher_recv) =
        chunk_mesher_thread(device.clone(), queue.clone());

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

    let mut audio_manager = AudioManager::new();
    let mut cursor_grabbed = false;
    let mut frame_count = 0;
    let mut last_frame = Instant::now();
    let mut chunks_rendered = 0;
    let mut total_chunks = 0;
    let mut render_distance = 16;
    let mut chunklines_shown = false;
    let mut chatmsg_buf = String::new();
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    if cursor_grabbed {
                        camera_controller.process_mouse(&mut camera, *delta);
                    }
                }
                _ => {}
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !imgui_ctx.io().want_capture_keyboard {
                    camera_controller.process_events(event);
                }

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
                                VirtualKeyCode::F1 => {
                                    if input.state == ElementState::Pressed {
                                        cursor_grabbed = !cursor_grabbed;

                                        window.set_cursor_grab(cursor_grabbed).ok();
                                        window.set_cursor_visible(!cursor_grabbed);
                                    }
                                }
                                VirtualKeyCode::F4 => {
                                    if input.state == ElementState::Pressed {
                                        chunklines_shown = !chunklines_shown;
                                    }
                                }
                                VirtualKeyCode::F6 => {
                                    if input.state == ElementState::Pressed {
                                        chunks.chunks.iter_mut().for_each(|c| {
                                            c.1.sections.iter_mut().for_each(|cs| {
                                                if let Some(cs) = cs {
                                                    cs.dirty = true;
                                                }
                                            })
                                        });
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
                update_interpolation(&mut world, frame_delta);

                audio_manager.maintain();
                audio_manager.set_listener_transform(
                    camera.position,
                    Quaternion::from(Euler {
                        x: cgmath::Deg(0.),
                        y: cgmath::Deg(180.0 - camera.orientation.y),
                        z: cgmath::Deg(0.),
                    }),
                );

                // * Receive chunks
                // ! Yes i know doing this just before rendering a frame isn't great but it doesnt need to be yet.
                let mut packet_quota = 256;
                loop {
                    if let Some(p) = connection.read() {
                        match p {
                            net::packets::Packet::MapChunkBulk(p) => {
                                let mut data_offset = 0;
                                let mut c = Cursor::new(&p.data);
                                let mut z = ZlibDecoder::new(&mut c);

                                let mut data = Vec::new();
                                if z.read_to_end(&mut data).is_err() {
                                    warn!("Chunk data failed to decompress");
                                    continue;
                                }

                                for (_i, cm) in p.meta.iter().enumerate() {
                                    let bytes_read = chunks
                                        .load_chunk(
                                            (cm.chunk_x, cm.chunk_z),
                                            cm.primary_bitmap,
                                            cm.add_bitmap,
                                            p.sky_light_sent,
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
                            net::packets::Packet::MapChunk(p) => {
                                let mut c = Cursor::new(&p.compressed_chunk_data.data);
                                let mut z = ZlibDecoder::new(&mut c);

                                let mut data = Vec::new();
                                if z.read_to_end(&mut data).is_err() {
                                    warn!("Chunk data failed to decompress");
                                    continue;
                                }

                                chunks
                                    .load_chunk(
                                        (p.x, p.z),
                                        p.bit_map,
                                        p.add_bit_map,
                                        false,
                                        p.ground_up,
                                        &data,
                                    )
                                    .unwrap();
                            }
                            net::packets::Packet::Respawn { .. } => {
                                chunks.chunks.clear();

                                // Shrink to reclaim memory
                                chunks.chunks.shrink_to_fit();
                            }
                            net::packets::Packet::BlockChange(p) => {
                                chunks.set_block(
                                    p.location.x,
                                    p.location.y as i32,
                                    p.location.z,
                                    p.kind.0 as u8,
                                );
                            }
                            net::packets::Packet::MultiBlockChange(p) => {
                                for r in p.records {
                                    chunks.set_block(
                                        p.chunk_x * 16 + r.x as i32,
                                        r.y as i32,
                                        p.chunk_z * 16 + r.z as i32,
                                        r.block_id as u8,
                                    );
                                }
                            }
                            net::packets::Packet::EntityMoveLook(p) => {
                                let ent = ecs::get_or_insert(&mut world, p.entity_id);
                                if let Ok((pos, interp)) = world.query_one_mut::<(
                                    &mut Position,
                                    &mut InterpolatedPosition,
                                )>(
                                    ent
                                ) {
                                    pos.0 += Vector3::new(
                                        p.d_x.0 as f32,
                                        p.d_y.0 as f32,
                                        p.d_z.0 as f32,
                                    );

                                    interp.delta = ecs::TICK_DELTA
                                }
                            }
                            net::packets::Packet::RelEntityMove(p) => {
                                let ent = ecs::get_or_insert(&mut world, p.entity_id);
                                if let Ok((pos, interp)) = world.query_one_mut::<(
                                    &mut Position,
                                    &mut InterpolatedPosition,
                                )>(
                                    ent
                                ) {
                                    pos.0 += Vector3::new(
                                        p.d_x.0 as f32,
                                        p.d_y.0 as f32,
                                        p.d_z.0 as f32,
                                    );

                                    interp.delta = ecs::TICK_DELTA
                                }
                            }
                            net::packets::Packet::EntityVelocity(p) => {
                                let ent = ecs::get_or_insert(&mut world, p.entity_id);
                                if let Ok((_pos, v)) =
                                    world.query_one_mut::<(&mut Position, &mut Velocity)>(ent)
                                {
                                    v.0 = Vector3::new(
                                        p.velocity_x as f32,
                                        p.velocity_y as f32,
                                        p.velocity_z as f32,
                                    ) * ecs::VELOCITY_UNIT;
                                }
                            }
                            net::packets::Packet::EntityTeleport(p) => {
                                let ent = ecs::get_or_insert(&mut world, p.entity_id);
                                if let Ok((pos, interp)) = world.query_one_mut::<(
                                    &mut Position,
                                    &mut InterpolatedPosition,
                                )>(
                                    ent
                                ) {
                                    pos.0 = Point3::new(p.x.0 as f32, p.y.0 as f32, p.z.0 as f32);

                                    interp.delta = ecs::TICK_DELTA / 5.;
                                }
                            }
                            net::packets::Packet::SpawnEntityLiving(p) => {
                                let ent = ecs::get_or_insert(&mut world, p.entity_id.0);
                                if let Ok((pos, v)) =
                                    world.query_one_mut::<(&mut Position, &mut Velocity)>(ent)
                                {
                                    pos.0 = Point3::new(p.x.0 as f32, p.y.0 as f32, p.z.0 as f32);
                                    v.0 = Vector3::new(
                                        p.velocity_x as f32,
                                        p.velocity_y as f32,
                                        p.velocity_z as f32,
                                    ) * ecs::VELOCITY_UNIT;
                                }
                            }
                            net::packets::Packet::SpawnEntity(p) => {
                                let ent = ecs::get_or_insert(&mut world, p.entity_id.0);
                                if let Ok(pos) = world.query_one_mut::<&mut Position>(ent) {
                                    pos.0 = Point3::new(p.x.0 as f32, p.y.0 as f32, p.z.0 as f32);
                                }
                            }
                            net::packets::Packet::NamedEntitySpawn(p) => {
                                let ent = ecs::get_or_insert(&mut world, p.entity_id.0);
                                if let Ok(pos) = world.query_one_mut::<&mut Position>(ent) {
                                    pos.0 = Point3::new(p.x.0 as f32, p.y.0 as f32, p.z.0 as f32);
                                }
                            }
                            net::packets::Packet::EntityDestroy(p) => {
                                for e in p.entity_ids.data {
                                    let eid = ecs::get_or_insert(&mut world, e);
                                    world.despawn(eid).ok();
                                }
                            }
                            net::packets::Packet::NamedSoundEffect(p) => {
                                let path_glob = format!(
                                    "assets/minecraft/sounds/{}*.ogg",
                                    p.sound_name.replace('.', "/")
                                );
                                if let Ok(paths) = glob::glob(&path_glob) {
                                    let paths: Vec<std::path::PathBuf> =
                                        paths.filter(|p| p.is_ok()).map(|p| p.unwrap()).collect();
                                    if paths.len() > 0 {
                                        let r = rand::thread_rng().gen::<usize>();
                                        let path = paths[r % paths.len()]
                                            .clone()
                                            .into_os_string()
                                            .into_string()
                                            .unwrap();
                                        audio_manager
                                            .play(
                                                &path,
                                                [p.x as f32 / 8., p.y as f32 / 8., p.z as f32 / 8.]
                                                    .into(),
                                                p.volume,
                                                p.pitch as f32 / 63.,
                                            )
                                            .ok();
                                    }
                                };
                            }
                            net::packets::Packet::PositionClientbound(p) => {
                                camera.position = Point3::new(p.x as f32, p.y as f32, p.z as f32);
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

                let mut dirty_chunks = vec![];
                if dirty_chunk_count != 0 {
                    let mut chunk_meshing_quota = 8;
                    for (coord, chunk) in chunks.chunks.iter_mut() {
                        for cy in 0..16 {
                            if let Some(cd) = chunk.get_section_mut(cy) {
                                if cd.dirty {
                                    dirty_chunks.push((coord.0, cy, coord.1));

                                    chunk_meshing_quota -= 1;
                                }
                            }
                        }

                        if chunk_meshing_quota == 0 {
                            break;
                        }
                    }
                }

                // dirty_chunks.dedup();

                for c in &mut dirty_chunks {
                    let data = ChunkSectionContext::new(&chunks, Point3::new(c.0, c.1 as i32, c.2));

                    if let Some(cd) = chunks
                        .get_mut(&(c.0, c.2))
                        .and_then(|cc| cc.get_section_mut(c.1))
                    {
                        if chunkmesher_send
                            .try_send(ChunkMeshingRequest {
                                chunk_pos: Point3::new(c.0, c.1 as i32, c.2),
                                data,
                                buffers: None,
                            })
                            .is_ok()
                        {
                            cd.dirty = false;
                        }
                    }
                }

                // Get finished chunks from the chunk mesher thread
                let mut count = 0;
                while let Ok(rd) = chunkmesher_recv.try_recv() {
                    if let Some(cd) = chunks
                        .get_mut(&(rd.position.x, rd.position.z))
                        .and_then(|cc| cc.get_section_mut(rd.position.y as u8))
                    {
                        count += 1;
                        cd.renderdata = Some(rd);
                    }
                }

                if count != 0 {
                    println!("Received {} chunks", count);
                }

                chunks.chunks.retain(|c, _| {
                    let chunkpos_real = Vector2::new(c.0 as f32, c.1 as f32) * 16.;

                    // Unload chunk when it goes out of range (1.25 * render distance)
                    // ! currently uses hardcoded distance of 24 until i figure out a better way to determine unload distance (may cause problems if the server has a higher render distance)
                    if chunkpos_real.distance(camera.position.to_homogeneous().xz())
                        > (24. * 2. * 16.)
                    {
                        false
                    } else {
                        true
                    }
                });

                // Send player position every tick
                // FIXME: If you dont have vsync enabled (or a 60hz monitor) this is going to hurt
                if frame_count % 3 == 0 {
                    connection
                        .write(&net::packets::Packet::PositionLook(
                            net::packets::play::serverbound::PositionLook {
                                x: camera.position.x as f64,
                                stance: camera.position.y as f64 - 1.62,
                                y: camera.position.y as f64,
                                z: camera.position.z as f64,
                                yaw: camera.orientation.y,
                                pitch: camera.orientation.x,
                                on_ground: false,
                            },
                        ))
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
                        ui.text(format!(
                            "{} chunks waiting to be submitted for meshing",
                            dirty_chunk_count
                        ));
                        ui.separator();
                        ui.text(format!(
                            "Press F1 to {} cursor",
                            if cursor_grabbed { "unlock" } else { "lock" }
                        ));
                        ui.text(format!(
                            "Press F4 to {} chunk borders",
                            if chunklines_shown { "hide" } else { "show" }
                        ));
                        ui.text(format!("Press F6 to reload chunks"));
                    });

                imgui::Window::new("Settings").build(&ui, || {
                    imgui::Slider::new("Render distance", 2, 64).build(&ui, &mut render_distance);
                    imgui::Slider::new("FOV", 30., 110.).build(&ui, &mut camera.fovy);
                });

                imgui::Window::new("Chat").build(&ui, || {
                    let enter_hit = ui
                        .input_text("Message", &mut chatmsg_buf)
                        .enter_returns_true(true)
                        .build();

                    if enter_hit || ui.button("Send") {
                        connection
                            .write(&net::packets::Packet::ChatServerbound(
                                net::packets::play::serverbound::ChatServerbound {
                                    message: chatmsg_buf.clone(),
                                },
                            ))
                            .ok();
                        chatmsg_buf.clear();
                    }
                });

                imgui::Window::new("Entities").build(&ui, || {
                    for (e, pos) in world.query::<&Position>().iter() {
                        ui.text(format!("{:?} - {:?}", e, pos));
                    }
                });

                // if thread_net_recv.is_finished() {
                //     *control_flow = ControlFlow::Exit;
                // }

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
                                    let _center = Vector3::new(
                                        (cr.position.x * 16 + 8) as f32,
                                        (cr.position.y * 16) as f32 + 8.,
                                        (cr.position.z * 16 + 8) as f32,
                                    );

                                    let chunkpos_real = Vector3::new(
                                        cr.position.x as f32,
                                        cr.position.y as f32,
                                        cr.position.z as f32,
                                    ) * 16.;

                                    if chunkpos_real
                                        .distance(camera.position.to_homogeneous().xyz())
                                        < (render_distance as f32 * 2. * 16.)
                                    {
                                        let aabb = collision::Aabb3 {
                                            min: cgmath::Point3::new(
                                                chunkpos_real.x,
                                                chunkpos_real.y,
                                                chunkpos_real.z,
                                            ),
                                            max: cgmath::Point3::new(
                                                chunkpos_real.x + 16.,
                                                chunkpos_real.y + 16.,
                                                chunkpos_real.z + 16.,
                                            ),
                                        };
                                        if camera.is_in_frustrum(&aabb) {
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
                    for (_, position) in world.query::<&InterpolatedPosition>().iter() {
                        debugcube.render(
                            &mut render_pass,
                            position.position + Vector3::new(0., 0.5, 0.),
                        );
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
