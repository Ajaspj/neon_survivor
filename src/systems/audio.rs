use crate::game::resources::GameSettings;
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

#[derive(Resource)]
pub struct AudioAssets {
    pub shoot: Handle<AudioSource>,
    pub enemy_death: Handle<AudioSource>,
    pub player_hurt: Handle<AudioSource>,
    pub levelup: Handle<AudioSource>,
    pub xp_pickup: Handle<AudioSource>,
    pub bgm: Handle<AudioSource>,
}

#[derive(Component)]
pub struct BgmMusic;

pub fn setup_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = AudioAssets {
        shoot: asset_server.load("audio/shoot.wav"),
        enemy_death: asset_server.load("audio/enemy_death.wav"),
        player_hurt: asset_server.load("audio/player_hurt.wav"),
        levelup: asset_server.load("audio/levelup.wav"),
        xp_pickup: asset_server.load("audio/xp_pickup.wav"),
        bgm: asset_server.load("audio/bgm.wav"),
    };
    commands.insert_resource(assets);
}

pub fn handle_play_sound_messages(
    mut commands: Commands,
    mut messages: MessageReader<PlaySoundMessage>,
    audio_assets: Option<Res<AudioAssets>>,
    settings: Res<GameSettings>,
) {
    if !settings.sound_enabled {
        return;
    }
    let Some(assets) = audio_assets else {
        return;
    };

    for msg in messages.read() {
        let handle = match msg.0 {
            SoundType::Shoot => &assets.shoot,
            SoundType::EnemyDeath => &assets.enemy_death,
            SoundType::PlayerHurt => &assets.player_hurt,
            SoundType::LevelUp => &assets.levelup,
            SoundType::XpPickup => &assets.xp_pickup,
        };

        commands.spawn((
            AudioPlayer::new(handle.clone()),
            PlaybackSettings::DESPAWN
                .with_volume(bevy::audio::Volume::Linear(settings.master_volume)),
        ));
    }
}

pub fn start_bgm(
    mut commands: Commands,
    audio_assets: Option<Res<AudioAssets>>,
    settings: Res<GameSettings>,
    existing_bgm: Query<Entity, With<BgmMusic>>,
) {
    if !settings.music_enabled || !existing_bgm.is_empty() {
        return;
    }
    let Some(assets) = audio_assets else {
        return;
    };

    commands.spawn((
        AudioPlayer::new(assets.bgm.clone()),
        PlaybackSettings::LOOP
            .with_volume(bevy::audio::Volume::Linear(settings.master_volume * 0.4)),
        BgmMusic,
    ));
}

pub fn stop_bgm(mut commands: Commands, bgm_query: Query<Entity, With<BgmMusic>>) {
    for entity in &bgm_query {
        commands.entity(entity).despawn();
    }
}
