pub mod game_over;
pub mod hud;
pub mod level_up;
pub mod menu;
pub mod pause;

use bevy::prelude::*;
use game_over::*;
use hud::*;
use level_up::*;
use menu::*;
use pause::*;
use crate::game::state::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Main Menu State
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, handle_menu_buttons.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);

        // In-Game HUD State
        app.add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(
                Update,
                (
                    update_hud,
                    toggle_pause,
                ).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), despawn_hud);

        // Pause Menu State
        app.add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(
                Update,
                (
                    handle_pause_buttons,
                    toggle_pause,
                ).run_if(in_state(GameState::Paused)),
            )
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu);

        // Level-Up Upgrade Selection State
        app.add_systems(OnEnter(GameState::LevelUp), setup_level_up_menu)
            .add_systems(Update, handle_level_up_buttons.run_if(in_state(GameState::LevelUp)))
            .add_systems(OnExit(GameState::LevelUp), despawn_level_up_menu);

        // Game Over State
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over_menu)
            .add_systems(Update, handle_game_over_buttons.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_menu);
    }
}

