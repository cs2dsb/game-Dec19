use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Zoom {
    pub zoom: f32,
}

impl Default for Zoom {
    fn default() -> Self {
        Self {
            zoom: 1.,
        }
    }
}