use std::sync::Arc;
use winit::{dpi::Position, window::Window};
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

    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    position: [f32; 2],
    rotation: f32,
    scale: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub transform: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            transform: Self::identity(),
        }
    }

    fn identity() -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
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
    fn create_transform_matrix(
        position: [f32; 2],
        rotation: f32,
        scale: [f32; 2],
    ) -> [[f32; 4]; 4] {
        let cos = rotation.cos();
        let sin = rotation.sin();

        let sx = scale[0];
        let sy = scale[1];

        [
            [cos * sx, -sin * sy, 0.0, 0.0],
            [sin * sx,  cos * sy, 0.0, 0.0],
            [0.0,       0.0,      1.0, 0.0],
            [position[0], position[1], 0.0, 1.0],
        ]
    }

    pub fn set_transform(
        &mut self,
        position: [f32; 2],
        rotation: f32,
        scale: [f32; 2],
    ) {
        self.position = position;
        self.rotation = rotation;
        self.scale = scale;

        let transform = Self::create_transform_matrix(
            position,
            rotation,
            scale,
        );

        let uniforms = Uniforms {
            transform,
        };

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );
    }

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

        let uniform_bind_group_layout =
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("Runa Uniform Bind Group Layout"),

                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,

                            visibility: wgpu::ShaderStages::VERTEX,

                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,

                                has_dynamic_offset: false,

                                min_binding_size: None,
                            },

                            count: None,
                        },
                    ],
                },
            );

        let pipeline_layout =
            device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Runa Pipeline Layout"),

                    bind_group_layouts: &[
                        Some(&uniform_bind_group_layout)
                    ],

                    immediate_size: 0,
                },
            );
        
        let uniforms = Uniforms::new();

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Runa Transform Uniform Buffer"),

                contents: bytemuck::cast_slice(&[uniforms]),

                usage:
                    wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        let uniform_bind_group =
            device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    label: Some("Runa Uniform Bind Group"),

                    layout: &uniform_bind_group_layout,

                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,

                            resource:
                                uniform_buffer.as_entire_binding(),
                        },
                    ],
                },
            );

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shader.wgsl")
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Runa Render Pipeline"),

                layout: Some(&pipeline_layout),

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

        let mut renderer = Self {
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

            uniform_buffer,
            uniform_bind_group,

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
        };

        renderer.set_transform(
            [0.0, 0.0],
            0.5,
            [0.7, 0.4],
        );

        renderer
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

            render_pass.set_bind_group(
                0,
                &self.uniform_bind_group,
                &[],
            );

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