use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugDraw {
    pub pathfinding: bool,
    pub velocity: bool,
    pub tower_range: bool,
    pub tower_target: bool,
    pub tower_los: bool,
}

impl Default for DebugDraw {
  fn default() -> Self {
    Self {
        pathfinding: false,
        velocity: false,
        tower_range: false,
        tower_target: false,
        tower_los: false,
    }
  }
}