mod engine;

use engine::{App, Game};

struct MyGame {
    position: f32,
    velocity: f32,
}

impl MyGame {
    fn new() -> Self {
        Self {
            position: 0.0,
            velocity: 100.0,
        }
    }
}

impl Game for MyGame {
    fn fixed_update(&mut self, dt: f32) {
        self.position += self.velocity * dt;

        println!(
            "Position: {:.2}",
            self.position
        );
    }

    fn update(&mut self, _dt: f32) {
        // General game logic.
    }

    fn render(&mut self) {
        // Renderer nanti masuk di Phase 2.
    }
}

fn main() {
    let game = MyGame::new();

    let app = App::new(
        "Runa Engine",
        1280,
        720,
        game,
    );

    app.run();
}