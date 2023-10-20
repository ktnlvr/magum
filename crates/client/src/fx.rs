use bevy::prelude::*;

use crate::core::{DamageTakenEvent, HealthPool};

pub fn damage_numbers(mut events: EventReader<DamageTakenEvent>, hp: Query<&HealthPool>) {
    for DamageTakenEvent {
        taken_by, damage, ..
    } in events.into_iter()
    {
        if let Ok(entt) = hp.get(*taken_by) {
            println!(
                "Damage dealt to {taken_by:?}: {damage} damage, now at {} HP",
                entt.current_hp
            );
        }
    }
}
