
use bevy::prelude::*;
use owanimo::Board;

use crate::owanimo_impl::CartBoart;

use super::CartesianState;

use super::Puyo;

use super::CartesianBoard6x12;

pub fn owanimo_puyos(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut Puyo, Entity)>,
) {
    for (mut board_state, board_entity) in boards.iter_mut() {
        if board_state.state != CartesianState::Owanimo {
            continue;
        }
        let cart_boart = CartBoart {
            board: board_entity,
            puyos: puyos.reborrow(),
        };
        let groups = cart_boart.owanimo_grouper();
        let binding = groups.as_ref();
        let binding = binding.owanimo_pop(4);
        let binding = binding.owanimo_nuisance(&cart_boart);
        drop(cart_boart);
        if binding.groups.len() == 0 {
            board_state.state = CartesianState::Still;
            continue;
        }
        for g in &binding {
            for p in g.iter() {
                let Some(p) = p else { continue };
                let Ok(mut p) = puyos.get_mut(p.clone()) else {
                    continue;
                };
                p.0.popping = Some(1.0);
            }
        }
        board_state.state = CartesianState::Banishing;
    }
}
