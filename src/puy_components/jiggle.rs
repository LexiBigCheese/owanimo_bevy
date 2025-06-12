use bevy::prelude::*;

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Default)]
pub struct VertJiggle {
    pub offset: f32,
    pub vel: f32,
    ///If this reaches zero, the animation MUST stop!
    pub life: f32,
}

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
            stiff: 1.0,
            damp: 1.0,
        }
    }
}

pub fn integrate_vert_jiggle(
    mut vert_jiggles: Query<&mut VertJiggle>,
    time: Res<Time>,
    sd: Res<PuyoStiffDamp>,
) {
    vert_jiggles.par_iter_mut().for_each(|mut vj| {
        let acc = sd.stiff * -vj.offset + vj.life.max(0.1).recip();
        let acc = acc * time.delta_secs();
        let vel = (vj.vel + acc) * sd.damp;
        *vj = VertJiggle {
            offset: vj.offset + vj.vel * time.delta_secs(),
            vel, //acc ∝ -offset, acc ∝ 1/life
            life: vj.life - time.delta_secs(),
        };
    });
}
