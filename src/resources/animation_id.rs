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
    DieUp,
    DieDown,
    DieLeft,
    DieRight,
    DieUpLeft,
    DieDownLeft,
    DieUpRight,
    DieDownRight,
}

impl AnimationId {
    pub fn end_control(&self) -> EndControl {
        match self {
            &AnimationId::DieUp |
            &AnimationId::DieDown |
            &AnimationId::DieLeft |
            &AnimationId::DieRight |
            &AnimationId::DieUpLeft |
            &AnimationId::DieDownLeft |
            &AnimationId::DieUpRight |
            &AnimationId::DieDownRight => EndControl::Stay,
            _ => EndControl::Loop(None),
        }
    }
    pub fn is_terminal(&self) -> bool {
        match self {
            &AnimationId::DieUp |
            &AnimationId::DieDown |
            &AnimationId::DieLeft |
            &AnimationId::DieRight |
            &AnimationId::DieUpLeft |
            &AnimationId::DieDownLeft |
            &AnimationId::DieUpRight |
            &AnimationId::DieDownRight => true,
            _ => false,
        }
    }
}