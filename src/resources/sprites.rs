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
use regex::Regex;
use rand::{
    thread_rng,
    seq::SliceRandom,
};
use crate::resources::{
    AnimationId,
    NamedSpriteSheet,
    NamedSpriteSheetHandle,
    NamedAnimationSet,
    NamedAnimationSetHandle,
};

const CHARACTER_SPRITE_SHEET_RON: &str = "sprite_sheets/character_0.ron";
const CHARACTER_SPRITE_SHEET_PNG: &str = "sprite_sheets/character_0.png";
const CHARACTER_ANIMATION_SET_RON: &str = "animations/character_0.ron";
const TILESET_SPRITE_SHEET_RON: &str = "sprite_sheets/tiles.ron";
const TILESET_SPRITE_SHEET_PNG: &str = "sprite_sheets/tiles.png";


#[derive(Debug)]
pub enum Error {
    AssetLoadingIncomplete,
}

/// Holds handles to required assets
pub struct SpritesLoader {
    character_named_sprites_handle: NamedSpriteSheetHandle,
    character_named_animation_handle: NamedAnimationSetHandle,
    character_sheet_handle: Handle<SpriteSheet>,
    tiles_named_sprites_handle: NamedSpriteSheetHandle,
    tiles_sheet_handle: Handle<SpriteSheet>,
    progress_counter: ProgressCounter,
}

impl SpritesLoader {
    /// Creates a new SpritesLoader which kicks off loading of the assets required by Sprites
    pub fn new(world: &mut World, mut progress_counter: ProgressCounter) -> Self {  
        let character_named_sprites_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                CHARACTER_SPRITE_SHEET_RON,
                RonFormat,
                &mut progress_counter,
                &world.read_resource::<AssetStorage<NamedSpriteSheet>>(),
            )
        };  

        let character_named_animation_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                CHARACTER_ANIMATION_SET_RON,
                RonFormat,
                &mut progress_counter,
                &world.read_resource::<AssetStorage<NamedAnimationSet>>(),
            )
        };

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                CHARACTER_SPRITE_SHEET_PNG,
                ImageFormat::default(),
                &mut progress_counter,
                &texture_storage,
            )
        };

        let character_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                CHARACTER_SPRITE_SHEET_RON,
                SpriteSheetFormat(texture_handle),
                &mut progress_counter,
                &sheet_storage,
            )
        };

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                TILESET_SPRITE_SHEET_PNG,
                ImageFormat::default(),
                &mut progress_counter,
                &texture_storage,
            )
        };

        let tiles_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                TILESET_SPRITE_SHEET_RON,
                SpriteSheetFormat(texture_handle),
                &mut progress_counter,
                &sheet_storage,
            )
        };  

        let tiles_named_sprites_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                TILESET_SPRITE_SHEET_RON,
                RonFormat,
                &mut progress_counter,
                &world.read_resource::<AssetStorage<NamedSpriteSheet>>(),
            )
        };  

        Self {
            character_named_sprites_handle,
            character_named_animation_handle,
            character_sheet_handle,
            tiles_sheet_handle,
            tiles_named_sprites_handle,
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

pub enum TileDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeftCorner,
    UpLeftPoint,
    UpRightCorner,
    UpRightPoint,
    DownLeftCorner,
    DownLeftPoint,
    DownRightCorner,
    DownRightPoint,
}

#[derive(Debug)]
pub struct Tiles {
    up: Vec<SpriteRender>,
    down: Vec<SpriteRender>,
    left: Vec<SpriteRender>,
    right: Vec<SpriteRender>,
    up_left_corner: Vec<SpriteRender>,
    up_left_point: Vec<SpriteRender>,
    up_right_corner: Vec<SpriteRender>,
    up_right_point: Vec<SpriteRender>,
    down_left_corner: Vec<SpriteRender>,
    down_left_point: Vec<SpriteRender>,
    down_right_corner: Vec<SpriteRender>,
    down_right_point: Vec<SpriteRender>,
}

