use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Read, 
        System, 
        Entities, 
        Join,
    },
    utils::fps_counter::FpsCounter,
};

const PRINT_FREQ: f32 = 2.;

#[derive(Default)]
pub struct FpsLog {
    pub print_elapsed: f32,
}

impl<'s> System<'s> for FpsLog {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        Read<'s, FpsCounter>,
    );

    fn run(&mut self, (entities, time, fps): Self::SystemData) {
        
        let delta_seconds = time.delta_seconds();
        self.print_elapsed += delta_seconds;

        if self.print_elapsed >= PRINT_FREQ {
            self.print_elapsed -= PRINT_FREQ;

            let mut count = 0;
            for _ in (&entities).join() {
                count += 1;
            }

            let fps = fps.sampled_fps();
            println!("FPS: {}, Entities: {}", fps, count);
        }
    }
}

