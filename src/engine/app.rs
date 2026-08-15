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
}

impl App {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width,
            height,

            window: None,

            time: Time::new(),

            position: 0.0,
            velocity: 100.0,
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

                let dt = self.time.delta_seconds();

                self.update(dt);
                self.render();

                if self.time.frame_count() % 60 == 0 {
                    println!(
                        "FPS: {:.1} | Delta: {:.4} | Frame: {}",
                        self.time.fps(),
                        dt,
                        self.time.frame_count(),
                    );
                }

                // Request frame berikutnya.
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            _ => {}
        }
    }
}

impl App {
    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;

        if self.time.frame_count() % 60 == 0 {
            println!("Position: {:.2}", self.position);
        }
    }

    fn render(&mut self) {
        // Renderer akan masuk Phase 2.
    }
}