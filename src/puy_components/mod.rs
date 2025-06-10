use std::fmt::Debug;

use bevy::prelude::*;
use bevy_rand::{global::GlobalEntropy, prelude::Xoshiro128Plus};
use owanimo::Board;
use rand::Rng;

use crate::{owanimo_impl::CartBoart, puy_ass::PuyoAssets};

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Default)]
pub struct CartesianBoard6x12 {
    pub state: CartesianState,
    pub score: u64,
    pub chain: usize,
    pub max_chain: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Reflect)]
#[reflect(Default)]
pub enum CartesianState {
    #[default]
    Still,
    Fall,
    ///Owanimo found targets
    Owanimo,
    Banishing,
}

pub use puyo_component::{Puyo, PuyoType};

pub mod puyo_component;

pub mod spawn_cartes_board;

pub use spawn_cartes_board::spawn_cartes_board;

pub mod spawn_puyo;

pub use spawn_puyo::spawn_puyo;

pub mod fall_puyo {
    use bevy::prelude::*;

    use super::CartesianState;

    use super::Puyo;

    use super::CartesianBoard6x12;

    pub fn fall_puyo(
        mut boards: Query<(&mut CartesianBoard6x12,)>,
        mut puyos: Query<(&mut Puyo, &mut Transform)>,
        time: Res<Time>,
    ) {
        for (mut board,) in boards.iter_mut() {
            if board.state != CartesianState::Fall {
                continue;
            }
            board.state = CartesianState::Owanimo;
            let mut filled_spaces: std::collections::HashSet<(u32, u32)> = Default::default();
            for (puy, _) in puyos.iter() {
                if puy.fall_velocity == None {
                    filled_spaces.insert(puy.grid_pos);
                }
            }
            let mut n_done = 0;
            let mut n_falling = 0;
            let mut n_oof = 0;
            for (mut puy, mut trans) in puyos.iter_mut() {
                n_done += 1;
                let Some(mut fall_velocity) = puy.fall_velocity else {
                    continue;
                };

                let tt = trans.translation;
                let mut pos = (tt.x.round() as u32, (tt.y / 0.8).ceil() as u32);
                if pos.1 == 0 || filled_spaces.get(&(pos.0, pos.1 - 1)).is_some() {
                    puy.fall_velocity = None;
                    while filled_spaces.get(&pos).is_some() {
                        pos.1 += 1;
                    }
                    puy.grid_pos = pos;
                    filled_spaces.insert(pos);
                    trans.translation = vec3(pos.0 as f32, pos.1 as f32 * 0.8, 0.0);
                    n_oof += 1;
                    continue;
                } else {
                    fall_velocity += 9.8 * time.delta_secs();
                    puy.fall_velocity = Some(fall_velocity);
                    trans.translation += vec3(0.0, -fall_velocity * time.delta_secs(), 0.0);
                    board.state = CartesianState::Fall;
                    n_falling += 1;
                }
            }
            if board.state == CartesianState::Fall {
                println!("WAAA T {} O {} F {}", n_done, n_oof, n_falling);
            } else {
                println!("OOF! T {} O {} F {}", n_done, n_oof, n_falling);
            }
        }
    }
}

pub use puy_randomizers::{other_randomise_puys, randomise_puys};

pub mod puy_randomizers {
    use bevy::prelude::*;

    use bevy_rand::prelude::Xoshiro128Plus;

    use bevy_rand::global::GlobalEntropy;
    use rand::Rng;

    use crate::{puy_ass::PuyoAssets, puy_components::spawn_puyo};

    use super::{CartesianBoard6x12, CartesianState, Puyo, PuyoType};

    pub fn randomise_puys(
        mut cmds: Commands,
        kbd: Res<ButtonInput<KeyCode>>,
        puy_ass: Res<PuyoAssets>,
        mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
        puyos: Query<(&Puyo, Entity)>,
        mut rng: GlobalEntropy<Xoshiro128Plus>,
    ) {
        if kbd.just_pressed(KeyCode::Space) {
            for (_, puy) in puyos.iter() {
                cmds.entity(puy).despawn();
            }
            for (mut brd_state, brd) in boards.iter_mut() {
                brd_state.state = CartesianState::Fall;
                for x in 0..6 {
                    for y in 0..12 {
                        use PuyoType::*;
                        let kind = match rng.random_range(0..7) {
                            0 => Nuisance,
                            1 => Red,
                            2 => Green,
                            3 => Blue,
                            4 => Yellow,
                            5 => Purple,
                            _ => continue,
                        };
                        spawn_puyo(&mut cmds, &puy_ass, brd, kind, (x, y));
                    }
                }
            }
        }
    }

    pub fn other_randomise_puys(
        mut cmds: Commands,
        puy_ass: Res<PuyoAssets>,
        mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
        puyos: Query<(&Puyo, Entity)>,
        mut rng: GlobalEntropy<Xoshiro128Plus>,
    ) {
        for (mut board_state, brd) in boards.iter_mut() {
            if board_state.state != CartesianState::Still {
                continue;
            }
            board_state.state = CartesianState::Fall;
            for (_, oh) in puyos.iter().filter(|(puy, _)| puy.board == brd) {
                cmds.entity(oh).despawn();
            }
            for x in 0..20 {
                for y in 0..24 {
                    use PuyoType::*;
                    let kind = match rng.random_range(0..7) {
                        0 => Nuisance,
                        1 => Red,
                        2 => Green,
                        3 => Blue,
                        4 => Yellow,
                        5 => Purple,
                        _ => continue,
                    };
                    spawn_puyo(&mut cmds, &puy_ass, brd, kind, (x, y));
                }
            }
        }
    }
}

pub fn owanimo_puyos(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<(&mut puyo_component::Puyo, Entity)>,
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

pub fn banish_puyos(
    mut cmds: Commands,
    mut puyos: Query<(&mut puyo_component::Puyo, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut puy, mut trans, ent) in puyos.iter_mut() {
        let Some(mut life) = puy.popping else {
            continue;
        };
        life -= 1.3 * time.delta_secs();
        puy.popping = Some(life);
        trans.scale = Vec3::ONE * life;
        if life <= 0.0 {
            cmds.entity(ent).despawn();
        }
    }
}

pub fn finish_banishing_puyo(
    mut boards: Query<(&mut CartesianBoard6x12, Entity)>,
    mut puyos: Query<&mut puyo_component::Puyo>,
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
        for mut puyo in puyos.iter_mut().filter(|puyo| &puyo.board == &ent) {
            puyo.fall_velocity = Some(0.0);
        }
        println!("Down We Go!");
    }
}
