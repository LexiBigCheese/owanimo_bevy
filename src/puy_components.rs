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

#[derive(Component, Debug, Reflect)]
pub struct Puyo {
    pub board: Entity,
    pub grid_pos: (u32, u32),
    pub ty: PuyoType,
    pub popping: Option<f32>,
    //If this is settled, this is None
    pub fall_velocity: Option<f32>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default, Reflect)]
#[reflect(Default)]
pub enum PuyoType {
    #[default]
    Nuisance,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}

impl Debug for PuyoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\x1B[0m",
            match self {
                PuyoType::Nuisance => "\x1B[0m●",
                PuyoType::Green => "\x1B[92m●",
                PuyoType::Red => "\x1B[91m●",
                PuyoType::Blue => "\x1B[94m●",
                PuyoType::Yellow => "\x1B[93m●",
                PuyoType::Purple => "\x1B[35m●",
            }
        )
    }
}

pub fn spawn_cartes_board(
    cmd: &mut Commands,
    puy_ass: &Res<PuyoAssets>,
    at: Transform,
    contents: &str,
) {
    let board = cmd
        .spawn((
            CartesianBoard6x12 {
                state: CartesianState::Fall,
                ..Default::default()
            },
            at,
            Visibility::Visible,
        ))
        .id();
    let lines = contents
        .lines()
        .filter(|x| !x.trim().is_empty())
        .take(12)
        .collect::<Vec<_>>();
    let start_line = lines.len() - 1;
    for (y_bottom, line) in lines.into_iter().enumerate() {
        let y = start_line - y_bottom;
        for (x, chr) in line.trim().chars().take(6).enumerate() {
            use PuyoType::*;
            let kind = match chr {
                ' ' | '_' => continue,
                'O' | 'o' | '0' => Nuisance,
                'R' | 'r' => Red,
                'G' | 'g' => Green,
                'B' | 'b' => Blue,
                'Y' | 'y' => Yellow,
                'P' | 'p' => Purple,
                _ => continue,
            };
            spawn_puyo(cmd, puy_ass, board, kind, (x, y));
        }
    }
}

pub fn spawn_puyo(
    cmd: &mut Commands,
    puy_ass: &Res<PuyoAssets>,
    board: Entity,
    kind: PuyoType,
    pos: (usize, usize),
) {
    println!("Spawning a {:?} at {:?}", kind, pos);
    let puyo = cmd
        .spawn((
            Puyo {
                board,
                ty: kind,
                grid_pos: (pos.0 as u32, pos.1 as u32),
                popping: None,
                fall_velocity: Some(0.0),
            },
            ChildOf(board),
            Transform::from_xyz(pos.0 as f32, pos.1 as f32 * 0.8, 0.0),
            Visibility::Visible,
        ))
        .id();
    puy_ass.spawn(cmd, kind, puyo);
}

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
    mut puyos: Query<(&mut Puyo, &mut Transform, Entity)>,
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
        for mut puyo in puyos.iter_mut().filter(|puyo| &puyo.board == &ent) {
            puyo.fall_velocity = Some(0.0);
        }
        println!("Down We Go!");
    }
}
