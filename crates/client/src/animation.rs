use std::time::Duration;

use bevy::{
    prelude::{Component, Plugin, Query, Res, Timer, TimerMode, Transform, Update},
    time::Time,
};

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, animator_system);
    }
}

pub trait Clip
where
    Self: Send + Sync + 'static,
{
    fn animate(&self, time_normalized: f32) -> Transform;
}

#[derive(Component, Default)]
pub struct Animator {
    clips: Vec<(Box<dyn Clip>, Timer)>,
}

impl Animator {
    pub fn play(&mut self, clip: impl Clip, duration: Duration) {
        self.clips
            .push((Box::new(clip), Timer::new(duration, TimerMode::Once)))
    }
}

pub fn animator_system(mut animators: Query<(&mut Transform, &mut Animator)>, time: Res<Time>) {
    for (mut transform, mut animator) in animators.iter_mut() {
        animator.clips.iter_mut().for_each(|(_, timer)| {
            timer.tick(time.delta());
        });

        *transform = animator
            .clips
            .iter()
            .filter(|&(_, t)| !t.finished())
            .map(|(clip, timer)| clip.animate(timer.percent()))
            .fold(Transform::IDENTITY, |acc, n| acc.mul_transform(n));
    }
}
