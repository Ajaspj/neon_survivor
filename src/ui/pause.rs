use crate::game::state::GameState;
use crate::game::ResetGameMessage;
use bevy::prelude::*;

#[derive(Component)]
pub struct PauseRoot;

#[derive(Component)]
pub enum PauseButtonAction {
    Resume,
    Restart,
    MainMenu,
}

pub fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn setup_pause_menu(mut commands: Commands) {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.04, 0.08, 0.82)),
            PauseRoot,
        ))
        .id();

    // Box container
    let container = commands
        .spawn((
            Node {
                width: Val::Px(380.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(28.0)),
                row_gap: Val::Px(16.0),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.07, 0.14, 0.95)),
            BorderColor::all(Color::srgb(0.0, 0.9, 1.0)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SYSTEM PAUSED"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.95, 1.0)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("// ARENA SIMULATION SUSPENDED //"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.7)),
            ));

            // Buttons
            spawn_pause_button(
                parent,
                "RESUME OPERATION",
                PauseButtonAction::Resume,
                Color::srgb(0.0, 0.95, 1.0),
            );
            spawn_pause_button(
                parent,
                "RESTART MISSION",
                PauseButtonAction::Restart,
                Color::srgb(1.0, 0.7, 0.1),
            );
            spawn_pause_button(
                parent,
                "ABORT TO MAIN MENU",
                PauseButtonAction::MainMenu,
                Color::srgb(1.0, 0.2, 0.4),
            );
        })
        .id();

    commands.entity(root).add_child(container);
}

fn spawn_pause_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: PauseButtonAction,
    border_color: Color,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(280.0),
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.8)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.10, 0.20, 0.9)),
            BorderColor::all(border_color),
            action,
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.98, 1.0)),
        ));
}

pub fn handle_pause_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut reset_writer: MessageWriter<ResetGameMessage>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &PauseButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgba(0.0, 0.7, 0.9, 0.95));
                match action {
                    PauseButtonAction::Resume => next_state.set(GameState::Playing),
                    PauseButtonAction::Restart => {
                        reset_writer.write(ResetGameMessage);
                        next_state.set(GameState::Playing);
                    }
                    PauseButtonAction::MainMenu => next_state.set(GameState::MainMenu),
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.12, 0.22, 0.38, 0.95));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.06, 0.10, 0.20, 0.9));
            }
        }
    }
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
