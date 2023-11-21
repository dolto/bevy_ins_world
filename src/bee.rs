use bevy::prelude::*;
use crate::{config::*, animation_indices::{AnimationIndices, AnimationTimer}};

pub fn spawn_1black_queen_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    superiority: f32,
    pos: Vec2
){
    spawn_custom_bee(
        commands, asset_server, texture_atlases, superiority, pos, 6,8
    );
}
pub fn spawn_1black_normal_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    superiority: f32,
    pos: Vec2
){
    spawn_custom_bee(
        commands, asset_server, texture_atlases, superiority, pos, 0,2
    );
}
pub fn spawn_1black_male_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    superiority: f32,
    pos: Vec2
){
    spawn_custom_bee(
        commands, asset_server, texture_atlases, superiority, pos, 12,14
    );
}


fn spawn_custom_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    superiority: f32,
    pos: Vec2,
    first: usize, last: usize
){
    let texture_handle: Handle<Image> = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 6, 11, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices::from_ping_pong(first,last);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite{
                color: Color::YELLOW,
                custom_size:Some(Vec2::splat(16. * superiority)),
                ..default()
            },
            transform: Transform::from_translation(pos.extend(pos.y/8.*-1.)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    ));
}