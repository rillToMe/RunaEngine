use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent, MouseScrollDelta, MouseButton},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{Renderer, Time, input::Input};
use crate::engine::math::Transform;

pub trait Game {
    fn fixed_update(
        &mut self,
        dt: f32,
        input: &Input,
    );

    fn update(
        &mut self,
        dt: f32,
        input: &Input,
    );

    fn render(
        &mut self,
    );
}

pub struct App<G: Game> {
    title: String,
    width: u32,
    height: u32,

    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,

    time: Time,
    input: Input,

    target_frame_time: Duration,
    next_frame_time: Instant,


    camera_position: [f32; 2],
    camera_speed: f32,

    camera_zoom: f32,

    camera_rotation: f32,

    game: G,

}

impl<G: Game> App<G> {
    pub fn new(
        title: impl Into<String>,
        width: u32,
        height: u32,
        game: G,
    ) -> Self {
        let now = Instant::now();

        Self {
            title: title.into(),
            width,
            height,

            window: None,
            renderer: None,

            time: Time::new(),
            input: Input::new(),

            target_frame_time: Duration::from_secs_f64(1.0 / 60.0),
            next_frame_time: now,

            camera_position: [100.0, 250.0],
            camera_speed: 300.0,

            camera_zoom: 1.0,

            camera_rotation: 0.0,

            game,
        }
    }

    

    pub fn run(mut self) {
        let event_loop =
            EventLoop::new().expect("failed to create event loop");

        event_loop
            .run_app(&mut self)
            .expect("failed to run application");
    }
}

impl<G: Game> ApplicationHandler for App<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title(&self.title)
                    .with_inner_size(
                        winit::dpi::LogicalSize::new(
                            self.width,
                            self.height,
                        ),
                    ),
            )
            .expect("failed to create window");

        let window = Arc::new(window);

        let renderer =
            pollster::block_on(
                Renderer::new(window.clone())
            );

        self.renderer = Some(renderer);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                self.input.update_key(
                    key,
                    state,
                );
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,

                    MouseScrollDelta::PixelDelta(position) => {
                        position.y as f32 / 50.0
                    }
                };

                self.camera_zoom += scroll * 0.1;

                self.camera_zoom =
                    self.camera_zoom.clamp(0.25, 4.0);
            }

            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => {
                self.input.update_mouse_button(
                    button,
                    state,
                );
            }

            WindowEvent::CursorMoved {
                position,
                ..
            } => {
                self.input.set_mouse_position([
                    position.x as f32,
                    position.y as f32,
                ]);
            }

            WindowEvent::RedrawRequested => {
                self.time.update();

                const MAX_FIXED_STEPS: u32 = 5;

                let mut fixed_steps = 0;

                while self.time.consume_fixed_step()
                    && fixed_steps < MAX_FIXED_STEPS
                {
                    self.game.fixed_update(
                        Time::fixed_delta_seconds(),
                        &self.input,
                    );

                    fixed_steps += 1;
                }

                if fixed_steps == MAX_FIXED_STEPS {
                    self.time.reset_accumulator();
                }

                // Variable timestep update.
                self.game.update(
                    self.time.delta_seconds(),
                    &self.input,
                );

                // Camera movement.
                let dt = self.time.delta_seconds();

                if self.input.is_key_down(KeyCode::KeyW)
                    || self.input.is_key_down(KeyCode::ArrowUp)
                {
                    self.camera_position[1] -=
                        self.camera_speed * dt;
                }

                if self.input.is_key_down(KeyCode::KeyS)
                    || self.input.is_key_down(KeyCode::ArrowDown)
                {
                    self.camera_position[1] +=
                        self.camera_speed * dt;
                }

                if self.input.is_key_down(KeyCode::KeyA)
                    || self.input.is_key_down(KeyCode::ArrowLeft)
                {
                    self.camera_position[0] -=
                        self.camera_speed * dt;
                }

                if self.input.is_key_down(KeyCode::KeyD)
                    || self.input.is_key_down(KeyCode::ArrowRight)
                {
                    self.camera_position[0] +=
                        self.camera_speed * dt;
                }

                let zoom_speed = 1.0;

                if self.input.is_key_down(KeyCode::KeyE) {
                    self.camera_zoom +=
                        zoom_speed * dt;
                }

                if self.input.is_key_down(KeyCode::KeyQ) {
                    self.camera_zoom -=
                        zoom_speed * dt;
                }

                self.camera_zoom = self.camera_zoom.clamp(0.25, 4.0);

                let rotation_speed = 2.0;

                if self.input.is_key_down(KeyCode::KeyZ) {
                    self.camera_rotation -=
                        rotation_speed * dt;
                }

                if self.input.is_key_down(KeyCode::KeyX) {
                    self.camera_rotation +=
                        rotation_speed * dt;
                }

                if self.input.is_mouse_button_down(
                    MouseButton::Middle
                ) {
                    let [dx, dy] =
                        self.input.mouse_delta();

                    let pan_speed =
                        1.0 / self.camera_zoom;

                    self.camera_position[0] -=
                        dx * pan_speed;

                    self.camera_position[1] -=
                        dy * pan_speed;
                }

                if let Some(renderer) = &mut self.renderer {
                    renderer.set_camera_position(
                        self.camera_position
                    );

                    renderer.set_camera_rotation(
                        self.camera_rotation
                    );

                    renderer.update_camera(dt);
                }

                // Rendering.
                self.render();
                self.input.end_frame();

                if self.time.frame_count() % 60 == 0 {
                    println!(
                        "FPS: {:.1} | Delta: {:.4} | Frame: {}",
                        self.time.fps(),
                        self.time.delta_seconds(),
                        self.time.frame_count(),
                    );
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();

        if now >= self.next_frame_time {
            if let Some(window) = &self.window {
                window.request_redraw();
            }

            self.next_frame_time =
                now + self.target_frame_time;
        } else {
            event_loop.set_control_flow(
                winit::event_loop::ControlFlow::WaitUntil(
                    self.next_frame_time,
                ),
            );
        }
    }
}

impl<G: Game> App<G> {
    fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            renderer.set_camera_zoom(self.camera_zoom);

            let sprite = renderer.default_sprite();
            let test_sprite = renderer.test_sprite();

            // Texture A
            renderer.draw_sprite(
                &sprite,
                &Transform {
                    position: [100.0, 100.0],
                    rotation: 0.0,
                    scale: [1.0, 1.0],
                },
            );

            // Texture B
            renderer.draw_sprite(
                &test_sprite,
                &Transform {
                    position: [500.0, 100.0],
                    rotation: 0.0,
                    scale: [0.75, 0.75],
                },
            );

            // Texture A
            renderer.draw_sprite(
                &sprite,
                &Transform {
                    position: [300.0, 400.0],
                    rotation: 0.35,
                    scale: [0.5, 0.5],
                },
            );

            // Texture B
            renderer.draw_sprite(
                &test_sprite,
                &Transform {
                    position: [700.0, 400.0],
                    rotation: -0.3,
                    scale: [0.5, 0.5],
                },
            );

            renderer.render();
        }

        self.game.render();
    }
}