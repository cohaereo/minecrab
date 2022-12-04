use cgmath::{InnerSpace, Matrix4, One, Point3, Vector2, Vector3, Zero};
use collision::Frustum;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Camera {
    pub up: Vector3<f32>,
    pub front: Vector3<f32>,
    pub right: Vector3<f32>,
    pub position: Point3<f32>,
    pub orientation: Vector2<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub vp: Matrix4<f32>,
    pub fov_scale: f32,
}

fn within(min: f32, value: f32, max: f32) -> bool {
    min <= value && value <= max
}

impl Camera {
    pub fn new() -> Self {
        Self {
            up: Vector3::unit_y(),
            front: Vector3::unit_z(),
            right: Vector3::unit_x(),
            position: Point3::new(0., 0., 0.),
            orientation: Vector2::zero(),
            aspect: 1280.0 / 720.0,
            fovy: 80.0,
            znear: 0.01,
            zfar: 1000.0,
            vp: Matrix4::one(),
            fov_scale: 1.0,
        }
    }

    fn update_vectors(&mut self) {
        let mut front = Vector3::zero();
        front.x = -self.orientation.x.to_radians().cos() * self.orientation.y.to_radians().sin();
        front.y = -self.orientation.x.to_radians().sin();
        front.z = self.orientation.x.to_radians().cos() * self.orientation.y.to_radians().cos();
        self.front = front.normalize();
        self.right = self.front.cross(Vector3::unit_y()).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    fn build_view_projection_matrix(&mut self) -> Matrix4<f32> {
        self.update_vectors();

        let view = Matrix4::look_at(self.position, self.position + self.front, self.up);
        let proj = cgmath::perspective(
            cgmath::Rad(self.fovy.to_radians() * self.fov_scale),
            self.aspect,
            self.znear,
            self.zfar,
        );
        self.vp = proj * view;

        self.vp
    }

    pub fn is_in_frustrum(&self, aabb: &collision::Aabb3<f32>) -> bool {
        let frust = Frustum::from_matrix4(self.vp).unwrap();
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
            view_proj: [[0.; 4]; 4],
        }
    }

    pub fn update_view_proj(&mut self, camera: &mut Camera) {
        self.view_proj = *camera.build_view_projection_matrix().as_ref();
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
    pub velocity: Vector3<f32>,
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
            velocity: Vector3::zero(),
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
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn process_mouse(&self, camera: &mut Camera, delta: (f64, f64)) {
        let mut offset = Vector2::new(delta.1 as f32 * 0.8, delta.0 as f32) * 0.15;
        if self.is_zoomed {
            offset *= 0.25;
        }

        camera.orientation += offset;
    }

    pub fn update_camera(&mut self, camera: &mut Camera, delta: f32) {
        let speed = (delta * self.speed) * if self.is_shift_pressed { 2. } else { 1. };
        self.velocity = Vector3::zero();
        if self.is_forward_pressed {
            self.velocity += camera.front * speed;
        }
        if self.is_backward_pressed {
            self.velocity -= camera.front * speed;
        }

        if self.is_right_pressed {
            self.velocity += camera.right * speed;
        }
        if self.is_left_pressed {
            self.velocity -= camera.right * speed;
        }

        // camera.position += self.velocity;

        camera.fov_scale = if self.is_zoomed { 0.25 } else { 1.0 };

        camera.orientation.x = camera.orientation.x.clamp(-89.9, 89.9)
    }
}
