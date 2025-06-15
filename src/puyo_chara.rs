use std::fmt::Debug;

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

impl Debug for PuyoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\x1B[0m",
            match self {
                PuyoType::Nuisance => "\x1B[0m●",
                PuyoType::NuisanceBL => "\x1B[0m𜰹",
                PuyoType::NuisanceBR => "\x1B[0m𜰺",
                PuyoType::NuisanceTL => "\x1B[0m𜰵",
                PuyoType::NuisanceTR => "\x1B[0m𜰶",
                PuyoType::Green => "\x1B[92m●",
                PuyoType::Red => "\x1B[91m●",
                PuyoType::Blue => "\x1B[94m●",
                PuyoType::Yellow => "\x1B[93m●",
                PuyoType::Purple => "\x1B[35m●",
            }
        )
    }
}
