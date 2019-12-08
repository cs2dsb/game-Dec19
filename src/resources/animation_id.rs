use amethyst::animation::EndControl;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
    WalkUpLeft,
    WalkDownLeft,
    WalkUpRight,
    WalkDownRight,
}

impl AnimationId {
    pub fn end_control(&self) -> EndControl {
        match self {
            _ => EndControl::Loop(None),
        }
    }
    pub fn is_terminal(&self) -> bool {
        match self {
            _ => false,
        }
    }
}