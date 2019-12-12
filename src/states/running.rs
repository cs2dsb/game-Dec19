use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
    renderer::Transparent,
};
use crate::resources::{
    TileDirection,
    Sprites,
};

#[derive(Default)]
pub struct Running;

impl SimpleState for Running {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);  
        init_room(world);
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

            /*
            // Listen to any key events
            if let Some(event) = input::get_key(&event) {
                log::info!("handling key event: {:?}", event);
            }
            */
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

fn add_tile(world: &mut World, tile: TileDirection, x: f32, y: f32) {
    let sprite = world.read_resource::<Sprites>().get_tile(tile);

    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    world
        .create_entity()
        .with(sprite)
        .with(transform)
        .with(Transparent)
        .build();
}

fn init_room(world: &mut World) {
    for i in (0..40).rev() {
        add_tile(world, TileDirection::Left,
            50. + 32. * (i as f32),
            50. + 16. * (i as f32),
        );
    }
}