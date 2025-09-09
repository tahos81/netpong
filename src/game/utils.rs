use bevy::math::Vec2;

// Clamp velocity angle to prevent ball from going too vertical
pub fn clamp_velocity_angle(velocity: Vec2, max_angle: f32) -> Vec2 {
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
