use bevy::{
    ecs::resource::Resource,
    prelude::{Deref, DerefMut},
};

#[derive(Resource, Default)]
pub struct Score(pub u32, pub u32);

#[derive(Resource, Deref, DerefMut)]
pub struct Paused(pub bool);

impl Paused {
    pub fn new(paused: bool) -> Self {
        Paused(paused)
    }
}
