use super::Puyo;
use bevy::prelude::*;

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
