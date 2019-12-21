use amethyst::{
    core::{
        timing::Time, 
        transform::Transform,
    },
    ecs::prelude::{
        Join, 
        Read, 
        System, 
        WriteStorage,
        ReadStorage,
        Entities,
    },
};
use crate::{
    components::{
        Velocity,
        Projectile,
        Map,
    },
};

pub struct ProjectileMover;

impl<'s> System<'s> for ProjectileMover {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, Map>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, velocities, mut transforms, projectiles, maps, time): Self::SystemData) {
        let mut map = None;
        for m in (&maps).join() {
            map = Some(m);
            break;
        }
        for (e, v, t, _) in (&entities, &velocities, &mut transforms, &projectiles).join() {
            t.prepend_translation_x(v.velocity.x * time.delta_seconds());
            t.prepend_translation_y(v.velocity.y * time.delta_seconds());

            if let Some(map) = map {
                if let Some(cell) = map.world_to_cell_index(t.translation().xy()) {
                    if map.is_wall(cell.0 as u32, cell.1 as u32) {
                        entities.delete(e).expect("Failed to delete entity");
                    }
                }
            }
        }
    }
}