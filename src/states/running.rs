use amethyst::{
    core::transform::Transform,
    input::{
        is_close_requested, 
        is_key_down, 
        VirtualKeyCode, 
        InputEvent,
        ScrollDirection,
    },
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
    controls::WindowFocus,
    // Why is this located here? Import direct from winnit instead?
    renderer::rendy::wsi::winit::MouseButton,
};
use crate::{
    resources::{Zoom, CameraMove},
    util::constants::CAMERA_Z,
};

const ZOOM_MAX: f32 = 2.5;
const ZOOM_MIN: f32 = 0.3;

#[derive(Default)]
pub struct Running {
    middle_mouse_down: bool,
}

impl SimpleState for Running {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);  
        log::info!("Running");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || 
                   is_key_down(&event, VirtualKeyCode::Escape) || 
                   is_key_down(&event, VirtualKeyCode::Q) 
                {
                    Trans::Quit
                } else {
                    Trans::None
                }
            },
            StateEvent::Input(input) => {
                if data.world.read_resource::<WindowFocus>().is_focused {
                    match input {
                        InputEvent::MouseWheelMoved(dir) => {
                            let zoom = data.world.read_resource::<Zoom>().zoom * match dir {
                                ScrollDirection::ScrollUp => 1.1,
                                ScrollDirection::ScrollDown => 0.9,
                                _ => 1.,
                            };
                            data.world.write_resource::<Zoom>().zoom = zoom.max(ZOOM_MIN).min(ZOOM_MAX);    
                        },
                        InputEvent::MouseButtonPressed(MouseButton::Middle) => self.middle_mouse_down = true,
                        InputEvent::MouseButtonReleased(MouseButton::Middle) => self.middle_mouse_down = false,
                        InputEvent::MouseMoved { delta_x, delta_y } => if self.middle_mouse_down {
                            let mut camera_move = data.world.write_resource::<CameraMove>();
                            // I don't know why x is switched but there we are
                            camera_move.delta.x -= delta_x;
                            camera_move.delta.y += delta_y;
                        },
                        _ => {},
                    }
                }
                Trans::None
            },
            _ => Trans::None,
        }
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
    transform.set_translation_xyz(width * 0.5, height * 0.5, CAMERA_Z);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}