use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::{CartesianBoard6x12, CartesianState, Puyo, PuyoType, puyo_component::PuyoState};

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Default)]
pub struct VertJiggleSource(pub f32);

#[derive(Resource, Clone, Copy, Reflect)]
#[reflect(Default)]
pub struct PuyoStiffDamp {
    pub stiff: f32,
    pub damp: f32,
}

impl Default for PuyoStiffDamp {
    fn default() -> Self {
        PuyoStiffDamp {
            stiff: 80.0,
            damp: 0.8,
        }
    }
}

fn integrate_vert_jiggle(
    mut vert_jiggles: Query<&mut PuyoState>,
    time: Res<Time>,
    sd: Res<PuyoStiffDamp>,
) {
    let dt = time.delta_secs();
    vert_jiggles.par_iter_mut().for_each(|mut vj| {
        let PuyoState::Jiggle {
            mut offset,
            mut velocity,
            mut life,
        } = *vj
        else {
            return;
        };
        let acc = sd.stiff * -offset;
        let acc = acc * dt;
        velocity = (velocity + acc) * sd.damp;
        offset = (offset + velocity * dt) * life;
        life -= dt * 0.1;
        *vj = PuyoState::Jiggle {
            offset,
            velocity,
            life,
        };
    });
}

//TODO: just send the entire puyo's data, cloned,
// and perhaps do this more intelligently, perhaps globally managed
fn get_boards(
    all_puys: Query<(&Puyo, Entity)>,
) -> HashMap<(Entity, (u32, u32)), (Entity, PuyoType)> {
    let mut le_map: HashMap<(Entity, (u32, u32)), (Entity, PuyoType)> = Default::default();
    for (puy, ent) in all_puys.iter() {
        le_map.insert((puy.board, puy.grid_pos), (ent, puy.ty));
    }
    le_map
}

#[derive(Resource)]
pub struct AllBoards(pub HashMap<(Entity, (u32, u32)), (Entity, PuyoType)>);

impl Default for AllBoards {
    fn default() -> Self {
        AllBoards(Default::default())
    }
}

fn update_all_boards(all_puys: Query<(&Puyo, Entity)>, mut allboards: ResMut<AllBoards>) {
    allboards.0 = get_boards(all_puys);
}

///NOTE: DO NOT PASS NEGATIVE FALLOFFS INTO FUNCTION
///
/// NOTE 2: Known Missing Case:
/// ```text
/// r
///  r
///
/// Rg
/// ```
/// the top left falling puyo receives jiggle from below (does not display visually),
/// it is transferred to the top right falling puyo (not visually),
/// then transferred down to the bottom right green puyo (visually)
fn spread_jiggle_recursive(
    cmd: &mut Commands,
    put: &mut Query<&mut PuyoState>,
    map: &HashMap<(Entity, (u32, u32)), (Entity, PuyoType)>,
    done: &mut HashSet<(Entity, (u32, u32))>,
    strength: f32,
    min_strength: f32,
    falloff: f32,
    location: (Entity, (u32, u32)),
    ent: Entity,
    col: PuyoType,
) {
    if strength < min_strength {
        return;
    }
    let Ok(mut state) = put.get_mut(ent) else {
        return;
    };
    *state = if let PuyoState::Jiggle {
        offset, velocity, ..
    } = *state
    {
        PuyoState::Jiggle {
            offset,
            velocity: velocity - strength,
            life: 1.0,
        }
    } else {
        PuyoState::Jiggle {
            offset: 0.0,
            velocity: -strength,
            life: 1.0,
        }
    };
    done.insert(location);
    let (board, (x, y)) = location;
    let neighbors = {
        use crate::puy_components::puyo_component::Direction::{
            self, Down as D, Left as L, Right as R, Up as U,
        };
        let mapr = |x, y, dir: Direction| {
            map.get(&(board, (x, y))).and_then(|(ent, ty)| {
                if dir == D || col.spreads_jiggle(*ty, dir) {
                    Some(((board, (x, y)), ent, ty))
                } else {
                    None
                }
            })
        };
        let mapr2 = |x, y, dir| {
            if !done.contains(&(board, (x, y))) {
                mapr(x, y, dir)
            } else {
                None
            }
        };
        [
            if x > 0 { mapr2(x - 1, y, L) } else { None },
            if y > 0 { mapr2(x, y - 1, D) } else { None },
            mapr2(x + 1, y, R),
            mapr2(x, y + 1, U),
        ]
        .into_iter()
        .flatten()
    };
    for (location, ent, col) in neighbors {
        let strength = strength - falloff;
        spread_jiggle_recursive(
            cmd,
            put,
            map,
            done,
            strength,
            min_strength,
            falloff,
            location,
            *ent,
            *col,
        );
    }
}

