use glam::{Mat4, Vec2, Vec3, Vec4};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
]);

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Default)]
pub struct Camera {
    up: Vec3,
    front: Vec3,
    right: Vec3,
    // pub eye: glam::Vec3,
    // pub target: glam::Vec3,
    // pub up: glam::Vec3,
    pub position: Vec3,
    pub orientation: Vec2,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub vp: Mat4,
}

fn within(min: f32, value: f32, max: f32) -> bool {
    min <= value && value <= max
}

impl Camera {
    pub fn new() -> Self {
        Self {
            up: Vec3::Y,
            front: Vec3::Z,
            right: Vec3::X,
            position: Vec3::new(136.30138437834088, 128.000000181198082, 301.54814082141263),
            // position: Vec3::new(22., 77., 40.),
            // position: Vec3::default(),
            orientation: Vec2::default(),
            aspect: 800.0 / 600.0,
            fovy: 80.0,
            znear: 0.1,
            zfar: 1000.0,
            ..Default::default()
        }
    }

    fn update_vectors(&mut self) {
        let mut front = Vec3::default();
        front.x = -self.orientation.x.to_radians().cos() * self.orientation.y.to_radians().sin();
        front.y = -self.orientation.x.to_radians().sin();
        front.z = self.orientation.x.to_radians().cos() * self.orientation.y.to_radians().cos();
        self.front = front.normalize();
        self.right = self.front.cross(Vec3::Y).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    fn build_view_projection_matrix(&mut self) -> glam::Mat4 {
        self.update_vectors();
        let view = Mat4::look_at_rh(self.position, self.position + self.front, self.up);
        let proj =
            glam::Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
        self.vp = proj * view;

        self.vp
    }

    pub fn is_in_frustrum(&self, aabb: AABB, transform: Mat4) -> bool {
        let mvp = self.vp * transform;
        let corners = [
            mvp * Vec4::new(aabb.min.x, aabb.min.y, aabb.min.z, 1.0), // x y z
            mvp * Vec4::new(aabb.max.x, aabb.min.y, aabb.min.z, 1.0), // X y z
            mvp * Vec4::new(aabb.min.x, aabb.max.y, aabb.min.z, 1.0), // x Y z
            mvp * Vec4::new(aabb.max.x, aabb.max.y, aabb.min.z, 1.0), // X Y z
            mvp * Vec4::new(aabb.min.x, aabb.min.y, aabb.max.z, 1.0), // x y Z
            mvp * Vec4::new(aabb.max.x, aabb.min.y, aabb.max.z, 1.0), // X y Z
            mvp * Vec4::new(aabb.min.x, aabb.max.y, aabb.max.z, 1.0), // x Y Z
            mvp * Vec4::new(aabb.max.x, aabb.max.y, aabb.max.z, 1.0), // X Y Z
        ];

        for ct in corners {
            if within(-ct.w, ct.x, ct.w) && within(-ct.w, ct.y, ct.w) && within(0., ct.z, ct.w) {
                return true;
            }
        }

        return false;
    }
}

// TODO: Replace this with something better yea?
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &mut Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_left_arrow_pressed: bool,
    is_right_arrow_pressed: bool,
    is_up_arrow_pressed: bool,
    is_down_arrow_pressed: bool,
    is_shift_pressed: bool,
    boost: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_left_arrow_pressed: false,
            is_right_arrow_pressed: false,
            is_up_arrow_pressed: false,
            is_down_arrow_pressed: false,
            is_shift_pressed: false,
            boost: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.is_left_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.is_right_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Up => {
                        self.is_up_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.is_down_arrow_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift | VirtualKeyCode::RShift => {
                        self.is_shift_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LControl => {
                        self.boost = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, delta: f32) {
        let velocity = (delta * self.speed) * if self.is_shift_pressed { 2. } else { 1. };
        if self.is_forward_pressed {
            camera.position += camera.front * velocity;
        }
        if self.is_backward_pressed {
            camera.position -= camera.front * velocity;
        }

        if self.is_right_pressed {
            camera.position += camera.right * velocity;
        }
        if self.is_left_pressed {
            camera.position -= camera.right * velocity;
        }

        if self.is_right_arrow_pressed {
            camera.orientation.y += 16.0 * velocity;
        }
        if self.is_left_arrow_pressed {
            camera.orientation.y -= 16.0 * velocity;
        }

        if self.is_up_arrow_pressed {
            camera.orientation.x -= 16.0 * velocity;
        }
        if self.is_down_arrow_pressed {
            camera.orientation.x += 16.0 * velocity;
        }

        if self.boost {
            camera.position += camera.front * 50.0;
            self.boost = false;
        }

        // println!(
        //     "{:.2} {:.2} {:.2}",
        //     camera.position.x, camera.position.y, camera.position.z
        // );
    }
}
