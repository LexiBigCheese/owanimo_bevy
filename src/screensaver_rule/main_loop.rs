use bevy::prelude::*;
use bevy_rand::{global::GlobalEntropy, prelude::Xoshiro128Plus};
use owanimo::Board;

use crate::{puy_ass::PuyoAssets, puyo_chara::PUYO_HEIGHT};

use super::{EveryoneSPhysProp, IsSPuyo, SBState, SBoard, SPJiggle, SPPhysics, SPState, SPhysProp};

pub fn main_loop(
    mut cmd: Commands,
    time: Res<Time>,
    mut rng: GlobalEntropy<Xoshiro128Plus>,
    puyo_assets: Res<PuyoAssets>,
    physprop: Res<EveryoneSPhysProp>,
    mut boards: Query<(&mut SBoard, Entity)>,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
) {
    for (mut board, board_entity) in boards.iter_mut() {
        board_update(
            &mut cmd,
            &mut rng,
            &time,
            &puyo_assets,
            &physprop.as_ref().spp,
            puyo_transforms.reborrow(),
            board.as_mut(),
            board_entity,
        );
    }
}

#[derive(Debug)]
pub(crate) enum NextUp {
    Continue,
    CastOwanimo,
    StartPhysics,
    Still,
    Impossible(WhyImpossible),
}

#[derive(Debug)]
pub(crate) enum WhyImpossible {
    PuyoBanishingInPhysicsUpdate,
    BanishingWhenNotBanishing,
    PuyoPhysicsInBanishUpdate,
}

static CHAIN_POWER_TABLE: &'static [u64] = &[
    0, 8, 16, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512, 544,
    576, 608, 640, 672,
];

static GROUP_BONUS_TABLE: owanimo::standard::GroupBonusTable = owanimo::standard::GroupBonusTable {
    table: &[0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 10],
};

static COLOR_BONUS_TABLE: owanimo::standard::ColorBonusTable = owanimo::standard::ColorBonusTable {
    table: &[0, 0, 3, 6, 12, 24],
};

fn board_update(
    cmd: &mut Commands,
    rng: &mut GlobalEntropy<Xoshiro128Plus>,
    time: &Res<Time>,
    puyo_assets: &Res<PuyoAssets>,
    physprop: &SPhysProp,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
    mut board: &mut SBoard,
    board_entity: Entity,
) {
    let prev_board_state = board.state.clone();
    let mut next_up = match board.state {
        SBState::Still => {
            spawn_random::spawn_random(cmd, rng, puyo_assets, 24, &mut board, board_entity)
        }
        SBState::Physics => {
            board_physics::board_physics(time, physprop, puyo_transforms.reborrow(), &mut board)
        }
        SBState::Banish { .. } => {
            board_banish::board_banish(cmd, time, puyo_transforms.reborrow(), &mut board)
        }
    };
    loop {
        match next_up {
            NextUp::Continue => break,
            NextUp::CastOwanimo => {
                let groups = board.owanimo_grouper();
                let binding = groups.as_ref();
                let binding = binding.owanimo_pop(4);
                if binding.groups.is_empty() {
                    next_up = NextUp::Still;
                    continue;
                }
                let binding = binding.owanimo_nuisance(board);
                let chain_power = CHAIN_POWER_TABLE
                    .get(board.chain)
                    .unwrap_or(CHAIN_POWER_TABLE.last().unwrap());
                let score_x_score = owanimo::standard::score(
                    board,
                    &binding,
                    &owanimo::standard::TrivialPiecesCleared,
                    &0,
                    chain_power,
                    &COLOR_BONUS_TABLE,
                    &GROUP_BONUS_TABLE,
                );
                board.score += score_x_score.0 * score_x_score.1;
                board.chain += 1;
                board.state = SBState::Banish { life: 1.0 };
                for group in binding.groups {
                    for &handle in group.iter() {
                        let Some(puy) = board.get_mut_at(handle) else {
                            continue;
                        };
                        puy.state = SPState::new_banishing();
                    }
                }
                next_up = NextUp::Continue;
            }
            NextUp::StartPhysics => {
                board.state = SBState::Physics;
                next_up = NextUp::Continue;
            }
            NextUp::Still => {
                board.state = SBState::Still;
                next_up = NextUp::Continue;
            }
            NextUp::Impossible(why) => panic!("Board Impossibility: {:?}", why),
        }
    }
}

mod board_banish;
mod board_physics;
pub mod spawn_random;
