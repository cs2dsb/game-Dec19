use amethyst::{
    ecs::{Component, DenseVecStorage},
    core::math::Vector2,
};
use crate::components::PathNode;

/// Navigator indicates a sprite that navigates the map
#[derive(Clone, Copy)]
pub struct Navigator {
    pub x: usize,
    pub y: usize,
}

impl Component for Navigator {
    type Storage = DenseVecStorage<Self>;
}

impl Into<PathNode> for Navigator {
    fn into(self) -> PathNode {
        PathNode::new(self.x as i32, self.y as i32)
    }
}

impl Into<Vector2<f32>> for Navigator {
    fn into(self) -> Vector2<f32> {
        Vector2::new(self.x as f32, self.y as f32)
    }
}