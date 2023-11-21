use bevy::{prelude::*, utils::{HashSet, HashMap}};

//유닛에 공통적으로 들어가는 컴포넌트들

#[derive(Component)]
pub struct BaseState{
    pub name: String,
    pub state: State,
    pub extra_state: State,

    pub skill: HashSet<i32>,

    pub hunger: f32,
    pub fun: f32,
    pub clean: HashMap<i32, (f32, f32)>, //무엇으로 부터 더러운지
    pub excretion: f32,
    pub stress: f32,
    pub sleep: f32,
    pub kind: u8,

    pub res_memory: HashSet<Entity>
}
pub struct State{
    pub str: f32,
    pub int: f32,
    pub dex: f32,
    pub cha: f32,
    pub con: f32,
}
impl Default for State{
    fn default() -> Self {
        State { str: 0., int: 0., dex: 0., cha: 0., con: 0. }
    }
}
#[derive(Component)]
pub struct Personality{
    pub relationship: HashMap<Entity, i32>, //누구와 관계가 어느정도인지
    pub mision: HashMap<Entity, Vec<HashMap<i32, (f32, f32, f32)>>> // 누가 미션을 줬고, 어떤 미션(혹은 아이템)이며, 수량과 품질은 어느정도로 필요하며, 언제까지 완수인지
}
pub enum Purpose{
    ResourceGathering(i32),
    ResourceStoraging,
    RelieveDesire,
    Management,
    Patrol,
    Idle
}
#[derive(Component)]
pub struct UnitAction{
    pub purpose: Option<Purpose>, // 현재 목적이 무엇인지 (행동 방침이 뭔가)
    pub target: Option<Entity>, //뭘 타겟으로 하고있는지(보통 이동목표)
    pub timer: Timer
}
impl Default for UnitAction{
    fn default() -> Self {
        UnitAction { purpose: None, target: None, timer: Timer::from_seconds(0.7, TimerMode::Repeating) }
    }
}

#[derive(Bundle)]
pub struct UnitBundle{
    pub base_state: BaseState,
    pub unit_action: UnitAction,
}