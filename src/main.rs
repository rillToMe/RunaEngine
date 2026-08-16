mod engine;

use engine::{App, Game};
use crate::engine::input::{Action, Input,};
use winit::{event::MouseButton, keyboard::KeyCode};

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
    fn fixed_update(
        &mut self,
        _dt: f32,
        _input: &Input,
    ) {
    }

    fn update(
        &mut self,
        dt: f32,
        input: &Input,
    ) {

        if input.is_key_pressed(KeyCode::Space) {
        println!("SPACE PRESSED!");
        }

        if input.is_key_released(KeyCode::Space) {
            println!("SPACE RELEASED!");
        }

        if input.is_mouse_button_pressed(MouseButton::Left) {
            println!("Left Button Pressed");
        }

        if input.is_mouse_button_released(MouseButton::Left) {
            println!("Left Button Released")
        }

        if input.is_mouse_button_pressed(MouseButton::Right) {
            println!("Right Button Pressed");
        }

        if input.is_mouse_button_released(MouseButton::Right) {
            println!("Right Button Released")
        }

        if input.is_action_down(Action::MoveRight) {
            self.position += self.velocity * dt;
        }

        if input.is_action_down(Action::MoveLeft) {
            self.position -= self.velocity * dt;
        }

        if input.is_action_pressed(Action::Jump) {
            println!("JUMP!");
        }

        if input.is_action_pressed(Action::Attack) {
            println!("ATTACK!");
        }
    }

    fn render(&mut self) {
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