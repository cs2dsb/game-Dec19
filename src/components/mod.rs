use amethyst::{
    ApplicationBuilder, DataDispose,
};

mod velocity;
pub use velocity::Velocity;

mod animation;
pub use animation::Animation;

mod age;
pub use age::Age;

pub mod map;
pub use map::Map;

mod navigator;
pub use navigator::Navigator;

mod path;
pub use path::{
    Path,
    PathNode,
};

mod color;
pub use color::Color;

mod creep;
pub use creep::Creep;

mod tower;
pub use tower::{ Tower, BulletTower };

mod projectile;
pub use projectile::Projectile;

/// This allows systems to be commented in and out without causing runtime errors
pub fn register_components<S, T, E, X>(builder: ApplicationBuilder<S, T, E, X>) -> ApplicationBuilder<S, T, E, X>
where
    T: DataDispose + 'static, 
{
    builder
        .register::<Velocity>()
        .register::<Animation>()
        .register::<Age>()
        .register::<Map>()
        .register::<Navigator>()
        .register::<Path>()
        .register::<Color>()
        .register::<Creep>()
        .register::<BulletTower>()
        .register::<Tower>()
        .register::<Projectile>()
}