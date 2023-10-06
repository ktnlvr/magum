use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct HealthPool {
    pub max_hp: u32,
    pub current_hp: u32,

    pub just_died: bool,
}

impl HealthPool {
    pub fn new(max_hp: u32, current_hp: u32) -> Self {
        assert!(max_hp >= current_hp);

        HealthPool {
            max_hp,
            current_hp,
            just_died: false,
        }
    }
}
