mod camera;
mod texture;
mod sprite;
mod assets;

pub use camera::Camera;
pub use texture::Texture;
pub use sprite::{
    Sprite,
    TextureHandle,
};

pub use assets::TextureManager;

use std::sync::Arc;
use winit::{window::Window};
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
use crate::engine::math::Transform;

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

    instance_buffer: wgpu::Buffer,
    instance_capacity: usize,

    texture_manager: TextureManager,
    default_texture: TextureHandle,

    camera: Camera,

    position: [f32; 2],
    rotation: f32,
    scale: [f32; 2],

    draw_commands: Vec<DrawCommand>,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
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

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SpriteInstance {
    pub transform: [[f32; 4]; 4],
}


impl Vertex {
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x2,
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

pub struct DrawCommand {
    pub texture: TextureHandle,
    pub transform: Transform,
}

impl SpriteInstance {
    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        let attributes = wgpu::vertex_attr_array![
            2 => Float32x4,
            3 => Float32x4,
            4 => Float32x4,
            5 => Float32x4,
        ];

        wgpu::VertexBufferLayout {
            array_stride:
                std::mem::size_of::<SpriteInstance>()
                    as wgpu::BufferAddress,

            step_mode:
                wgpu::VertexStepMode::Instance,

            attributes: Box::leak(
                Box::new(attributes)
            ),
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
            [cos * sx,  sin * sx, 0.0, 0.0],
            [-sin * sy, cos * sy, 0.0, 0.0],
            [0.0,       0.0,      1.0, 0.0],
            [position[0], position[1], 0.0, 1.0],
        ]
    }

    fn multiply_matrix(
        a: [[f32; 4]; 4],
        b: [[f32; 4]; 4],
    ) -> [[f32; 4]; 4] {
        let mut result = [[0.0; 4]; 4];

        for col in 0..4 {
            for row in 0..4 {
                result[col][row] =
                    a[0][row] * b[col][0]
                    + a[1][row] * b[col][1]
                    + a[2][row] * b[col][2]
                    + a[3][row] * b[col][3];
            }
        }

        result
    }

    pub fn default_sprite(&self) -> Sprite {
        Sprite::new(self.default_texture)
    }

    pub fn draw_sprite(
        &mut self,
        sprite: &Sprite,
        transform: &Transform,
    ) {
        self.draw_commands.push(
            DrawCommand {
                texture: sprite.texture,
                transform: *transform,
            }
        );
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

        let model = Self::create_transform_matrix(
            position,
            rotation,
            scale,
        );

        let projection =
            self.camera.projection_matrix();

        let transform =
            Self::multiply_matrix(
                projection,
                model,
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

    fn create_instance(&self, transform: Transform) -> SpriteInstance {
        let model = Self::create_transform_matrix(
            transform.position,
            transform.rotation,
            transform.scale,
        );

        let projection = self.camera.projection_matrix();

        SpriteInstance {
            transform: Self::multiply_matrix(
                projection,
                model,
            ),
        }
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

        let mut texture_manager =
            TextureManager::new(&device);

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

        let texture_bind_group_layout =
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("Runa Texture Bind Group Layout"),

                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,

                            visibility:
                                wgpu::ShaderStages::FRAGMENT,

                            ty: wgpu::BindingType::Texture {
                                sample_type:
                                    wgpu::TextureSampleType::Float {
                                        filterable: true,
                                    },

                                view_dimension:
                                    wgpu::TextureViewDimension::D2,

                                multisampled: false,
                            },

                            count: None,
                        },

                        wgpu::BindGroupLayoutEntry {
                            binding: 1,

                            visibility:
                                wgpu::ShaderStages::FRAGMENT,

                            ty: wgpu::BindingType::Sampler(
                                wgpu::SamplerBindingType::Filtering
                            ),

                            count: None,
                        },
                    ],
                },
            );

        let texture_bytes = include_bytes!(
            "../../../assets/icons.png"
        );

        let texture_handle = texture_manager.load(
            &device,
            &queue,
            texture_bytes,
            "Icons Texture",
        );

        println!(
            "Loaded texture: {:?}",
            texture_handle
        );

        let pipeline_layout =
            device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Runa Pipeline Layout"),

                    bind_group_layouts: &[
                        Some(&uniform_bind_group_layout),
                        Some(&texture_bind_group_layout),
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

        let instance_capacity = 1000;

        let instance_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Runa Sprite Instance Buffer"),

                size: (
                    instance_capacity
                        * std::mem::size_of::<SpriteInstance>()
                ) as u64,

                usage:
                    wgpu::BufferUsages::VERTEX
                    | wgpu::BufferUsages::COPY_DST,

                mapped_at_creation: false,
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

                    buffers: &[
                        Some(Vertex::layout()),
                        Some(SpriteInstance::layout()),
                    ],
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
                                wgpu::BlendState::ALPHA_BLENDING
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
                position: [0.0, 0.0],
                uv: [0.0, 0.0],
            },

            // Top-right
            Vertex {
                position: [200.0, 0.0],
                uv: [1.0, 0.0],
            },

            // Bottom-right
            Vertex {
                position: [200.0, 200.0],
                uv: [1.0, 1.0],
            },

            // Bottom-left
            Vertex {
                position: [0.0, 200.0],
                uv: [0.0, 1.0],
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

        let camera = Camera::new(
            config.width as f32,
            config.height as f32,
        );


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

            instance_buffer,
            instance_capacity,

            texture_manager,
            default_texture: texture_handle,

            camera,

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],

            draw_commands: Vec::new(),

        };



        renderer
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                output
            }

            wgpu::CurrentSurfaceTexture::Outdated
            | wgpu::CurrentSurfaceTexture::Lost => {
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
            let instances: Vec<SpriteInstance> = self
                .draw_commands
                .iter()
                .map(|command| {
                    self.create_instance(command.transform)
                })
                .collect();

            let instance_count = instances.len();

            if instance_count > self.instance_capacity {
                panic!(
                    "Too many sprites: {} (capacity: {})",
                    instance_count,
                    self.instance_capacity
                );
            }

            if !instances.is_empty() {
                self.queue.write_buffer(
                    &self.instance_buffer,
                    0,
                    bytemuck::cast_slice(&instances),
                );
            }

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

            let texture = self
                .texture_manager
                .get(self.default_texture)
                .expect("default texture not found");

            render_pass.set_bind_group(
                1,
                &texture.bind_group,
                &[],
            );

            render_pass.set_vertex_buffer(
                0,
                self.vertex_buffer.slice(..),
            );

            render_pass.set_vertex_buffer(
                1,
                self.instance_buffer.slice(..),
            );

            render_pass.set_index_buffer(
                self.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );

            if instance_count > 0 {
                render_pass.draw_indexed(
                    0..self.num_indices,
                    0,
                    0..instance_count as u32,
                );
            }

        }

        self.queue.submit(
            std::iter::once(
                encoder.finish()
            )
        );

        self.queue.present(output);

        // Command sudah diproses.
        self.draw_commands.clear();
    }
}