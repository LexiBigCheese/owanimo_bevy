use bevy::prelude::*;
use owanimo::gravity::GravityBoard;

use crate::owanimo_impl::CartBoart;
use crate::owanimo_impl::GravityCartBoart;

use super::CartesianState;
use super::puyo_component::PuyoState;

use super::Puyo;

use super::CartesianBoard6x12;

///TransitionToFallOrStill → FallOrJiggle
///TransitionToFallOrStill → Still
pub fn falljiggle_or_still(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut Puyo, Entity)>,
    mut states: Query<&mut PuyoState>,
) {
    for (mut board, ent) in boards
        .iter_mut()
        .filter(|(board, _)| board.state == CartesianState::TransitionToFallOrStill)
    {
        let mut cartboart = GravityCartBoart {
            board: ent,
            puyos: puyos.reborrow(),
            states: states.reborrow(),
        };
        board.state = if cartboart.fall() {
            println!("T2FJorS → FallOrJiggle");
            CartesianState::Physics
        } else {
            println!("T2FJorS → Still");
            CartesianState::Still
        };
    }
}
