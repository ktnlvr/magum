use bevy::prelude::*;

use super::HealthPool;

#[derive(Debug, Event, Clone, Copy)]
pub struct DealDamageEvent {
    pub from_position: Vec2,
    pub damage: u32,
    pub target: Entity,
}

#[derive(Debug, Event)]
pub struct DamageTakenEvent {
    pub from_position: Vec2,
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

    for DealDamageEvent {
        target,
        damage,
        from_position,
    } in damage_deal.into_iter()
    {
        let Ok((entity, mut hp)) = health_pools.get_mut(*target) else {
            continue;
        };

        if hp.current_hp == 0 {
            continue;
        }

        if hp.current_hp <= *damage {
            hp.just_died = true;
        }

        hp.current_hp = hp.current_hp.saturating_sub(*damage);

        damage_taken.send(DamageTakenEvent {
            damage: *damage,
            from_position: *from_position,
            taken_by: entity,
            killing_blow: hp.just_died,
        })

        // TODO: emit death event
    }
}
