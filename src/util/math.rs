use amethyst::core::math::Vector2;

pub const PI: f32                     = core::f32::consts::PI;
pub const DEG_TO_RAD_MULTIPLIER: f32  = PI / 180.;

pub const fn radians(degrees: f32) -> f32 {
    degrees * DEG_TO_RAD_MULTIPLIER
}


const EPSILON: f32 = 1e-6;

#[derive(Debug)]
pub enum QuadraticRoots {
    None,
    One(f32),
    Two(f32, f32),
}

// Returns the solutions for a quadratic if there are any
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> QuadraticRoots {
    if a.abs() < EPSILON {
        if b.abs() < EPSILON {
            if c.abs() < EPSILON {
                QuadraticRoots::One(0.)
            } else {
                QuadraticRoots::None
            }
        } else {
            QuadraticRoots::One(-c/b)
        }
    } else {
        let disc = b.powi(2) - 4. * a * c;
        if disc >= 0. {
            let disc = disc.sqrt();
            let a2 = a * 2.;
            QuadraticRoots::Two((-b-disc)/a2, (-b+disc)/a2)
        } else {
            QuadraticRoots::None
        }
    }
}

/// Calculates a projectile firing solution that will hit a moving target if it is possible to do so
pub fn intercept(origin: Vector2<f32>, target: Vector2<f32>, target_velocity: Vector2<f32>, intercept_speed: f32) -> Option<Vector2<f32>> {
    let d = target - origin;

    // Quadratic equation 
    let a = target_velocity.x.powi(2) + target_velocity.y.powi(2) - intercept_speed.powi(2);
    let b = 2. * (target_velocity.x * d.x + target_velocity.y * d.y);
    let c = d.x.powi(2) + d.y.powi(2);

    let solution = |t| target + target_velocity * t;

    match solve_quadratic(a, b, c) {
        QuadraticRoots::None => None,
        QuadraticRoots::One(t) => Some(solution(t)),
        QuadraticRoots::Two(t0, t1) => {
            let min = t0.min(t1);
            if min >= 0. {
                Some(solution(min))
            } else {
                Some(solution(t0.max(t1)))
            }
        },
    }
}