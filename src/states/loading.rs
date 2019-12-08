use amethyst::{
    prelude::*,
    assets::{
        ProgressCounter,
        Completion,
    },
};
use crate::{
    resources::{
        Sprites,
        SpritesLoader,
    },
    states::Running,
};

#[derive(Default)]
pub struct Loading {
    sprites_loader: Option<SpritesLoader>,
}

impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;     

        let sprites_loader = SpritesLoader::new(world, ProgressCounter::default());
        self.sprites_loader = Some(sprites_loader);
    }

     fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(sprites_loader) = self.sprites_loader.as_ref() {
            match sprites_loader.complete() {
                Completion::Loading => Trans::None,
                Completion::Complete => {
                    log::info!("Asset loading complete.");

                    // Create the sprites resource now that it's assets are loaded
                    let sprites = Sprites::new(&mut data.world, sprites_loader)
                        .expect("Failed to create Sprites after loading complete");
                        
                    // Insert it into the world
                    data.world.insert(sprites);
                    
                    Trans::Switch(Box::new(Running))
                },
                Completion::Failed => {
                    log::error!("Failed to load assets, exiting");
                    Trans::Quit
                },
            }
        } else {
            log::error!("Missing sprites_loader");
            Trans::Quit
        }
    }
}