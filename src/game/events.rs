use bevy::ecs::event::Event;

use super::common::Player;

#[derive(Event)]
pub struct Scored {
    pub player: Player,
}
