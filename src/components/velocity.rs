use amethyst::{
    ecs::{Component, DenseVecStorage},
    core::math::{
        Vector2,
        Rotation2,
        Unit,
    },
};
use crate::util::{
    limit,
    math::radians,
};
use rand::random;

union Transmute<T: Copy, U: Copy> {
    from: T,
    to: U,
}

const X_AXIS: Unit<Vector2<f32>> = unsafe {
    Transmute::<[f32; 2], Unit<Vector2<f32>>> { 
        from: [1.0, 0.0] }.to
};

//const X_AXIS: Unit<Vector2<f32>> = Vector2::x_axis();

#[derive(Debug)]
pub struct Velocity {
    pub velocity: Vector2<f32>,
    pub max_speed: f32,
    pub speed: f32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Velocity {
    pub fn new(x: f32, y: f32, max_speed: f32) -> Self {
        let velocity = Vector2::new(x, y);
        let speed = velocity.magnitude();
        Self {
            velocity,
            max_speed,
            speed,
        }
    }

    pub fn rand(min: f32, max: f32) -> Self {
        let range = max - min;
        let mag = random::<f32>() * range + min;

        let velocity = Vector2::new_random()
            .add_scalar(-0.5)
            .normalize()
            * mag;

        Self::new(velocity.x, velocity.y, max)
    }

    pub fn clamp(&mut self) {
        limit(&mut self.velocity, self.max_speed);
    }
    
    pub fn angle(&self) -> f32 {
        let rot = Rotation2::rotation_between(&self.velocity, &X_AXIS);
        let mut angle = rot.angle();
        while angle < 0. {
            angle += radians(360.);
        }
        angle
    }
}