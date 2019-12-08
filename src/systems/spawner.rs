use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{
        Read, 
        System, 
        Entities, 
        ReadExpect,
        LazyUpdate,
        Builder,
    },
    utils::fps_counter::FpsCounter,
    window::ScreenDimensions,
    renderer::{
        debug_drawing::DebugLinesComponent,
        Transparent,
    },
};
use rand::random;
use crate::{
    components::{
        Velocity,
        Animation,
    },
    resources::Sprites,
};


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
        screen_dimensions: &ReadExpect<ScreenDimensions>,
    ) {
        let transform = {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                random::<f32>() * screen_dimensions.width(),
                random::<f32>() * screen_dimensions.height(),
                0.,
            );
            transform
        };

        let velocity = Velocity::rand(10., 100.);
        let animation = Animation::default();

        let sprite_components = sprites_resource.get_character_1_components();
        
        let mut builder = lazy_update
            .create_entity(entities)
            .with(animation)
            .with(Transparent)
            .with(transform)
            .with(velocity)
            //.with(DebugLinesComponent::new())
            ;

        builder = sprite_components.apply(builder);
        builder.build();

        self.count += 1;
    }
}

impl<'s> System<'s> for Spawner {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Option<Read<'s, Sprites>>,
        Read<'s, Time>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, FpsCounter>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities, 
            lazy_update,
            sprites_resource,
            time,
            screen_dimensions,
            fps,
        ) = data;

        if sprites_resource.is_none() { 
            return;
        }

        let delta_seconds = time.delta_seconds();
        self.elapsed += delta_seconds;

        if self.elapsed >= 0.02 {
            self.elapsed -= 0.02;
            let fps = fps.sampled_fps();
            if fps > 59.5  && self.count < 10000 {
                self.spawn_boid(
                    &entities,
                    &lazy_update,
                    &sprites_resource.unwrap(),
                    &screen_dimensions,
                );
            }
        }
    }
}

