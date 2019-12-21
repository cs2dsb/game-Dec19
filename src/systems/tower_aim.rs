use amethyst::{
    core::transform::Transform,
    ecs::prelude::{
        Join, 
        ReadStorage, 
        System, 
        WriteStorage,
        Entities,
    },
};
use std::f32;
use crate::{
    components::{
        Creep,
        Tower,
        Map,
    },
    util::iso_distance,
};

pub struct TowerAim;

impl<'s> System<'s> for TowerAim {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Creep>,
        WriteStorage<'s, Tower>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Map>,
    );

    fn run(&mut self, (entities, creeps, mut towers, transforms, maps): Self::SystemData) {
        let mut map = None;
        for (entity, m) in (&entities, &maps).join() {
            if entities.is_alive(entity) { 
                map = Some(m);
                break;
            }
        }

        if let Some(map) = map {
            for (tower_entity, tower, tower_transform) in (&entities, &mut towers, &transforms).join() {
                // Skip any dead ones
                if !entities.is_alive(tower_entity) { continue }

                let mut best_range = f32::MAX;
                tower.target = None;

                for (creep_entity, _, creep_transform) in (&entities, &creeps, &transforms).join() {
                    // Skip any dead ones
                    if !entities.is_alive(creep_entity) { continue }

                    // How far away is the creep
                    let distance = iso_distance(tower_transform.translation(), creep_transform.translation());

                    // Check if it's in range
                    if distance > tower.range { continue }

                    // Check line of sight
                    if let Some(origin) = map.world_to_cell_index(tower_transform.translation().xy()) {
                        if let Some(target) = map.world_to_cell_index(creep_transform.translation().xy()) {
                            if !map.line_of_sight(origin, target) { continue }
                        } else {
                            log::warn!("Creep {:?} outside world bounds", creep_entity);
                        }
                    } else {
                        log::warn!("Tower {:?} outside world bounds", tower_entity);
                    }

                    if distance < best_range {
                        best_range = distance;
                        tower.target = Some(creep_entity);
                    }
                }
            }
        }
    }
}