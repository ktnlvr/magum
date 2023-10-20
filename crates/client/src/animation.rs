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
        Duration::ZERO
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
        self.timer = Timer::new(state.duration(), TimerMode::Once);
        let old_state = replace(&mut self.state, state);
        old_state
    }

    pub fn mutate_state(&mut self, mut mutator: impl FnMut(&mut T)) {
        mutator(&mut self.state)
    }
}

impl<T: AnimatorStateMachine> From<T> for Animator<T> {
    fn from(state: T) -> Self {
        let duration = state.duration();

        Self {
            state,
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

pub fn animator_system<T: AnimatorStateMachine>(
    mut animators: Query<(&mut Transform, &mut Animator<T>)>,
    time: Res<Time>,
) {
    for (mut transform, mut animator) in animators.iter_mut() {
        if animator.state.duration() != Duration::ZERO {
            animator.timer.tick(time.delta());
        }

        if animator.timer.just_finished() {
            let next_state = animator.state.next().unwrap_or_default();
            animator.transition_into(next_state);
        } else {
            *transform = animator.state.calculate_transform(animator.timer.percent());
        }
    }
}
