use std::collections::HashMap;
use std::fs;

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

    pub fn contains(
        &self,
        handle: TextureHandle,
    ) -> bool {
        self.textures.contains_key(&handle)
    }
}

pub struct AssetManager {
    textures: TextureManager,
    texture_cache: HashMap<String, TextureHandle>,
}

impl AssetManager {
    pub fn new(
        device: &wgpu::Device,
    ) -> Self {
        Self {
            textures: TextureManager::new(device),
            texture_cache: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
    ) -> TextureHandle {
        if let Some(&handle) =
            self.texture_cache.get(path)
        {
            return handle;
        }

        let bytes = fs::read(path)
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to load asset '{}': {}",
                    path,
                    error
                )
            });

        self.load_texture_cached(
            device,
            queue,
            &bytes,
            path,
        )
    }

    pub fn load_texture_cached(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        path: &str,
    ) -> TextureHandle {
        if let Some(&handle) =
            self.texture_cache.get(path)
        {
            return handle;
        }

        let handle = self.textures.load(
            device,
            queue,
            bytes,
            path,
        );

        self.texture_cache.insert(
            path.to_string(),
            handle,
        );

        handle
    }

    pub fn get_texture(
        &self,
        handle: TextureHandle,
    ) -> Option<&Texture> {
        self.textures.get(handle)
    }
}