use std::collections::HashSet;

use super::{CartesianBoard6x12, CartesianState, Puyo, puyo_component::PuyoState};
use bevy::prelude::*;

pub fn banish_puyos(
    mut cmds: Commands,
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut Puyo, &mut PuyoState, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    let mut boards_to_transition_to_fall_or_still =
        boards.iter().map(|(_, ent)| ent).collect::<HashSet<_>>();
    for (puy, mut state, mut trans, ent) in puyos.iter_mut() {
        let PuyoState::Banish { mut life } = *state else {
            continue;
        };
        boards_to_transition_to_fall_or_still.remove(&puy.board);
        life -= 1.3 * time.delta_secs();
        *state = PuyoState::Banish { life };
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
