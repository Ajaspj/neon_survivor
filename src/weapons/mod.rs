pub mod projectile;

use bevy::prelude::*;
use projectile::*;
use crate::game::state::GameState;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnProjectileMessage>()
            .add_systems(
                Update,
                (
                    handle_spawn_projectile_messages,
                    update_projectiles,
                ).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::GameOver), cleanup_projectiles)
            .add_systems(OnEnter(GameState::MainMenu), cleanup_projectiles);
    }
}

