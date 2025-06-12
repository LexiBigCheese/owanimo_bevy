use std::collections::HashSet;

use super::{CartesianBoard6x12, CartesianState, Puyo};
use bevy::prelude::*;

pub fn banish_puyos(
    mut cmds: Commands,
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut Puyo, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    let mut boards_to_transition_to_fall_or_still =
        boards.iter().map(|(_, ent)| ent).collect::<HashSet<_>>();
    for (mut puy, mut trans, ent) in puyos.iter_mut() {
        let Some(mut life) = puy.popping else {
            continue;
        };
        boards_to_transition_to_fall_or_still.remove(&puy.board);
        life -= 1.3 * time.delta_secs();
        puy.popping = Some(life);
        trans.scale = Vec3::ONE * life;
        if life <= 0.0 {
            cmds.entity(ent).despawn();
        }
    }
    for board in boards_to_transition_to_fall_or_still {
        let Ok((mut board, _)) = boards.get_mut(board) else {
            continue;
        };
        if board.state == CartesianState::Banishing {
            board.state = CartesianState::TransitionToFallOrStill;
        }
    }
}
