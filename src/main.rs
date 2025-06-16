pub mod puy_ass;
pub mod puyo_chara;
pub mod screensaver_rule;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
};
use bevy_rand::{global::GlobalEntropy, plugin::EntropyPlugin, prelude::Xoshiro128Plus};
use puy_ass::PuyoAssets;
use puyo_chara::PuyoType;
use screensaver_rule::{EveryoneSPhysProp, SBState};

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
        .add_plugins(ResourceInspectorPlugin::<EveryoneSPhysProp>::default())
        .run();
}

fn main_plugin(app: &mut App) {
    app.init_resource::<PuyoAssets>()
        .register_type::<PuyoType>()
        .add_systems(Startup, start);
}

fn start(mut cmd: Commands) {
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
    cmd.spawn((
        screensaver_rule::SBoard {
            score: 0,
            chain: 0,
            columns: (0..10).map(|_| vec![]).collect(),
            state: SBState::Still,
        },
        Transform::from_xyz(-10.5, -9.6, -29.7),
        InheritedVisibility::VISIBLE,
    ));
    cmd.spawn((
        screensaver_rule::SBoard {
            score: 0,
            chain: 0,
            columns: (0..10).map(|_| vec![]).collect(),
            state: SBState::Still,
        },
        Transform::from_xyz(0.5, -9.6, -29.7),
        InheritedVisibility::VISIBLE,
    ));
}
