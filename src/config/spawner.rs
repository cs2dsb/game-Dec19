use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Spawner {
    pub spawn_delay: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub max_spawns: Option<usize>,
    pub max_age: Option<f32>,
    pub tower_range: f32,
    pub max_towers: Option<usize>,
}

impl Default for Spawner {
  fn default() -> Self {
    Self {
        spawn_delay: 0.5,
        min_speed: 1.,
        max_speed: 50.,
        max_spawns: Some(10),
        max_age: None,
        tower_range: 10.,
        max_towers: Some(10),
    }
  }
}