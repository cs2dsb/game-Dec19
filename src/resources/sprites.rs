use amethyst::{
    assets::{
        AssetStorage, 
        Loader,
        ProgressCounter,
        RonFormat,
        Handle,
        Completion,
    },
    ecs::prelude::{Builder, World, WorldExt},
    renderer::{
        ImageFormat, 
        SpriteRender, 
        SpriteSheet, 
        SpriteSheetFormat, 
        Texture,
    },
    animation::{
        AnimationControlSet,
        EndControl,
        AnimationCommand,
        Animation,
        SpriteRenderPrimitive,
        Sampler,
        InterpolationFunction,
        SpriteRenderChannel,
        AnimationSet,
    },
};
use crate::resources::{
    AnimationId,
    NamedSpriteSheet,
    NamedSpriteSheetHandle,
    NamedAnimationSet,
    NamedAnimationSetHandle,
};

const SPRITE_SHEET_RON: &str = "sprite_sheets/character_0.ron";
const ANIMATION_SET_RON: &str = "animations/character_0.ron";
const SPRITE_SHEET_PNG: &str = "sprite_sheets/character_0.png";

#[derive(Debug)]
pub enum Error {
    AssetLoadingIncomplete,
}

/// Holds handles to required assets
pub struct SpritesLoader {
    named_sprites_handle: NamedSpriteSheetHandle,
    named_animation_handle: NamedAnimationSetHandle,
    character_1_sheet_handle: Handle<SpriteSheet>,
    progress_counter: ProgressCounter,
}

impl SpritesLoader {
    /// Creates a new SpritesLoader which kicks off loading of the assets required by Sprites
    pub fn new(world: &mut World, mut progress_counter: ProgressCounter) -> Self {  
        let named_sprites_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                SPRITE_SHEET_RON,
                RonFormat,
                &mut progress_counter,
                &world.read_resource::<AssetStorage<NamedSpriteSheet>>(),
            )
        };  

        let named_animation_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                ANIMATION_SET_RON,
                RonFormat,
                &mut progress_counter,
                &world.read_resource::<AssetStorage<NamedAnimationSet>>(),
            )
        };

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                SPRITE_SHEET_PNG,
                ImageFormat::default(),
                &mut progress_counter,
                &texture_storage,
            )
        };

        let character_1_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                SPRITE_SHEET_RON,
                SpriteSheetFormat(texture_handle),
                &mut progress_counter,
                &sheet_storage,
            )
        };

        Self {
            named_sprites_handle,
            named_animation_handle,
            character_1_sheet_handle,
            progress_counter,
        }
    }

    pub fn complete(&self) -> Completion {
        self.progress_counter.complete()
    }

    pub fn is_complete(&self) -> bool {
        self.progress_counter.is_complete()
    }
}

/// Holds handles to loaded assets for sprites and animations
pub struct Sprites {
    character_1_sprite_render: SpriteRender,
    character_1_animation_set: AnimationSet<AnimationId, SpriteRender>,
}

impl Sprites {
    /// Creates a new Sprites resources. Assets added to the progress counter by SpritesLoader must have finished loading before this is called
    pub fn new(world: &mut World, sprites_loader: &SpritesLoader) -> Result<Self, Error> {
        if !sprites_loader.is_complete() {
            return Err(Error::AssetLoadingIncomplete);
        }

        let SpritesLoader {
            named_sprites_handle,
            named_animation_handle,
            character_1_sheet_handle,
            ..
        } = sprites_loader;

        let mut character_1_animation_set = AnimationSet::new();
        {
            let named_sprites_storage = world.read_resource::<AssetStorage<NamedSpriteSheet>>();
            let named_sprites = named_sprites_storage
                .get(&named_sprites_handle)
                .expect(&format!("NamedSpriteSheet {:?} missing in Sprites::new", named_sprites_handle));

            let animation_set_storage = world.read_resource::<AssetStorage<NamedAnimationSet>>();
            let named_animations = animation_set_storage
                .get(&named_animation_handle)
                .expect(&format!("NamedAnimationSet {:?} missing in Sprites::new", named_animation_handle));

            let mut sampler_storage = world.write_resource::<AssetStorage<Sampler<SpriteRenderPrimitive>>>();
            let mut animation_storage = world.write_resource::<AssetStorage<Animation<SpriteRender>>>();

            for anim in named_animations.animations.iter() {
                assert_eq!(anim.input.len(), anim.output.len(), 
                    "Animation input and output vectors must have the same length");

                let mut sampler: Sampler<SpriteRenderPrimitive> = Sampler {
                    input: anim.input.clone(),
                    output: Vec::new(),
                    function: InterpolationFunction::Step,
                };

                for name in anim.output.iter() {
                    let frame = named_sprites
                        .sprites
                        .iter()
                        .position(|x| &x.name == name)
                        .expect(&format!("Failed to find frame named {}", name));
                    let sprite_index = SpriteRenderPrimitive::SpriteIndex(frame);
                    sampler.output.push(sprite_index);
                }
                let sampler_handle = sampler_storage.insert(sampler);

                let animation_handle = animation_storage.insert(Animation {
                    nodes: vec![
                        (0, SpriteRenderChannel::SpriteIndex, sampler_handle),
                    ],
                });

                character_1_animation_set.animations.insert(anim.id, animation_handle);
            }
        }

        let character_1_sprite_render = SpriteRender {
            sprite_sheet: character_1_sheet_handle.clone(),
            sprite_number: 0,
        };

        Ok(Self {
            character_1_sprite_render,
            character_1_animation_set
        })
    }

    pub fn get_character_1_components(&self) -> AnimatedSpriteComponents {
        AnimatedSpriteComponents {
            sprite_render: self.character_1_sprite_render.clone(),
            animation_set: self.character_1_animation_set.clone(),
            control_set: AnimationControlSet::default(),
            default_animation: None
        }
    }
}

pub struct AnimatedSpriteComponents {
    // The sprite sheet
    pub sprite_render: SpriteRender,
    // The set of animations
    pub animation_set: AnimationSet<AnimationId, SpriteRender>,
    // The animation controls
    pub control_set: AnimationControlSet<AnimationId, SpriteRender>,
    // Optional default animation to add to control_set. Assumed to loop forever and is started immediatly
    pub default_animation: Option<AnimationId>,
}

impl AnimatedSpriteComponents {
    pub fn apply<B>(mut self, builder: B) -> B 
    where
        B: Builder
    {
        if let Some(default_animation) = self.default_animation.take() {
            let anim = self.animation_set.get(&default_animation).expect("AnimatedSpriteComponents.default_animation was missing from animation set");
            self.control_set.add_animation(
                default_animation,
                anim,
                EndControl::Loop(None),
                1.0,
                AnimationCommand::Start,
            );
        }

        builder
            .with(self.sprite_render)
            .with(self.animation_set)
            .with(self.control_set)
    }
}