fn spread_jiggle_sources(
    mut cmd: Commands,
    sources: Query<(&Puyo, &VertJiggleSource, Entity)>,
    mut put: Query<&mut PuyoState>,
    puys: Res<AllBoards>,
) {
    //This is probably quite the expensive algorithm, due to it's time complexity
    // so to help with the time complexity, i'll quickly HashMap<(board,(x,y)),Entity>
    for (source_puy, vjs, ent) in sources.iter() {
        cmd.entity(ent).remove::<VertJiggleSource>();
        spread_jiggle_recursive(
            &mut cmd,
            &mut put,
            &puys.0,
            &mut Default::default(),
            vjs.0,
            0.1,
            0.25,
            (source_puy.board, source_puy.grid_pos),
            ent,
            source_puy.ty,
        );
    }
}

fn remove_jiggles(
    // mut cmds: Commands,
    mut vjs: Query<(&mut PuyoState, &mut Transform, &Puyo)>,
) {
    for (mut vj, mut trans, puyo) in vjs.iter_mut() {
        let PuyoState::Jiggle {
            offset,
            velocity,
            life,
        } = *vj
        else {
            continue;
        };
        if life <= 0.0 || (velocity.abs() < 0.0025 && offset.abs() < 0.0025) {
            *vj = PuyoState::Still;
            trans.translation = puyo.grid_to_vec();
            trans.scale = Vec3::ONE;
        }
    }
}

fn act_jiggles(
    all_boards: Res<AllBoards>,
    brds: Query<(&CartesianBoard6x12, Entity)>,
    mut puys: Query<(&Puyo, &PuyoState, &mut Transform)>,
) {
    //Board,x -> y,Puyo
    let mut columns: HashMap<(Entity, u32), Vec<(u32, Entity, f32)>> = Default::default();
    for ((board, (x, y)), (puyent, ty)) in all_boards.0.iter() {
        if let Ok((puy, state, _)) = puys.get(*puyent) {
            let vj = match *state {
                PuyoState::Fall { .. } => continue,
                PuyoState::Jiggle { offset, .. } => offset,
                _ => 0.0,
            };
            columns
                .entry((*board, *x))
                .and_modify(|col| col.push((*y, *puyent, vj)))
                .or_insert_with(|| vec![(*y, *puyent, vj)]);
        }
    }
    for (board_state, board_ent) in brds
        .iter()
        .filter(|(bs, _)| bs.state == CartesianState::Physics)
    {
        for ((maybe_board_ent, x), mut col) in columns.iter_mut() {
            if maybe_board_ent != &board_ent {
                continue;
            }
            col.sort_by_key(|(key, _, _)| *key);
            let mut y_top = 0.0f32;
            for (y, puy_ent, offset) in col {
                let Ok((_, _, mut transform)) = puys.get_mut(*puy_ent) else {
                    continue;
                };
                let y = *y as f32;
                let offset = offset.max(-1.0);
                let y_scale = offset + 1.0;
                let xz_scale = y_scale.max(0.5).recip();
                transform.scale = vec3(xz_scale, y_scale, xz_scale);
                y_top += y_scale * 0.4;
                transform.translation = vec3(*x as f32, y_top, 0.0);
                y_top += y_scale * 0.4;
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<PuyoStiffDamp>()
        .init_resource::<AllBoards>()
        .register_type::<VertJiggleSource>()
        .register_type::<PuyoStiffDamp>()
        .add_systems(
            Update,
            (
                update_all_boards,
                spread_jiggle_sources,
                integrate_vert_jiggle,
                remove_jiggles,
                act_jiggles,
            ),
        );
}
