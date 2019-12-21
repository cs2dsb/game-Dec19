use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Creep;

impl Component for Creep {
    type Storage = NullStorage<Self>;
}