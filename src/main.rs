pub mod puy_ass;
pub mod puyo_chara;
pub mod screensaver_rule;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rand::{plugin::EntropyPlugin, prelude::Xoshiro128Plus};
use puy_ass::PuyoAssets;
use puyo_chara::PuyoType;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlyCameraPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(EntropyPlugin::<Xoshiro128Plus>::default())
        .add_plugins(main_plugin)
        .add_plugins(screensaver_rule::screensaver_rule_plugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn main_plugin(app: &mut App) {
    app.init_resource::<PuyoAssets>()
        .register_type::<PuyoType>()
        .add_systems(Startup, start);
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
            intensity: 15000000.0,
            ..Default::default()
        },
        Transform::from_xyz(3.0, 3.0, -7.0),
    ));
}
