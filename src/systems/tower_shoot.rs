use amethyst::{
    core::{ timing::Time, transform::Transform },
    ecs::prelude::{
        Join, 
        ReadStorage, 
        System, 
        WriteStorage,
        Entities,
        Read,
        LazyUpdate,
        ReadExpect,
    },
};
use crate::{
    components::{
        Creep,
        Tower,
        Velocity,
    },
    util::{
        spawn::spawn_projectile,
        math::intercept,
    },
    config::Tower as TowerConfig,
};
use rand::{
    thread_rng,
    Rng,
};

pub struct TowerShoot;

impl<'s> System<'s> for TowerShoot {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Creep>,
        WriteStorage<'s, Tower>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, TowerConfig>,
    );

    fn run(&mut self, (entities, _creeps, mut towers, transforms, velocities, time, lazy_update, config): Self::SystemData) {
        let delta_seconds = time.delta_seconds();

        for (tower_entity, tower, tower_transform) in (&entities, &mut towers, &transforms).join() {
            // Skip dead towers
            if !entities.is_alive(tower_entity) { continue }

            // Advance time
            tower.elapsed += delta_seconds;

            // If the tower is ready to fire and has a target
            if tower.elapsed >= tower.frequency && tower.target.is_some() {
                // Skip dead targets
                let target = tower.target.unwrap();
                if entities.is_alive(target) {
                    if let (Some(target_transform), Some(target_velocity)) = (transforms.get(target), velocities.get(target)) {
                        let origin = tower_transform.translation().xy();
                        let target = target_transform.translation().xy();
                        if let Some(solution) = intercept(
                            origin,
                            target,
                            target_velocity.velocity, 
                            config.projectile_speed,
                        ) {
                            tower.elapsed = 0.;

                            // Solution given is a place to aim
                            let dx = solution.x - origin.x;
                            let dy = solution.y - origin.y;

                            let jitter = if config.aim_jitter > 0. {
                                let j = config.aim_jitter * 0.5;
                                thread_rng().gen_range(-j, j)
                            } else {
                                0.
                            };

                            let angle = dy.atan2(dx) + jitter;

                            spawn_projectile(
                                &entities,
                                &lazy_update,
                                tower_transform.clone(),
                                Velocity::new(
                                    // *60 is because velocity is scaled by seconds elapsed
                                    angle.cos() * config.projectile_speed * 60.,
                                    angle.sin() * config.projectile_speed * 60.,
                                ),
                            );
                        }
                    }

                }
            }
        }
    }
}