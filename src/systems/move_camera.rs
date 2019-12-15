use amethyst::{
    core::{
        timing::Time,
        transform::Transform,
    },
    ecs::prelude::{
        Join, 
        System, 
        WriteStorage,
        Read,
        ReadExpect,
    },
    renderer::{Camera, camera::Projection},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};
use crate::resources::Zoom;

const MOVE_VELOCITY: f32 = 1000.;

pub struct MoveCamera;

impl<'s> System<'s> for MoveCamera {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Zoom>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut cameras, mut transforms, time, input, zoom, screen_dims): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        for (camera, transform) in (&mut cameras, &mut transforms).join() {
            let x = input.axis_value("move_x").expect("Missing move_x input");
            let y = -input.axis_value("move_z").expect("Missing move_z input");

            transform.prepend_translation_x(MOVE_VELOCITY * x * delta_seconds);
            transform.prepend_translation_y(MOVE_VELOCITY * y * delta_seconds);

            let half_width = screen_dims.width() * 0.5 * zoom.zoom;
            let half_height = screen_dims.height() * 0.5 * zoom.zoom;
            match camera.projection_mut() {
                Projection::Orthographic(o) => {
                    o.set_left_and_right(-half_width, half_width);
                    o.set_bottom_and_top(-half_height, half_height);
                },
                _ => {},
            }
        }
    }
}