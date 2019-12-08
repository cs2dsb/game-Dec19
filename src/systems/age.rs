use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Read, 
        System, 
        WriteStorage,
        Join,
    },
};
use crate::components::{
    Age as AgeComponent,
};

pub struct Age;

impl<'s> System<'s> for Age {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, AgeComponent>,
    );

    fn run(&mut self, (time, mut ages): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        for a in (&mut ages).join() {
            a.age += delta_seconds;
        }
    }
}