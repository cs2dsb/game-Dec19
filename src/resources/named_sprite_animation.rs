use amethyst::{
    assets::{Handle, Asset},
    ecs::VecStorage,
};
use serde::{Serialize, Deserialize};
use super::AnimationId;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NamedAnimation {
    pub id: AnimationId,
    pub input: Vec<f32>,
    pub output: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NamedAnimationSet {
    pub animations: Vec<NamedAnimation>,
}

/// A handle to a `AnimationSet` asset.
pub type NamedAnimationSetHandle = Handle<NamedAnimationSet>;

impl Asset for NamedAnimationSet {
    const NAME: &'static str = "game::NamedAnimationSet";
    // use `Self` if the type is directly serialized.
    type Data = Self;
    type HandleStorage = VecStorage<NamedAnimationSetHandle>;
}