use bevy::prelude::*;

use super::CartesianState;
use super::jiggle::VertJiggleSource;
use super::puyo_component::PuyoState;

use super::Puyo;

use super::CartesianBoard6x12;

///STATE TRANSITIONS:
/// - `Physics -> Owanimo`
/// - `Physics -> Physics`
pub fn fall_puyo(
    mut cmds: Commands,
    mut boards: Query<(&mut CartesianBoard6x12,)>,
    mut puyos: Query<(&mut Puyo, &mut Transform, &mut PuyoState, Entity)>,
    time: Res<Time>,
) {
    for (mut board,) in boards
        .iter_mut()
        .filter(|(bs,)| bs.state == CartesianState::Physics)
    {
        board.state = CartesianState::Owanimo;
        // let mut n_done = 0;
        // let mut n_falling = 0;
        // let mut n_oof = 0;
        for (puy, mut trans, mut state, ent) in puyos.iter_mut() {
            // n_done += 1;
            let mut velocity = match *state {
                PuyoState::Jiggle { .. } => {
                    board.state = CartesianState::Physics;
                    continue;
                }
                PuyoState::Fall { velocity } => velocity,
                _ => continue,
            };

            let tt = trans.translation;
            let puy_float_target = (puy.grid_pos.1 as f32) * 0.8;

            if tt.y <= puy_float_target {
                *state = PuyoState::Still;
                trans.translation = puy.grid_to_vec();
                trans.scale = Vec3::ONE;
                cmds.entity(ent).insert(VertJiggleSource(velocity * 0.3));
                // n_oof += 1;
                continue;
            } else {
                velocity += 9.8 * time.delta_secs();
                *state = PuyoState::Fall { velocity };
                trans.translation += vec3(0.0, -velocity * time.delta_secs(), 0.0);
                let clamped_vel = velocity.remap(0.0, 9.0, 1.0, 2.0).clamp(1.0, 2.0);
                let x_scale = 1.0 / clamped_vel;
                let y_scale = 1.0 * clamped_vel;
                trans.scale = vec3(x_scale, y_scale, x_scale);
                board.state = CartesianState::Physics;
                // n_falling += 1;
            }
        }
        // if board.state == CartesianState::FallOrJiggle {
        //     println!("WAAA T {} O {} F {}", n_done, n_oof, n_falling);
        // } else {
        //     println!("OOF! T {} O {} F {}", n_done, n_oof, n_falling);
        // }
    }
}
