use bevy::prelude::*;

use crate::{puy_ass::PuyoAssets, puyo_chara::PUYO_HEIGHT};

use super::{
    Dir, EveryoneSPhysProp, IsSPuyo, SBState, SBoard, SPJiggle, SPPhysics, SPhysProp, SPuyo,
};

pub fn main_loop(
    mut cmd: Commands,
    time: Res<Time>,
    puyo_assets: Res<PuyoAssets>,
    physprop: Res<EveryoneSPhysProp>,
    mut boards: Query<(&mut SBoard, Entity)>,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
) {
    for (mut board, board_entity) in boards.iter_mut() {
        board_update(
            &mut cmd,
            &time,
            &puyo_assets,
            &physprop.as_ref().spp,
            puyo_transforms.reborrow(),
            board.as_mut(),
            board_entity,
        );
    }
}

#[derive(Debug)]
enum NextUp {
    Nothing,
    CastOwanimo,
    Impossible(WhyImpossible),
}

#[derive(Debug)]
enum WhyImpossible {
    PuyoBanishingInPhysicsUpdate,
}

fn board_update(
    cmd: &mut Commands,
    time: &Res<Time>,
    puyo_assets: &Res<PuyoAssets>,
    physprop: &SPhysProp,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
    mut board: &mut SBoard,
    board_entity: Entity,
) {
    let mut next_up = match board.state {
        SBState::Still => NextUp::Nothing,
        SBState::Physics => board_physics(time, physprop, puyo_transforms.reborrow(), &mut board),
        SBState::Banish => {
            todo!()
        }
    };

    loop {
        match next_up {
            NextUp::Nothing => break,
            NextUp::CastOwanimo => todo!(),
            NextUp::Impossible(why) => panic!("Board Impossibility: {:?}", why),
        }
    }
}

impl SPuyo {
    pub fn get_jiggle_height(&self) -> f32 {
        use super::{SPJiggle, SPPhysics, SPState};
        let val = match self.state {
            SPState::Still(..) => 1.0,
            SPState::Physics(SPPhysics::Jiggle(SPJiggle { offset, .. })) => 1.0 + offset,
            _ => 0.0,
        };
        val * PUYO_HEIGHT
    }
}

struct JigglePropagation {
    impulse: f32,
    came_from: Dir,
    at: (usize, usize),
}

fn board_physics(
    time: &Res<Time>,
    physics_properties: &SPhysProp,
    mut puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
    board: &mut SBoard,
) -> NextUp {
    let dt = time.delta_secs();
    let mut next_up = NextUp::CastOwanimo;
    let mut jiggle_propagations = vec![];
    for (x, col) in board.columns.iter_mut().enumerate() {
        let mut jiggle_offset = 0.0;
        for (y, puyo) in col.into_iter().enumerate() {
            match &mut puyo.state {
                super::SPState::Still(..) => {
                    jiggle_offset += PUYO_HEIGHT;
                }
                super::SPState::Physics(spphysics) => {
                    next_up = NextUp::Nothing;
                    match spphysics {
                        super::SPPhysics::Fall(spfall) => {
                            spfall.velocity += physics_properties.gravity * dt;
                            let Ok(mut puyo_transform) = puyo_transforms.get_mut(puyo.entity)
                            else {
                                continue;
                            };
                            puyo_transform.translation.y -= spfall.velocity;
                            let clamped_vel =
                                spfall.velocity.remap(0.0, 9.0, 1.0, 2.0).clamp(1.0, 2.0);
                            let x_scale = 1.0 / clamped_vel;
                            let y_scale = 1.0 * clamped_vel;
                            puyo_transform.scale = vec3(x_scale, y_scale, x_scale);
                            if puyo_transform.translation.y <= jiggle_offset {
                                jiggle_propagations.push(JigglePropagation {
                                    impulse: spfall.velocity
                                        * physics_properties.velocity_to_impact,
                                    came_from: Dir::U,
                                    at: (x, y),
                                });
                            }
                        }
                        super::SPPhysics::Jiggle(spjiggle) => {
                            let Ok(mut puyo_transform) = puyo_transforms.get_mut(puyo.entity)
                            else {
                                continue;
                            };
                            puyo_transform.translation.y = jiggle_offset;
                            let acc = physics_properties.jiggle_stiff * -spjiggle.offset * dt;
                            spjiggle.momentum =
                                (spjiggle.momentum + acc) * physics_properties.jiggle_damp;
                            spjiggle.offset += spjiggle.momentum * dt;
                            if spjiggle.life < 0.1 {
                                spjiggle.offset *= spjiggle.life * 10.0;
                            }
                            if spjiggle.life <= 0.0
                                || (spjiggle.offset.abs() < 0.0025
                                    && spjiggle.momentum.abs() < 0.01)
                            {
                                puyo_transform.scale = Vec3::ONE;
                                jiggle_offset += PUYO_HEIGHT;
                                puyo.state = Default::default();
                            } else {
                                spjiggle.life -= dt * 0.333; //TODO: Make magical numbers physics properties
                                let offset = spjiggle.offset.max(-1.0);
                                let y_scale = offset + 1.0;
                                let xz_scale = y_scale.max(0.5).recip();
                                puyo_transform.scale = vec3(xz_scale, y_scale, xz_scale);
                                jiggle_offset += offset * PUYO_HEIGHT;
                            }
                        }
                    }
                }
                super::SPState::Banish(..) => {
                    return NextUp::Impossible(WhyImpossible::PuyoBanishingInPhysicsUpdate);
                }
            }
            // puyo_transform.translation.y =
        }
    }
    for propagation in jiggle_propagations {
        propagate_jiggle(propagation, physics_properties, board);
    }
    next_up
}

fn propagate_jiggle(
    propagation: JigglePropagation,
    physprop: &SPhysProp,
    board: &mut SBoard,
) -> Option<()> {
    let JigglePropagation {
        impulse,
        came_from,
        at,
    } = propagation;
    let SPhysProp {
        impact_falloff,
        min_impactable,
        ..
    } = physprop;
    if impulse < *min_impactable {
        return None;
    }
    use super::SPJiggle;
    use super::SPPhysics;
    use super::SPState::*;
    match &mut board.get_mut_at(at)?.state {
        state @ Still(..) => {
            *state = Physics(SPPhysics::Jiggle(SPJiggle {
                momentum: impulse,
                life: 1.0,
                offset: 0.0,
            }));
            Some(())
        }
        Physics(SPPhysics::Jiggle(SPJiggle { momentum, life, .. })) => {
            *momentum += impulse;
            *life = 1.0;
            Some(())
        }
        _ => None,
    }?;
    for dir in came_from.others() {
        let Some(at) = dir + at else { continue };
        propagate_jiggle(
            JigglePropagation {
                impulse: impulse - *impact_falloff,
                came_from: -dir,
                at,
            },
            physprop,
            board,
        );
    }
    Some(())
}
