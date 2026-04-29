

use glam::{Vec3, Mat4};



pub struct Camera {

    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub rotation_sens: f32,
    pub zoom_sens: f32,
    pub pan_sens: f32,

}



impl Camera {

    pub fn new( size: winit::dpi::PhysicalSize<u32> ) -> Self {

        Self {

            eye: Vec3::new(0.0, 0.0, 0.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::Y,
            aspect: (size.width as f32/ size.height as f32),
            fov: 45.0_f32.to_radians(),
            znear:  1e-3,
            zfar: 1e3,
            yaw: 0.0,
            pitch: 0.0,
            distance: 5.0,
            rotation_sens: 0.005,
            zoom_sens: 0.5,
            pan_sens: 0.01,

        }

    }

    pub fn matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fov, self.aspect, self.znear, self.zfar);

        proj * view
    }

    pub fn update(&mut self) {

        let x = self.yaw.cos() * self.pitch.cos();
        let y = self.pitch.sin();
        let z = self.yaw.sin() * self.pitch.cos();

        let dir = Vec3::new(x, y, z).normalize();
        self.eye = self.target - ( dir * self.distance );

    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {

        self.yaw += dx * self.rotation_sens;
        self.pitch -= dy * self.rotation_sens;
        self.pitch = self.pitch.clamp(-1.5, 1.5);
        self.update();

    }

    pub fn zoom(&mut self, delta: f32) {

        self.distance -= delta * self.zoom_sens;
        self.distance = self.distance.max(0.01);
        self.update();
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {

        let forward = ( self.target - self.eye ).normalize();
        let right = forward.cross(self.up).normalize();
        let actual_up = right.cross(forward.normalize());
        let offset = ( right * -dx * self.pan_sens ) + ( actual_up * dy * self.pan_sens );
        self.eye += offset;
        self.target += offset;

    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.aspect = size.width as f32 /  size.height as f32;
    }

}

