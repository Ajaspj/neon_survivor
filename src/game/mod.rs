pub mod resources;
pub mod state;

pub use resources::*;
pub use state::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<Score>()
            .init_resource::<SurvivalTimer>()
            .init_resource::<WaveManager>()
            .init_resource::<PlayerStats>()
            .init_resource::<WeaponConfig>()
            .init_resource::<ArenaBounds>()
            .init_resource::<ScreenShake>()
            .init_resource::<GameSettings>()
            .init_resource::<OfferedUpgrades>()
            .add_message::<ResetGameMessage>()
            .add_systems(Startup, (setup_camera, setup_arena))
            .add_systems(
                Update,
                (
                    update_screen_shake,
                    animate_arena_grid,
                    handle_reset_game,
                ),
            );
    }
}

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct ArenaGridTile {
    pub pulse_speed: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct ArenaWall;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        GameCamera,
        Transform::from_xyz(0.0, 0.0, 999.0),
    ));
}

fn setup_arena(
    mut commands: Commands,
    arena: Res<ArenaBounds>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. Cyber Dark Floor Backdrop
    commands.spawn((
        Sprite {
            color: Color::srgb(0.03, 0.045, 0.09), // Deep neon navy
            custom_size: Some(Vec2::new(arena.half_width * 2.6, arena.half_height * 2.6)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    // 2. Playable Arena Floor with glowing tint
    commands.spawn((
        Sprite {
            color: Color::srgb(0.04, 0.06, 0.12),
            custom_size: Some(Vec2::new(arena.half_width * 2.0, arena.half_height * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -5.0),
    ));

    // 3. Procedural Neon Grid Lines
    let step_x = 60.0;
    let step_y = 60.0;
    let grid_color = Color::srgba(0.0, 0.8, 1.0, 0.09);

    // Vertical lines
    let mut x = -arena.half_width;
    while x <= arena.half_width {
        commands.spawn((
            Sprite {
                color: grid_color,
                custom_size: Some(Vec2::new(1.2, arena.half_height * 2.0)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, -4.0),
            ArenaGridTile {
                pulse_speed: 1.5,
                phase: (x / 100.0).sin(),
            },
        ));
        x += step_x;
    }

    // Horizontal lines
    let mut y = -arena.half_height;
    while y <= arena.half_height {
        commands.spawn((
            Sprite {
                color: grid_color,
                custom_size: Some(Vec2::new(arena.half_width * 2.0, 1.2)),
                ..default()
            },
            Transform::from_xyz(0.0, y, -4.0),
            ArenaGridTile {
                pulse_speed: 1.5,
                phase: (y / 100.0).cos(),
            },
        ));
        y += step_y;
    }

    // 4. Glowing Neon Laser Perimeter Walls
    let wall_thickness = 4.0;
    let wall_color = Color::srgb(0.0, 0.95, 1.0); // Bright neon cyan
    let glow_color = Color::srgba(0.0, 0.7, 1.0, 0.25);

    let w = arena.half_width * 2.0;
    let h = arena.half_height * 2.0;

    // Outer glow + inner crisp line for each wall
    // Top
    commands.spawn((
        Sprite {
            color: glow_color,
            custom_size: Some(Vec2::new(w + 20.0, wall_thickness * 4.0)),
            ..default()
        },
        Transform::from_xyz(0.0, arena.half_height, -2.0),
    ));
    commands.spawn((
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(w, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, arena.half_height, -1.0),
        ArenaWall,
    ));

    // Bottom
    commands.spawn((
        Sprite {
            color: glow_color,
            custom_size: Some(Vec2::new(w + 20.0, wall_thickness * 4.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -arena.half_height, -2.0),
    ));
    commands.spawn((
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(w, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, -arena.half_height, -1.0),
        ArenaWall,
    ));

    // Left
    commands.spawn((
        Sprite {
            color: glow_color,
            custom_size: Some(Vec2::new(wall_thickness * 4.0, h + 20.0)),
            ..default()
        },
        Transform::from_xyz(-arena.half_width, 0.0, -2.0),
    ));
    commands.spawn((
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, h)),
            ..default()
        },
        Transform::from_xyz(-arena.half_width, 0.0, -1.0),
        ArenaWall,
    ));

    // Right
    commands.spawn((
        Sprite {
            color: glow_color,
            custom_size: Some(Vec2::new(wall_thickness * 4.0, h + 20.0)),
            ..default()
        },
        Transform::from_xyz(arena.half_width, 0.0, -2.0),
    ));
    commands.spawn((
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, h)),
            ..default()
        },
        Transform::from_xyz(arena.half_width, 0.0, -1.0),
        ArenaWall,
    ));

    // 5. Corner Beacons (high-tech neon nodes)
    let corner_mesh = meshes.add(Circle::new(8.0));
    let corner_mat = materials.add(Color::srgb(1.0, 0.0, 0.8)); // Neon magenta node
    let corners = [
        Vec2::new(-arena.half_width, -arena.half_height),
        Vec2::new(arena.half_width, -arena.half_height),
        Vec2::new(-arena.half_width, arena.half_height),
        Vec2::new(arena.half_width, arena.half_height),
    ];
    for corner in corners {
        commands.spawn((
            Mesh2d(corner_mesh.clone()),
            MeshMaterial2d(corner_mat.clone()),
            Transform::from_xyz(corner.x, corner.y, 0.0),
        ));
    }
}

fn animate_arena_grid(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &ArenaGridTile)>,
) {
    let t = time.elapsed_secs();
    for (mut sprite, tile) in &mut query {
        let pulse = 0.5 + 0.5 * (t * tile.pulse_speed + tile.phase).sin();
        let alpha = 0.05 + 0.06 * pulse;
        sprite.color = Color::srgba(0.0, 0.85, 1.0, alpha);
    }
}

fn update_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    settings: Res<GameSettings>,
    mut query: Query<&mut Transform, With<GameCamera>>,
) {
    shake.tick(time.delta_secs());
    if let Ok(mut transform) = query.single_mut() {
        if settings.screen_shake_enabled {
            let offset = shake.offset();
            transform.translation.x = offset.x;
            transform.translation.y = offset.y;
        } else {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}

pub fn handle_reset_game(
    mut messages: MessageReader<ResetGameMessage>,
    mut score: ResMut<Score>,
    mut timer: ResMut<SurvivalTimer>,
    mut waves: ResMut<WaveManager>,
    mut player_stats: ResMut<PlayerStats>,
    mut weapon_config: ResMut<WeaponConfig>,
) {
    for _ in messages.read() {
        *score = Score::default();
        *timer = SurvivalTimer::default();
        *waves = WaveManager::default();
        *player_stats = PlayerStats::default();
        *weapon_config = WeaponConfig::default();
    }
}
