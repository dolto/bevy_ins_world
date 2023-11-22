use bevy::{prelude::*, utils::HashMap};
use noise::{Perlin, NoiseFn};
use rand::Rng;

use crate::{config::*, inventory::Inventory, unit_state::UnitState};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ResEntity{
    pub weight: UnitState,
    pub need_skill: Vec<i32>,
    pub need_time: f32
}

pub fn gen_random_world(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen());
    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16., 16.),
        6,
        11,
        None, None
    );
    let handle = texture_atlases.add(texture_atlas);
    for x in 0..MAP_W as i32 {
        for y in 0..MAP_H as i32 {
            let noise_val = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
            let chance = rng.gen_range(0.0..1.0);

            if noise_val > 0.4 && noise_val < 0.42 && chance > 0.5{
                commands.spawn(
                    (
                        SpriteSheetBundle{
                            texture_atlas: handle.clone(),
                            sprite: TextureAtlasSprite{
                                custom_size: Some(Vec2::splat(rng.gen_range(8.0..=16.))),
                                index: rng.gen_range(33..=35),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(x as f32 * 8., y as f32 * 8., y as f32 * -1.)),
                            ..default()
                        },
                    )
                );
            }
            else if noise_val > 0.3 {
                let mut inventory = Inventory{
                    items:HashMap::with_capacity(4),
                    size: 0.,
                    weight: 0.,
                    max_size: 100.,
                    max_weight: 100.,
                    only: false,
                };
                inventory.add(SUGER_WATER, (rng.gen_range(5.0..15.), rng.gen_range(5.0..30.)));
                inventory.add(POLLEN, (rng.gen_range(15.0..30.), rng.gen_range(5.0..10.)));
                inventory.add(FLAWER_SEED, (rng.gen_range(5.0..10.), rng.gen_range(1.0..10.)));
                inventory.add(GRASS, (rng.gen_range(15.0..35.), rng.gen_range(2.0..5.)));
                commands.spawn(
                    (
                        SpriteSheetBundle{
                            texture_atlas: handle.clone(),
                            sprite: TextureAtlasSprite{
                                custom_size: Some(Vec2::splat(rng.gen_range(8.0..=16.))),
                                index: rng.gen_range(27..=29),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(x as f32 * 8., y as f32 * 8., y as f32 * -1.)),
                            ..default()
                        },
                        inventory,
                        ResEntity{
                            weight: UnitState { str: 0.3, int: 0., dex: 0.6, cha: 0., con: 0.1 },
                            need_skill: vec![FLOWER_RES_GECHAR],
                            need_time: 1.
                        }
                    )
                );
            }
            else if noise_val > 0.1{
                let mut inventory = Inventory{
                    items:HashMap::with_capacity(2),
                    size: 0.,
                    weight: 0.,
                    max_size: 100.,
                    max_weight: 100.,
                    only: false,
                };
                inventory.add(GRASS_SEED, (rng.gen_range(5.0..10.), rng.gen_range(1.0..10.)));
                inventory.add(GRASS, (rng.gen_range(25.0..45.), rng.gen_range(5.0..15.)));
                commands.spawn(
                    (
                        SpriteSheetBundle{
                            texture_atlas: handle.clone(),
                            sprite: TextureAtlasSprite{
                                custom_size: Some(Vec2::splat(rng.gen_range(8.0..=16.))),
                                index: rng.gen_range(21..=23),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(x as f32 * 8., y as f32 * 8., y as f32 * -1.)),
                            ..default()
                        },
                        inventory,
                        ResEntity{
                            weight: UnitState { str: 0.5, int: 0., dex: 0.4, cha: 0., con: 0.1 },
                            need_skill: vec![],
                            need_time: 1.
                        }
                    )
                );
            }

            // // Mountains
            // if noise_val > 0.3 && noise_val < 0.31 {
            //     tiles.push(Tile::new((x, y), 4, 6));
            // }
        }
    }
}