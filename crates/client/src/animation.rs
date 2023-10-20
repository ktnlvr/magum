use std::time::Duration;

use bevy::{
    prelude::{Component, Query, Res, Timer, Transform},
    time::{Time, TimerMode},
};

pub struct AnimatorPlugin;

pub trait AnimatorStateMachine
where
    Self: Default + Sized + Send + Sync + Clone + 'static,
{
    fn calculate_transform(&self, t: f32) -> Transform;

    fn duration(&self) -> Duration {
        Duration::from_secs(1)
    }

    fn next(&self) -> Option<Self> {
        Some(Self::default())
    }
}

#[derive(Component, Default, Clone)]
pub struct Animator<T: AnimatorStateMachine> {
    state: T,
    timer: Timer,
}

impl<T: AnimatorStateMachine> Animator<T> {
    pub fn transition_into(&mut self, state: T) -> T {
        use std::mem::*;
        let old_state = replace(&mut self.state, state);
        self.timer = Timer::new(self.state.duration(), TimerMode::Once);
        old_state
    }
}

pub fn animator_system<T: AnimatorStateMachine>(
    mut animators: Query<(&mut Transform, &mut Animator<T>)>,
    time: Res<Time>,
) {
    for (mut transform, mut animator) in animators.iter_mut() {
        animator.timer.tick(time.delta());

        if animator.timer.just_finished() {
            let next_state = animator.state.next().unwrap_or_default();
            animator.transition_into(next_state);
        } else {
            *transform = animator.state.calculate_transform(animator.timer.percent());
        }
    }
}
