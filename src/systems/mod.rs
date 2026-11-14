pub mod audio;
pub mod collision;
pub mod progression;

use bevy::prelude::*;
use audio::*;
use collision::*;
use progression::*;
use crate::game::state::GameState;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlaySoundMessage>()
            .add_systems(Startup, setup_audio_assets)
            .add_systems(
                Update,
                (
                    handle_play_sound_messages,
                ),
            )
            .add_systems(
                Update,
                (
                    bullet_enemy_collision,
                    player_enemy_collision,
                    update_xp_gems,
                    update_wave_and_timers,
                    manage_bgm,
                ).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::MainMenu), stop_bgm)
            .add_systems(OnEnter(GameState::GameOver), stop_bgm)
            .add_systems(OnEnter(GameState::GameOver), cleanup_xp_gems)
            .add_systems(OnEnter(GameState::MainMenu), cleanup_xp_gems);
    }
}
