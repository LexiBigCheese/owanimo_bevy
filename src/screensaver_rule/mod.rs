pub mod main_loop;
pub mod owanimo_impl;

use std::fmt::Debug;

use bevy::prelude::*;

use crate::puyo_chara::PuyoType;

#[derive(Component, Debug, Reflect)]
pub struct SBoard {
    pub score: u64,
    pub chain: usize,
    pub columns: Vec<Vec<SPuyo>>,
    pub state: SBState,
}

#[derive(Debug, Reflect, Default, Clone)]
pub enum SBState {
    #[default]
    Still,
    Physics,
    Banish {
        life: f32,
    },
}

#[derive(Debug, Reflect)]
pub struct SPuyo {
    pub kind: PuyoType,
    pub entity: Entity,
    pub state: SPState,
}

/// Exists to limit transforms that the rule iterates over
#[derive(Debug, Reflect, Component, Clone, Copy)]
pub struct IsSPuyo;

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

impl SPState {
    pub fn new_falling() -> Self {
        SPState::Physics(SPPhysics::Fall(SPFall { velocity: 0.0 }))
    }
    pub fn new_banishing() -> Self {
        SPState::Banish(SPBanish {})
    }
    pub fn new_jiggle(momentum: f32) -> Self {
        SPState::Physics(SPPhysics::Jiggle(SPJiggle {
            momentum,
            offset: 0.0,
            life: 1.0,
        }))
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
pub struct SPBanish {}

#[derive(Debug, Reflect)]
pub struct SPhysProp {
    pub gravity: f32,
    pub velocity_to_impact: f32,
    pub impact_falloff: f32,
    pub min_impactable: f32,
    pub jiggle_stiff: f32,
    pub jiggle_damp: f32,
}

impl Default for SPhysProp {
    fn default() -> Self {
        SPhysProp {
            gravity: 9.8 * 2.0,
            velocity_to_impact: 0.3,
            impact_falloff: 0.5,
            min_impactable: 0.2,
            jiggle_stiff: 80.0,
            jiggle_damp: 0.8,
        }
    }
}

#[derive(Debug, Reflect, Resource, Default)]
pub struct EveryoneSPhysProp {
    pub spp: SPhysProp,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl jiggly_fever::Direction for Dir {
    fn opposite(self) -> Self {
        -self
    }
    fn other_directions(self) -> impl Iterator<Item = Self> {
        self.others().into_iter()
    }
    const UP: Dir = Dir::U;
}

impl core::ops::Add<(usize, usize)> for Dir {
    type Output = Option<(usize, usize)>;
    fn add(self, rhs: (usize, usize)) -> Self::Output {
        use Dir::*;
        match (self, rhs) {
            (L, (0, _)) => None,
            (D, (_, 0)) => None,
            (L, (col, row)) => Some((col - 1, row)),
            (R, (col, row)) => Some((col + 1, row)),
            (U, (col, row)) => Some((col, row + 1)),
            (D, (col, row)) => Some((col, row - 1)),
        }
    }
}

impl core::ops::Neg for Dir {
    type Output = Self;
    fn neg(self) -> Self::Output {
        use Dir::*;
        match self {
            U => D,
            D => U,
            L => R,
            R => L,
        }
    }
}

impl Dir {
    pub fn others(self) -> [Dir; 3] {
        use Dir::*;
        match self {
            U => [R, D, L],
            R => [D, L, U],
            D => [L, U, R],
            L => [U, R, D],
        }
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Dir::*;
        write!(
            f,
            "{}",
            match self {
                U => "↑",
                D => "↓",
                L => "←",
                R => "→",
            }
        )
    }
}

pub fn screensaver_rule_plugin(app: &mut App) {
    app.register_type::<SBoard>()
        .register_type::<SBState>()
        .register_type::<SPuyo>()
        .register_type::<IsSPuyo>()
        .register_type::<SPState>()
        .register_type::<SPStill>()
        .register_type::<SPPhysics>()
        .register_type::<SPFall>()
        .register_type::<SPJiggle>()
        .register_type::<SPBanish>()
        .register_type::<SPhysProp>()
        .register_type::<EveryoneSPhysProp>()
        .init_resource::<EveryoneSPhysProp>()
        .add_systems(Update, main_loop::main_loop);
}
