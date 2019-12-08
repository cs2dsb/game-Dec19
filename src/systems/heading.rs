use amethyst::{
    ecs::prelude::{
        Join, 
        ReadStorage, 
        System, 
        WriteStorage,
    },
};

use crate::{
    components::{
        Velocity,
        Animation,
    },
    resources::AnimationId,
    util::math::radians,
};

pub struct Heading;

impl<'s> System<'s> for Heading {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Animation>,
    );

    fn run(&mut self, (velocities, mut animation): Self::SystemData) {
        for (velocity, animation) in (&velocities, &mut animation).join() {
            let angle = velocity.angle();
            let animation_id = match angle {
                x if (radians(0.)..radians(22.5)).contains(&x) => AnimationId::WalkRight,
                x if (radians(22.5)..radians(67.5)).contains(&x) => AnimationId::WalkDownRight,
                x if (radians(67.5)..radians(112.5)).contains(&x) => AnimationId::WalkDown,
                x if (radians(112.5)..radians(157.5)).contains(&x) => AnimationId::WalkDownLeft,
                x if (radians(157.5)..radians(202.5)).contains(&x) => AnimationId::WalkLeft,
                x if (radians(202.5)..radians(247.5)).contains(&x) => AnimationId::WalkUpLeft,
                x if (radians(247.5)..radians(292.5)).contains(&x) => AnimationId::WalkUp,
                x if (radians(292.5)..radians(337.5)).contains(&x) => AnimationId::WalkUpRight,
                x if (radians(337.5)..radians(360.)).contains(&x) => AnimationId::WalkRight,
                _ => unreachable!(),
            };


            animation.next = Some(animation_id);
        }
    }
}