use amethyst::{
    renderer::{
        debug_drawing::DebugLinesComponent,
        palette::Srgba,
    },
    core::{
        transform::Transform,
        math::{
            Vector2,
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
        ReadExpect,
    },
};

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use crate::{
    components::{
        Velocity,
        Path,
        Color,
        Tower,
        Map,
        Projectile,
    },
    util::{
        iso_to_screen,
        TILE_W,
        TILE_H,
        constants::DEBUG_Z,
    },
    config::DebugDraw as DebugDrawConfig,
};

pub struct DebugDraw;

impl<'s> System<'s> for DebugDraw {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Path>,
        ReadStorage<'s, Tower>,
        WriteStorage<'s, DebugLinesComponent>,
        WriteStorage<'s, Color>,
        ReadExpect<'s, DebugDrawConfig>,
        ReadStorage<'s, Map>,
        ReadStorage<'s, Projectile>,
    );

    fn run(&mut self, (
        entities,
        transforms, 
        velocities, 
        paths,
        towers,
        mut debug_comps,
        mut colors,
        config,
        maps,
        projectiles,
    ): Self::SystemData) {
        #[cfg(feature = "profiler")]
        profile_scope!("debug_draw_system");

        //Create colors for debug entities without them
        let mut new_colors = Vec::new();
        for (e, _, _) in (&entities, &debug_comps, !&colors).join() {
            if !entities.is_alive(e) { continue }
            let color = Color::rand();
            new_colors.push((e, color));
        }

        for (e, color) in new_colors {
            colors.insert(e, color).expect("Failed to insert color component");
        }

        for (entity, transform, debug, color) in (&entities, &transforms, &mut debug_comps, &colors).join() {
            let color: Srgba = color.clone().into();
            let origin = {
                let mut origin = Point3::from(*transform.translation());
                origin.z = DEBUG_Z;
                origin
            };

            if config.velocity {
                if let Some(velocity) = velocities.get(entity) {                    
                    let ve = Vector3::new(velocity.velocity.x, velocity.velocity.y, 0.);
                    debug.add_line(
                        origin,
                        origin + ve * 60.,
                        Srgba::new(1., 1., 1., 1.),
                    );
                }
            }

            if config.pathfinding {
                if let Some(path) = paths.get(entity) {
                    if let Some(path) = &path.path {
                        for i in 1..path.0.len() {
                            let prev = &path.0[i-1];
                            let current = &path.0[i];

                            let sp = iso_to_screen(prev.clone().into());
                            let prev_point = Point3::new(sp.x, sp.y, DEBUG_Z);
                            
                            let sp = iso_to_screen(current.clone().into());
                            let current_point = Point3::new(sp.x, sp.y, DEBUG_Z);

                            debug.add_line(
                                prev_point,
                                current_point,
                                color,
                            );
                        }
                    }
                }
            }

            if config.tower_range || config.tower_target {
                if let Some(tower) = towers.get(entity) {
                    if config.tower_range {
                        add_ellipse_2d(
                            debug,
                            origin,
                            tower.range * TILE_W * 2_f32.sqrt(),
                            tower.range * TILE_H * 2_f32.sqrt(),
                            20,
                            color,
                        );
                    }

                    let mut map = None;

                    if config.tower_los {
                        for (e, m) in (&entities, &maps).join() {
                            if !entities.is_alive(e) { continue }
                            map = Some(m);
                            break;
                        }
                    }

                    if config.tower_target {
                        if let Some(target) = tower.target {
                            if let Some(target_transform) = transforms.get(target) {
                                if let Some(map) = map {
                                    if let (Some(origin), Some(target)) = (
                                        map.world_to_cell_index(transform.translation().xy()),
                                        map.world_to_cell_index(target_transform.translation().xy())
                                    ) {
                                        for visit in map.ray_visit(origin, target) {
                                            let pos = iso_to_screen(Vector2::new(visit.0 as f32, visit.1 as f32));
                                            let screen = Point3::new(pos.x, pos.y, DEBUG_Z);
                                            
                                            debug.add_circle_2d(
                                                screen,
                                                10.,
                                                20,
                                                color,
                                            );                                            
                                        }
                                    }
                                }                                

                                let mut target = Point3::from(*target_transform.translation());
                                target.z = DEBUG_Z;

                                debug.add_circle_2d(
                                    target,
                                    10.,
                                    20,
                                    color,
                                );   

                                debug.add_line(
                                    origin,
                                    target,
                                    color,
                                );
                            }
                        }
                    }
                }
            }

            if config.projectiles {
                if let Some(_) = projectiles.get(entity) {
                    debug.add_circle_2d(
                        origin,
                        5.,
                        4,
                        color,
                    );

                }
            }
        }
    }
}

pub fn add_ellipse_2d(debug: &mut DebugLinesComponent, center: Point3<f32>, radius_x: f32, radius_y: f32, points: u32, color: Srgba) {
    let mut prev = None;

    for i in 0..=points {
        let a = std::f32::consts::PI * 2.0 / (points as f32) * (i as f32);
        let x = radius_x * a.cos();
        let y = radius_y * a.sin();
        let point = [center[0] + x, center[1] + y, center[2]].into();

        if let Some(prev) = prev {
            debug.add_line(prev, point, color);
        }

        prev = Some(point);
    }
}