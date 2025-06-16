use bevy::prelude::*;
use jiggly_fever::{JigglyBoard, PhysicsProperties, SlimePropsIn, SlimePropsOut, SlimeState};

use super::{
    super::{Dir, IsSPuyo, SBoard, SPJiggle, SPPhysics, SPState, SPhysProp, SPuyo},
    NextUp,
};

use crate::{puyo_chara::PUYO_HEIGHT, screensaver_rule::SPFall};

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

pub(crate) struct SBoardJiggleFever<'board, 'world, 'state, 'trans> {
    board: &'board mut SBoard,
    puyo_transforms: Query<'world, 'state, &'trans mut Transform, With<IsSPuyo>>,
    impact_falloff: f32,
}

impl SPState {
    fn to_jiggly_fever(&self) -> jiggly_fever::SlimeState {
        use jiggly_fever::SlimeState;

        match *self {
            SPState::Still(..) => SlimeState::Settled,
            SPState::Physics(SPPhysics::Fall(SPFall { velocity })) => {
                SlimeState::Falling { velocity }
            }
            SPState::Physics(SPPhysics::Jiggle(SPJiggle {
                momentum,
                offset,
                life,
            })) => SlimeState::Jiggling {
                momentum,
                offset,
                life,
            },
            SPState::Banish(..) => SlimeState::Settled,
        }
    }
    fn overwrite_jiggly_fever(&mut self, new_state: jiggly_fever::SlimeState) {
        use jiggly_fever::SlimeState;
        *self = match (*self, new_state) {
            (
                _,
                SlimeState::Jiggling {
                    momentum,
                    offset,
                    life,
                },
            ) => SPState::Physics(SPPhysics::Jiggle(SPJiggle {
                momentum,
                offset,
                life,
            })),
            (_, SlimeState::Falling { velocity }) => {
                SPState::Physics(SPPhysics::Fall(SPFall { velocity }))
            }
            (SPState::Still(otherwise), SlimeState::Settled) => SPState::Still(otherwise),
            (_, SlimeState::Settled) => Default::default(),
        };
    }
}

impl<'board, 'world, 'state, 'trans> JigglyBoard
    for SBoardJiggleFever<'board, 'world, 'state, 'trans>
{
    type Dir = Dir;
    type Loc = (usize, usize);

    fn apply_dir_to_loc(
        &self,
        dir: Self::Dir,
        loc: Self::Loc,
        impulse: f32,
    ) -> Option<(Self::Loc, f32)> {
        let o = (dir + loc)?;
        let a = self.board.get_at(loc)?;
        let b = self.board.get_at(o)?;
        let yay = |factor| Some((o, impulse * factor));
        let yippee = yay(self.impact_falloff);
        use crate::puyo_chara::PuyoType::*;
        use Dir::*;
        match (a.kind, b.kind, dir) {
            (_, _, U) => None,
            (NuisanceBL, NuisanceBR, R) => yippee,
            (NuisanceBR, NuisanceBL, L) => yippee,
            // (NuisanceBL, NuisanceTL, U) => yippee,
            (NuisanceTL, NuisanceBL, D) => yippee,
            // (NuisanceBR, NuisanceTR, U) => yippee,
            (NuisanceTR, NuisanceBR, D) => yippee,
            (NuisanceTL, NuisanceTR, R) => yippee,
            (NuisanceTR, NuisanceTL, L) => yippee,
            (Nuisance | NuisanceBL | NuisanceBR | NuisanceTL | NuisanceTR, _, _)
            | (_, Nuisance | NuisanceBL | NuisanceBR | NuisanceTL | NuisanceTR, _) => None,
            (_, _, D) => yippee,
            (ak, bk, L | R) => {
                if matches!(
                    a.state,
                    SPState::Still(..) | SPState::Physics(SPPhysics::Jiggle(..))
                ) && matches!(
                    b.state,
                    SPState::Still(..) | SPState::Physics(SPPhysics::Jiggle(..))
                ) && ak == bk
                {
                    yippee
                } else {
                    None
                }
            }
        }
    }

    fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = Self::Loc>> {
        self.board
            .columns
            .iter()
            .enumerate()
            .map(|(x, col)| (0..col.len()).into_iter().map(move |y| (x, y)))
    }

    fn mut_slime_with(&mut self, loc: Self::Loc, f: impl FnOnce(SlimePropsIn) -> SlimePropsOut) {
        let Some(col) = self.board.columns.get_mut(loc.0) else {
            return;
        };
        let Some(puyo) = col.get_mut(loc.1) else {
            return;
        };
        let Ok(mut transform) = self.puyo_transforms.get_mut(puyo.entity) else {
            return;
        };
        let spi = jiggly_fever::SlimePropsIn {
            state: puyo.state.to_jiggly_fever(),
            y_bottom: transform.translation.y * PUYO_HEIGHT.recip(),
        };
        let spo = f(spi);
        puyo.state.overwrite_jiggly_fever(spo.state);
        transform.translation.y = spo.y_bottom * PUYO_HEIGHT;
        transform.scale = vec3(spo.x_scale, spo.y_scale, spo.x_scale);
    }

    fn impulse_jiggle_with(&mut self, loc: Self::Loc, f: impl FnOnce(SlimeState) -> SlimeState) {
        let Some(col) = self.board.columns.get_mut(loc.0) else {
            return;
        };
        let Some(puyo) = col.get_mut(loc.1) else {
            return;
        };
        let slime_state = puyo.state.to_jiggly_fever();
        let new_state = f(slime_state);
        puyo.state.overwrite_jiggly_fever(new_state);
    }
}

impl SPhysProp {
    fn to_jiggle_physprop(&self) -> PhysicsProperties {
        let SPhysProp {
            gravity,
            velocity_to_impact,
            min_impactable,
            jiggle_stiff,
            jiggle_damp,
            ..
        } = *self;
        PhysicsProperties {
            gravity,
            velocity_to_impact,
            min_impactable,
            jiggle_stiff,
            jiggle_damp,
            jiggle_life_decrease_rate: 0.333,
            jiggle_life_threshold: 0.1,
            jiggle_life_threshold_inverse: 10.0,
            jiggle_offset_epsilon: 0.0025,
            jiggle_momentum_epsilon: 0.0025,
        }
    }
}

pub(crate) fn board_physics(
    time: &Res<Time>,
    physics_properties: &SPhysProp,
    puyo_transforms: Query<&mut Transform, With<IsSPuyo>>,
    board: &mut SBoard,
) -> NextUp {
    let mut jf = SBoardJiggleFever {
        board,
        puyo_transforms,
        impact_falloff: physics_properties.impact_falloff,
    };
    let physprop = physics_properties.to_jiggle_physprop();
    if jf.run_physics(time.delta_secs(), &physprop) {
        NextUp::CastOwanimo
    } else {
        NextUp::Continue
    }
}
