pub const PI: f32                     = core::f32::consts::PI;
pub const DEG_TO_RAD_MULTIPLIER: f32  = PI / 180.;

pub const fn radians(degrees: f32) -> f32 {
    degrees * DEG_TO_RAD_MULTIPLIER
}