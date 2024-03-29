#[allow(unused_imports)]
use crate::systems::{
    Heading,
    Animator,
    NavigatorMover,
    ProjectileMover,
    DebugDraw,
    ClearDebug,
    Bouncer,
    Spawner,
    FpsLog,
    Age,
    Murder,
    MoveCamera,
    MapGenerator,
    PathFinder,
    TowerAim,
    TowerShoot,
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
        builder.add(MapGenerator, "map_generator_system", &[]);
        builder.add(ClearDebug, "clear_debug_system", &[]);
        builder.add(NavigatorMover, "navigator_mover_system", &[]); 
        builder.add(ProjectileMover, "projectile_mover_system", &[]); 
        //builder.add(Bouncer, "bouncer_system", &["mover_system"]);
        builder.add(Heading, "heading_system", &["navigator_mover_system"]);     
        builder.add(DebugDraw, "debug_draw_system", &["navigator_mover_system", "projectile_mover_system"]);
        builder.add(Spawner::default(), "spawner_system", &[]);
        builder.add(FpsLog::default(), "fps_log_system", &[]);
        builder.add(Age, "age_system", &[]);
        builder.add(Murder, "murder_system", &["age_system"]);
        builder.add(Animator, "animator_system", &["heading_system"]);  
        builder.add(MoveCamera, "move_camera_system", &[]);
        builder.add(PathFinder, "path_finder_system", &[]);
        builder.add(TowerAim, "tower_aim_system", &["navigator_mover_system"]);
        builder.add(TowerShoot, "tower_shoot_system", &["tower_aim_system"]);
        Ok(())
    }
}