use bevy::prelude::*;

#[derive(Debug, Reflect, Component)]
pub struct RigidBody {
    pub mass: f32,
    pub velocity: Vec2,
}

impl RigidBody {
    pub fn new_resting(mass: f32) -> Self {
        Self {
            mass,
            velocity: Vec2::ZERO,
        }
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::new_resting(1.)
    }
}

#[derive(Debug, Reflect)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Rectangle { half_extents: Vec2 },
}

impl ColliderShape {
    pub fn circle(r: f32) -> Self {
        Self::Circle { radius: r }
    }

    pub fn rect(half_extents: Vec2) -> Self {
        Self::Rectangle { half_extents }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Collider {
    pub shape: ColliderShape,
    pub dynamic: bool,
}

pub fn move_physics_bodies(
    mut bodies: Query<(&mut Transform, &mut RigidBody)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (mut transform, body) in bodies.iter_mut() {
        transform.translation += Vec3::new(body.velocity.x, body.velocity.y, 0.) * dt;
    }
}
