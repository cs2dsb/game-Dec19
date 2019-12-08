use amethyst::{
    renderer::{
        debug_drawing::DebugLinesComponent,
        palette::Srgba,
    },
    core::{
        transform::Transform,
        math::{
            Vector3,
            Point3,
        },
    },
    ecs::prelude::{
        Join,
        ReadStorage, 
        System, 
        WriteStorage, 
    },
};

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use crate::components::Velocity;

pub struct DebugDraw;

impl<'s> System<'s> for DebugDraw {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (
        transforms, 
        velocities, 
        mut debug_comps,
    ): Self::SystemData) {
        #[cfg(feature = "profiler")]
        profile_scope!("debug_draw_system");

        for (t, v, debug) in (
            &transforms, 
            &velocities,
            &mut debug_comps,
        ).join() {
            let origin = Point3::from(*t.translation());
            let ve = Vector3::new(v.velocity.x, v.velocity.y, 0.);
            
            debug.add_line(
                origin,
                origin + ve,
                Srgba::new(1., 1., 1., 1.),
            );
        }
    }
}