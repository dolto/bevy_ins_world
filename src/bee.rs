use bevy::{prelude::*, utils::{HashMap, HashSet}};
use rand::Rng;
use crate::{config::*, animation_indices::*, unit_state::*, inventory::Inventory, setup::ResEntity};

pub const BASE_SKILL: [i32; 4] = [
    FLOWER_RES_GECHAR, MAKE_HONEY, MAKE_BEE_WAX, MAKE_COMB
];

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bee{
    pub jal: f32, // 로열젤리 기반 능력
    pub extra_jal: f32,
    pub belong: Option<Entity>, //소속
    pub influence: f32 
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeeQean{
    pub order: Option<Purpose>,
    pub resource: HashMap<i32, f32>
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
    commands.entity(entity).insert(BeeQean{
        order: None,
        resource: HashMap::with_capacity(10)
    });
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

    let base_state = UnitState{
        str: superiority * 10.,
        int: superiority * 10.,
        cha: superiority * 4.,
        con: superiority * 15.,
        dex: superiority * 10.,
    };
    
    let entity = commands.spawn((
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
                state: base_state.clone(),
                extra_state: UnitState::default(),
                skill: base_skill,
                hunger: 0.,
                fun: 0.,
                clean: HashMap::with_capacity(9),
                excretion: 0.,
                stress: 0.,
                sleep: 0.,
                kind: rng.gen(),
                res_memory: HashMap::with_capacity(30),
                gathering_time: 0.
            },
            unit_action: UnitAction::default(),
        },
        Inventory{
            items: HashMap::with_capacity(30),
            size: 0.,
            max_size: base_state.str * 1.5 + base_state.dex * 2.,
            weight: 0.,
            max_weight: base_state.str * 2. + base_state.con * 1.5,
            only: false
        }
    )).id();
    commands.entity(entity).insert(Bee{
        belong: if belong.is_none(){Some(entity)} else{belong},
        jal: 0.,
        extra_jal: 0.,
        influence: 0.
    });

    entity
}

