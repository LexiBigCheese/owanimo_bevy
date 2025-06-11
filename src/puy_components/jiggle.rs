use bevy::prelude::*;

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Default)]
pub struct VertJiggle {
    pub offset: f32,
    pub vel: f32,
}

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Default)]
pub struct VertJiggleSource(pub f32);

pub fn integrate_vert_jiggle(mut vert_jiggles: Query<&mut VertJiggle>, time: Res<Time>) {
    vert_jiggles.par_iter_mut().for_each(|mut vj| {
        *vj = VertJiggle {
            offset: vj.offset + vj.vel * time.delta_secs(),
            vel: todo!(),
        }
    });
}
