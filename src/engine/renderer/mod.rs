use std::sync::Arc;

use winit::window::Window;

use wgpu::{
    Adapter,
    Device,
    Instance,
    Queue,
    Surface,
    SurfaceConfiguration,
};

pub struct Renderer {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,

    surface: Surface<'static>,
    config: SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(window)
            .expect("failed to create surface");

        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    ..Default::default()
                },
            )
            .await
            .expect("failed to find suitable GPU adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Runa Engine Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features:
                    wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("failed to create GPU device");

        let config = surface
            .get_default_config(
                &adapter,
                1280,
                720,
            )
            .expect("surface is not supported by adapter");

        surface.configure(&device, &config);

        Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            config,
        }
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                output
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(
                    &self.device,
                    &self.config,
                );

                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(
                    &self.device,
                    &self.config,
                );

                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout => {
                eprintln!("Surface timeout");
                return;
            }

            wgpu::CurrentSurfaceTexture::Occluded => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                eprintln!("Surface validation error");
                return;
            }
        };

        let view = output
            .texture
            .create_view(
                &wgpu::TextureViewDescriptor::default(),
            );

        let mut encoder = self.device
            .create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("Runa Render Encoder"),
                },
            );

        {
            let _render_pass =
                encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("Runa Render Pass"),

                        color_attachments: &[Some(
                            wgpu::RenderPassColorAttachment {
                                view: &view,
                                depth_slice: None,
                                resolve_target: None,

                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                        wgpu::Color {
                                            r: 0.05,
                                            g: 0.05,
                                            b: 0.08,
                                            a: 1.0,
                                        },
                                    ),

                                    store: wgpu::StoreOp::Store,
                                },
                            },
                        )],

                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                        multiview_mask: None,
                    },
                );
        }

        self.queue.submit(
            std::iter::once(
                encoder.finish()
            )
        );

        self.queue.present(output);
    }
}