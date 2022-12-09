use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

// audio channels
#[derive(Resource)]
pub struct BackgroundMusicAudioChannel;
#[derive(Resource)]
pub struct MenuAudioChannel;
#[derive(Resource)]
pub struct SoundEffectsAudioChannel;

pub fn start_background_audio_system(
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
) {
    audio_channel
        .play(asset_server.load("sounds/deflector_soundtrack.mp3"))
        .looped();
}

pub fn set_audio_volume_system(
    background_audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    menu_audio_channel: Res<AudioChannel<MenuAudioChannel>>,
    effects_audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    background_audio_channel.set_volume(0.05);
    menu_audio_channel.set_volume(0.05);
    effects_audio_channel.set_volume(0.70);
}
