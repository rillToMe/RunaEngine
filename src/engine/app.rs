use std::time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use super::Time;

pub struct App {
    title: String,
    width: u32,
    height: u32,

    window: Option<Window>,

    time: Time,

    position: f32,
    velocity: f32,

    target_frame_time: Duration,
    next_frame_time: Instant,

}

impl App {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        let now = Instant::now();

        Self {
            title: title.into(),
            width,
            height,

            window: None,

            time: Time::new(),

            position: 0.0,
            velocity: 100.0,

            target_frame_time: Duration::from_secs_f64(1.0 / 60.0),
            next_frame_time: now,
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

impl ApplicationHandler for App {
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

        self.window = Some(window);

        // Mulai meminta redraw.
        if let Some(window) = &self.window {
            window.request_redraw();
        }
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

                while self.time.consume_fixed_step() {
                    self.fixed_update();
                }

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

            self.next_frame_time = now + self.target_frame_time;
        } else {
            event_loop.set_control_flow(
                winit::event_loop::ControlFlow::WaitUntil(self.next_frame_time)
            );
        }
    }
}

impl App {
    fn fixed_update(&mut self) {
        let dt = Time::fixed_delta_seconds();

        self.position += self.velocity * dt;

        if self.time.frame_count() % 60 == 0 {
            println!("Position: {:.2}", self.position);
        }
    }

    fn render(&mut self) {
        // Renderer akan masuk Phase 2.
    }
}