use glam::{Mat4, Vec2};

pub struct Camera {
    pub position: Vec2,
    pub rotation: f32,
    pub zoom: f32,

    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
            width,
            height,
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
        self.position = position.into();
    }

    pub fn position(&self) -> [f32; 2] {
        self.position.into()
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.01);
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