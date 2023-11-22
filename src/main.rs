use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use bevy_pancam::{PanCam, PanCamPlugin};
use inventory::{inventory_heal, InventoryPlugin};
use config::*;
use animation_indices::*;
use setup::gen_random_world;
use bee::*;
use unit_state::UnitStatePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod config;
mod animation_indices;
mod inventory;
mod setup;
mod bee;
mod unit_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins((
            PanCamPlugin::default(),
            UnitStatePlugin,
            InventoryPlugin,
            AnimationIndicesPlugin,
            WorldInspectorPlugin::new(),
            BeePlugin
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_translation(Vec3::new((MAP_W/2*8) as f32, (MAP_H/2*8) as f32, 0.)),
        camera_2d: Camera2d{
            clear_color: ClearColorConfig::Custom(Color::GRAY)
        },
        ..default()
    }).insert(PanCam::default());

    spawn_1black_queen_bee(&mut commands, &asset_server, &mut texture_atlases,1.,Vec2::new((MAP_W/2*8) as f32, (MAP_H/2*8) as f32));
    gen_random_world(&mut commands, &asset_server, &mut texture_atlases);
}


