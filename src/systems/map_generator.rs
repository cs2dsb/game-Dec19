use amethyst::{
    core::transform::Transform,
    ecs::prelude::{
        Entities,
        System, 
        ReadStorage,
        ReadExpect,
        LazyUpdate,
        Read,
        Builder,
    },
    renderer::Transparent,
};
use crate::{
    components::Map,
    config::Map as MapConfig,
    resources::{ 
        Sprites,
        TileDirection,
    },
    util::constants::{
        FLOOR_Z,
        WALL_Z,
    },
};

pub struct MapGenerator;

fn tile_map(neigh: u32) -> u32 {
    match neigh {
        0 => 0, 
        2 => 1, 
        8 => 2, 
        10 => 3, 
        11 => 4, 
        16 => 5, 
        18 => 6,
        22 => 7, 
        24 => 8, 
        26 => 9, 
        27 => 10, 
        30 => 11, 
        31 => 12, 
        64 => 13,
        66 => 14, 
        72 => 15, 
        74 => 16, 
        75 => 17,
        80 => 18,
        82 => 19,
        86 => 20,
        88 => 21,
        90 => 22,
        91 => 23,
        94 => 24,
        95 => 25,
        104 => 26,
        106 => 27,
        107 => 28,
        120 => 29,
        122 => 30,
        123 => 31,
        126 => 32,
        127 => 33,
        208 => 34,
        210 => 35,
        214 => 36,
        216 => 37,
        218 => 38,
        219 => 39,
        222 => 40,
        223 => 41,
        248 => 42,
        250 => 43,
        251 => 44,
        254 => 45,
        255 => 46,
        _ => unreachable!(),
    }
}

const TILE_W: f32 = 32.;
const TILE_H: f32 = 32.;

fn iso_to_screen(x: f32, y: f32) -> (f32, f32) {
    (
        (x - y) * TILE_W,
        (x + y) * TILE_H / 2.,
    )
}

impl<'s> System<'s> for MapGenerator {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Map>,
        ReadExpect<'s, MapConfig>,
        Option<Read<'s, Sprites>>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, maps, map_config, sprites, lazy_update): Self::SystemData) {
        if maps.count() == 0 && 
            sprites.is_some() //The sprites resource is created in the loading state but this system gets called immediatly
        {
            let w = map_config.width;
            let h = map_config.height;
            let mut map = Map::new(w, h);
            let w = w as i32;
            let h = h as i32;
            map.generate(&map_config);

            let is_wall = |x, y| {
                x < 0 || x >= w || y < 0 || y >= h || map.is_wall(x as u32, y as u32)
            };
        
            for x in (0..w).rev() {
                for y in (0..h).rev() {
                    let this = is_wall(x, y);
                    
                    let wall = {
                        if !this {
                            None
                        } else {
                            let south = is_wall(x - 1, y);
                            let north = is_wall(x + 1, y);
                            let west = is_wall(x, y - 1);
                            let east = is_wall(x, y + 1);
                            let north_east = north && east && is_wall(x + 1, y + 1);
                            let north_west = north && west && is_wall(x + 1, y - 1);
                            let south_east = south && east && is_wall(x - 1, y + 1);
                            let south_west = south && west && is_wall(x - 1, y - 1);

                            let neighbour_index = 
                                1  * (north_west as i32) +  2  * (north as i32) +     4   * (north_east as i32) +
                                8  * (west as i32) +                                  16  * (east as i32) +
                                32 * (south_west as i32) +  64 * (south as i32) +     128 * (south_east as i32);

                            let tile = tile_map(neighbour_index as u32);    

                            Some(match tile {
                                36 => TileDirection::West,
                                41 => TileDirection::InnerCornerNorthWest,
                                12 => TileDirection::North,
                                28 => TileDirection::East,
                                33 => TileDirection::InnerCornerNorthEast,
                                42 => TileDirection::South,
                                45 => TileDirection::InnerCornerSouthWest,
                                44 => TileDirection::InnerCornerSouthEast,
                                46 => TileDirection::Solid,
                                7 => TileDirection::OuterCornerSouthEast,
                                4 => TileDirection::OuterCornerSouthWest,
                                34 => TileDirection::OuterCornerNorthEast,
                                26 => TileDirection::OuterCornerNorthWest,
                                // The following aren't exact matches
                                35 | 14 | 19 | 20 => TileDirection::West,
                                8 | 11 | 10 | 9 => TileDirection::North,
                                27 => TileDirection::East,
                                29 => TileDirection::South,

                                40 | 17 | 37 | 25 | 
                                21 | 15 | 16 | 39 |
                                22 | 24 | 23 | 38 => TileDirection::InnerCornerNorthWest,
                                31 => TileDirection::InnerCornerNorthEast,
                                43 => TileDirection::InnerCornerSouthWest,

                                1 | 5 | 6 => TileDirection::OuterCornerSouthEast,
                                13 | 18 => TileDirection::OuterCornerNorthEast,
                                2 | 3 => TileDirection::OuterCornerSouthWest,

                                32 => TileDirection::Solid,
                                //This is a bad match (it's a single column)
                                0 => TileDirection::OuterCornerSouthEast,

                                t => {
                                    log::info!("Blob: {}", t);
                                    TileDirection::Blob(t as usize)
                                },
                            })
                        }
                    };

                    create_tile(
                        &entities,
                        &lazy_update,
                        sprites.as_ref().expect("Missing sprites resource"),
                        TileDirection::Floor,
                        (x as f32, y as f32, FLOOR_Z),
                    );

                    if let Some(wall) = wall {
                        create_tile(
                            &entities,
                            &lazy_update,
                            sprites.as_ref().expect("Missing sprites resource"),
                            wall,
                            (x as f32, y as f32, WALL_Z),
                        );
                    }
                }
            }

            lazy_update.create_entity(&entities)
                .with(map)
                .build();
        }
    }
}

fn create_tile(
        entities: &Entities,
        lazy_update: &LazyUpdate,
        sprites_resource: &Sprites,
        direction: TileDirection,
        (x, y, z): (f32, f32, f32),
) {
    let (x, y) = iso_to_screen(x, y);
    let transform = {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, z);
        transform
    };

    let sprite = sprites_resource.get_tile(direction);
    
    lazy_update
        .create_entity(entities)
        .with(sprite)
        .with(transform)
        .with(Transparent)
        .build();
}


/*
fn add_tile(world: &mut World, tile: TileDirection, x: f32, y: f32) {
    let sprite = world.read_resource::<Sprites>().get_tile(tile);

    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    world
        .create_entity()
        .with(sprite)
        .with(transform)
        .with(Transparent)
        .build();
}

fn init_room(world: &mut World) {
    for i in (0..40).rev() {
        add_tile(world, TileDirection::Left,
            50. + 32. * (i as f32),
            50. + 16. * (i as f32),
        );
    }
}*/