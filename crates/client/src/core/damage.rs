use bevy::prelude::*;

use super::HealthPool;

#[derive(Debug, Event, Clone, Copy)]
pub struct DealDamageEvent {
    pub damage: u32,
    pub target: Entity,
}

#[derive(Debug, Event)]
pub struct DamageTakenEvent {
    pub damage: u32,
    pub taken_by: Entity,
    pub killing_blow: bool,
}

pub fn damage_system(
    mut health_pools: Query<(Entity, &mut HealthPool)>,
    mut damage_deal: EventReader<DealDamageEvent>,
    mut damage_taken: EventWriter<DamageTakenEvent>,
) {
    for (_entity, mut pool) in health_pools.iter_mut() {
        pool.just_died = false;
    }

    for DealDamageEvent { target, damage } in damage_deal.into_iter() {
        let Ok((entity, mut hp)) = health_pools.get_mut(*target) else {
            continue;
        };

        if hp.just_died {
            hp.just_died = false;
        }

        if hp.current_hp == 0 {
            continue;
        }

        if hp.current_hp <= *damage {
            hp.just_died = true;
        }

        hp.current_hp = hp.current_hp.saturating_sub(*damage);

        damage_taken.send(DamageTakenEvent {
            damage: *damage,
            taken_by: entity,
            killing_blow: hp.just_died,
        })

        // TODO: emit death event
    }
}
