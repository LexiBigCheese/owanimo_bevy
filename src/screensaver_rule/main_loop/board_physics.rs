use bevy::prelude::*;

use super::{
    super::{Dir, IsSPuyo, SBoard, SPJiggle, SPPhysics, SPState, SPhysProp, SPuyo},
    NextUp, WhyImpossible,
};

use crate::puyo_chara::PUYO_HEIGHT;

impl SPuyo {
    pub fn get_jiggle_height(&self) -> f32 {
        let val = match self.state {
            SPState::Still(..) => 1.0,
            SPState::Physics(SPPhysics::Jiggle(SPJiggle { offset, .. })) => 1.0 + offset,
            _ => 0.0,
        };
        val * PUYO_HEIGHT
    }
}

pub(crate) struct JigglePropagation {
    pub(crate) impulse: f32,
    pub(crate) came_from: Dir,
    pub(crate) at: (usize, usize),
}

pub(crate) fn board_physics(
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
                SPState::Still(..) => {
                    let Ok(mut puyo_transform) = puyo_transforms.get_mut(puyo.entity) else {
                        continue;
                    };
                    puyo_transform.translation.y = jiggle_offset;
                    jiggle_offset += PUYO_HEIGHT;
                }
                SPState::Physics(spphysics) => {
                    next_up = NextUp::Continue;
                    match spphysics {
                        SPPhysics::Fall(spfall) => {
                            spfall.velocity += physics_properties.gravity * dt;
                            let Ok(mut puyo_transform) = puyo_transforms.get_mut(puyo.entity)
                            else {
                                continue;
                            };
                            puyo_transform.translation.y -= spfall.velocity * dt;
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
                                puyo.state = SPState::default();
                            }
                        }
                        SPPhysics::Jiggle(spjiggle) => {
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
                                let offset = (-spjiggle.offset).max(-1.0);
                                let y_scale = offset + 1.0;
                                let xz_scale = y_scale.max(0.5).recip();
                                puyo_transform.scale = vec3(xz_scale, y_scale, xz_scale);
                                jiggle_offset += y_scale * PUYO_HEIGHT;
                            }
                        }
                    }
                }
                SPState::Banish(..) => {
                    return NextUp::Impossible(WhyImpossible::PuyoBanishingInPhysicsUpdate);
                }
            }
            // println!("JiggleOffset: {}", jiggle_offset);
            // puyo_transform.translation.y =
        }
    }
    for propagation in jiggle_propagations {
        propagate_jiggle(propagation, physics_properties, board);
    }
    next_up
}

pub(crate) fn propagate_jiggle(
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
    use SPState::*;
    let mut puy = board.get_mut_at(at)?;
    let mut can_go_on = false;
    if let Physics(SPPhysics::Jiggle(SPJiggle { momentum, .. })) = puy.state {
        puy.state = SPState::new_jiggle(momentum + impulse);
        can_go_on = true;
    };
    if matches!(puy.state, Still(..)) {
        puy.state = SPState::new_jiggle(impulse);
        can_go_on = true;
    }
    if !can_go_on {
        return None;
    };
    for dir in came_from.others() {
        let Some(puy) = board.get_at(at) else {
            continue;
        };
        let Some(at) = dir + at else { continue };
        let Some(other_puy) = board.get_at(at) else {
            continue;
        };
        if dir == Dir::U {
            continue;
        };
        if dir == Dir::L || dir == Dir::R {
            let (pk, opk) = (puy.kind, other_puy.kind);
            use crate::puyo_chara::PuyoType::*;
            use Dir::*;
            match (pk, opk, dir) {
                (NuisanceBL, NuisanceBR, R) => (),
                (NuisanceBR, NuisanceBL, L) => (),
                (NuisanceBL, NuisanceTL, U) => (),
                (NuisanceTL, NuisanceBL, D) => (),
                (NuisanceBR, NuisanceTR, U) => (),
                (NuisanceTR, NuisanceBR, D) => (),
                (NuisanceTL, NuisanceTR, R) => (),
                (NuisanceTR, NuisanceTL, L) => (),
                (Nuisance | NuisanceBL | NuisanceBR | NuisanceTL | NuisanceTR, _, _)
                | (_, Nuisance | NuisanceBL | NuisanceBR | NuisanceTL | NuisanceTR, _) => continue,
                (a, b, _) => {
                    if a != b {
                        continue;
                    }
                }
            };
        }
        propagate_jiggle(
            JigglePropagation {
                impulse: impulse * *impact_falloff,
                came_from: -dir,
                at,
            },
            physprop,
            board,
        );
    }
    Some(())
}
