use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Age {
    pub age: f32,
    pub max_age: Option<f32>,
}

impl Component for Age {
    type Storage = DenseVecStorage<Self>;
}