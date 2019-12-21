use amethyst::core::math::{
    Vector2,
    Vector3,
};

pub const TILE_W: f32 = 32.;
pub const TILE_H: f32 = 16.;

pub fn iso_to_screen(iso: Vector2<f32>) -> Vector3<f32> {
    let ix = iso.x;
    let iy = iso.y;

    let sx = (ix - iy) * TILE_W;
    let sy = (ix + iy) * TILE_H;
    let sz = 0.5 - ix * 0.001 - iy * 0.001;

    Vector3::new(sx, sy, sz)
}

pub fn screen_to_iso(screen: Vector2<f32>) -> Vector2<f32> {
    let sx = screen.x;
    let sy = screen.y;

    let ix = (sx / TILE_W + sy / TILE_H) / 2.;
    let iy = (sy / TILE_H - sx / TILE_W) / 2.;

    Vector2::new(ix, iy)
}

pub fn iso_distance(screen1: &Vector3<f32>, screen2: &Vector3<f32>) -> f32 {
    let iso1 = screen_to_iso(screen1.xy());
    let iso2 = screen_to_iso(screen2.xy());

    iso1.metric_distance(&iso2)
}