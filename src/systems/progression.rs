use bevy::prelude::*;
use rand::seq::SliceRandom;
use crate::effects::particles::{SpawnFloatingTextMessage, SpawnParticlesMessage};
use crate::game::resources::{OfferedUpgrades, PlayerStats, Score, ScreenShake, SurvivalTimer, UpgradeType, WaveManager};
use crate::game::state::GameState;
use crate::player::Player;
use crate::systems::audio::{PlaySoundMessage, SoundType};

#[derive(Component)]
pub struct XpGem {
    pub value: u32,
    pub float_timer: f32,
}

pub fn update_xp_gems(
    mut commands: Commands,
    time: Res<Time>,
    mut player_stats: ResMut<PlayerStats>,
    mut offered_upgrades: ResMut<OfferedUpgrades>,
    mut screen_shake: ResMut<ScreenShake>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<&Transform, With<Player>>,
    mut gem_query: Query<(Entity, &mut Transform, &mut XpGem), Without<Player>>,
    mut particle_writer: MessageWriter<SpawnParticlesMessage>,
    mut text_writer: MessageWriter<SpawnFloatingTextMessage>,
    mut sound_writer: MessageWriter<PlaySoundMessage>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let dt = time.delta_secs();
    let magnet_r = player_stats.magnet_radius;

    for (entity, mut gem_transform, mut gem) in &mut gem_query {
        gem.float_timer += dt;
        let pos = gem_transform.translation.truncate();
        let to_player = player_pos - pos;
        let dist = to_player.length();

        // Crystal spin & float animation
        let rot_speed = 3.2;
        gem_transform.rotation = Quat::from_rotation_z(gem.float_timer * rot_speed);

        // Magnetic Attraction
        if dist < magnet_r && dist > 0.001 {
            // Speed accelerates as it gets closer
            let pull_speed = 420.0 * (1.0 + (magnet_r - dist) / magnet_r * 2.0);
            let dir = to_player / dist;
            gem_transform.translation.x += dir.x * pull_speed * dt;
            gem_transform.translation.y += dir.y * pull_speed * dt;
        }

        // Collection Check
        if dist < 22.0 {
            let leveled_up = player_stats.add_xp(gem.value);

            // Pickup sound & sparkle
            sound_writer.write(PlaySoundMessage(SoundType::XpPickup));
            particle_writer.write(SpawnParticlesMessage {
                position: pos,
                count: 4,
                color: Color::srgb(0.0, 1.0, 0.6),
                speed_min: 30.0,
                speed_max: 90.0,
                lifetime: 0.2,
                size: 3.0,
            });

            // Check Level-Up trigger
            if leveled_up {
                sound_writer.write(PlaySoundMessage(SoundType::LevelUp));
                screen_shake.add(0.4);

                // Nova sparkle around player
                particle_writer.write(SpawnParticlesMessage {
                    position: player_pos,
                    count: 30,
                    color: Color::srgb(1.0, 0.0, 0.8), // Hot magenta
                    speed_min: 100.0,
                    speed_max: 300.0,
                    lifetime: 0.6,
                    size: 5.0,
                });

                text_writer.write(SpawnFloatingTextMessage {
                    position: player_pos + Vec2::new(0.0, 30.0),
                    text: format!("LEVEL {}!", player_stats.level),
                    color: Color::srgb(0.0, 1.0, 1.0),
                    font_size: 26.0,
                });

                // Pick 3 random distinct upgrades
                let mut rng = rand::rng();
                let mut all_upgrades = UpgradeType::all().to_vec();
                all_upgrades.shuffle(&mut rng);
                offered_upgrades.cards = all_upgrades
                    .into_iter()
                    .take(3)
                    .map(|u| u.card())
                    .collect();

                // Open level up screen
                next_state.set(GameState::LevelUp);
            }

            commands.entity(entity).despawn();
        }
    }
}

pub fn update_wave_and_timers(
    time: Res<Time>,
    mut survival_timer: ResMut<SurvivalTimer>,
    mut wave_manager: ResMut<WaveManager>,
    mut score: ResMut<Score>,
    mut text_writer: MessageWriter<SpawnFloatingTextMessage>,
) {
    let dt = time.delta_secs();
    survival_timer.seconds += dt;
    score.tick(dt);

    if wave_manager.announcement_timer > 0.0 {
        wave_manager.announcement_timer -= dt;
    }

    wave_manager.wave_timer += dt;
    if wave_manager.wave_timer >= wave_manager.wave_duration {
        wave_manager.advance_wave();

        // Wave Announcement banner
        text_writer.write(SpawnFloatingTextMessage {
            position: Vec2::new(0.0, 100.0),
            text: format!("--- WAVE {} INCOMING ---", wave_manager.current_wave),
            color: Color::srgb(1.0, 0.2, 0.6),
            font_size: 28.0,
        });
    }
}

pub fn cleanup_xp_gems(
    mut commands: Commands,
    query: Query<Entity, With<XpGem>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
