#![feature(const_fn)]

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow, RenderDebugLines},
        types::DefaultBackend,
        RenderingBundle,
        sprite::SpriteRender,
    },
    utils::{
        application_root_dir,
        fps_counter::FpsCounterBundle,
    },
    //config::Config,
    animation::AnimationBundle,
    assets::{
        //PrefabLoaderSystemDesc,
        Processor,
    },
    input::{InputBundle, StringBindings},
    controls::MouseFocusUpdateSystemDesc,
};
use log::LevelFilter;

pub mod states;
pub mod resources;
pub mod components;
pub mod systems;
pub mod config;
pub mod util;

fn create_logger(level: LevelFilter) {
  use std::io;

  let gfx_device_gl_level = if level > LevelFilter::Warn {
    LevelFilter::Warn
  } else {
     level
  };

  let color_config = fern::colors::ColoredLevelConfig::new();
  fern::Dispatch::new()
    .chain(io::stdout())
    .level(level)
    .level_for("gfx_device_gl", gfx_device_gl_level)
    .format(move |out, message, record| {
      let color = color_config.get_color(&record.level());
      out.finish(format_args!(
        "{time}: [{level}][{target}] {color}{message}{color_reset}",
        time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        level = record.level(),
        target = record.target(),
        color = format!("\x1B[{}m", color.to_fg_str()),
        message = message,
        color_reset = "\x1B[0m",
      ))
    })
    .apply()
    .expect("Failed to create fern logger");
}

pub fn run() -> amethyst::Result<()> {
    create_logger(LevelFilter::Info);

    let app_root_path = application_root_dir()?;
    let assets_path = app_root_path.join("assets");
    let config_path = app_root_path.join("config");
    let display_config_path = config_path.join("display_config.ron");
    let binding_config_path = config_path.join("binding_config.ron");
    let game_config_path = config_path.join("game_config.ron");

    let game_config = config::Game::load_no_fallback(&game_config_path).expect("Failed to load game config");

    let game_data = GameDataBuilder::default()
        .with(Processor::<resources::NamedSpriteSheet>::new(), "", &[])
        .with(Processor::<resources::NamedAnimationSet>::new(), "", &[])
        /*
        .with_system_desc(
            PrefabLoaderSystemDesc::<resources::MyPrefabData>::default(),
            "scene_loader",
            &[],
        )
        */
        .with_bundle(            
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(binding_config_path)?,
        )?
        .with_bundle(AnimationBundle::<resources::AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.2, 0.1, 0.2, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(systems::Bundle)?
        .with_system_desc(
            MouseFocusUpdateSystemDesc::default(),
            "mouse_focus_update_system",
            &[]);

    let mut builder = Application::build(
        assets_path, 
        states::Loading::default(),
    )?;
    builder = game_config.register(builder);
    builder = components::register_components(builder);
    
    let mut game = builder
        .build(game_data)?;

    game.run();

    Ok(())
}
