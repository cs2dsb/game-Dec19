use amethyst::{
    core::{
        timing::Time, 
        transform::Transform,
        math::Vector2,
    },
    ecs::prelude::{
        Join, 
        Read, 
        System, 
        WriteStorage,
    },
};
use std::time::{
    Instant,
    Duration,
};
use crate::{
    components::{
        Velocity,
        Path,
        Navigator,
    },
    util::{
        constants::CHARACTER_Z_OFFSET,
        set_magnitude,
        iso_to_screen,
    },
};

pub struct Mover;

impl<'s> System<'s> for Mover {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Path>,
        WriteStorage<'s, Navigator>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut velocities, mut transforms, mut paths, mut navigators, time): Self::SystemData) {
        for (v, t, p, n) in (&mut velocities, &mut transforms, &mut paths, &mut navigators).join() {
            if let (Some((path, _)), Some(i), start_time) = (&p.path, p.path_i, p.start_time) {
                if i >= path.len() { 
                    continue;
                }
                let next_pos: Vector2<f32> = path[i].into();
                let prev_pos: Vector2<f32> = if i == 0 {
                    n.clone().into()
                } else {
                    path[i - 1].into()
                };

                // Update the velocity angle
                v.velocity = next_pos - prev_pos;

                // Scale it to the right speed
                set_magnitude(&mut v.velocity, v.speed);

                let distance = prev_pos.metric_distance(&next_pos);
                let travel_time = Duration::from_secs_f32(distance / v.speed);
                let now = Instant::now();
                let start_time = start_time.unwrap_or(now);
                let elapsed_time = now - start_time;

                if elapsed_time >= travel_time {
                    p.path_i = Some(i+1);
                    p.start_time = Some(now);
                }

                // Capped so it doesn't overshoot
                let lerp_time = elapsed_time.min(travel_time);
                let pos = prev_pos + v.velocity * lerp_time.as_secs_f32();

                let (prev_x, prev_y) = {
                    let t = t.translation();
                    (t.x, t.y)
                };
                let mut screen_pos = iso_to_screen(pos);
                screen_pos.z += CHARACTER_Z_OFFSET;
                t.set_translation(screen_pos);

                //TODO: This is so the heading system works, probably fix it so heading is aware of map space velocity instead of screen space?
                v.velocity = Vector2::new(screen_pos.x, screen_pos.y) - Vector2::new(prev_x, prev_y);
            } else {
               t.prepend_translation_x(v.velocity.x * time.delta_seconds());
               t.prepend_translation_y(v.velocity.y * time.delta_seconds());
            }
        }
    }
}