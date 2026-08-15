use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use super::{Renderer, Time};
use crate::engine::math::Transform;

pub trait Game {
    fn fixed_update(&mut self, dt: f32);

    fn update(&mut self, dt: f32);

    fn render(&mut self);
}

pub struct App<G: Game> {
    title: String,
    width: u32,
    height: u32,

    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,

    time: Time,

    target_frame_time: Duration,
    next_frame_time: Instant,

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

            target_frame_time: Duration::from_secs_f64(1.0 / 60.0),
            next_frame_time: now,

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

            WindowEvent::RedrawRequested => {
                self.time.update();

                const MAX_FIXED_STEPS: u32 = 5;

                let mut fixed_steps = 0;

                while self.time.consume_fixed_step()
                    && fixed_steps < MAX_FIXED_STEPS
                {
                    self.game
                        .fixed_update(Time::fixed_delta_seconds());

                    fixed_steps += 1;
                }

                if fixed_steps == MAX_FIXED_STEPS {
                    self.time.reset_accumulator();
                }

                // Variable timestep update.
                self.game.update(
                    self.time.delta_seconds()
                );

                // Rendering.
                self.render();

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
            let sprite = renderer.default_sprite();

            // Sprite 1
            renderer.draw_sprite(
                &sprite,
                &Transform {
                    position: [100.0, 100.0],
                    rotation: 0.0,
                    scale: [1.0, 1.0],
                },
            );

            // Sprite 2
            renderer.draw_sprite(
                &sprite,
                &Transform {
                    position: [500.0, 100.0],
                    rotation: 0.0,
                    scale: [0.75, 0.75],
                },
            );

            // Sprite 3
            renderer.draw_sprite(
                &sprite,
                &Transform {
                    position: [300.0, 400.0],
                    rotation: 0.35,
                    scale: [0.5, 0.5],
                },
            );

            renderer.render();
        }

        self.game.render();
    }
}