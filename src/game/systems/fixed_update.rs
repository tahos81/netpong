use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::game::{
    common::Player,
    components::{Ball, Collider, Paddle, Velocity},
    constants::*,
    events::Scored,
    resources::Paused,
    utils::clamp_velocity_angle,
};

pub struct FixedUpdatePlugin;

impl Plugin for FixedUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                handle_velocity,      // move ball
                handle_input,         // move paddles
                check_for_collisions, // handle collisions
            )
                .chain(),
        );
    }
}

/// Framerate-independent input handling
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paused: ResMut<Paused>,
    mut query: Query<(&mut Transform, &Paddle)>,
) {
    if paused.0 {
        if keyboard_input.just_pressed(KeyCode::Space) {
            paused.0 = false;
        }
        return;
    }

    for (mut transform, paddle) in &mut query {
        let mut direction = Vec2::ZERO;

        match paddle.0 {
            Player::Left => {
                if keyboard_input.pressed(KeyCode::KeyW) {
                    direction.y += 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    direction.y -= 1.0;
                }
            }
            Player::Right => {
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    direction.y += 1.0;
                }
                if keyboard_input.pressed(KeyCode::ArrowDown) {
                    direction.y -= 1.0;
                }
            }
        }

        if direction != Vec2::ZERO {
            transform.translation += (direction * PADDLE_SPEED * time.delta_secs()).extend(0.0);
            transform.translation = transform.translation.clamp(
                Vec3::new(f32::MIN, BOTTOM_WALL + PADDLE_SIZE.y / 2., 0.0),
                Vec3::new(f32::MAX, TOP_WALL - PADDLE_SIZE.y / 2., 0.0),
            );
        }
    }
}

fn handle_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
    paused: Res<Paused>,
) {
    if !paused.0 {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * time.delta_secs();
            transform.translation.y += velocity.y * time.delta_secs();
        }
    }
}

fn check_for_collisions(
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, (With<Collider>, Without<Ball>)>,
    mut events: EventWriter<Scored>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();

    if ball_transform.translation.x < LEFT_EDGE {
        events.write(Scored {
            player: Player::Right,
        });
        return;
    } else if ball_transform.translation.x > RIGHT_EDGE {
        events.write(Scored {
            player: Player::Left,
        });
        return;
    }

    if ball_transform.translation.y > TOP_WALL && ball_velocity.y > 0. {
        ball_velocity.y = -ball_velocity.y;
    }
    if ball_transform.translation.y < BOTTOM_WALL && ball_velocity.y < 0. {
        ball_velocity.y = -ball_velocity.y;
    }

    for collider_transform in &collider_query {
        let ball_body = BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS);
        let collider_body = Aabb2d::new(
            collider_transform.translation.truncate(),
            collider_transform.scale.truncate() / 2.,
        );

        if ball_body.intersects(&collider_body) {
            // Determine which paddle (left or right) based on position
            let is_left_paddle = collider_transform.translation.x < 0.0;

            // Only reflect if ball is moving toward the paddle (prevents multiple collisions)
            let should_reflect = (is_left_paddle && ball_velocity.x < 0.0)
                || (!is_left_paddle && ball_velocity.x > 0.0);

            if should_reflect {
                // Calculate hit offset (-1 at bottom, +1 at top of paddle)
                let offset = ((ball_transform.translation.y - collider_transform.translation.y)
                    / (PADDLE_SIZE.y / 2.0))
                    .clamp(-1.0, 1.0);

                // Basic reflection: reverse X direction
                let mut new_velocity = Vec2::new(-ball_velocity.x, ball_velocity.y);

                // Add spin based on where ball hit the paddle
                // Hitting top of paddle adds upward velocity, bottom adds downward
                new_velocity.y += offset * BALL_SPEED * SPIN_FACTOR;

                // Normalize and maintain constant speed
                new_velocity = new_velocity.normalize() * BALL_SPEED;

                // Clamp angle to prevent ball from going too vertical
                let max_angle = std::f32::consts::FRAC_PI_3; // 60 degrees
                new_velocity = clamp_velocity_angle(new_velocity, max_angle);

                // Update ball velocity
                ball_velocity.0 = new_velocity;
            }
        }
    }
}
