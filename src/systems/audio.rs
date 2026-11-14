use crate::game::resources::GameSettings;
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundType {
    Shoot,
    EnemyDeath,
    PlayerHurt,
    LevelUp,
    XpPickup,
}

#[derive(Message, Clone)]
pub struct PlaySoundMessage(pub SoundType);

#[derive(Resource, Default)]
pub struct AudioAssets {
    pub shoot: Option<Handle<AudioSource>>,
    pub enemy_death: Option<Handle<AudioSource>>,
    pub player_hurt: Option<Handle<AudioSource>>,
    pub levelup: Option<Handle<AudioSource>>,
    pub xp_pickup: Option<Handle<AudioSource>>,
    pub bgm: Option<Handle<AudioSource>>,
}

#[derive(Component)]
pub struct BgmMusic;

fn load_audio_safely(
    asset_server: &AssetServer,
    relative_path: &'static str,
) -> Option<Handle<AudioSource>> {
    let full_path = std::path::Path::new("assets").join(relative_path);
    if !full_path.exists() {
        warn!(
            "Audio file '{}' not found on disk. Continuing without this audio effect.",
            full_path.display()
        );
        return None;
    }
    Some(asset_server.load(relative_path))
}

pub fn setup_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading game audio assets from assets/audio/...");
    let assets = AudioAssets {
        shoot: load_audio_safely(&asset_server, "audio/shoot.wav"),
        enemy_death: load_audio_safely(&asset_server, "audio/enemy_death.wav"),
        player_hurt: load_audio_safely(&asset_server, "audio/player_hurt.wav"),
        levelup: load_audio_safely(&asset_server, "audio/levelup.wav"),
        xp_pickup: load_audio_safely(&asset_server, "audio/xp_pickup.wav"),
        bgm: load_audio_safely(&asset_server, "audio/bgm.wav"),
    };
    commands.insert_resource(assets);
}

fn is_audio_ready(
    handle: &Handle<AudioSource>,
    asset_server: &AssetServer,
    audio_sources: &Assets<AudioSource>,
    sound_name: &str,
) -> bool {
    match asset_server.load_state(handle.id()) {
        LoadState::Loaded => audio_sources.get(handle).is_some(),
        LoadState::Failed(err) => {
            warn!(
                "Audio asset '{}' failed to load: {:?}. Audio playback skipped.",
                sound_name, err
            );
            false
        }
        LoadState::Loading => false,
        LoadState::NotLoaded => false,
    }
}

pub fn handle_play_sound_messages(
    mut commands: Commands,
    mut messages: MessageReader<PlaySoundMessage>,
    audio_assets: Option<Res<AudioAssets>>,
    asset_server: Res<AssetServer>,
    audio_sources: Res<Assets<AudioSource>>,
    settings: Res<GameSettings>,
) {
    if !settings.sound_enabled {
        return;
    }
    let Some(assets) = audio_assets else {
        return;
    };

    for msg in messages.read() {
        let (handle_opt, name) = match msg.0 {
            SoundType::Shoot => (&assets.shoot, "shoot"),
            SoundType::EnemyDeath => (&assets.enemy_death, "enemy_death"),
            SoundType::PlayerHurt => (&assets.player_hurt, "player_hurt"),
            SoundType::LevelUp => (&assets.levelup, "levelup"),
            SoundType::XpPickup => (&assets.xp_pickup, "xp_pickup"),
        };

        if let Some(handle) = handle_opt {
            if is_audio_ready(handle, &asset_server, &audio_sources, name) {
                commands.spawn((
                    AudioPlayer::new(handle.clone()),
                    PlaybackSettings::DESPAWN
                        .with_volume(bevy::audio::Volume::Linear(settings.master_volume)),
                ));
            }
        }
    }
}

pub fn manage_bgm(
    mut commands: Commands,
    audio_assets: Option<Res<AudioAssets>>,
    asset_server: Res<AssetServer>,
    audio_sources: Res<Assets<AudioSource>>,
    settings: Res<GameSettings>,
    existing_bgm: Query<Entity, With<BgmMusic>>,
) {
    if !settings.music_enabled || !existing_bgm.is_empty() {
        return;
    }
    let Some(assets) = audio_assets else {
        return;
    };

    if let Some(ref handle) = assets.bgm {
        if is_audio_ready(handle, &asset_server, &audio_sources, "bgm") {
            commands.spawn((
                AudioPlayer::new(handle.clone()),
                PlaybackSettings::LOOP
                    .with_volume(bevy::audio::Volume::Linear(settings.master_volume * 0.4)),
                BgmMusic,
            ));
        }
    }
}

pub fn stop_bgm(mut commands: Commands, bgm_query: Query<Entity, With<BgmMusic>>) {
    for entity in &bgm_query {
        commands.entity(entity).despawn();
    }
}
