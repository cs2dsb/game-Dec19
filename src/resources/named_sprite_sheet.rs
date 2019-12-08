use amethyst::{
    assets::{Handle, Asset},
    ecs::VecStorage,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NamedSpritePosition {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub offsets: Option<[f32; 2]>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NamedSpriteSheet {
    pub texture_width: u32,
    pub texture_height: u32,
    pub sprites: Vec<NamedSpritePosition>,
}

/// A handle to a `NamedSpriteSheet` asset.
pub type NamedSpriteSheetHandle = Handle<NamedSpriteSheet>;

impl Asset for NamedSpriteSheet {
    const NAME: &'static str = "game::NamedSpriteSheet";
    // use `Self` if the type is directly serialized.
    type Data = Self;
    type HandleStorage = VecStorage<NamedSpriteSheetHandle>;
}