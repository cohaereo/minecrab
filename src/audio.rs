use cgmath::{Point3, Quaternion, Vector3, Zero};
use cpal::traits::{DeviceTrait, HostTrait};

type AudioHandle =
    oddio::Handle<oddio::SpatialBuffered<oddio::Stop<oddio::Gain<oddio::FramesSignal<f32>>>>>;

pub struct ClipInstance {
    position: Point3<f32>,
    handle: AudioHandle,
}

pub struct AudioManager {
    device: cpal::Device,
    stream: cpal::Stream,
    scene_handle: oddio::Handle<oddio::SpatialScene>,
    camera_pos: Point3<f32>,
    camera_ori: Quaternion<f32>,

    instances: Vec<ClipInstance>,
}

impl AudioManager {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let sample_rate = device.default_output_config().unwrap().sample_rate();
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };
        let (scene_handle, scene) = oddio::split(oddio::SpatialScene::new());
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let frames = oddio::frame_stereo(data);
                    oddio::run(&scene, sample_rate.0, frames);
                },
                move |err| {
                    eprintln!("{}", err);
                },
            )
            .unwrap();
        Self {
            device,
            stream,
            scene_handle,
            camera_pos: Point3::new(0., 0., 0.),
            camera_ori: Quaternion::zero(),
            instances: vec![],
        }
    }

    // FIXME: Sound pitch doesn't seem to work 100% right (eg. levers being very low pitched, they use the click sound at ~50% pitch)
    // TODO: Caching sounds
    pub fn play(
        &mut self,
        path: &str,
        position: Point3<f32>,
        volume: f32,
        pitch: f32,
    ) -> anyhow::Result<()> {
        let f = std::fs::File::open(path)?;
        let mut ogg = lewton::inside_ogg::OggStreamReader::new(f)?;
        let mut samples: Vec<i16> = vec![];
        while let Ok(Some(pck_samples)) = ogg.read_dec_packet_itl() {
            samples.extend(pck_samples);
        }

        let adjusted_samplerate = (ogg.ident_hdr.audio_sample_rate as f32 * pitch) as u32;
        let frames = oddio::Frames::from_iter(
            adjusted_samplerate,
            samples.iter().map(|s| *s as f32 / 32767.),
        );

        let basic_signal: oddio::FramesSignal<_> = oddio::FramesSignal::from(frames);
        let speed = oddio::Gain::new(basic_signal);
        let pos: Vector3<f32> = position - self.camera_pos;
        let mut signal = self.scene_handle.control().play_buffered(
            speed,
            oddio::SpatialOptions {
                position: Point3::new(pos.x, pos.y, pos.z).into(),
                radius: 1.0,
                ..Default::default()
            },
            100.,
            adjusted_samplerate,
            samples.len() as f32 / adjusted_samplerate as f32,
        );

        signal
            .control::<oddio::Gain<_>, _>()
            .set_amplitude_ratio(volume);

        self.instances.push(ClipInstance {
            position,
            handle: signal,
        });

        Ok(())
    }

    pub fn maintain(&mut self) {
        // self.instances.retain(|c| c.handle.control::<oddio::Spatial<_>, _)
    }

    pub fn set_listener_transform(&mut self, position: Point3<f32>, orientation: Quaternion<f32>) {
        self.scene_handle
            .control()
            .set_listener_rotation(orientation.into());

        for clip in &mut self.instances {
            let mut spatial_control = clip.handle.control::<oddio::SpatialBuffered<_>, _>();
            let newpos: Vector3<f32> = clip.position - position;
            spatial_control.set_motion(
                Point3::new(newpos.x, newpos.y, newpos.z).into(),
                Vector3::zero().into(),
                false,
            )
        }

        self.camera_pos = position;
        self.camera_ori = orientation;
    }
}
