use std::ops::{Add, Mul};

use bevy::{prelude::*, utils::{HashSet, HashMap}};
use rand::Rng;

use crate::config::NAME_LIST;

//유닛에 공통적으로 들어가는 컴포넌트들

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BaseState{
    pub name: String,
    pub state: UnitState,
    pub extra_state: UnitState,

    pub skill: HashSet<i32>,

    pub hunger: f32,
    pub fun: f32,
    pub clean: HashMap<i32, f32>, //무엇으로 부터 더러운지
    pub excretion: f32,
    pub stress: f32,
    pub sleep: f32,

    pub kind: u8,

    pub res_memory: HashMap<Entity, (Timer, f32)>,
    pub gathering_time: f32
}
impl Default for BaseState{
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        BaseState { 
            name: NAME_LIST[rng.gen_range(0..NAME_LIST.len())].to_string(),
            state: UnitState::default(), 
            extra_state: UnitState::default(), 
            skill: HashSet::with_capacity(10), 
            hunger: 0., 
            fun: 0., 
            clean: HashMap::with_capacity(10), 
            excretion: 0., 
            stress: 0., 
            sleep: 0., 
            kind: rng.gen(), 
            res_memory: HashMap::with_capacity(10),
            gathering_time: 0.
        }
    }
}
impl BaseState{
    pub fn get_speed(&self) -> f32{
        let mut speed = (self.state.str + self.extra_state.str) * 1.2 + (self.state.dex + self.extra_state.dex);
        if self.hunger > 80. {
            speed *= 0.7;
        }
        if self.sleep > 80. {
            speed *= 0.7;
        }
        let mut clean = 0.;
        self.clean.iter().for_each(|(_, v)|{
            clean += v;
        });
        if clean > 100. {
            speed *= 1. / (clean / 100.);
        }
        speed
    }
}
#[derive(Reflect, Clone)]
pub struct UnitState{
    pub str: f32,
    pub int: f32,
    pub dex: f32,
    pub cha: f32,
    pub con: f32,
}
impl Default for UnitState{
    fn default() -> Self {
        UnitState { str: 0., int: 0., dex: 0., cha: 0., con: 0. }
    }
}
impl Add for UnitState{
    fn add(self, rhs: Self) -> Self::Output {
        UnitState{
            str: self.str + rhs.str,
            int: self.int + rhs.int,
            dex: self.dex + rhs.dex,
            cha: self.cha + rhs.cha,
            con: self.con + rhs.con
        }
    }
    type Output = Self;
}
impl Mul for UnitState{
    fn mul(self, rhs: Self) -> Self::Output {
        UnitState{
            str: self.str * rhs.str,
            int: self.int * rhs.int,
            dex: self.dex * rhs.dex,
            cha: self.cha * rhs.cha,
            con: self.con * rhs.con
        }
    }
    type Output = Self;
}
impl UnitState{
    pub fn combine(&self) -> f32{
        self.str + self.cha + self.con + self.dex + self.int
    }
}
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Personality{
    pub relationship: HashMap<Entity, i32>, //누구와 관계가 어느정도인지
    pub mision: HashMap<Entity, Vec<HashMap<i32, (f32, f32, f32)>>> // 누가 미션을 줬고, 어떤 미션(혹은 아이템)이며, 수량과 품질은 어느정도로 필요하며, 언제까지 완수인지
}
#[derive(Reflect, Clone, PartialEq)]
pub enum Purpose{
    ResourceGathering(i32),
    ResourceStoraging,
    RelieveDesire,
    Management,
    Making(i32),
    Patrol,
    //Idle,
    Attack,
    MakeOrder
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UnitAction{
    pub purpose: Option<Purpose>, // 현재 목적이 무엇인지 (행동 방침이 뭔가)
    pub target: Option<Entity>, //뭘 타겟으로 하고있는지(보통 이동목표)
    pub timer: Timer,
    pub purpose_is_end: Option<Purpose>
}
impl Default for UnitAction{
    fn default() -> Self {
        UnitAction { purpose: None, target: None, timer: Timer::from_seconds(0.7, TimerMode::Repeating), purpose_is_end: None }
    }
}

#[derive(Bundle)]
pub struct UnitBundle{
    pub base_state: BaseState,
    pub unit_action: UnitAction,
}

pub struct UnitStatePlugin;
impl Plugin for UnitStatePlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            update_desire_and_action,
            unit_move_action
        ));
    }
}

pub fn update_desire_and_action(
    mut units: Query<(&mut UnitAction, &mut BaseState)>,
    time: Res<Time>
){
    let delta = time.delta();
    for (mut unit, mut base_state) in &mut units{
        unit.timer.tick(delta);
        if unit.timer.finished(){
            let max_desire = base_state.stress
            .max(base_state.excretion)
            .max(base_state.fun)
            .max(base_state.hunger)
            .max(base_state.sleep);
            if max_desire >= 70.{
                unit.purpose = Some(Purpose::RelieveDesire);
            }
            base_state.stress += 0.7 - base_state.fun * 0.01;
            base_state.hunger += 0.5;
            base_state.excretion += 1. - base_state.hunger * 0.01;
            base_state.sleep += 0.3;
            base_state.fun += 0.5;
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct UnitMove(pub Vec3);
pub fn unit_move_action(
    mut commands: Commands,
    mut moving_man: Query<(&mut Transform, &BaseState, &UnitMove, Entity)>,
    time: Res<Time>
){
    let delta_sec = time.delta_seconds();
    for (mut trans, base, m_point, entity) in &mut moving_man{
        let distance = trans.translation.distance(m_point.0);
        if distance > 6.{
            let speed = base.get_speed() / delta_sec;
            let mut direction = trans.translation - m_point.0;
            let direc_max = direction.x + direction.y + direction.z;
            direction = Vec3::from_array([direction.x / direc_max, direction.y / direc_max, direction.z / direc_max]);
            trans.translation += direction * speed;
        }else{
            commands.entity(entity).remove::<UnitMove>();
        }
    }
}