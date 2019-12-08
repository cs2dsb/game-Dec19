use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage, ReadExpect},
    window::ScreenDimensions,
};

use crate::components::Velocity;

pub struct Bouncer;

impl<'s> System<'s> for Bouncer {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (
        mut transforms, 
        mut velocities, 
        dims,
    ): Self::SystemData) {
        let width = dims.width();
        let height = dims.height();

        for (v, t) in (&mut velocities, &mut transforms).join() {
            if v.velocity.x < 0. {
                let x = t.translation().x;
                if x < 0. {
                    t.set_translation_x(x + width);
                }
            } else if v.velocity.x > 0. {
                let x = t.translation().x;
                if x > width {
                    t.set_translation_x(x - width);
                }
            }

            if v.velocity.y < 0. {
                let y = t.translation().y;
                if y < 0. {
                    t.set_translation_y(y + height);
                }
            } else if v.velocity.y > 0. {
                let y = t.translation().y;
                if y > height {
                    t.set_translation_y(y - height);
                }
            }
        }
    }
}