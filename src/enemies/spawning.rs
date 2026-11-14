use bevy::prelude::*;
use rand::Rng;
use crate::game::resources::{ArenaBounds, WaveManager};
use crate::enemies::{Enemy, EnemyType};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut wave_manager: ResMut<WaveManager>,
    arena: Res<ArenaBounds>,
    active_enemies: Query<&Enemy>,
) {
    let dt = time.delta_secs();
    wave_manager.spawn_timer += dt;

    if wave_manager.spawn_timer < wave_manager.spawn_interval {
        return;
    }
    wave_manager.spawn_timer = 0.0;

    let current_count = active_enemies.iter().count();
    let max_alive = 90; // Performance guardrail for 8 GB RAM & integrated GPU
    if current_count >= max_alive {
        return;
    }

    if wave_manager.enemies_spawned_this_wave >= wave_manager.max_enemies_this_wave {
        return;
    }

    let mut rng = rand::rng();
    wave_manager.enemies_spawned_this_wave += 1;

    // Pick spawn position along the arena perimeter
    let side = rng.random_range(0..4);
    let offset = 40.0;
    let spawn_pos = match side {
        0 => Vec2::new(rng.random_range(-arena.half_width..arena.half_width), arena.half_height + offset), // Top
        1 => Vec2::new(rng.random_range(-arena.half_width..arena.half_width), -arena.half_height - offset), // Bottom
        2 => Vec2::new(-arena.half_width - offset, rng.random_range(-arena.half_height..arena.half_height)), // Left
        _ => Vec2::new(arena.half_width + offset, rng.random_range(-arena.half_height..arena.half_height)), // Right
    };

    // Determine enemy type based on current wave
    let wave = wave_manager.current_wave;
    let roll: f32 = rng.random_range(0.0..1.0);
    let enemy_type = if wave == 1 {
        EnemyType::Chaser
    } else if wave == 2 {
        if roll < 0.35 {
            EnemyType::Runner
        } else {
            EnemyType::Chaser
        }
    } else if wave == 3 {
        if roll < 0.18 {
            EnemyType::Tank
        } else if roll < 0.55 {
            EnemyType::Runner
        } else {
            EnemyType::Chaser
        }
    } else {
        if roll < 0.25 {
            EnemyType::Tank
        } else if roll < 0.65 {
            EnemyType::Runner
        } else {
            EnemyType::Chaser
        }
    };

    // Wave stat scaling (+7% HP, +4% speed per wave after wave 1)
    let hp_mult = 1.0 + (wave - 1) as f32 * 0.08;
    let spd_mult = (1.0 + (wave - 1) as f32 * 0.035).min(1.4);

    let (base_hp, base_speed, damage, score, xp, base_color, size) = match enemy_type {
        EnemyType::Chaser => (
            35.0 * hp_mult,
            155.0 * spd_mult,
            12.0,
            150,
            10,
            Color::srgb(1.0, 0.55, 0.0), // Neon Amber/Orange
            Vec2::new(26.0, 26.0),
        ),
        EnemyType::Runner => (
            18.0 * hp_mult,
            245.0 * spd_mult,
            8.0,
            220,
            15,
            Color::srgb(1.0, 0.05, 0.6), // Neon Hot Magenta
            Vec2::new(20.0, 20.0),
        ),
        EnemyType::Tank => (
            160.0 * hp_mult,
            75.0 * spd_mult,
            28.0,
            500,
            40,
            Color::srgb(0.65, 0.0, 1.0), // Deep Neon Violet
            Vec2::new(42.0, 42.0),
        ),
    };

    // Inner bright core
    let core_entity = commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(size * 0.45),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.1),
    )).id();

    // Outer glow halo
    let glow_color = match base_color {
        Color::Srgba(c) => Color::srgba(c.red, c.green, c.blue, 0.28),
        _ => Color::srgba(1.0, 0.5, 0.0, 0.28),
    };
    let halo_entity = commands.spawn((
        Sprite {
            color: glow_color,
            custom_size: Some(size * 1.5),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.1),
    )).id();

    // Main enemy entity
    commands.spawn((
        Enemy {
            enemy_type,
            health: base_hp,
            max_health: base_hp,
            speed: base_speed,
            damage,
            score_value: score,
            hit_flash_timer: 0.0,
            base_color,
            size,
            xp_drop: xp,
        },
        Sprite {
            color: base_color,
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(spawn_pos.x, spawn_pos.y, 2.0),
    )).add_children(&[halo_entity, core_entity]);
}

