use amethyst::{
    ApplicationBuilder, DataDispose,
};
use serde::{Serialize, Deserialize};

pub mod map;
pub use self::map::Map;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Game {
    pub map: Map,
}

impl Game {
    pub fn register<S, T, E, X>(self, builder: ApplicationBuilder<S, T, E, X>) -> ApplicationBuilder<S, T, E, X>
    where
        T: DataDispose + 'static, 
    {
        builder
            .with_resource(self.map)
    }
}