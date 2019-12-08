#[allow(unused_imports)]
use crate::systems::{
    Heading,
    Animator,
    Mover,
    DebugDraw,
    ClearDebug,
    Bouncer,
    Spawner,
    FpsLog,
};
use amethyst::{
    core::{
        bundle::SystemBundle,
    },
    ecs::prelude::{
        DispatcherBuilder,
        World,
    },
    error::Error,
};

pub struct Bundle;

impl<'a, 'b> SystemBundle<'a, 'b> for Bundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(ClearDebug, "clear_debug_system", &[]);
        builder.add(Mover, "mover_system", &[]); 
        builder.add(Bouncer, "bouncer_system", &["mover_system"]);
        builder.add(Heading, "heading_system", &["bouncer_system"]);     
        builder.add(Animator, "animator_system", &["heading_system"]);  
        builder.add(DebugDraw, "debug_draw_system", &["mover_system"]);
        builder.add(Spawner::default(), "spawner_system", &[]);
        builder.add(FpsLog::default(), "fps_log_system", &[]);
        Ok(())
    }
}