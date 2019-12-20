use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugDraw {
    pub pathfinding: bool,
    pub velocity: bool,
}

impl Default for DebugDraw {
  fn default() -> Self {
    Self {
        pathfinding: false,
        velocity: false,
    }
  }
}