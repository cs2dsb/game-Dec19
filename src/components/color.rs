use amethyst::{
    ecs::{Component, DenseVecStorage},
    renderer::palette::Srgba,
};
use random_color::RandomColor;
use std::convert::Into;

///Component represents the color of an entity
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Component for Color {
        type Storage = DenseVecStorage<Self>;
}

impl Color {
    pub fn rand() -> Self {
        let rgb = RandomColor::new()
            .to_rgb_array();
        Self::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
            1.0
        )
    }
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r, g, b, a
        }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.r,
            self.g,
            self.b,
            self.a,
        ]
    }
}

impl Into<Srgba> for Color {
    fn into(self) -> Srgba {        
        Srgba::new(
            self.r,
            self.g,
            self.b,
            self.a,
        )
    }
}