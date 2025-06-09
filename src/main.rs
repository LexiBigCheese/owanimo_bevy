pub mod owanimo_impl;
pub mod puy_ass;
pub mod puy_components;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rand::{plugin::EntropyPlugin, prelude::Xoshiro128Plus};
use puy_ass::PuyoAssets;
use puy_components::{
    banish_puyos, fall_puyo, finish_banishing_puyo, owanimo_puyos, randomise_puys,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlyCameraPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(EntropyPlugin::<Xoshiro128Plus>::default())
        .add_plugins(main_plugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn main_plugin(app: &mut App) {
    app.init_resource::<PuyoAssets>()
        .register_type::<puy_components::Puyo>()
        .register_type::<puy_components::PuyoType>()
        .register_type::<puy_components::CartesianState>()
        .register_type::<puy_components::CartesianBoard6x12>()
        .add_systems(Startup, start)
        .add_systems(Update, randomise_puys)
        .add_systems(
            Update,
            (
                finish_banishing_puyo,
                banish_puyos,
                owanimo_puyos,
                fall_puyo,
            ),
        );
}

fn start(mut cmd: Commands, puy_ass: Res<PuyoAssets>) {
    cmd.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 30.0 * PI / 180.0,
            ..Default::default()
        }),
        Transform::from_xyz(0.0, 0.0, 10.0),
        // FlyCamera::default(),
    ));
    cmd.spawn((
        PointLight {
            range: 40.0,
            ..Default::default()
        },
        Transform::from_xyz(3.0, 3.0, -7.0),
    ));
    puy_components::spawn_cartes_board(
        &mut cmd,
        &puy_ass,
        Transform::from_xyz(-3.0, -6.0 * 0.7, -10.0),
        "
        rrrrrr
        gggggg
        bb__bb
        yy__yy
        pp__pp
        oo__oo
        rr__rr
        gg__gg
        bb__bb
        yy__yy
        ______
        oooooo
    ",
    );
}
