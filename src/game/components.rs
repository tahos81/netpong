use bevy::{
    ecs::component::Component,
    math::Vec2,
    prelude::{Deref, DerefMut},
};

use super::common::Player;

#[derive(Component, Deref)]
pub struct Paddle(pub Player);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct ScoreboardUi;
