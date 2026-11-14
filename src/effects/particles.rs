use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub drag: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub start_size: Vec2,
    pub end_size: Vec2,
    pub start_color: Color,
    pub end_color: Color,
}

#[derive(Component)]
pub struct FloatingText {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub start_color: Color,
}

#[derive(Message, Clone)]
pub struct SpawnParticlesMessage {
    pub position: Vec2,
    pub count: usize,
    pub color: Color,
    pub speed_min: f32,
    pub speed_max: f32,
    pub lifetime: f32,
    pub size: f32,
}

#[derive(Message, Clone)]
pub struct SpawnFloatingTextMessage {
    pub position: Vec2,
    pub text: String,
    pub color: Color,
    pub font_size: f32,
}

pub fn handle_particle_messages(
    mut commands: Commands,
    mut messages: MessageReader<SpawnParticlesMessage>,
    active_particles: Query<&Particle>,
) {
    let current_count = active_particles.iter().count();
    let max_allowed = 350; // Cap to keep low memory & high FPS on integrated GPUs
    if current_count >= max_allowed {
        return;
    }

    let mut rng = rand::rng();
    let available_slots = max_allowed - current_count;

    for msg in messages.read() {
        let spawn_count = msg.count.min(available_slots);
        for _ in 0..spawn_count {
            let angle = rng.random_range(0.0..(2.0 * PI));
            let speed = rng.random_range(msg.speed_min..msg.speed_max);
            let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);
            let lifetime = msg.lifetime * rng.random_range(0.8..1.2);
            let p_size = msg.size * rng.random_range(0.7..1.3);

            let end_color = match msg.color {
                Color::Srgba(c) => Color::srgba(c.red, c.green, c.blue, 0.0),
                _ => Color::srgba(0.0, 1.0, 1.0, 0.0),
            };

            commands.spawn((
                Sprite {
                    color: msg.color,
                    custom_size: Some(Vec2::splat(p_size)),
                    ..default()
                },
                Transform::from_xyz(msg.position.x, msg.position.y, 10.0),
                Particle {
                    velocity: vel,
                    drag: 3.5,
                    lifetime,
                    max_lifetime: lifetime,
                    start_size: Vec2::splat(p_size),
                    end_size: Vec2::splat(p_size * 0.2),
                    start_color: msg.color,
                    end_color,
                },
            ));
        }
    }
}

pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut sprite, mut particle) in &mut query {
        particle.lifetime -= dt;
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Apply movement with drag
        let drag = particle.drag;
        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;
        particle.velocity *= (1.0 - drag * dt).max(0.0);

        // Progress 0.0 -> 1.0
        let t = 1.0 - (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);

        // Interpolate size
        let size = particle.start_size.lerp(particle.end_size, t);
        sprite.custom_size = Some(size);

        // Interpolate alpha
        if let (Color::Srgba(start), Color::Srgba(end)) = (particle.start_color, particle.end_color) {
            let r = start.red + (end.red - start.red) * t;
            let g = start.green + (end.green - start.green) * t;
            let b = start.blue + (end.blue - start.blue) * t;
            let a = start.alpha + (end.alpha - start.alpha) * t;
            sprite.color = Color::srgba(r, g, b, a);
        }
    }
}

pub fn handle_floating_text_messages(
    mut commands: Commands,
    mut messages: MessageReader<SpawnFloatingTextMessage>,
) {
    let mut rng = rand::rng();
    for msg in messages.read() {
        let offset_x = rng.random_range(-10.0..10.0);
        let offset_y = rng.random_range(-5.0..15.0);
        let vel_x = rng.random_range(-15.0..15.0);
        let vel_y = rng.random_range(45.0..75.0);

        commands.spawn((
            Text2d::new(&msg.text),
            TextFont {
                font_size: msg.font_size,
                ..default()
            },
            TextColor(msg.color),
            Transform::from_xyz(
                msg.position.x + offset_x,
                msg.position.y + offset_y,
                20.0,
            ),
            FloatingText {
                velocity: Vec2::new(vel_x, vel_y),
                lifetime: 0.8,
                max_lifetime: 0.8,
                start_color: msg.color,
            },
        ));
    }
}

pub fn update_floating_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TextColor, &mut FloatingText)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut text_color, mut float_text) in &mut query {
        float_text.lifetime -= dt;
        if float_text.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        transform.translation.x += float_text.velocity.x * dt;
        transform.translation.y += float_text.velocity.y * dt;
        float_text.velocity.y *= (1.0 - 1.2 * dt).max(0.0);

        let t = 1.0 - (float_text.lifetime / float_text.max_lifetime).clamp(0.0, 1.0);
        if let Color::Srgba(c) = float_text.start_color {
            let alpha = c.alpha * (1.0 - t * t);
            text_color.0 = Color::srgba(c.red, c.green, c.blue, alpha);
        }
    }
}
