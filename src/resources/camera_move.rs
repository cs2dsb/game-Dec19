use serde::{Deserialize, Serialize};
use amethyst::core::math::Vector3;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraMove {
    pub delta: Vector3<f32>,
}

impl Default for CameraMove {
    fn default() -> Self {
        Self {
            delta: Vector3::zeros(),
        }
    }
}