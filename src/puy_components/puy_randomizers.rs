use bevy::prelude::*;

use bevy_rand::prelude::Xoshiro128Plus;

use bevy_rand::global::GlobalEntropy;
use owanimo::gravity::GravityBoard;
use rand::Rng;

use crate::{owanimo_impl::CartBoart, puy_ass::PuyoAssets, puy_components::spawn_puyo};

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
            brd_state.state = CartesianState::FallOrJiggle;
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
    mut puyos: Query<(&mut Puyo, Entity)>,
    mut rng: GlobalEntropy<Xoshiro128Plus>,
) {
    for (mut board_state, brd) in boards.iter_mut() {
        if board_state.state != CartesianState::Still {
            continue;
        }
        board_state.state = CartesianState::FallOrJiggle;
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
        board_state.state = CartesianState::JustPlaced;
    }
}
pub fn then_fall_puyos_after_placing_them(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut Puyo, Entity)>,
) {
    for (mut board_state, board) in boards
        .iter_mut()
        .filter(|(bs, _)| bs.state == CartesianState::JustPlaced)
    {
        let mut cartboart = CartBoart {
            board,
            puyos: puyos.reborrow(),
        };
        if cartboart.fall() {
            board_state.state = CartesianState::FallOrJiggle;
        } else {
            board_state.state = CartesianState::Still;
        }
    }
}
