use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct Score(pub u32, pub u32);

#[derive(Resource)]
pub struct Paused(pub bool);
