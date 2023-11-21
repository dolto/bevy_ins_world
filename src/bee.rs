use bevy::{prelude::*, utils::{HashMap, HashSet}};
use rand::Rng;
use crate::{config::*, animation_indices::{AnimationIndices, AnimationTimer}, unit_state::{UnitBundle, BaseState, State, UnitAction}};

pub const BASE_SKILL: [i32; 3] = [
    FLOWER_RES_GECHAR, MAKE_HONEY, MAKE_BEE_WAX
];

#[derive(Component)]
pub struct Bee{
    pub jal: f32, // 로열젤리 기반 능력
    pub extra_jal: f32,
    pub belong: Option<Entity>, //소속
}

#[derive(Component)]
pub struct BeeQean{
}

pub fn spawn_1black_queen_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    superiority: f32,
    pos: Vec2
){
    let entity = spawn_custom_bee(
        commands, asset_server, texture_atlases, superiority, pos, 6,8, None
    );
    commands.entity(entity).insert(BeeQean{});
}
pub fn spawn_1black_normal_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    superiority: f32,
    pos: Vec2
){
    spawn_custom_bee(
        commands, asset_server, texture_atlases, superiority, pos, 0,2, None
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
        commands, asset_server, texture_atlases, superiority, pos, 12,14, None
    );
}


fn spawn_custom_bee(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    superiority: f32,
    pos: Vec2,
    first: usize, last: usize, belong: Option<Entity>
) -> Entity{
    let texture_handle: Handle<Image> = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 6, 11, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices::from_ping_pong(first,last);
    let mut rng = rand::thread_rng();
    let mut base_skill = HashSet::with_capacity(4);
    BASE_SKILL.iter().for_each(|s|{
        base_skill.insert(*s);
    });
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite{
                color: Color::YELLOW,
                custom_size:Some(Vec2::splat(24. * superiority)),
                ..default()
            },
            transform: Transform::from_translation(pos.extend(pos.y/8.*-1.)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        UnitBundle{
            base_state: BaseState{
                name: NAME_LIST[rng.gen_range(0..NAME_LIST.len())].to_string(),
                state: State{
                    str: superiority * 10.,
                    int: superiority * 10.,
                    cha: superiority * 4.,
                    con: superiority * 15.,
                    dex: superiority * 10.,
                },
                extra_state: State::default(),
                skill: base_skill,
                hunger: 0.,
                fun: 0.,
                clean: HashMap::with_capacity(9),
                excretion: 0.,
                stress: 0.,
                sleep: 0.,
                kind: rng.gen(),
                res_memory: HashSet::with_capacity(30)
            },
            unit_action: UnitAction::default(),
        },
        Bee{
            belong,
            jal: 0.,
            extra_jal: 0.
        }
    )).id()
}

pub fn update_desire_and_action(

){

}