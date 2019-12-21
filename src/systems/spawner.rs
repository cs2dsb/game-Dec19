use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Read, 
        System, 
        Entities, 
        LazyUpdate,
        ReadStorage,
        Join,
        ReadExpect,
    },
    utils::fps_counter::FpsCounter,
};
use crate::{
    components::Map,
    resources::Sprites,
    util::spawn::{ spawn_creep, spawn_tower, random_room },
    config::{
        Spawner as SpawnerConfig,
        Tower as TowerConfig,
    },
};
use std::usize;

#[derive(Default)]
pub struct Spawner {
    pub elapsed: f32,
    pub creep_count: usize,
    pub tower_count: usize,
}

impl Spawner {
    fn spawn_creep(
        &mut self,
        entities: &Entities,
        lazy_update: &LazyUpdate,
        sprites_resource: &Read<Sprites>,
        map: &Map,
        spawner_config: &SpawnerConfig,
    ) {
        if self.creep_count < spawner_config.max_spawns.unwrap_or(usize::MAX) {
            if let Ok(room) = random_room(map) {
                spawn_creep(entities, lazy_update, sprites_resource, room, spawner_config);
                self.creep_count += 1;
            }
        }
    }

    fn spawn_towers(
        &mut self,
        entities: &Entities,
        lazy_update: &LazyUpdate,
        sprites_resource: &Read<Sprites>,
        map: &Map,
        spawner_config: &SpawnerConfig,
        tower_config: &TowerConfig,
    ) {
        let rooms = map.rooms();
        while self.tower_count < spawner_config.max_towers.unwrap_or(5).min(rooms.len()) {
            if rooms.len() == 0 { return } 

            let room = &rooms[self.tower_count % rooms.len()];
            spawn_tower(entities, lazy_update, sprites_resource, room, tower_config);

            self.tower_count += 1;
        }
    }
}

impl<'s> System<'s> for Spawner {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Option<Read<'s, Sprites>>,
        Read<'s, Time>,
        Read<'s, FpsCounter>,
        ReadStorage<'s, Map>,
        ReadExpect<'s, SpawnerConfig>,
        ReadExpect<'s, TowerConfig>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities, 
            lazy_update,
            sprites_resource,
            time,
            fps,
            maps,
            spawner_config,
            tower_config,
        ) = data;

        if sprites_resource.is_none() { 
            return;
        }

        let mut map = None;

        for (e, m) in (&entities, &maps).join() {
            if entities.is_alive(e) {
                map = Some(m);
                break;
            }
        }

        if map.is_none() { 
            return;
        }

        if self.tower_count == 0 {
            self.spawn_towers(
                &entities,
                &lazy_update,
                sprites_resource.as_ref().unwrap(),
                &map.unwrap(),
                &spawner_config,
                &tower_config,
            );
        }

        let delta_seconds = time.delta_seconds();
        self.elapsed += delta_seconds;

        if self.elapsed >= spawner_config.spawn_delay {
            self.elapsed -= spawner_config.spawn_delay ;
            let fps = fps.sampled_fps();
            if fps > 59.5 {
                self.spawn_creep(
                    &entities,
                    &lazy_update,
                    sprites_resource.as_ref().unwrap(),
                    &map.unwrap(),
                    &spawner_config,
                );
            }
        }
        
    }
}

