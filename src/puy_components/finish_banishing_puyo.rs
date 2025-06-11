use bevy::prelude::*;
use owanimo::gravity::GravityBoard;

use crate::owanimo_impl::CartBoart;

use super::CartesianState;

use super::Puyo;

use super::CartesianBoard6x12;

pub fn finish_banishing_puyo(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut Puyo, Entity)>,
) {
    for (mut board, ent) in boards
        .iter_mut()
        .filter(|(board, _)| board.state == CartesianState::Banishing)
    {
        board.state = CartesianState::Fall;
        for (puyo, _) in puyos.iter().filter(|(puyo, _)| &puyo.board == &ent) {
            if puyo.popping.is_some() {
                board.state = CartesianState::Banishing;
                println!("Wuh Nah");
                break;
            }
        }
        //board.state was Banishing when we entered here, check if we should start falling
        if board.state == CartesianState::Fall {
            let mut cartboart = CartBoart {
                board: ent,
                puyos: puyos.reborrow(),
            };
            if !cartboart.fall() {
                board.state = CartesianState::Still;
            }
        }
        println!("Down We Go!");
    }
}
