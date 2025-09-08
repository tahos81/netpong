use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

const LEFT_EDGE: f32 = -650.0;
const RIGHT_EDGE: f32 = 650.0;

const BOTTOM_WALL: f32 = -350.;
const TOP_WALL: f32 = 350.;

const PADDLE_MARGIN: f32 = 72.0;
const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 120.0);
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

const BALL_RADIUS: f32 = 8.0;
const BALL_SPEED: f32 = 700.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.0);

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

#[derive(Debug)]
enum Player {
    Left,
    Right,
}

#[derive(Component)]
struct Paddle(Player);

#[derive(Component)]
struct Ball;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct ScoreboardUi;

#[derive(Event)]
pub struct Scored {
    player: Player,
}

#[derive(Resource)]
struct Score(u32, u32);

#[derive(Resource)]
struct Paused(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_camera, spawn_paddles, spawn_ball, spawn_scoreboard),
        )
        .insert_resource(Score(0, 0))
        .insert_resource(Paused(true))
        .add_event::<Scored>()
        .add_systems(
            FixedUpdate,
            (handle_input, handle_velocity, check_for_collisions),
        )
        .add_systems(Update, (handle_scored, update_scoreboard))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_paddles(mut commands: Commands) {
    // Left paddle
    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(LEFT_EDGE + PADDLE_MARGIN, 0.0, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle(Player::Left),
        Collider,
    ));

    // Right paddle
    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(RIGHT_EDGE - PADDLE_MARGIN, 0.0, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle(Player::Right),
        Collider,
    ));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::srgb(0.75, 0.25, 0.25))),
        Transform::default(),
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));
}

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        )],
    ));
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = format!("{} - {}", score.0, score.1);
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

    if ball_transform.translation.y > TOP_WALL {
        ball_velocity.y = -ball_velocity.y;
    }
    if ball_transform.translation.y < BOTTOM_WALL {
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
                let spin_factor = 0.50; // How much the paddle position affects angle
                new_velocity.y += offset * BALL_SPEED * spin_factor;

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

// Clamp velocity angle to prevent ball from going too vertical
fn clamp_velocity_angle(velocity: Vec2, max_angle: f32) -> Vec2 {
    let speed = velocity.length();
    if speed == 0.0 {
        return velocity;
    }

    let angle = velocity.y.atan2(velocity.x.abs());

    // Clamp angle between -max_angle and max_angle
    let clamped_angle = angle.clamp(-max_angle, max_angle);

    // Reconstruct velocity with clamped angle, preserving x direction
    let x_sign = velocity.x.signum();
    Vec2::new(
        clamped_angle.cos() * speed * x_sign,
        clamped_angle.sin() * speed,
    )
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
