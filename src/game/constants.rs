use bevy::{color::Color, math::Vec2, ui::Val};

pub const LEFT_EDGE: f32 = -650.0;
pub const RIGHT_EDGE: f32 = 650.0;

pub const BOTTOM_WALL: f32 = -350.;
pub const TOP_WALL: f32 = 350.;

pub const PADDLE_MARGIN: f32 = 72.0;
pub const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 120.0);
pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

pub const BALL_RADIUS: f32 = 8.0;
pub const BALL_SPEED: f32 = 700.0;
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.0);
pub const SPIN_FACTOR: f32 = 0.5;
pub const BALL_COLOR: Color = Color::srgb(0.75, 0.25, 0.25);

pub const SCOREBOARD_FONT_SIZE: f32 = 33.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
