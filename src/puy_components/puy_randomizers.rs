
use bevy::prelude::*;

use bevy_rand::prelude::Xoshiro128Plus;

use bevy_rand::global::GlobalEntropy;
use rand::Rng;

use crate::{puy_ass::PuyoAssets, puy_components::spawn_puyo};

use super::{CartesianBoard6x12, CartesianState, Puyo, PuyoType};

pub fn randomise_puys(
    mut cmds: Commands,
    kbd: Res<ButtonInput<KeyCode>>,
    puy_ass: Res<PuyoAssets>,
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    puyos: Query<(&Puyo, Entity)>,
    mut rng: GlobalEntropy<Xoshiro128Plus>,
) {
    if kbd.just_pressed(KeyCode::Space) {
        for (_, puy) in puyos.iter() {
            cmds.entity(puy).despawn();
        }
        for (mut brd_state, brd) in boards.iter_mut() {
            brd_state.state = CartesianState::Fall;
            for x in 0..6 {
                for y in 0..12 {
                    use PuyoType::*;
                    let kind = match rng.random_range(0..7) {
                        0 => Nuisance,
                        1 => Red,
                        2 => Green,
                        3 => Blue,
                        4 => Yellow,
                        5 => Purple,
                        _ => continue,
                    };
                    spawn_puyo(&mut cmds, &puy_ass, brd, kind, (x, y));
                }
            }
        }
    }
}

pub fn other_randomise_puys(
    mut cmds: Commands,
    puy_ass: Res<PuyoAssets>,
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    puyos: Query<(&Puyo, Entity)>,
    mut rng: GlobalEntropy<Xoshiro128Plus>,
) {
    for (mut board_state, brd) in boards.iter_mut() {
        if board_state.state != CartesianState::Still {
            continue;
        }
        board_state.state = CartesianState::Fall;
        for (_, oh) in puyos.iter().filter(|(puy, _)| puy.board == brd) {
            cmds.entity(oh).despawn();
        }
        for x in 0..20 {
            for y in 0..24 {
                use PuyoType::*;
                let kind = match rng.random_range(0..7) {
                    0 => Nuisance,
                    1 => Red,
                    2 => Green,
                    3 => Blue,
                    4 => Yellow,
                    5 => Purple,
                    _ => continue,
                };
                spawn_puyo(&mut cmds, &puy_ass, brd, kind, (x, y));
            }
        }
    }
}
