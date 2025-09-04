use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use bevy::window::WindowResized;

const BALL_RADIUS: f32 = 8.0;
const BALL_SPEED: f32 = 1200.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.0);

#[derive(Debug)]
#[allow(dead_code)]
enum Player {
    One,
    Two,
    Three,
    Four,
}

#[derive(Component)]
struct Paddle(Player);

#[derive(Component)]
struct Ball;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_paddles, spawn_ball))
        .add_systems(Update, apply_margin_on_resize)
        .add_systems(
            FixedUpdate,
            (handle_input, handle_velocity, check_for_collisions),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_paddles(mut commands: Commands) {
    let rect_size = Vec2::new(24.0, 128.0);

    let base_sprite = Sprite {
        color: Color::srgb(0.25, 0.75, 0.25),
        custom_size: Some(rect_size),
        ..default()
    };

    // Left paddle
    commands.spawn((
        base_sprite.clone(),
        Transform::default(),
        Paddle(Player::One),
        Collider,
    ));

    // Right paddle
    commands.spawn((
        base_sprite,
        Transform::default(),
        Paddle(Player::Two),
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

/// Align paddles when the window is resized
fn apply_margin_on_resize(
    mut events: EventReader<WindowResized>,
    mut query: Query<(&mut Transform, &Sprite, &Paddle)>,
) {
    let margin = 32.0;
    for e in events.read() {
        for (mut transform, sprite, paddle) in &mut query {
            let rect_width = sprite.custom_size.unwrap().x;
            let rect_height = sprite.custom_size.unwrap().y;

            match paddle.0 {
                Player::One => transform.translation.x = -e.width / 2.0 + margin + rect_width / 2.0,
                Player::Two => transform.translation.x = e.width / 2.0 - margin - rect_width / 2.0,
                Player::Three => {
                    transform.translation.y = e.height / 2.0 - margin - rect_height / 2.0
                }
                Player::Four => {
                    transform.translation.y = -e.height / 2.0 + margin + rect_height / 2.0
                }
            }
        }
    }
}

/// Framerate-independent input handling
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Paddle)>,
) {
    let speed = 210.0; // units per second

    for (mut transform, paddle) in &mut query {
        let mut velocity = Vec2::ZERO;

        match paddle.0 {
            Player::One => {
                if keyboard_input.pressed(KeyCode::KeyW) {
                    velocity.y += 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    velocity.y -= 1.0;
                }
            }
            Player::Two => {
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    velocity.y += 1.0;
                }
                if keyboard_input.pressed(KeyCode::ArrowDown) {
                    velocity.y -= 1.0;
                }
            }
            _ => {}
        }

        if velocity != Vec2::ZERO {
            transform.translation += (velocity.normalize() * speed * time.delta_secs()).extend(0.0);
        }
    }
}

fn handle_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn check_for_collisions(
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, &Sprite), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();
    for (collider_transform, sprite) in &collider_query {
        let ball_body = BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS);
        let collider_body = Aabb2d::new(
            collider_transform.translation.truncate(),
            sprite.custom_size.unwrap_or(Vec2::ONE) / 2.,
        );

        if ball_body.intersects(&collider_body) {
            // Calculate the normal from the ball position relative to the collider center
            let ball_pos = ball_transform.translation.truncate();
            let collider_pos = collider_transform.translation.truncate();
            let normal = (ball_pos - collider_pos).normalize();

            **ball_velocity = ball_velocity.reflect(normal);
        }
    }
}
