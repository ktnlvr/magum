use crate::player::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct HeroBundle {
    pub name: Name,
    pub marker: PlayerMarker,
    pub motor: PlayerMotor,
    pub computed_visibility: ComputedVisibility,

    pub visibility: Visibility,

    #[bundle()]
    pub transform: TransformBundle,
}

impl Default for HeroBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Hero"),
            visibility: Visibility::Visible,
            marker: Default::default(),
            motor: Default::default(),
            computed_visibility: Default::default(),
            transform: Default::default(),
        }
    }
}
