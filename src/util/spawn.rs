use amethyst::{
    core::{
        transform::Transform,
        math::Vector2,
    },
    ecs::prelude::{
        Read, 
        Entities, 
        LazyUpdate,
        Builder,
        Entity,
    },
    renderer::{
        debug_drawing::DebugLinesComponent,
        Transparent,
    },
};
use crate::{
    components::{
        Velocity,
        Animation,
        Age,
        Navigator,
        Map,
        Creep,
        BulletTower,
        Tower,
        map::MapObject,
    },
    resources::Sprites,
    util::{
        constants::CHARACTER_Z_OFFSET,
        iso_to_screen,
    },
    config::Spawner as SpawnerConfig,
};
use rand::{
    thread_rng,
    seq::SliceRandom,
};

#[derive(Debug)]
pub enum Error {
    MapHasNoRooms,
}

#[derive(Debug, Clone, Copy)]
pub enum SpawnType {
    Creep,
    BulletTower,
}

pub fn random_room(map: &Map) -> Result<&MapObject, Error> {
    map
        .rooms()
        .choose(&mut thread_rng())
        .ok_or(Error::MapHasNoRooms)
}

pub fn spawn(
    entities: &Entities,
    lazy_update: &LazyUpdate,
    sprites_resource: &Read<Sprites>,
    room: &MapObject,
    spawner_config: &SpawnerConfig,
    type_: SpawnType,
) -> Entity {
    
    let mut builder = lazy_update
        .create_entity(entities)
        .with(Animation::default())
        .with(Transparent);

    match type_ {
        SpawnType::Creep => {
            let centre = room.centre();
            let map_pos = Vector2::new(centre.x, centre.z);
            let transform = {
                let mut screen_pos = iso_to_screen(map_pos);
                screen_pos.z += CHARACTER_Z_OFFSET;
                
                let mut transform = Transform::default();
                transform.set_translation(screen_pos);
                transform
            };
            let sprite_components = sprites_resource.get_character_1_components();
            
            let navigator = Navigator {
                x: map_pos.x as usize,
                y: map_pos.y as usize,
            };

            builder = sprite_components.apply(builder);
            builder = builder
                .with(transform)
                .with(Creep)
                .with(Velocity::rand(spawner_config.min_speed, spawner_config.max_speed))
                .with(Age {
                    age: 0.,
                    max_age: spawner_config.max_age,
                })
                .with(navigator)
                .with(DebugLinesComponent::new());
        },
        SpawnType::BulletTower => {
            let pos = room.pos();
            let size = room.size();
            let map_pos = Vector2::new(pos.x + size.x - 1., pos.z + size.z - 1.);
            let transform = {
                let mut screen_pos = iso_to_screen(map_pos);
                screen_pos.z += CHARACTER_Z_OFFSET;
                
                let mut transform = Transform::default();
                transform.set_translation(screen_pos);
                transform
            };
            let sprite_components = sprites_resource.get_character_1_components();
            
            builder = sprite_components.apply(builder);
            builder = builder
                .with(transform)
                .with(Tower::new(spawner_config.tower_range))
                .with(BulletTower::default())
                .with(DebugLinesComponent::new());
        },
    }

    builder.build()
}