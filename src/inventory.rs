use std::ops::Range;

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

#[derive(Component)]
pub struct Inventory{
    pub items: HashMap<i32,(f32, f32)>,
    pub size: f32,
    pub weight: f32,
    pub only: bool
}
impl Inventory{
    pub fn add(&mut self, category:i32 ,value: (f32,f32)){
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
    }
}

#[derive(Component)]
pub struct InventoryHeal{
    pub is_heal: bool,
    pub timer: Timer,
    pub items: HashMap<i32,(Range<f32>, Range<f32>)>
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