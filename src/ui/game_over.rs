use bevy::prelude::*;
use crate::game::{PlayerStats, ResetGameMessage, Score, SurvivalTimer, WaveManager};
use crate::game::state::GameState;

#[derive(Component)]
pub struct GameOverRoot;

#[derive(Component)]
pub enum GameOverButtonAction {
    Restart,
    MainMenu,
}

pub fn setup_game_over_menu(
    mut commands: Commands,
    score: Res<Score>,
    timer: Res<SurvivalTimer>,
    waves: Res<WaveManager>,
    player_stats: Res<PlayerStats>,
) {
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(22.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.04, 0.01, 0.02, 0.88)),
        GameOverRoot,
    )).id();

    // 1. Header Box
    let header = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new("NEURAL LINK SEVERED"),
            TextFont {
                font_size: 46.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.1, 0.2)), // Crimson Neon
        ));

        parent.spawn((
            Text::new("// OPERATOR VESSEL DESTROYED //"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.4, 0.5)),
        ));
    }).id();

    // 2. Stats Panel
    let stats_panel = commands.spawn((
        Node {
            width: Val::Px(420.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(24.0)),
            row_gap: Val::Px(12.0),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.02, 0.04, 0.95)),
        BorderColor::all(Color::srgb(1.0, 0.15, 0.3)),
    )).with_children(|panel| {
        spawn_stat_row(panel, "FINAL SCORE:", &format!("{}", score.current), Color::srgb(1.0, 0.9, 0.2));
        spawn_stat_row(panel, "HIGH SCORE:", &format!("{}", score.high_score), Color::srgb(1.0, 0.6, 0.1));
        spawn_stat_row(panel, "SURVIVAL TIME:", &timer.formatted(), Color::srgb(0.0, 1.0, 0.9));
        spawn_stat_row(panel, "WAVES SURVIVED:", &format!("{}", waves.current_wave), Color::srgb(1.0, 0.2, 0.8));
        spawn_stat_row(panel, "ENEMIES DEFEATED:", &format!("{}", waves.total_kills), Color::srgb(1.0, 0.3, 0.3));
        spawn_stat_row(panel, "HIGHEST LEVEL:", &format!("LVL {}", player_stats.level), Color::srgb(0.2, 1.0, 0.6));
    }).id();

    // 3. Action Buttons Row
    let buttons = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(18.0),
            ..default()
        },
    )).with_children(|parent| {
        spawn_game_over_button(parent, "REDEPLOY MISSION", GameOverButtonAction::Restart, Color::srgb(0.0, 0.95, 1.0));
        spawn_game_over_button(parent, "RETURN TO BASE", GameOverButtonAction::MainMenu, Color::srgb(0.8, 0.3, 1.0));
    }).id();

    commands.entity(root).add_children(&[header, stats_panel, buttons]);
}

fn spawn_stat_row(parent: &mut ChildSpawnerCommands, label: &str, value: &str, val_color: Color) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|row| {
        row.spawn((
            Text::new(label),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(Color::srgb(0.75, 0.85, 0.95)),
        ));
        row.spawn((
            Text::new(value),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(val_color),
        ));
    });
}

fn spawn_game_over_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: GameOverButtonAction,
    border_color: Color,
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(46.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.8)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.12, 0.04, 0.08, 0.9)),
        BorderColor::all(border_color),
        action,
    )).with_child((
        Text::new(label),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
    ));
}

pub fn handle_game_over_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut reset_writer: MessageWriter<ResetGameMessage>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GameOverButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgba(0.8, 0.1, 0.2, 0.95));
                match action {
                    GameOverButtonAction::Restart => {
                        reset_writer.write(ResetGameMessage);
                        next_state.set(GameState::Playing);
                    }
                    GameOverButtonAction::MainMenu => {
                        reset_writer.write(ResetGameMessage);
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.25, 0.06, 0.12, 0.95));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.12, 0.04, 0.08, 0.9));
            }
        }
    }
}

pub fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
