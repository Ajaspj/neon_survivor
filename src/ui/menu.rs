use crate::game::state::GameState;
use crate::game::{GameSettings, ResetGameMessage};
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    ToggleSound,
    ToggleScreenShake,
    Exit,
}

pub fn setup_main_menu(mut commands: Commands, settings: Res<GameSettings>) {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(18.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.03, 0.07, 0.75)),
            MainMenuRoot,
        ))
        .id();

    // 1. Cyber Header Title Box
    let title_box = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(35.0)),
            ..default()
        },))
        .with_children(|parent| {
            // Glowing Main Title
            parent.spawn((
                Text::new("NEON SURVIVOR"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.95, 1.0)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("// HIGH-OCTANE CYBERPUNK ARCADE SURVIVAL //"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.1, 0.7)),
            ));

            // Instructions
            parent.spawn((
                Text::new(
                    "[WASD / ARROWS] Move   |   [MOUSE / CLICK] Aim & Fire   |   [ESC] Pause",
                ),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgba(0.8, 0.9, 1.0, 0.7)),
            ));
        })
        .id();

    // 2. Interactive Buttons Box
    let sound_label = if settings.sound_enabled {
        "AUDIO: ENABLED"
    } else {
        "AUDIO: MUTED"
    };
    let shake_label = if settings.screen_shake_enabled {
        "SCREEN SHAKE: ON"
    } else {
        "SCREEN SHAKE: OFF"
    };

    let buttons_box = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(14.0),
            ..default()
        },))
        .with_children(|parent| {
            // Play Button
            spawn_menu_button(
                parent,
                "ENTER ARENA",
                MenuButtonAction::Play,
                Color::srgb(0.0, 0.95, 1.0),
            );

            // Sound Toggle Button
            spawn_menu_button(
                parent,
                sound_label,
                MenuButtonAction::ToggleSound,
                Color::srgb(0.8, 0.4, 1.0),
            );

            // Screen Shake Toggle Button
            spawn_menu_button(
                parent,
                shake_label,
                MenuButtonAction::ToggleScreenShake,
                Color::srgb(0.8, 0.4, 1.0),
            );

            // Exit Button
            spawn_menu_button(
                parent,
                "DISCONNECT",
                MenuButtonAction::Exit,
                Color::srgb(1.0, 0.2, 0.4),
            );
        })
        .id();

    commands
        .entity(root)
        .add_children(&[title_box, buttons_box]);
}

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: MenuButtonAction,
    border_color: Color,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(280.0),
                height: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.08, 0.16, 0.92)),
            BorderColor::all(border_color),
            action,
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.98, 1.0)),
        ));
}

pub fn handle_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<GameSettings>,
    mut reset_writer: MessageWriter<ResetGameMessage>,
    mut app_exit: MessageWriter<AppExit>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &MenuButtonAction,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut bg_color, action, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgba(0.0, 0.7, 0.9, 0.95));
                match action {
                    MenuButtonAction::Play => {
                        reset_writer.write(ResetGameMessage);
                        next_state.set(GameState::Playing);
                    }
                    MenuButtonAction::ToggleSound => {
                        settings.sound_enabled = !settings.sound_enabled;
                        settings.music_enabled = settings.sound_enabled;
                        if let Some(&child) = children.first() {
                            if let Ok(mut text) = text_query.get_mut(child) {
                                text.0 = if settings.sound_enabled {
                                    "AUDIO: ENABLED".into()
                                } else {
                                    "AUDIO: MUTED".into()
                                };
                            }
                        }
                    }
                    MenuButtonAction::ToggleScreenShake => {
                        settings.screen_shake_enabled = !settings.screen_shake_enabled;
                        if let Some(&child) = children.first() {
                            if let Ok(mut text) = text_query.get_mut(child) {
                                text.0 = if settings.screen_shake_enabled {
                                    "SCREEN SHAKE: ON".into()
                                } else {
                                    "SCREEN SHAKE: OFF".into()
                                };
                            }
                        }
                    }
                    MenuButtonAction::Exit => {
                        app_exit.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.12, 0.22, 0.38, 0.95));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.05, 0.08, 0.16, 0.92));
            }
        }
    }
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
