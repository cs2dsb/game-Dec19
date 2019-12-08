use amethyst::ecs::{Component, DenseVecStorage};
use crate::resources::AnimationId;

#[derive(Default)]
pub struct Animation {
    pub current: Option<AnimationId>,
    pub next: Option<AnimationId>,
    pub is_done: bool,
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}