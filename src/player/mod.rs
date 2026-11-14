pub mod combat;
pub mod movement;

use bevy::prelude::*;
use combat::*;
use movement::*;
use crate::game::state::GameState;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub facing_angle: f32,
    pub engine_trail_timer: f32,
}

#[derive(Component)]
pub struct PlayerAura;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    player_aiming,
                    player_combat,
                    update_player_invincibility,
                ).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::GameOver), despawn_player)
            .add_systems(OnEnter(GameState::MainMenu), despawn_player);
    }
}

pub fn setup_player(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    // If player already exists, do not duplicate
    if !query.is_empty() {
        return;
    }

    // Outer Neon Glow Aura
    let aura_entity = commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.8, 1.0, 0.22),
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.1),
        PlayerAura,
    )).id();

    // Central Cockpit Core
    let cockpit_entity = commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0), // Bright White Core
            custom_size: Some(Vec2::new(14.0, 14.0)),
            ..default()
        },
        Transform::from_xyz(4.0, 0.0, 0.1),
    )).id();

    // Main Player Ship Entity
    commands.spawn((
        Player {
            velocity: Vec2::ZERO,
            facing_angle: 0.0,
            engine_trail_timer: 0.0,
        },
        Sprite {
            color: Color::srgb(0.0, 0.95, 1.0), // Electric Cyan
            custom_size: Some(Vec2::new(32.0, 26.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    )).add_children(&[aura_entity, cockpit_entity]);
}

pub fn despawn_player(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
