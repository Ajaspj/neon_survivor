use bevy::prelude::*;
use crate::game::resources::{PlayerStats, WeaponConfig};
use crate::player::Player;
use crate::systems::audio::{PlaySoundMessage, SoundType};
use crate::weapons::projectile::SpawnProjectileMessage;

pub fn player_combat(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut weapon_config: ResMut<WeaponConfig>,
    mut projectile_writer: MessageWriter<SpawnProjectileMessage>,
    mut sound_writer: MessageWriter<PlaySoundMessage>,
    mut query: Query<(&Transform, &mut Player)>,
) {
    let Ok((transform, mut player)) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    if weapon_config.fire_timer > 0.0 {
        weapon_config.fire_timer -= dt;
    }

    let is_firing = mouse.pressed(MouseButton::Left) || keyboard.pressed(KeyCode::Space);

    if is_firing && weapon_config.fire_timer <= 0.0 {
        weapon_config.fire_timer = 1.0 / weapon_config.fire_rate;

        let base_angle = player.facing_angle;
        let count = weapon_config.projectile_count;
        let spread = weapon_config.projectile_spread;
        let ship_pos = transform.translation.truncate();
        let forward = Vec2::new(base_angle.cos(), base_angle.sin());
        let spawn_pos = ship_pos + forward * 22.0;

        for i in 0..count {
            let angle_offset = if count > 1 {
                -spread * 0.5 + (spread / (count - 1) as f32) * i as f32
            } else {
                0.0
            };

            let proj_angle = base_angle + angle_offset;
            let dir = Vec2::new(proj_angle.cos(), proj_angle.sin());

            projectile_writer.write(SpawnProjectileMessage {
                position: spawn_pos,
                direction: dir,
                speed: weapon_config.projectile_speed,
                damage: weapon_config.projectile_damage,
                pierce: weapon_config.projectile_pierce,
            });
        }

        // Slight kickback recoil
        player.velocity -= forward * 45.0;

        // Sound trigger
        sound_writer.write(PlaySoundMessage(SoundType::Shoot));
    }
}

pub fn update_player_invincibility(
    time: Res<Time>,
    mut player_stats: ResMut<PlayerStats>,
    mut query: Query<&mut Sprite, With<Player>>,
) {
    let dt = time.delta_secs();
    if player_stats.invincibility_timer > 0.0 {
        player_stats.invincibility_timer = (player_stats.invincibility_timer - dt).max(0.0);

        if let Ok(mut sprite) = query.single_mut() {
            // Rapid blink effect during i-frames
            let blink = ((player_stats.invincibility_timer * 20.0).sin() > 0.0) as i32;
            let alpha = if blink == 1 { 0.3 } else { 0.95 };
            sprite.color = Color::srgba(0.0, 0.95, 1.0, alpha);
        }
    } else if let Ok(mut sprite) = query.single_mut() {
        sprite.color = Color::srgb(0.0, 0.95, 1.0);
    }
}
