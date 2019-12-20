use amethyst::ecs::{Component, DenseVecStorage};

/// Navigator indicates a sprite that navigates the map
pub struct Navigator;

impl Component for Navigator {
    type Storage = DenseVecStorage<Self>;
}