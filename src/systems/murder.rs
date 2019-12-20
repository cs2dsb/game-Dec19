use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage, Entities},
};

use crate::{
    components::{
        Age,
        Animation,
        Velocity,
    },
    resources::{
        AnimationId,
    },
};

pub struct Murder;

impl<'s> System<'s> for Murder {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Age>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, ages, mut animation, mut velocities, mut transforms): Self::SystemData) {
        for (e, age) in (&entities, &ages).join() {
            if !entities.is_alive(e) {
                continue;
            }
            if let Some(max_age) = age.max_age {
                if age.age > max_age {
                    if let Some(anim) = animation.get_mut(e) {
                        velocities.remove(e);

                        let (next, wait) = match anim.current {
                            Some(AnimationId::WalkUp) => (Some(AnimationId::DieUp), false),
                            Some(AnimationId::WalkDown) => (Some(AnimationId::DieDown), false),
                            Some(AnimationId::WalkLeft) => (Some(AnimationId::DieLeft), false),
                            Some(AnimationId::WalkRight) => (Some(AnimationId::DieRight), false),
                            Some(AnimationId::WalkUpLeft) => (Some(AnimationId::DieUpLeft), false),
                            Some(AnimationId::WalkDownLeft) => (Some(AnimationId::DieDownLeft), false),
                            Some(AnimationId::WalkUpRight) => (Some(AnimationId::DieUpRight), false),
                            Some(AnimationId::WalkDownRight) => (Some(AnimationId::DieDownRight), false),
                            /*
                            Some(AnimationId::DieUp) => (None, false),
                            Some(AnimationId::DieDown) => (None, false),
                            Some(AnimationId::DieLeft) => (None, false),
                            Some(AnimationId::DieRight) => (None, false),
                            Some(AnimationId::DieUpLeft) => (None, false),
                            Some(AnimationId::DieDownLeft) => (None, false),
                            Some(AnimationId::DieUpRight) => (None, false),
                            Some(AnimationId::DieDownRight) => (None, false), 
                            */
                            _ => (None, false),
                        };

                        if (!wait || anim.is_done) && next.is_some() {
                            anim.next = next;
                            if wait { //Hack
                                if let Some(transform) = transforms.get_mut(e) {
                                    let z = transform.translation().z * 0.99;
                                    transform.set_translation_z(z);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}