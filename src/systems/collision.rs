use bevy::prelude::*;
use crate::effects::particles::{SpawnFloatingTextMessage, SpawnParticlesMessage};
use crate::enemies::Enemy;
use crate::game::resources::{PlayerStats, Score, ScreenShake, WaveManager};
use crate::game::state::GameState;
use crate::player::Player;
use crate::systems::audio::{PlaySoundMessage, SoundType};
use crate::systems::progression::XpGem;
use crate::weapons::projectile::Projectile;

pub fn bullet_enemy_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut wave_manager: ResMut<WaveManager>,
    mut screen_shake: ResMut<ScreenShake>,
    mut projectile_query: Query<(Entity, &Transform, &mut Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut particle_writer: MessageWriter<SpawnParticlesMessage>,
    mut text_writer: MessageWriter<SpawnFloatingTextMessage>,
    mut sound_writer: MessageWriter<PlaySoundMessage>,
) {
    for (proj_entity, proj_transform, mut projectile) in &mut projectile_query {
        let proj_pos = proj_transform.translation.truncate();
        let proj_radius = 8.0;

        for (enemy_entity, enemy_transform, mut enemy) in &mut enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            let enemy_radius = enemy.size.x * 0.45;

            let dist = (proj_pos - enemy_pos).length();
            if dist < proj_radius + enemy_radius {
                // Apply Damage
                let dmg = projectile.damage;
                enemy.health -= dmg;
                enemy.hit_flash_timer = 0.08;

                // Spawn Hit Sparks
                particle_writer.write(SpawnParticlesMessage {
                    position: proj_pos,
                    count: 4,
                    color: Color::srgba(1.0, 1.0, 1.0, 0.9),
                    speed_min: 70.0,
                    speed_max: 180.0,
                    lifetime: 0.16,
                    size: 3.5,
                });

                // Floating Damage Number
                text_writer.write(SpawnFloatingTextMessage {
                    position: enemy_pos,
                    text: format!("-{}", dmg as u32),
                    color: Color::srgb(1.0, 0.85, 0.2),
                    font_size: 16.0,
                });

                // Projectile Pierce Handling
                if projectile.pierce_remaining > 0 {
                    projectile.pierce_remaining -= 1;
                }
                if projectile.pierce_remaining == 0 {
                    commands.entity(proj_entity).despawn();
                }

                // Check Enemy Death
                if enemy.health <= 0.0 {
                    // Score with combo
                    score.add(enemy.score_value);
                    wave_manager.total_kills += 1;

                    // Sound
                    sound_writer.write(PlaySoundMessage(SoundType::EnemyDeath));

                    // Massive Death Particle Explosion
                    let p_count = match enemy.enemy_type {
                        crate::enemies::EnemyType::Tank => 28,
                        crate::enemies::EnemyType::Runner => 14,
                        crate::enemies::EnemyType::Chaser => 18,
                    };
                    particle_writer.write(SpawnParticlesMessage {
                        position: enemy_pos,
                        count: p_count,
                        color: enemy.base_color,
                        speed_min: 80.0,
                        speed_max: 260.0,
                        lifetime: 0.45,
                        size: 4.5,
                    });

                    // Tank death screen shake
                    if enemy.enemy_type == crate::enemies::EnemyType::Tank {
                        screen_shake.add(0.35);
                    }

                    // Drop XP Gem
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.0, 1.0, 0.55), // Emerald / Lime neon
                            custom_size: Some(Vec2::new(12.0, 12.0)),
                            ..default()
                        },
                        Transform::from_xyz(enemy_pos.x, enemy_pos.y, 3.0),
                        XpGem {
                            value: enemy.xp_drop,
                            float_timer: 0.0,
                        },
                    ));

                    // Despawn Enemy
                    commands.entity(enemy_entity).despawn();
                }

                // Stop checking other enemies if projectile is depleted
                if projectile.pierce_remaining == 0 {
                    break;
                }
            }
        }
    }
}

pub fn player_enemy_collision(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut screen_shake: ResMut<ScreenShake>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
    mut particle_writer: MessageWriter<SpawnParticlesMessage>,
    mut text_writer: MessageWriter<SpawnFloatingTextMessage>,
    mut sound_writer: MessageWriter<PlaySoundMessage>,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let player_radius = 16.0;

    for (mut enemy_transform, enemy) in &mut enemy_query {
        let enemy_pos = enemy_transform.translation.truncate();
        let enemy_radius = enemy.size.x * 0.4;

        let diff = player_pos - enemy_pos;
        let dist = diff.length();

        if dist < player_radius + enemy_radius {
            if player_stats.take_damage(enemy.damage) {
                // Screen shake on hit
                screen_shake.add(0.45);

                // Sound
                sound_writer.write(PlaySoundMessage(SoundType::PlayerHurt));

                // Floating damage warning
                text_writer.write(SpawnFloatingTextMessage {
                    position: player_pos + Vec2::new(0.0, 20.0),
                    text: format!("-{} HP", enemy.damage as u32),
                    color: Color::srgb(1.0, 0.1, 0.2),
                    font_size: 20.0,
                });

                // Spark burst
                particle_writer.write(SpawnParticlesMessage {
                    position: player_pos,
                    count: 10,
                    color: Color::srgba(1.0, 0.2, 0.2, 0.9),
                    speed_min: 50.0,
                    speed_max: 160.0,
                    lifetime: 0.3,
                    size: 4.0,
                });

                // Knock enemy back slightly
                if dist > 0.001 {
                    let knockback = -diff.normalize() * 35.0;
                    enemy_transform.translation.x += knockback.x;
                    enemy_transform.translation.y += knockback.y;
                }

                // Check Player Death
                if player_stats.health <= 0.0 {
                    // Massive death supernova
                    particle_writer.write(SpawnParticlesMessage {
                        position: player_pos,
                        count: 40,
                        color: Color::srgb(0.0, 0.95, 1.0),
                        speed_min: 90.0,
                        speed_max: 320.0,
                        lifetime: 0.8,
                        size: 6.0,
                    });

                    commands.entity(player_entity).despawn();
                    next_state.set(GameState::GameOver);
                    break;
                }
            }
        }
    }
}
