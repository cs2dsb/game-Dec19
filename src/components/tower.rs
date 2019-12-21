use amethyst::ecs::{
    Component, 
    DenseVecStorage,
    NullStorage,
    Entity,
};

#[derive(Default)]
pub struct Tower {
    pub range: f32,
    pub target: Option<Entity>,
}

impl Tower {
    pub fn new(range: f32) -> Self {
        Self {
            range,
            target: None,
        }
    }
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Default)]
pub struct BulletTower;

impl Component for BulletTower {
    type Storage = NullStorage<Self>;
}