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
        Entities,
    },
};

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use crate::{
    components::{
        Velocity,
        Path,
        Color,
    },
    util::iso_to_screen,
};

pub struct DebugDraw;

impl<'s> System<'s> for DebugDraw {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Path>,
        WriteStorage<'s, DebugLinesComponent>,
        WriteStorage<'s, Color>,
    );

    fn run(&mut self, (
        entities,
        transforms, 
        velocities, 
        paths,
        mut debug_comps,
        mut colors,
    ): Self::SystemData) {
        #[cfg(feature = "profiler")]
        profile_scope!("debug_draw_system");

        //Create colors for entities without them
        let mut new_colors = Vec::new();
        for (e, _) in (&entities, !&colors).join() {
            if !entities.is_alive(e) { continue }
            let color = Color::rand();
            new_colors.push((e, color));
        }

        for (e, color) in new_colors {
            colors.insert(e, color).expect("Failed to insert color component");
        }

        for (t, v, p, debug, c) in (
            &transforms, 
            &velocities,
            &paths,
            &mut debug_comps,
            &colors,
        ).join() {
            let origin = Point3::from(*t.translation());
            let ve = Vector3::new(v.velocity.x, v.velocity.y, 0.);
            
            debug.add_line(
                origin,
                origin + ve * 60.,
                Srgba::new(1., 1., 1., 1.),
            );

            if let Some(path) = &p.path {
                let color: Srgba = c.clone().into();
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
                        color,
                    );
                }
            }
        }
    }
}