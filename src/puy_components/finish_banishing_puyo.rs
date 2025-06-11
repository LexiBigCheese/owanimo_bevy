use bevy::prelude::*;

use super::CartesianState;

use super::Puyo;

use super::CartesianBoard6x12;

pub fn finish_banishing_puyo(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<&mut Puyo>,
) {
    for (mut board, ent) in boards.iter_mut() {
        if board.state != CartesianState::Banishing {
            continue;
        }
        board.state = CartesianState::Fall;
        for puyo in puyos.iter().filter(|puyo| &puyo.board == &ent) {
            if puyo.popping.is_some() {
                board.state = CartesianState::Banishing;
                println!("Wuh Nah");
                break;
            }
        }
        //TODO: use <CartBoard as GravityBoard>::fall here
        for mut puyo in puyos.iter_mut().filter(|puyo| &puyo.board == &ent) {
            puyo.fall_velocity = Some(0.0);
        }
        println!("Down We Go!");
    }
}
