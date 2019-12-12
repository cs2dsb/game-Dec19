use amethyst::{
    ecs::prelude::{
        Entities,
        System, 
        ReadStorage,
        ReadExpect,
        LazyUpdate,
        Read,
        Builder,
    },
};
use crate::{
    components::Map,
    config::Map as MapConfig,
};

pub struct MapGenerator;

impl<'s> System<'s> for MapGenerator {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Map>,
        ReadExpect<'s, MapConfig>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, maps, map_config, lazy_update): Self::SystemData) {
        if maps.count() == 0 {
            let mut map = Map::new(map_config.width, map_config.height);
            map.generate(&map_config);
            
            lazy_update.create_entity(&entities)
                .with(map)
                .build();
        }
    }
}