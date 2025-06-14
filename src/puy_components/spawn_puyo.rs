use super::{Puyo, puyo_component::PuyoState};
use bevy::prelude::*;

use super::PuyoType;

use crate::puy_ass::PuyoAssets;

pub fn spawn_puyo(
    cmd: &mut Commands,
    puy_ass: &Res<PuyoAssets>,
    board: Entity,
    kind: PuyoType,
    pos: (usize, usize),
) {
    // println!("Spawning a {:?} at {:?}", kind, pos);
    let puyo = cmd
        .spawn((
            Puyo {
                board,
                ty: kind,
                grid_pos: (pos.0 as u32, pos.1 as u32),
            },
            PuyoState::Still,
            ChildOf(board),
            Transform::from_xyz(pos.0 as f32, pos.1 as f32 * 0.8, 0.0),
            Visibility::Visible,
        ))
        .id();
    puy_ass.spawn(cmd, kind, puyo);
}
