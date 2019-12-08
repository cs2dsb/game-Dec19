use serde::{Deserialize, Serialize};

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}