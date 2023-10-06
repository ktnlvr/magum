use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct HealthPool {
    pub max_hp: u32,
    pub current_hp: u32,

    pub just_died: bool,
}

impl HealthPool {
    pub fn new(max_hp: u32) -> Self {
        HealthPool {
            max_hp,
            current_hp: max_hp,
            just_died: false,
        }
    }
}
