use cgmath::{AbsDiffEq, One};
use collision::Frustum;
use glam::{Mat4, Vec2, Vec3, Vec4};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub struct Camera {
    up: Vec3,
    front: Vec3,
    right: Vec3,
    pub position: Vec3,
    pub orientation: Vec2,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub vp: Mat4,
    pub fov_scale: f32,
    vp_cg: cgmath::Matrix4<f32>,
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
            position: Vec3::new(0., 0., 0.),
            orientation: Vec2::default(),
            aspect: 1280.0 / 720.0,
            fovy: 80.0,
            znear: 0.1,
            zfar: 1000.0,
            vp: Mat4::IDENTITY,
            fov_scale: 1.0,
            vp_cg: cgmath::Matrix4::one(),
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
        let proj = glam::Mat4::perspective_rh(
            self.fovy.to_radians() * self.fov_scale,
            self.aspect,
            self.znear,
            self.zfar,
        );
        self.vp = proj * view;

        let c0 = self.vp.col(0);
        let c1 = self.vp.col(1);
        let c2 = self.vp.col(2);
        let c3 = self.vp.col(3);

        self.vp_cg = cgmath::Matrix4::from_cols(
            cgmath::Vector4::new(c0.x, c0.y, c0.z, c0.w),
            cgmath::Vector4::new(c1.x, c1.y, c1.z, c1.w),
            cgmath::Vector4::new(c2.x, c2.y, c2.z, c2.w),
            cgmath::Vector4::new(c3.x, c3.y, c3.z, c3.w),
        );

        self.vp
    }

    pub fn is_in_frustrum(&self, aabb: &collision::Aabb3<f32>) -> bool {
        // TODO: use cgmath for the whole project instead of glam
        let frust = Frustum::from_matrix4(self.vp_cg).unwrap();
        frust.contains(aabb) != collision::Relation::Out
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
    is_shift_pressed: bool,
    is_zoomed: bool,
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
            is_shift_pressed: false,
            is_zoomed: false,
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
                    VirtualKeyCode::C => {
                        self.is_zoomed = is_pressed;
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

    pub fn process_mouse(&self, camera: &mut Camera, delta: (f64, f64)) {
        let mut offset = Vec2::new(delta.1 as f32 * 0.8, delta.0 as f32) * 0.35;
        if self.is_zoomed {
            offset *= 0.25;
        }

        camera.orientation += offset;
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

        if self.boost {
            camera.position += camera.front * 50.0;
            self.boost = false;
        }

        camera.fov_scale = if self.is_zoomed { 0.25 } else { 1.0 };

        camera.orientation.x = camera.orientation.x.clamp(-89.9, 89.9)
    }
}
