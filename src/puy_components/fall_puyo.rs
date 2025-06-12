use bevy::prelude::*;

use super::CartesianState;
use super::jiggle::VertJiggle;
use super::jiggle::VertJiggleSource;

use super::Puyo;

use super::CartesianBoard6x12;

///STATE TRANSITIONS:
/// - `FallOrJiggle -> Owanimo`
/// - `FallOrJiggle -> FallOrJiggle`
pub fn fall_puyo(
    mut cmds: Commands,
    mut boards: Query<(&mut CartesianBoard6x12,)>,
    mut puyos: Query<(&mut Puyo, &mut Transform, Option<&VertJiggle>, Entity)>,
    time: Res<Time>,
) {
    for (mut board,) in boards
        .iter_mut()
        .filter(|(bs,)| bs.state == CartesianState::FallOrJiggle)
    {
        board.state = CartesianState::Owanimo;
        let mut n_done = 0;
        let mut n_falling = 0;
        let mut n_oof = 0;
        for (mut puy, mut trans, vj, ent) in puyos.iter_mut() {
            n_done += 1;
            if vj.is_some() {
                board.state = CartesianState::FallOrJiggle;
            }
            let Some(mut fall_velocity) = puy.fall_velocity else {
                continue;
            };

            let tt = trans.translation;
            let puy_float_target = (puy.grid_pos.1 as f32) * 0.8;

            if tt.y <= puy_float_target {
                puy.fall_velocity = None;
                trans.translation = puy.grid_to_vec();
                trans.scale = Vec3::ONE;
                cmds.entity(ent)
                    .insert(VertJiggleSource(fall_velocity * 0.8));
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
                board.state = CartesianState::FallOrJiggle;
                n_falling += 1;
            }
        }
        // if board.state == CartesianState::FallOrJiggle {
        //     println!("WAAA T {} O {} F {}", n_done, n_oof, n_falling);
        // } else {
        //     println!("OOF! T {} O {} F {}", n_done, n_oof, n_falling);
        // }
    }
}
