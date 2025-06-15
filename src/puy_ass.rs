use std::f32::consts::PI;

use bevy::prelude::*;

use crate::puyo_chara::PuyoType;

#[derive(Resource)]
pub struct PuyoAssets {
    nuisance: Handle<Scene>,
    red: Handle<Scene>,
    green: Handle<Scene>,
    blue: Handle<Scene>,
    yellow: Handle<Scene>,
    purple: Handle<Scene>,
    big_nuisance: Handle<Scene>,
}

impl FromWorld for PuyoAssets {
    fn from_world(world: &mut World) -> Self {
        let Some(asset_server): Option<&AssetServer> = world.get_resource() else {
            panic!("No asset server!?!?")
        };
        PuyoAssets {
            nuisance: asset_server.load(GltfAssetLabel::Scene(0).from_asset("nuisancepuyo.glb")),
            red: asset_server.load(GltfAssetLabel::Scene(0).from_asset("redpuyo.glb")),
            green: asset_server.load(GltfAssetLabel::Scene(0).from_asset("greenpuyo.glb")),
            blue: asset_server.load(GltfAssetLabel::Scene(0).from_asset("bluepuyo.glb")),
            yellow: asset_server.load(GltfAssetLabel::Scene(0).from_asset("yellowpuyo.glb")),
            purple: asset_server.load(GltfAssetLabel::Scene(0).from_asset("purplepuyo.glb")),
            big_nuisance: asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("bignuisancepuyo.glb")),
        }
    }
}

impl PuyoAssets {
    pub fn spawn(&self, cmd: &mut Commands, kind: PuyoType, child_of: Entity) {
        if let PuyoType::NuisanceBR | PuyoType::NuisanceTL | PuyoType::NuisanceTR = kind {
            return;
        }
        cmd.spawn((
            SceneRoot(match kind {
                PuyoType::Nuisance => self.nuisance.clone(),
                PuyoType::Red => self.red.clone(),
                PuyoType::Green => self.green.clone(),
                PuyoType::Blue => self.blue.clone(),
                PuyoType::Yellow => self.yellow.clone(),
                PuyoType::Purple => self.purple.clone(),
                PuyoType::NuisanceBL => self.big_nuisance.clone(),
                _ => unreachable!(),
            }),
            ChildOf(child_of),
            Visibility::Inherited,
            Transform::from_scale(Vec3::ONE * 0.5)
                .with_rotation(Quat::from_axis_angle(Vec3::Y, -90.0 * (PI / 180.0))),
        ));
    }
}
