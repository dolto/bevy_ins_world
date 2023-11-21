use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub ping_pong: bool,
    pub is_back: bool
}
impl AnimationIndices{
    pub fn from_ping_pong(first: usize, last: usize) -> Self{
        AnimationIndices{first,last,ping_pong:true,is_back:false}
    }

    pub fn from_normal(first: usize, last: usize) -> Self{
        AnimationIndices{first,last,ping_pong:false,is_back:false}
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index == indices.last {
                if indices.ping_pong{
                    indices.is_back = true;
                }else{
                    sprite.index = indices.first;
                    return;
                }
            }
            
            if indices.ping_pong{
                if indices.is_back{
                    sprite.index -= 1;
                    if sprite.index == indices.first{
                        indices.is_back = false;
                    }
                } else{
                    sprite.index += 1;
                }
            }else{
                sprite.index += 1;
            }
        }
    }
}