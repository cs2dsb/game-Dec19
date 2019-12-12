use amethyst::{
    core::{
        timing::Time,
        transform::Transform,
    },
    ecs::prelude::{
        Join, 
        ReadStorage, 
        System, 
        WriteStorage,
        Read,
    },
    renderer::Camera,
    input::{InputHandler, StringBindings},
};

const MOVE_VELOCITY: f32 = 100.;

pub struct MoveCamera;

impl<'s> System<'s> for MoveCamera {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (cameras, mut transforms, time, input): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            let x = input.axis_value("move_x").expect("Missing move_x input");
            let y = -input.axis_value("move_z").expect("Missing move_z input");

            transform.prepend_translation_x(MOVE_VELOCITY * x * delta_seconds);
            transform.prepend_translation_y(MOVE_VELOCITY * y * delta_seconds);
        }
    }
}