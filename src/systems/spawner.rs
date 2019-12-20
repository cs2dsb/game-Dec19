use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{
        Read, 
        System, 
        Entities, 
        LazyUpdate,
        Builder,
        ReadStorage,
        Join,
        ReadExpect,
    },
    utils::fps_counter::FpsCounter,
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
use std::usize;

#[derive(Default)]
pub struct Spawner {
    pub elapsed: f32,
    pub count: usize,
}

//TODO: LazyUpdate is convenient for dev but it causes frame rate jitter when spawning lots of entities at the end of the frame
impl Spawner {
    fn spawn_boid(
        &mut self,
        entities: &Entities,
        lazy_update: &LazyUpdate,
        sprites_resource: &Read<Sprites>,
        map: &Map,
        spawner_config: &SpawnerConfig,
    ) {
        if let Some(room) = map
                    .rooms()
                    .choose(&mut thread_rng())
        {
            let centre = room.centre();
            let (map_x, map_y) = (centre.x, centre.z);
            let transform = {
                let mut screen_pos = iso_to_screen(map_x, map_y);
                screen_pos.z += CHARACTER_Z_OFFSET;
                
                let mut transform = Transform::default();
                transform.set_translation(screen_pos);
                transform
            };
            let navigator = Navigator {
                x: map_x as usize,
                y: map_y as usize,
            };

            let sprite_components = sprites_resource.get_character_1_components();            
            let mut builder = lazy_update
                .create_entity(entities)
                .with(Animation::default())
                .with(Transparent)
                .with(transform)
                .with(Velocity::rand(spawner_config.min_speed, spawner_config.max_speed))
                .with(Age {
                    age: 0.,
                    max_age: spawner_config.max_age,
                })
                .with(navigator)
                .with(DebugLinesComponent::new())
                ;

            builder = sprite_components.apply(builder);
            builder.build();

            self.count += 1;
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

        let delta_seconds = time.delta_seconds();
        self.elapsed += delta_seconds;

        if self.elapsed >= spawner_config.spawn_delay {
            self.elapsed -= spawner_config.spawn_delay ;
            let fps = fps.sampled_fps();
            if fps > 59.5  && self.count < spawner_config.max_spawns.unwrap_or(usize::MAX) {
                self.spawn_boid(
                    &entities,
                    &lazy_update,
                    &sprites_resource.unwrap(),
                    &map.unwrap(),
                    &spawner_config,
                );
            }
        }
        
    }
}

