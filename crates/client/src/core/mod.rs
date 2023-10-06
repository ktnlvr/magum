mod damage;
mod health;

use bevy::prelude::{Last, Update};
pub use damage::*;
pub use health::*;

pub struct CorePlugin;

impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DealDamageEvent>()
            .add_event::<DamageTakenEvent>()
            .add_systems(Update, damage_system)
            .add_systems(Last, cleanup_just_dead);
    }
}
