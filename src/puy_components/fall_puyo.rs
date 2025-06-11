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
        //TODO: once todo in finish_banishing_puyo.rs uses .fall (and in future, assure it is called when a piece is placed)
        // this function should make puyo accelerate towards their grid position,
        // and also this function should put puyo into a jiggle state (and propagate a jiggle)
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
                trans.scale = Vec3::ONE;
                n_oof += 1;
                continue;
            } else {
                fall_velocity += 9.8 * time.delta_secs();
                puy.fall_velocity = Some(fall_velocity);
                trans.translation += vec3(0.0, -fall_velocity * time.delta_secs(), 0.0);
                let clamped_vel = fall_velocity.remap(0.0, 9.0, 1.0, 2.0).clamp(1.0, 2.0);
                let x_scale = 1.0 / clamped_vel;
                let y_scale = 1.0 * clamped_vel;
                trans.scale = vec3(x_scale, y_scale, x_scale);
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
