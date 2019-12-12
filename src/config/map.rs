use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Room {
  pub name: String,
  pub frequency: f32,
  pub min_size: (u32, u32),
  pub max_size: (u32, u32),
}

impl Default for Room {
  fn default() -> Self {
    Self {
      name: "Default".to_string(),
      frequency: 1.0,
      min_size: (8, 8),
      max_size: (8, 8),
    }
  }
}

///This is stuff that is used to create entities from the map
#[derive(Debug, Deserialize, Serialize)]
pub struct MapEntity {
  //What does "1" in map translate to in the real world
  pub unit_size: f32,
  //How tall are walls in multples of unit_size
  pub wall_height: f32,
}

impl Default for MapEntity {
  fn default() -> Self {
    Self {
      unit_size: 64.0,
      wall_height: 1.0,
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Map {
  pub width: u32,
  pub height: u32,
  pub room_place_max_iterations: u32,
  pub corridor_turn_chance: f32,
  pub superfluous_doors_per_room: f32,
  pub room_sizes: Vec<Room>,
  pub entity: MapEntity,
  //Stuff inside this radius will have physics colliders
  pub physics_load_radius: f32,
}

impl Default for Map {
  fn default() -> Self {
    Self {
      width: 64,
      height: 64,
      room_place_max_iterations: 400,
      corridor_turn_chance: 0.1,
      superfluous_doors_per_room: 0.5,
      room_sizes: vec!(Default::default()),
      entity: Default::default(),
      physics_load_radius: 1000.0,
    }
  }
}