use glam::{Mat4, Vec2};

pub struct Camera {
    pub position: Vec2,
    pub target_position: Vec2,

    pub rotation: f32,
    pub target_rotation: f32,

    pub zoom: f32,
    pub target_zoom: f32,

    pub width: f32,
    pub height: f32,
    
    pub smoothness: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            target_position: Vec2::ZERO,

            rotation: 0.0,
            target_rotation: 0.0,

            zoom: 1.0,
            target_zoom: 1.0,

            width,
            height,

            smoothness: 8.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec2::splat(self.zoom).extend(1.0),
            glam::Quat::from_rotation_z(-self.rotation),
            (-self.position).extend(1.0),
        )
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let half_width = self.width * 0.5;
        let half_height = self.height * 0.5;

        Mat4::orthographic_rh(
            -half_width,
            half_width,
            -half_height,
            half_height,
            -100.0,
            100.0,
        )
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() *
            self.view_matrix()
    }

    pub fn set_position(
        &mut self,
        position: [f32; 2],
    ) {
        self.target_position = position.into();
    }

    pub fn update(&mut self, dt: f32) {
        let t = 1.0 - (-self.smoothness * dt).exp();

        self.position = self.position.lerp(
            self.target_position,
            t,
        );

        self.zoom +=
            (self.target_zoom - self.zoom) * t;

        let rotation_delta =
            (self.target_rotation - self.rotation + std::f32::consts::PI)
                .rem_euclid(std::f32::consts::TAU)
                - std::f32::consts::PI;

        self.rotation += rotation_delta * t;
    }

    pub fn position(&self) -> [f32; 2] {
        self.position.into()
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.target_rotation = rotation;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.target_zoom = zoom.clamp(0.25, 4.0);
    }

    pub fn resize(
        &mut self,
        width: f32,
        height: f32,
    ) {
        self.width = width;
        self.height = height;
    }
}