fn collect_named_sprites(regex: &str, named_sprites: &NamedSpriteSheet, sprite_sheet: Handle<SpriteSheet>) -> Vec<SpriteRender> {
    let regex = Regex::new(regex).expect("regex failed to compile");

    named_sprites
        .sprites.iter().enumerate()
        .filter(|(_, s)| regex.is_match(&s.name))
        .map(|(i, _)| SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: i,
        }).collect()
}

/// Holds handles to loaded assets for sprites and animations
pub struct Sprites {
    character_1_sprite_render: SpriteRender,
    character_1_animation_set: AnimationSet<AnimationId, SpriteRender>,
    tiles: Tiles,
}

impl Sprites {
    /// Creates a new Sprites resources. Assets added to the progress counter by SpritesLoader must have finished loading before this is called
    pub fn new(world: &mut World, sprites_loader: &SpritesLoader) -> Result<Self, Error> {
        if !sprites_loader.is_complete() {
            return Err(Error::AssetLoadingIncomplete);
        }

        let SpritesLoader {
            character_named_sprites_handle,
            character_named_animation_handle,
            character_sheet_handle,
            tiles_sheet_handle,
            tiles_named_sprites_handle,
            ..
        } = sprites_loader;

        let mut character_1_animation_set = AnimationSet::new();
        {
            let named_sprites_storage = world.read_resource::<AssetStorage<NamedSpriteSheet>>();
            let named_sprites = named_sprites_storage
                .get(&character_named_sprites_handle)
                .expect(&format!("NamedSpriteSheet {:?} missing in Sprites::new", character_named_sprites_handle));

            let animation_set_storage = world.read_resource::<AssetStorage<NamedAnimationSet>>();
            let named_animations = animation_set_storage
                .get(&character_named_animation_handle)
                .expect(&format!("NamedAnimationSet {:?} missing in Sprites::new", character_named_animation_handle));

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
            sprite_sheet: character_sheet_handle.clone(),
            sprite_number: 0,
        };

        let tiles = {
            let named_sprites_storage = world.read_resource::<AssetStorage<NamedSpriteSheet>>();
            let named_sprites = named_sprites_storage
                .get(&tiles_named_sprites_handle)
                .expect(&format!("NamedSpriteSheet {:?} missing in Sprites::new", tiles_named_sprites_handle));

            Tiles {
                up: collect_named_sprites(r"walls_up_\d+", &named_sprites, tiles_sheet_handle.clone()),
                down: collect_named_sprites(r"walls_down_\d+", &named_sprites, tiles_sheet_handle.clone()),
                left: collect_named_sprites(r"walls_left_\d+", &named_sprites, tiles_sheet_handle.clone()),
                right: collect_named_sprites(r"walls_right_\d+", &named_sprites, tiles_sheet_handle.clone()),
                up_left_corner: collect_named_sprites(r"walls_corner_up_left_\d+", &named_sprites, tiles_sheet_handle.clone()),
                up_left_point: collect_named_sprites(r"walls_point_up_left_\d+", &named_sprites, tiles_sheet_handle.clone()),
                up_right_corner: collect_named_sprites(r"walls_corner_up_right_\d+", &named_sprites, tiles_sheet_handle.clone()),
                up_right_point: collect_named_sprites(r"walls_point_up_right_\d+", &named_sprites, tiles_sheet_handle.clone()),
                down_left_corner: collect_named_sprites(r"walls_corner_down_left_\d+", &named_sprites, tiles_sheet_handle.clone()),
                down_left_point: collect_named_sprites(r"walls_point_down_left_\d+", &named_sprites, tiles_sheet_handle.clone()),
                down_right_corner: collect_named_sprites(r"walls_corner_down_right_\d+", &named_sprites, tiles_sheet_handle.clone()),
                down_right_point: collect_named_sprites(r"walls_point_down_right_\d+", &named_sprites, tiles_sheet_handle.clone()),                
            }
        };

        log::info!("{:#?}", tiles);

        Ok(Self {
            character_1_sprite_render,
            character_1_animation_set,
            tiles,
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

    pub fn get_tile(&self, tile: TileDirection) -> SpriteRender {
        match tile {
            TileDirection::Up => self.get_tile_up(),
            TileDirection::Down => self.get_tile_down(),
            TileDirection::Left => self.get_tile_left(),
            TileDirection::Right => self.get_tile_right(),
            TileDirection::UpLeftCorner => self.get_tile_up_left_corner(),
            TileDirection::UpLeftPoint => self.get_tile_up_left_point(),
            TileDirection::UpRightCorner => self.get_tile_up_right_corner(),
            TileDirection::UpRightPoint => self.get_tile_up_right_point(),
            TileDirection::DownLeftCorner => self.get_tile_down_left_corner(),
            TileDirection::DownLeftPoint => self.get_tile_down_left_point(),
            TileDirection::DownRightCorner => self.get_tile_down_right_corner(),
            TileDirection::DownRightPoint => self.get_tile_down_right_point(),
        }
    }

    pub fn get_tile_up(&self) -> SpriteRender {
        self
            .tiles
            .up
            .choose(&mut thread_rng())
            .expect("tiles.up vec was empty!")
            .clone()
    }

    pub fn get_tile_down(&self) -> SpriteRender {
        self
            .tiles
            .down
            .choose(&mut thread_rng())
            .expect("tiles.down vec was empty!")
            .clone()
    }

    pub fn get_tile_left(&self) -> SpriteRender {
        self
            .tiles
            .left
            .choose(&mut thread_rng())
            .expect("tiles.left vec was empty!")
            .clone()
    }

    pub fn get_tile_right(&self) -> SpriteRender {
        self
            .tiles
            .right
            .choose(&mut thread_rng())
            .expect("tiles.right vec was empty!")
            .clone()
    }

    pub fn get_tile_up_left_corner(&self) -> SpriteRender {
        self
            .tiles
            .up_left_corner
            .choose(&mut thread_rng())
            .expect("tiles.up_left_corner vec was empty!")
            .clone()
    }

    pub fn get_tile_up_left_point(&self) -> SpriteRender {
        self
            .tiles
            .up_left_point
            .choose(&mut thread_rng())
            .expect("tiles.up_left_point vec was empty!")
            .clone()
    }

    pub fn get_tile_up_right_corner(&self) -> SpriteRender {
        self
            .tiles
            .up_right_corner
            .choose(&mut thread_rng())
            .expect("tiles.up_right_corner vec was empty!")
            .clone()
    }

    pub fn get_tile_up_right_point(&self) -> SpriteRender {
        self
            .tiles
            .up_right_point
            .choose(&mut thread_rng())
            .expect("tiles.up_right_point vec was empty!")
            .clone()
    }

    pub fn get_tile_down_left_corner(&self) -> SpriteRender {
        self
            .tiles
            .down_left_corner
            .choose(&mut thread_rng())
            .expect("tiles.down_left_corner vec was empty!")
            .clone()
    }

    pub fn get_tile_down_left_point(&self) -> SpriteRender {
        self
            .tiles
            .down_left_point
            .choose(&mut thread_rng())
            .expect("tiles.down_left_point vec was empty!")
            .clone()
    }

    pub fn get_tile_down_right_corner(&self) -> SpriteRender {
        self
            .tiles
            .down_right_corner
            .choose(&mut thread_rng())
            .expect("tiles.down_right_corner vec was empty!")
            .clone()
    }

    pub fn get_tile_down_right_point(&self) -> SpriteRender {
        self
            .tiles
            .down_right_point
            .choose(&mut thread_rng())
            .expect("tiles.down_right_point vec was empty!")
            .clone()
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