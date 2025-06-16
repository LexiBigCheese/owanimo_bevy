use bevy::prelude::*;

use crate::screensaver_rule::{IsSPuyo, SBState, SBoard, SPFall, SPPhysics, SPState};

use super::{NextUp, WhyImpossible};

pub(crate) fn board_banish(
    cmd: &mut Commands,
    time: &Res<Time>,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
    board: &mut SBoard,
) -> NextUp {
    let SBState::Banish { life } = &mut board.state else {
        return NextUp::Impossible(WhyImpossible::BanishingWhenNotBanishing);
    };
    let dt = time.delta_secs();
    *life -= 1.3 * dt;
    if *life <= 0.0 {
        //delete all banishing puyos, start falling all puyo above
        let mut next_up = NextUp::Still;
        for col in &mut board.columns {
            let mut falling = false;
            let mut row = 0usize;
            while row < col.len() {
                match col[row].state {
                    SPState::Banish(..) => {
                        cmd.entity(col.remove(row).entity).despawn();
                        //TODO: Spawn some particles
                        falling = true;
                        next_up = NextUp::StartPhysics;
                    }
                    SPState::Still(..) => {
                        if falling {
                            col[row].state =
                                SPState::Physics(SPPhysics::Fall(SPFall { velocity: 0.0 }));
                        }
                        row += 1;
                    }
                    SPState::Physics(..) => {
                        return NextUp::Impossible(WhyImpossible::PuyoPhysicsInBanishUpdate);
                    }
                }
            }
        }
        next_up
    } else {
        let scale = Vec3::ONE * *life;
        for col in &mut board.columns {
            for puyo in col {
                let SPState::Banish(_) = puyo.state else {
                    continue;
                };
                let Ok(mut puyo_transform) = puyo_transforms.get_mut(puyo.entity) else {
                    continue;
                };
                puyo_transform.scale = scale;
            }
        }
        NextUp::Continue
    }
}
