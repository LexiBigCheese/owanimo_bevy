use std::fmt::Debug;

use bevy::{
    ecs::{component::Component, entity::Entity},
    math::{Vec3, vec3},
    reflect::{Reflect, prelude::ReflectDefault},
};

#[derive(Component, Debug, Reflect)]
#[require(PuyoState)]
pub struct Puyo {
    pub board: Entity,
    pub grid_pos: (u32, u32),
    pub ty: PuyoType,
}

#[derive(Debug, Reflect, Default, Copy, Clone, Component)]
pub enum PuyoState {
    #[default]
    Still,
    Fall {
        velocity: f32,
    },
    Jiggle {
        offset: f32,
        velocity: f32,
        life: f32,
    },
    Banish {
        life: f32,
    },
}

impl PuyoState {
    pub fn start_falling(&mut self) {
        *self = PuyoState::Fall { velocity: 0.0 };
    }
    pub fn start_popping(&mut self) {
        *self = PuyoState::Banish { life: 1.0 };
    }
}

impl Puyo {
    pub fn grid_to_vec(&self) -> Vec3 {
        vec3(self.grid_pos.0 as f32, self.grid_pos.1 as f32 * 0.8, 0.0)
    }
}

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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl PuyoType {
    pub fn spreads_jiggle(&self, other: Self, direction: Direction) -> bool {
        use Direction::{Down as D, Left as L, Right as R, Up as U};
        use PuyoType::{
            Nuisance, NuisanceBL as BL, NuisanceBR as BR, NuisanceTL as TL, NuisanceTR as TR,
        };
        match (*self, other, direction) {
            (BL, BR, R)
            | (BR, BL, L)
            | (TL, TR, R)
            | (TR, TL, L)
            | (BL, TL, U)
            | (TL, BL, D)
            | (BR, TR, U)
            | (TR, BR, D) => true,
            (BL | BR | TL | TR | Nuisance, _, _) | (_, BL | BR | TL | TR | Nuisance, _) => false,
            (x, y, _) => x == y,
        }
    }
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
