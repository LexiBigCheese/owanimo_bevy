use bevy::prelude::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default, Reflect)]
#[reflect(Default)]
pub enum PuyoType {
    #[default]
    Nuisance,
    NuisanceBL,
    NuisanceTL,
    NuisanceBR,
    NuisanceTR,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}
