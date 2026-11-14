use bevy::prelude::*;
use crate::enemies::Enemy;
use crate::player::Player;

pub fn enemy_ai(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &mut Transform, &Enemy), Without<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let dt = time.delta_secs();

    // Collect positions for separation
    let mut positions: Vec<(Entity, Vec2, f32)> = Vec::with_capacity(90);
    for (entity, transform, enemy) in &enemy_query {
        positions.push((entity, transform.translation.truncate(), enemy.size.x * 0.5));
    }

    for (entity, mut transform, enemy) in &mut enemy_query {
        let current_pos = transform.translation.truncate();
        let to_player = player_pos - current_pos;
        let dist = to_player.length();
        if dist < 0.001 {
            continue;
        }

        let mut move_dir = to_player / dist;

        // Soft avoidance from nearby enemy neighbors
        let mut separation = Vec2::ZERO;
        for &(other_entity, other_pos, other_radius) in &positions {
            if other_entity == entity {
                continue;
            }
            let diff = current_pos - other_pos;
            let d = diff.length();
            let min_dist = enemy.size.x * 0.5 + other_radius + 4.0;
            if d < min_dist && d > 0.001 {
                separation += (diff / d) * (1.0 - d / min_dist);
            }
        }

        if separation != Vec2::ZERO {
            move_dir = (move_dir * 0.75 + separation.normalize() * 0.55).normalize_or_zero();
        }

        // Apply translation
        transform.translation.x += move_dir.x * enemy.speed * dt;
        transform.translation.y += move_dir.y * enemy.speed * dt;

        // Rotate to face movement direction
        let angle = move_dir.y.atan2(move_dir.x);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn update_enemy_hit_flash(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Enemy)>,
) {
    let dt = time.delta_secs();
    for (mut sprite, mut enemy) in &mut query {
        if enemy.hit_flash_timer > 0.0 {
            enemy.hit_flash_timer -= dt;
            if enemy.hit_flash_timer <= 0.0 {
                sprite.color = enemy.base_color;
            } else {
                sprite.color = Color::srgb(1.0, 1.0, 1.0); // Bright flash
            }
        }
    }
}

