

pub struct ActionsPlugin;


// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<Actions>().add_system_set(
        //     SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        // );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}


use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(control_flying_sound),)
            ;
    }
}

#[derive(Default, Resource)]
struct FlyingAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

fn control_flying_sound(
    actions: Res<Actions>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.player_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.player_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
