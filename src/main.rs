use bevy::prelude::*;

mod effects;
mod enemies;
mod game;
mod player;
mod systems;
mod ui;
mod weapons;

use effects::EffectsPlugin;
use enemies::EnemiesPlugin;
use game::GamePlugin;
use player::PlayerPlugin;
use systems::SystemsPlugin;
use ui::UiPlugin;
use weapons::WeaponsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "NEON SURVIVOR".into(),
                resolution: (1280, 720).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // Core game systems and state
        .add_plugins(GamePlugin)
        // Visual effects, particles, and floating text
        .add_plugins(EffectsPlugin)
        // Weapons and projectile simulation
        .add_plugins(WeaponsPlugin)
        // Player controls, movement, and combat
        .add_plugins(PlayerPlugin)
        // Enemy types, wave spawning, and AI
        .add_plugins(EnemiesPlugin)
        // Audio, collision detection, and progression
        .add_plugins(SystemsPlugin)
        // User interface, HUD, and menus
        .add_plugins(UiPlugin)
        .run();
}
