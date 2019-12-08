use amethyst::{
    ecs::prelude::{Join, System, WriteStorage, Entities},
    renderer::debug_drawing::DebugLinesComponent,
};

pub struct ClearDebug;

impl<'s> System<'s> for ClearDebug {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (entities, mut debug): Self::SystemData) {
        for (_, d) in (&entities, &mut debug).join() {
            d.clear();
        }
    }
}