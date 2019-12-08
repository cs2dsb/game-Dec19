use amethyst::{
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
        
};
use log::info;
use crate::resources::{
    Sprites,
    AnimationId,
};

#[derive(Default)]
pub struct Running;

fn add_sprite(world: &mut World, animation: AnimationId, x: f32, y: f32) {
    let mut sprite_components = world.read_resource::<Sprites>().get_character_1_components();
    sprite_components.default_animation = Some(animation);
    
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 1.);
    {
        let scale = transform.scale_mut();
        scale.x = 1.;
        scale.y = 1.;
    }

    let mut builder = world.create_entity();
    builder = builder
        .with(transform);
    builder = sprite_components
        .apply(builder);
    builder.build();
}

impl SimpleState for Running {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);
        
        add_sprite(world, AnimationId::WalkRight, 150., 150.);       
        add_sprite(world, AnimationId::WalkLeft, 300., 150.);       
        add_sprite(world, AnimationId::WalkUp, 150., 300.);       
        add_sprite(world, AnimationId::WalkDown, 300., 300.);       

        log::info!("Running");
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || 
               is_key_down(&event, VirtualKeyCode::Escape) || 
               is_key_down(&event, VirtualKeyCode::Q) 
            {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }

        // Keep going
        Trans::None
    }
}



fn init_camera(world: &mut World) {
    let (width, height) = {
        let dimensions = world.read_resource::<ScreenDimensions>();
        (dimensions.width(), dimensions.height())
    };

    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}