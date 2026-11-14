use bevy::prelude::*;
use crate::enemies::Enemy;
use crate::game::resources::{PlayerStats, Score, SurvivalTimer, WaveManager};

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudHealthFill;

#[derive(Component)]
pub struct HudHealthText;

#[derive(Component)]
pub struct HudXpFill;

#[derive(Component)]
pub struct HudLevelText;

#[derive(Component)]
pub struct HudScoreText;

#[derive(Component)]
pub struct HudTimerText;

#[derive(Component)]
pub struct HudWaveText;

#[derive(Component)]
pub struct HudHostilesText;

#[derive(Component)]
pub struct HudWaveBanner;

pub fn setup_hud(mut commands: Commands) {
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        HudRoot,
    )).id();

    // 1. TOP STATS BAR
    let top_bar = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(45.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(18.0), Val::Px(6.0)),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.03, 0.05, 0.10, 0.85)),
        BorderColor::all(Color::srgb(0.0, 0.8, 1.0)),
    )).with_children(|parent| {
        // Wave Badge
        parent.spawn((
            Text::new("WAVE 01"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.9)),
            HudWaveText,
        ));

        // Survival Timer
        parent.spawn((
            Text::new("TIME: 00:00"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.9, 0.3)),
            HudTimerText,
        ));

        // Score
        parent.spawn((
            Text::new("SCORE: 0"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.2, 0.7)),
            HudScoreText,
        ));

        // Hostiles Count
        parent.spawn((
            Text::new("HOSTILES: 0"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.5, 0.1)),
            HudHostilesText,
        ));
    }).id();

    // 2. CENTER WAVE ANNOUNCEMENT BANNER
    let banner = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new(""),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.1, 0.6)),
            HudWaveBanner,
        ));
    }).id();

    // 3. BOTTOM BARS (HEALTH & XP)
    let bottom_box = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
    )).with_children(|parent| {
        // Health Bar Row
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(24.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                overflow: Overflow::clip(),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.08, 0.12, 0.85)),
            BorderColor::all(Color::srgb(0.0, 1.0, 0.5)),
        )).with_children(|bar| {
            // Health Fill
            bar.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.95, 0.45)),
                HudHealthFill,
            ));
            // Health Text Overlay
            bar.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    ..default()
                },
            )).with_child((
                Text::new("HP: 100 / 100"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                HudHealthText,
            ));
        });

        // XP Bar Row
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(18.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                overflow: Overflow::clip(),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.04, 0.14, 0.85)),
            BorderColor::all(Color::srgb(0.8, 0.1, 1.0)),
        )).with_children(|bar| {
            // XP Fill
            bar.spawn((
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.85, 0.1, 1.0)),
                HudXpFill,
            ));
            // Level & XP Percentage Text Overlay
            bar.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    ..default()
                },
            )).with_child((
                Text::new("LVL 1 - 0%"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                HudLevelText,
            ));
        });
    }).id();

    commands.entity(root).add_children(&[top_bar, banner, bottom_box]);
}

pub fn update_hud(
    player_stats: Res<PlayerStats>,
    score: Res<Score>,
    timer: Res<SurvivalTimer>,
    waves: Res<WaveManager>,
    enemies_query: Query<&Enemy>,
    mut health_fill_q: Query<&mut Node, (With<HudHealthFill>, Without<HudXpFill>)>,
    mut health_text_q: Query<&mut Text, (With<HudHealthText>, Without<HudScoreText>, Without<HudTimerText>, Without<HudWaveText>, Without<HudHostilesText>, Without<HudLevelText>, Without<HudWaveBanner>)>,
    mut xp_fill_q: Query<&mut Node, (With<HudXpFill>, Without<HudHealthFill>)>,
    mut level_text_q: Query<&mut Text, (With<HudLevelText>, Without<HudHealthText>, Without<HudScoreText>, Without<HudTimerText>, Without<HudWaveText>, Without<HudHostilesText>, Without<HudWaveBanner>)>,
    mut score_text_q: Query<&mut Text, (With<HudScoreText>, Without<HudHealthText>, Without<HudLevelText>, Without<HudTimerText>, Without<HudWaveText>, Without<HudHostilesText>, Without<HudWaveBanner>)>,
    mut timer_text_q: Query<&mut Text, (With<HudTimerText>, Without<HudHealthText>, Without<HudLevelText>, Without<HudScoreText>, Without<HudWaveText>, Without<HudHostilesText>, Without<HudWaveBanner>)>,
    mut wave_text_q: Query<&mut Text, (With<HudWaveText>, Without<HudHealthText>, Without<HudLevelText>, Without<HudScoreText>, Without<HudTimerText>, Without<HudHostilesText>, Without<HudWaveBanner>)>,
    mut hostiles_text_q: Query<&mut Text, (With<HudHostilesText>, Without<HudHealthText>, Without<HudLevelText>, Without<HudScoreText>, Without<HudTimerText>, Without<HudWaveText>, Without<HudWaveBanner>)>,
    mut banner_q: Query<&mut Text, (With<HudWaveBanner>, Without<HudHealthText>, Without<HudLevelText>, Without<HudScoreText>, Without<HudTimerText>, Without<HudWaveText>, Without<HudHostilesText>)>,
) {
    // 1. Health Bar Fill & Text
    let hp_pct = (player_stats.health / player_stats.max_health).clamp(0.0, 1.0) * 100.0;
    if let Ok(mut node) = health_fill_q.single_mut() {
        node.width = Val::Percent(hp_pct);
    }
    if let Ok(mut text) = health_text_q.single_mut() {
        text.0 = format!("HP: {} / {}", player_stats.health as u32, player_stats.max_health as u32);
    }

    // 2. XP Bar Fill & Text
    let xp_pct = (player_stats.current_xp as f32 / player_stats.xp_to_next_level as f32).clamp(0.0, 1.0) * 100.0;
    if let Ok(mut node) = xp_fill_q.single_mut() {
        node.width = Val::Percent(xp_pct);
    }
    if let Ok(mut text) = level_text_q.single_mut() {
        text.0 = format!("LVL {} [{}%]", player_stats.level, xp_pct as u32);
    }

    // 3. Top Stats
    if let Ok(mut text) = score_text_q.single_mut() {
        if score.combo > 1 {
            text.0 = format!("SCORE: {} (x{} COMBO)", score.current, score.combo);
        } else {
            text.0 = format!("SCORE: {}", score.current);
        }
    }
    if let Ok(mut text) = timer_text_q.single_mut() {
        text.0 = format!("TIME: {}", timer.formatted());
    }
    if let Ok(mut text) = wave_text_q.single_mut() {
        text.0 = format!("WAVE {:02}", waves.current_wave);
    }
    if let Ok(mut text) = hostiles_text_q.single_mut() {
        let count = enemies_query.iter().count();
        text.0 = format!("HOSTILES: {}", count);
    }

    // 4. Wave Banner
    if let Ok(mut text) = banner_q.single_mut() {
        if waves.announcement_timer > 0.0 {
            text.0 = format!("// WAVE {:02} ACTIVE //", waves.current_wave);
        } else {
            text.0 = "".into();
        }
    }
}

pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
