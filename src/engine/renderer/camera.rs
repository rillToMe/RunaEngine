#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: [f32; 2],
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: [0.0, 0.0],
            width,
            height,
        }
    }

    pub fn projection_matrix(&self) -> [[f32; 4]; 4] {
        let left = self.position[0];
        let right = left + self.width;

        let top = self.position[1];
        let bottom = top + self.height;

        [
            [
                2.0 / (right - left),
                0.0,
                0.0,
                0.0,
            ],

            [
                0.0,
                2.0 / (top - bottom),
                0.0,
                0.0,
            ],

            [
                0.0,
                0.0,
                -1.0,
                0.0,
            ],

            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                0.0,
                1.0,
            ],
        ]
    }
}