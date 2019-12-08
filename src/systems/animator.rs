use amethyst::{
    ecs::prelude::{
        Entities,
        Join,
        ReadStorage, 
        System, 
        WriteStorage,
    },
    renderer::{
        SpriteRender, 
    },
    animation::{
        AnimationSet,
        AnimationControlSet,
        AnimationCommand,
    },
};
use crate::{
    resources::AnimationId,
    components::Animation,
};

pub struct Animator;

impl<'s> System<'s> for Animator {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Animation>, //The component determining which animation should play
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>, //The collection of all animations
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>, //The running animation control set
    );

    fn run(&mut self, (entities, mut animation, animation_sets, mut animation_control_sets): Self::SystemData) {
        let mut died = Vec::new();
        for (e, animation_set, control_set, a) in (&entities, &animation_sets, &mut animation_control_sets, &mut animation).join() {
            if a.next != a.current {
                if let Some(current) = a.current {
                    if control_set.has_animation(current) {
                        //log::info!("Cancelling: {:?}", current);
                        control_set.abort(current);
                    }
                }
                if let Some(next) = a.next {
                    let end_control = next.end_control();
                    let is_terminal = next.is_terminal();
                    if is_terminal {
                        died.push(e);
                    }
                    control_set.add_animation(
                        next,
                        &animation_set.get(&next).unwrap(),
                        end_control,
                        1.0,
                        AnimationCommand::Start,
                    );
                }
                a.current = a.next;
                a.is_done = false;
            }
            if let Some(current) = a.current {
                a.is_done = !control_set.has_animation(current);
            } else {
                a.is_done = true;
            }
        }
        for e in died {
            animation.remove(e);
        }
    }    
}