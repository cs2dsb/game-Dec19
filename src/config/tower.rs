use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tower {
    pub range: f32,
    pub frequency: f32,
    pub projectile_speed: f32,
    /// Jitter in radians
    pub aim_jitter: f32,
}

impl Default for Tower {
  fn default() -> Self {
    Self {
        range: 10.,
        frequency: 0.5,
        projectile_speed: 200.,
        aim_jitter: 0.,
    }
  }
}