fn set_bee_purpose(
    mut action: Query<(&mut UnitAction, &Bee, &Inventory)>,
    belong: Query<&BeeQean>
){
    for (mut unit, bee, inventory) in &mut action{
        let _belong = bee.belong.unwrap();
        let belong_purpose = &belong.get(_belong).unwrap().order;
        let mut rng = rand::thread_rng();

        if unit.purpose.is_none(){
            if belong_purpose.is_none() && bee.influence >= 10.{
                unit.purpose = Some(Purpose::MakeOrder);
            }
            else if rng.gen_range(0..10) < 3{
                let temp = belong_purpose.clone();
                unit.purpose = temp;
            }
            else if rng.gen_range(0..10) < 3{
                let list = [Purpose::Management, Purpose::Patrol, Purpose::RelieveDesire];
                let temp = list[rng.gen_range(0..list.len())].clone();
                unit.purpose = Some(temp);
            }
            else if inventory.size > inventory.max_size * 0.5 || inventory.weight > inventory.max_weight * 0.5{
                let list = [0,2];
                unit.purpose = Some(Purpose::ResourceGathering(list[rng.gen_range(0..list.len())]));
            }
            else{
                unit.purpose = Some(Purpose::ResourceStoraging);
            }
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeeResourceGathering{
    pub id: i32,
    pub distance_ok: bool,
    pub is_moving: bool
}
fn bee_resource_gathering_ai(
    mut commands: Commands,
    mut bee: Query<(&Transform, &mut BaseState, &mut Inventory, &mut BeeResourceGathering, &mut UnitAction ,Entity)>,
    mut res_entity: Query<(&mut Inventory, &Transform, &ResEntity), With<ResEntity>>,
    time: Res<Time>
){
    let delta_sec = time.delta_seconds();
    //자원을 수집하기 위한 행동을 정의
    for (transform,mut base ,mut inventory,mut target_id,mut unit,entity) in &mut bee{
        if unit.purpose == Some(Purpose::RelieveDesire){
            commands.entity(entity).remove::<BeeResourceGathering>();
        }

        if inventory.size / inventory.max_size > 0.7 || inventory.weight / inventory.max_weight > 0.7{
            commands.entity(entity).remove::<BeeResourceGathering>();
            unit.purpose = Some(Purpose::ResourceStoraging);
        }
        else if unit.target.is_none(){
            if base.res_memory.is_empty(){
                commands.entity(entity).remove::<BeeResourceGathering>();
                unit.purpose = Some(Purpose::Patrol); //기억속 자원들 중 적합한 채집장소가 없는경우 정찰로 변경
            }
            else {
                let mut target_value:f32 = -99999.;
                let mut target: Option<Entity> = None;
                base.res_memory.iter().for_each(|(entity, (_, value))|{ //기억속에 가까운 거리와 많은 양의 경우
                    let (res_inven, res_trans,res_entity) = res_entity.get(*entity).unwrap();
                    let distance = transform.translation.distance(res_trans.translation);
                    let temp = (value * 3.) - distance;
                    let mut skill_complate = true;
                    for skill in res_entity.need_skill.iter(){
                        if !base.skill.contains(skill){
                            skill_complate = false;
                            break;
                        }
                    }
                    if res_inven.items.contains_key(&target_id.id) && skill_complate && res_inven.items[&target_id.id].1 <= 1.{
                        if target_value < temp {
                            target_value = temp;
                            target = Some(entity.clone());
                        }
                    }
                });
                if let Some(_target) = target{
                    unit.target = Some(_target);
                }else{
                    commands.entity(entity).remove::<BeeResourceGathering>();
                    unit.purpose = Some(Purpose::Patrol); //기억속 자원들 중 적합한 채집장소가 없는경우 정찰로 변경
                }
            }
        }
        else if target_id.distance_ok{
            let (mut res_inven, _,res_entity) = res_entity.get_mut(unit.target.unwrap()).unwrap();
            if res_inven.items[&target_id.id].1 <= 1.{
                target_id.distance_ok = false;
                target_id.is_moving = false;
                unit.target = None;
            }
            else{
                base.gathering_time += delta_sec;
                if base.gathering_time >= res_entity.need_time{
                    base.gathering_time = 0.;
                    let result = 
                        res_inven.gathering(base.state.clone() + base.extra_state.clone(), res_entity.weight.clone()
                    );
                    let mut inven_full = false;
                    result.iter().for_each(|(key, v)|{
                        let get_ok = inventory.add(key.clone(), v.clone());
                        if !get_ok{
                            res_inven.add(key.clone(), v.clone());
                            inven_full = true;
                        }
                    });
                    if inven_full{
                        commands.entity(entity).remove::<BeeResourceGathering>();
                        unit.purpose = Some(Purpose::ResourceStoraging);
                    }
                }
                
            }
        }
        else if !target_id.is_moving{
            let (_, trans,_) = res_entity.get(unit.target.unwrap()).unwrap();
            let distence = transform.translation.distance(trans.translation);
            if distence < 7.{
                target_id.distance_ok = true;
            }else{
                commands.entity(entity).insert(UnitMove(trans.translation));
                target_id.is_moving = true;
            }
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeeResourceStoraging;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeeMakeOrder;
fn bee_make_order_ai(
    
){

}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeeManagement;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeePatrol;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeeRelieveDesire;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BeeAttack;

fn bee_ai(
    mut commands: Commands,
    mut action: Query<(&mut UnitAction, Entity)>,
){
    for (mut unit, entity) in &mut action{
        if unit.purpose != unit.purpose_is_end{
            match unit.purpose {
                Some(Purpose::MakeOrder) => {
                    commands.entity(entity).insert(BeeMakeOrder);
                    unit.purpose_is_end = Some(Purpose::MakeOrder);
                },
                Some(Purpose::Management) => {
                    commands.entity(entity).insert(BeeManagement);
                    unit.purpose_is_end = Some(Purpose::Management);
                },
                Some(Purpose::RelieveDesire) => {
                    commands.entity(entity).insert(BeeRelieveDesire);
                    unit.purpose_is_end = Some(Purpose::RelieveDesire);
                },
                Some(Purpose::Attack) => {
                    commands.entity(entity).insert(BeeAttack);
                    unit.purpose_is_end = Some(Purpose::Attack);
                },
                Some(Purpose::ResourceGathering(i)) => {
                    commands.entity(entity).insert(BeeResourceGathering{
                        id: i,
                        distance_ok: false,
                        is_moving: false
                    });
                    unit.purpose_is_end = Some(Purpose::ResourceGathering(i));
                },
                Some(Purpose::ResourceStoraging) => {
                    commands.entity(entity).insert(BeeResourceStoraging);
                    unit.purpose_is_end = Some(Purpose::ResourceStoraging);
                },
                _ => {}
            }
        }
    }
}



pub struct BeePlugin;
impl Plugin for BeePlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            set_bee_purpose,
            bee_ai,
            bee_resource_gathering_ai,
        ));
    }
}