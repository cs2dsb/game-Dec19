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

use crate::components::{
    Velocity,
    Path,
};

const TILE_W: f32 = 32.;
const TILE_H: f32 = 32.;

fn iso_to_screen(x: f32, y: f32) -> (f32, f32) {
    (
        (x - y) * TILE_W,
        (x + y) * TILE_H / 2.,
    )
}

pub struct DebugDraw;

impl<'s> System<'s> for DebugDraw {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Path>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (
        transforms, 
        velocities, 
        paths,
        mut debug_comps,
    ): Self::SystemData) {
        #[cfg(feature = "profiler")]
        profile_scope!("debug_draw_system");

        for (t, v, p, debug) in (
            &transforms, 
            &velocities,
            &paths,
            &mut debug_comps,
        ).join() {
            let origin = Point3::from(*t.translation());
            let ve = Vector3::new(v.velocity.x, v.velocity.y, 0.);
            
            debug.add_line(
                origin,
                origin + ve,
                Srgba::new(1., 1., 1., 1.),
            );

            if let Some(path) = &p.path {
                for i in 1..path.0.len() {
                    let prev = &path.0[i-1];
                    let current = &path.0[i];

                    let (x, y) = iso_to_screen(prev.x as f32, prev.y as f32);
                    let origin = Point3::new(x, y, 0.5);
                    
                    let (x, y) = iso_to_screen(current.x as f32, current.y as f32);
                    let end = Point3::new(x, y, 0.5);

                    debug.add_line(
                        origin,
                        end,
                        Srgba::new(1., 0.2, 0.2, 1.0),
                    );
                }
            }
        }
    }
}