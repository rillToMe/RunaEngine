#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TextureHandle(pub u32);

pub struct Sprite {
    pub texture: TextureHandle,
}

impl Sprite {
    pub fn new(texture: TextureHandle) -> Self {
        Self { texture }
    }
}