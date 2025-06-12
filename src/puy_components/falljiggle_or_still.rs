use bevy::prelude::*;
use owanimo::gravity::GravityBoard;

use crate::owanimo_impl::CartBoart;

use super::CartesianState;
use super::jiggle::VertJiggle;

use super::Puyo;

use super::CartesianBoard6x12;

//TransitionToFallOrStill → FallOrJiggle
//TransitionToFallOrStill → Still
pub fn falljiggle_or_still(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut Puyo, Entity)>,
) {
    for (mut board, ent) in boards
        .iter_mut()
        .filter(|(board, _)| board.state == CartesianState::TransitionToFallOrStill)
    {
        let mut cartboart = CartBoart {
            board: ent,
            puyos: puyos.reborrow(),
        };
        board.state = if cartboart.fall() {
            println!("T2FJorS → FallOrJiggle");
            CartesianState::FallOrJiggle
        } else {
            println!("T2FJorS → Still");
            CartesianState::Still
        };
    }
}
