use bevy::prelude::*;
use crate::game::resources::{OfferedUpgrades, PlayerStats, UpgradeType, WeaponConfig};
use crate::game::state::GameState;

#[derive(Component)]
pub struct LevelUpRoot;

#[derive(Component)]
pub struct UpgradeButtonAction(pub UpgradeType);

pub fn setup_level_up_menu(
    mut commands: Commands,
    offered_upgrades: Res<OfferedUpgrades>,
) {
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(24.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.04, 0.08, 0.88)),
        LevelUpRoot,
    )).id();

    // 1. Title Banner
    let header = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new("SYSTEM LEVEL UP"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.85)),
        ));

        parent.spawn((
            Text::new("// CHOOSE ONE CYBERNETIC AUGMENTATION //"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.15, 0.7)),
        ));
    }).id();

    // 2. Cards Row
    let cards_container = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            ..default()
        },
    )).with_children(|parent| {
        for card in &offered_upgrades.cards {
            parent.spawn((
                Node {
                    width: Val::Px(240.0),
                    height: Val::Px(270.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(18.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.08, 0.16, 0.94)),
                BorderColor::all(Color::srgb(0.0, 0.85, 1.0)),
            )).with_children(|card_box| {
                // Top Category
                card_box.spawn((
                    Text::new(card.category),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.2, 1.0)),
                ));

                // Title
                card_box.spawn((
                    Text::new(card.title),
                    TextFont {
                        font_size: 19.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.95, 1.0)),
                ));

                // Description
                card_box.spawn((
                    Text::new(card.description),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.92, 1.0)),
                ));

                // Install Button
                card_box.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.5)),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.6, 0.8, 0.85)),
                    BorderColor::all(Color::srgb(0.0, 1.0, 1.0)),
                    UpgradeButtonAction(card.upgrade_type),
                )).with_child((
                    Text::new("INSTALL"),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
            });
        }
    }).id();

    commands.entity(root).add_children(&[header, cards_container]);
}

pub fn handle_level_up_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut player_stats: ResMut<PlayerStats>,
    mut weapon_config: ResMut<WeaponConfig>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UpgradeButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgb(0.0, 0.9, 1.0));
                // Apply chosen upgrade
                action.0.apply(&mut player_stats, &mut weapon_config);
                // Resume game
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.0, 0.8, 1.0, 0.95));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.0, 0.6, 0.8, 0.85));
            }
        }
    }
}

pub fn despawn_level_up_menu(mut commands: Commands, query: Query<Entity, With<LevelUpRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
