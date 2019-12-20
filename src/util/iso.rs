const TILE_W: f32 = 32.;
const TILE_H: f32 = 32.;

pub fn iso_to_screen(x: f32, y: f32) -> (f32, f32) {
    (
        (x - y) * TILE_W,
        (x + y) * TILE_H / 2.,
    )
}