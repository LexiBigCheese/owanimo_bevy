use bevy::prelude::*;

use crate::{puy_ass::PuyoAssets, puyo_chara::PUYO_HEIGHT};

use super::{EveryoneSPhysProp, IsSPuyo, SBState, SBoard, SPJiggle, SPPhysics, SPhysProp};

pub fn main_loop(
    mut cmd: Commands,
    time: Res<Time>,
    puyo_assets: Res<PuyoAssets>,
    physprop: Res<EveryoneSPhysProp>,
    mut boards: Query<(&mut SBoard, Entity)>,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
) {
    for (mut board, board_entity) in boards.iter_mut() {
        board_update(
            &mut cmd,
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
enum NextUp {
    Nothing,
    CastOwanimo,
    Impossible(WhyImpossible),
}

#[derive(Debug)]
enum WhyImpossible {
    PuyoBanishingInPhysicsUpdate,
}

fn board_update(
    cmd: &mut Commands,
    time: &Res<Time>,
    puyo_assets: &Res<PuyoAssets>,
    physprop: &SPhysProp,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
    mut board: &mut SBoard,
    board_entity: Entity,
) {
    let mut next_up = match board.state {
        SBState::Still => NextUp::Nothing,
        SBState::Physics => {
            board_physics::board_physics(time, physprop, puyo_transforms.reborrow(), &mut board)
        }
        SBState::Banish => {
            todo!()
        }
    };

    loop {
        match next_up {
            NextUp::Nothing => break,
            NextUp::CastOwanimo => todo!(),
            NextUp::Impossible(why) => panic!("Board Impossibility: {:?}", why),
        }
    }
}

mod board_physics;
