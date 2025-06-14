use std::fmt::Debug;

use bevy::prelude::*;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Default)]
pub struct CartesianBoard6x12 {
    pub state: CartesianState,
    pub score: u64,
    pub chain: usize,
    pub max_chain: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Reflect)]
#[reflect(Default)]
pub enum CartesianState {
    #[default]
    Still,
    JustPlaced,
    Physics,
    TransitionToFallOrStill,
    Owanimo,
    ///Owanimo found targets
    Banishing,
}

pub use puyo_component::{Puyo, PuyoType};

pub mod puyo_component;

pub mod jiggle;
pub mod spawn_cartes_board;

pub use spawn_cartes_board::spawn_cartes_board;

pub mod spawn_puyo;

pub use spawn_puyo::spawn_puyo;

pub mod fall_puyo;

pub use puy_randomizers::{
    other_randomise_puys, randomise_puys, then_fall_puyos_after_placing_them,
};

pub mod puy_randomizers;

pub use owanimo_puyos::owanimo_puyos;

pub mod owanimo_puyos;

pub use banish_puyos::banish_puyos;

pub mod banish_puyos;

pub use falljiggle_or_still::falljiggle_or_still;

pub mod falljiggle_or_still;
