use std::collections::HashMap;

use super::{Texture, TextureHandle};

pub struct TextureManager {
    textures: HashMap<TextureHandle, Texture>,
    next_id: u32,

    bind_group_layout: wgpu::BindGroupLayout,
}

impl TextureManager {
    pub fn new(
        device: &wgpu::Device,
    ) -> Self {
        let bind_group_layout =
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(
                        "Runa Texture Bind Group Layout"
                    ),

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

        Self {
            textures: HashMap::new(),
            next_id: 0,
            bind_group_layout,
        }
    }

    pub fn load(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> TextureHandle {
        let handle = TextureHandle(self.next_id);

        self.next_id += 1;

        let texture = Texture::from_bytes(
            device,
            queue,
            &self.bind_group_layout,
            bytes,
            label,
        );

        self.textures.insert(
            handle,
            texture,
        );

        handle
    }

    pub fn get(
        &self,
        handle: TextureHandle,
    ) -> Option<&Texture> {
        self.textures.get(&handle)
    }
}