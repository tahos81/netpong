use bevy::prelude::*;

use crate::game::{
    common::Player,
    components::{Ball, Paddle, ScoreboardUi, Velocity},
    constants::{BALL_SPEED, INITIAL_BALL_DIRECTION},
    events::Scored,
    resources::{Paused, Score},
};

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_scored, update_scoreboard));
    }
}

fn handle_scored(
    mut scored_events: EventReader<Scored>,
    mut score: ResMut<Score>,
    mut paused: ResMut<Paused>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    ball_query: Single<(&mut Velocity, &mut Transform), (With<Ball>, Without<Paddle>)>,
) {
    if !scored_events.is_empty() {
        paused.0 = true;

        let (mut ball_velocity, mut ball_transform) = ball_query.into_inner();
        *ball_transform = Transform::default();
        *ball_velocity = Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED);

        for mut transform in &mut paddle_query {
            transform.translation.y = 0.0;
        }

        let event = scored_events.read().next().unwrap();
        match event.player {
            Player::Left => {
                score.0 += 1;
            }
            Player::Right => {
                score.1 += 1;
            }
        }
        scored_events.clear();
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = format!("{} - {}", score.0, score.1);
}
