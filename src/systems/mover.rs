use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{
        Join, 
        Read, 
        ReadStorage, 
        System, 
        WriteStorage,
    },
};

use crate::components::Velocity;

pub struct Mover;

impl<'s> System<'s> for Mover {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, mut transforms, time): Self::SystemData) {
        for (v, t) in (&velocities, &mut transforms).join() {
            t.prepend_translation_x(v.velocity.x * time.delta_seconds());
            t.prepend_translation_y(v.velocity.y * time.delta_seconds());
        }
    }
}