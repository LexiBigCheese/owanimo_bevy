use bevy::prelude::*;

use crate::puyo_chara::PuyoType;

#[derive(Component, Debug, Reflect)]
pub struct SBoard {
    pub score: u64,
    pub columns: Vec<Vec<SPuyo>>,
    pub state: SBState,
}

#[derive(Debug, Reflect, Default)]
pub enum SBState {
    #[default]
    Still,
    Physics,
    Banish,
}

#[derive(Debug, Reflect)]
pub struct SPuyo {
    pub kind: PuyoType,
    pub entity: Entity,
    pub state: SPState,
}

#[derive(Debug, Reflect, Clone, Copy)]
pub enum SPState {
    Still(SPStill),
    Physics(SPPhysics),
    Banish(SPBanish),
}

impl Default for SPState {
    fn default() -> Self {
        SPState::Still(Default::default())
    }
}

#[derive(Debug, Reflect, Clone, Copy, Default)]
pub struct SPStill {}

#[derive(Debug, Reflect, Clone, Copy)]
pub enum SPPhysics {
    Fall(SPFall),
    Jiggle(SPJiggle),
}

#[derive(Debug, Reflect, Clone, Copy)]
pub struct SPFall {
    pub velocity: f32,
}

#[derive(Debug, Reflect, Clone, Copy)]
pub struct SPJiggle {
    pub momentum: f32,
    pub offset: f32,
    pub life: f32,
}

#[derive(Debug, Reflect, Clone, Copy)]
pub struct SPBanish {
    pub life: f32,
}
