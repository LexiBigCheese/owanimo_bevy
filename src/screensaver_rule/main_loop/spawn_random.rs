use bevy::prelude::*;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::Xoshiro128Plus;
use rand::Rng;

use crate::{
    puy_ass::PuyoAssets,
    puyo_chara::PUYO_HEIGHT,
    screensaver_rule::{IsSPuyo, SPState, SPuyo},
};

use super::{super::SBoard, NextUp};

pub(crate) fn spawn_random(
    cmd: &mut Commands,
    rng: &mut GlobalEntropy<Xoshiro128Plus>,
    puyo_assets: &Res<PuyoAssets>,
    rows: usize,
    board: &mut SBoard,
    board_ent: Entity,
) -> NextUp {
    board.score = 0;
    board.chain = 0;
    for (x, col) in board.columns.iter_mut().enumerate() {
        for puy in col.iter() {
            cmd.entity(puy.entity).despawn();
        }
        col.clear();
        let mut falling = false;
        for y in 0..rows {
            use crate::puyo_chara::PuyoType::*;
            let kind = match rng.random_range(0..7) {
                0 => Nuisance,
                1 => Red,
                2 => Green,
                3 => Blue,
                4 => Yellow,
                5 => Purple,
                _ => {
                    falling = true;
                    continue;
                }
            };
            let puyo_entity = cmd
                .spawn((
                    IsSPuyo,
                    Transform::from_xyz(x as f32, y as f32 * PUYO_HEIGHT, 0.0),
                    ChildOf(board_ent),
                    InheritedVisibility::VISIBLE,
                ))
                .id();
            puyo_assets.spawn(cmd, kind, puyo_entity);
            col.push(SPuyo {
                kind,
                state: if falling {
                    SPState::new_falling()
                } else {
                    Default::default()
                },
                entity: puyo_entity,
            })
        }
    }
    NextUp::StartPhysics
}
