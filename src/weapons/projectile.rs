use crate::effects::particles::SpawnParticlesMessage;
use crate::game::resources::ArenaBounds;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub lifetime: f32,
    pub pierce_remaining: u32,
    pub trail_timer: f32,
}

#[derive(Message, Clone)]
pub struct SpawnProjectileMessage {
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub damage: f32,
    pub pierce: u32,
}

pub fn handle_spawn_projectile_messages(
    mut commands: Commands,
    mut messages: MessageReader<SpawnProjectileMessage>,
    mut particle_writer: MessageWriter<SpawnParticlesMessage>,
) {
    for msg in messages.read() {
        let angle = msg.direction.y.atan2(msg.direction.x);
        let rotation = Quat::from_rotation_z(angle);

        // Muzzle flash particle burst at projectile spawn
        particle_writer.write(SpawnParticlesMessage {
            position: msg.position,
            count: 3,
            color: Color::srgba(0.0, 1.0, 1.0, 0.9),
            speed_min: 60.0,
            speed_max: 140.0,
            lifetime: 0.15,
            size: 3.5,
        });

        // Glowing outer laser shell
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.8, 1.0, 0.35),
                custom_size: Some(Vec2::new(26.0, 10.0)),
                ..default()
            },
            Transform::from_translation(msg.position.extend(4.0)).with_rotation(rotation),
        ));

        // Core high-intensity laser bolt
        commands.spawn((
            Sprite {
                color: Color::srgb(0.9, 1.0, 1.0), // Intense white-cyan core
                custom_size: Some(Vec2::new(20.0, 5.0)),
                ..default()
            },
            Transform::from_translation(msg.position.extend(5.0)).with_rotation(rotation),
            Projectile {
                damage: msg.damage,
                speed: msg.speed,
                direction: msg.direction,
                lifetime: 1.6,
                pierce_remaining: msg.pierce,
                trail_timer: 0.0,
            },
        ));
    }
}

pub fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    arena: Res<ArenaBounds>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
    mut particle_writer: MessageWriter<SpawnParticlesMessage>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut proj) in &mut query {
        proj.lifetime -= dt;
        if proj.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        let pos2d = transform.translation.truncate();
        if !arena.is_inside(pos2d, 50.0) {
            commands.entity(entity).despawn();
            continue;
        }

        // Move forward
        transform.translation.x += proj.direction.x * proj.speed * dt;
        transform.translation.y += proj.direction.y * proj.speed * dt;

        // Periodic trailing particle for glowing streak effect
        proj.trail_timer += dt;
        if proj.trail_timer >= 0.04 {
            proj.trail_timer = 0.0;
            particle_writer.write(SpawnParticlesMessage {
                position: transform.translation.truncate() - proj.direction * 10.0,
                count: 1,
                color: Color::srgba(0.0, 0.85, 1.0, 0.6),
                speed_min: 10.0,
                speed_max: 30.0,
                lifetime: 0.18,
                size: 3.0,
            });
        }
    }
}

pub fn cleanup_projectiles(mut commands: Commands, query: Query<Entity, With<Projectile>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
