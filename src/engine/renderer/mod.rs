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
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

pub struct Renderer {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,

    surface: Surface<'static>,
    config: SurfaceConfiguration,

    pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x3,
        ];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>()
                as wgpu::BufferAddress,

            step_mode: wgpu::VertexStepMode::Vertex,

            attributes: &Self::ATTRIBUTES,
        }
    }
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

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shader.wgsl")
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Runa Render Pipeline"),

                layout: None,

                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),

                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),

                    buffers: &[Some(Vertex::layout())],
                },

                primitive: wgpu::PrimitiveState::default(),

                depth_stencil: None,

                multisample: wgpu::MultisampleState::default(),

                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),

                    targets: &[Some(
                        wgpu::ColorTargetState {
                            format: config.format,

                            blend: Some(
                                wgpu::BlendState::REPLACE
                            ),

                            write_mask:
                                wgpu::ColorWrites::ALL,
                        }
                    )],
                }),

                multiview_mask: None,

                cache: None,
            },
        );

        let vertices = [
            // Top-left
            Vertex {
                position: [-0.5, 0.5],
                color: [1.0, 0.0, 0.0],
            },

            // Top-right
            Vertex {
                position: [0.5, 0.5],
                color: [0.0, 1.0, 0.0],
            },

            // Bottom-right
            Vertex {
                position: [0.5, -0.5],
                color: [0.0, 0.0, 1.0],
            },

            // Bottom-left
            Vertex {
                position: [-0.5, -0.5],
                color: [1.0, 1.0, 0.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Runa Triangle Vertex Buffer"),

                contents: bytemuck::cast_slice(&vertices),

                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let indices: &[u16] = &[
            0, 1, 2,
            0, 2, 3,
        ];

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Runa Quad Index Buffer"),

                contents: bytemuck::cast_slice(indices),

                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let num_indices = indices.len() as u32;

        Self {
            instance,
            adapter,
            device,
            queue,

            surface,
            config,

            pipeline,

            vertex_buffer,
            index_buffer,
            num_indices,
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
            let mut render_pass =
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

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_vertex_buffer(
                0,
                self.vertex_buffer.slice(..),
            );

            render_pass.set_index_buffer(
                self.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );

            render_pass.draw_indexed(
                0..self.num_indices,
                0,
                0..1,
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