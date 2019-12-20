use amethyst::{
    ApplicationBuilder, DataDispose,
};
use serde::{Serialize, Deserialize};

pub mod map;
pub use self::map::Map;

pub mod spawner;
pub use self::spawner::Spawner;

pub mod debug_draw;
pub use self::debug_draw::DebugDraw;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Game {
    pub map: Map,
    pub spawner: Spawner,
    pub debug_draw: DebugDraw,
}

impl Game {
    pub fn register<S, T, E, X>(self, builder: ApplicationBuilder<S, T, E, X>) -> ApplicationBuilder<S, T, E, X>
    where
        T: DataDispose + 'static, 
    {
        builder
            .with_resource(self.map)
            .with_resource(self.spawner)
            .with_resource(self.debug_draw)
    }
}