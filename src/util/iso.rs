use amethyst::core::math::Vector3;

const TILE_W: f32 = 32.;
const TILE_H: f32 = 32.;

pub fn iso_to_screen(x: f32, y: f32) -> Vector3<f32> {
    Vector3::new(
        (x - y) * TILE_W,
        (x + y) * TILE_H / 2.,
        0.5 
            - x * 0.001
            - y * 0.001,
    )
}