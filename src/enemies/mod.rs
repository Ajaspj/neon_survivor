pub mod ai;
pub mod spawning;

use bevy::prelude::*;
use ai::*;
use spawning::*;
use crate::game::state::GameState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyType {
    Chaser,
    Runner,
    Tank,
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: f32,
    #[allow(dead_code)]
    pub max_health: f32,
    pub speed: f32,
    pub damage: f32,
    pub score_value: u32,
    pub hit_flash_timer: f32,
    pub base_color: Color,
    pub size: Vec2,
    pub xp_drop: u32,
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies,
                enemy_ai,
                update_enemy_hit_flash,
            ).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_enemies)
        .add_systems(OnEnter(GameState::MainMenu), despawn_enemies);
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    query: Query<Entity, With<Enemy>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
