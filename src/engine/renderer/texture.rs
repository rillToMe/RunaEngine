pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,

    pub bind_group: wgpu::BindGroup,

    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        bytes: &[u8],
        label: &str,
    ) -> Self {
        let image = image::load_from_memory(bytes)
            .expect("failed to decode image")
            .to_rgba8();

        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(label),

                size,

                mip_level_count: 1,
                sample_count: 1,

                dimension: wgpu::TextureDimension::D2,

                format: wgpu::TextureFormat::Rgba8UnormSrgb,

                usage:
                    wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST,

                view_formats: &[],
            },
        );

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },

            &image,

            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },

            size,
        );

        let view = texture.create_view(
            &wgpu::TextureViewDescriptor::default(),
        );

        // Sampler
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some(label),

                address_mode_u:
                    wgpu::AddressMode::ClampToEdge,

                address_mode_v:
                    wgpu::AddressMode::ClampToEdge,

                address_mode_w:
                    wgpu::AddressMode::ClampToEdge,

                mag_filter:
                    wgpu::FilterMode::Nearest,

                min_filter:
                    wgpu::FilterMode::Nearest,

                mipmap_filter:
                    wgpu::MipmapFilterMode::Nearest,

                ..Default::default()
            },
        );

        // Bind group
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(label),

                layout,

                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,

                        resource:
                            wgpu::BindingResource::TextureView(
                                &view
                            ),
                    },

                    wgpu::BindGroupEntry {
                        binding: 1,

                        resource:
                            wgpu::BindingResource::Sampler(
                                &sampler
                            ),
                    },
                ],
            },
        );

        Self {
            texture,
            view,
            sampler,
            bind_group,

            width: dimensions.0,
            height: dimensions.1,
        }
    }
}