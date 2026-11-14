use bevy::prelude::*;
use crate::effects::particles::SpawnParticlesMessage;
use crate::game::GameCamera;
use crate::game::resources::{ArenaBounds, PlayerStats};
use crate::player::Player;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player_stats: Res<PlayerStats>,
    arena: Res<ArenaBounds>,
    mut particle_writer: MessageWriter<SpawnParticlesMessage>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut transform, mut player)) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    // 1. Gather Directional Input
    let mut input_dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        input_dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        input_dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        input_dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        input_dir.x += 1.0;
    }

    if input_dir != Vec2::ZERO {
        input_dir = input_dir.normalize();
    }

    // 2. Smooth Acceleration & Damping (Arcade Feel)
    let accel = 1800.0;
    let friction = 9.0;

    if input_dir != Vec2::ZERO {
        player.velocity += input_dir * accel * dt;
        let max_speed = player_stats.move_speed;
        if player.velocity.length() > max_speed {
            player.velocity = player.velocity.normalize() * max_speed;
        }

        // Engine Thruster Particle Trail
        player.engine_trail_timer += dt;
        if player.engine_trail_timer >= 0.05 {
            player.engine_trail_timer = 0.0;
            let exhaust_dir = -player.velocity.normalize_or_zero();
            let exhaust_pos = transform.translation.truncate() + exhaust_dir * 14.0;
            particle_writer.write(SpawnParticlesMessage {
                position: exhaust_pos,
                count: 1,
                color: Color::srgba(0.0, 0.9, 1.0, 0.7),
                speed_min: 20.0,
                speed_max: 60.0,
                lifetime: 0.22,
                size: 3.5,
            });
        }
    } else {
        // Apply friction deceleration
        player.velocity *= (1.0 - friction * dt).max(0.0);
    }

    // 3. Apply Velocity to Position
    transform.translation.x += player.velocity.x * dt;
    transform.translation.y += player.velocity.y * dt;

    // 4. Clamp within Arena Perimeter
    let clamped = arena.clamp(transform.translation.truncate(), 22.0);
    transform.translation.x = clamped.x;
    transform.translation.y = clamped.y;
}

pub fn player_aiming(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok((mut player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        let player_pos = player_transform.translation.truncate();
        let aim_vec = world_pos - player_pos;
        if aim_vec != Vec2::ZERO {
            let target_angle = aim_vec.y.atan2(aim_vec.x);
            player.facing_angle = target_angle;
            // Face the aiming direction
            player_transform.rotation = Quat::from_rotation_z(target_angle);
        }
    }
}
