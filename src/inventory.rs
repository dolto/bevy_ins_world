use std::ops::Range;

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

use crate::{config::*, unit_state::{UnitState, self}};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Inventory{
    pub items: HashMap<i32,(f32, f32)>, // 종류, 품질, 수량
    pub size: f32,
    pub max_size: f32,
    pub weight: f32,
    pub max_weight: f32,
    pub only: bool
}
impl Inventory{
    pub fn add(&mut self, category:i32 ,value: (f32,f32)) -> bool{
        let weight =
        match category {
            HONEY => {1.5},
            WORM => {2.},
            BEE_WAX => {2.},
            _ => {1.},
        };
        self.size += value.0;
        self.weight += value.0 * weight;
        if self.size > self.max_size || self.weight > self.max_weight{
            self.size -= value.0;
            self.weight -= value.0 * weight;
            return false;
        }
        if self.items.contains_key(&category){
            let _value = self.items[&category];
            let combine = value.0 + _value.0;
    
            let distens = value.0 / combine * value.0 + _value.0 / combine * _value.0;
            let new_value = value.1 + _value.1;
    
            self.items.insert(category, (distens, new_value));
        }
        else{
            self.items.insert(category, value);
        }
        true
    }
    pub fn gathering(
        &mut self,
        unit_state: UnitState,
        weight: UnitState,
    ) -> HashMap<i32, (f32, f32)>{
        let mut rng = rand::thread_rng();
        let power = (unit_state * weight).combine() / self.items.len() as f32;
        let mut result = HashMap::with_capacity(self.items.len());
        self.items.iter_mut().for_each(|(key,(q,v))|{
            let rand_point = rng.gen_range(0.7..=1.2);
            let _power = power*rand_point;
            result.insert(key.clone(), (q.clone(), _power.min(v.clone())));
            *v = (v.clone() - _power).max(0.01);
        });
        result
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InventoryHeal{
    pub is_heal: bool,
    pub timer: Timer,
    pub items: HashMap<i32,(Range<f32>, Range<f32>)>
}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            inventory_heal,
        ));
    }
}

pub fn inventory_heal(
    mut inventory: Query<(&mut Inventory, &mut InventoryHeal)>,
    time: Res<Time>
){
    let delta = time.delta();
    let mut rng = rand::thread_rng();
    for (mut inventory, mut heal) in inventory.iter_mut(){
        if !heal.is_heal{
            continue;
        }
        heal.timer.tick(delta);
        if heal.timer.finished(){
            heal.items.iter().for_each(|h|{
                inventory.add(*h.0, (rng.gen_range(h.1.0.clone()),rng.gen_range(h.1.1.clone())));
            })
        }
    }
}