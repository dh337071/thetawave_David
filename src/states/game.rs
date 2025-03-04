use super::AppStates;
use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::prelude::*;

use crate::{audio, player::PlayersResource};

// Start the game by entering the Game state
pub fn start_game_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    players_resource: Res<PlayersResource>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // check for keyboard or gamepad input
    let mut start_input = keyboard_input.just_released(KeyCode::Return)
        || keyboard_input.just_released(KeyCode::Space);

    for gamepad in gamepads.iter() {
        start_input |= gamepad_input.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        });
    }

    // if input read enter the game state
    if start_input && players_resource.player_inputs[0].is_some() {
        // set the state to game
        next_app_state.set(AppStates::Game);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // reset input
        keyboard_input.reset(KeyCode::Return);
        keyboard_input.reset(KeyCode::Space);

        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South,
            });
        }
    }
}

// Start the game by entering the Game state
pub fn start_instructions_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // check for keyboard or gamepad input
    let mut start_input = keyboard_input.just_released(KeyCode::Return)
        || keyboard_input.just_released(KeyCode::Space);

    for gamepad in gamepads.iter() {
        start_input |= gamepad_input.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        });
    }

    // if input read enter the game state
    if start_input {
        // set the state to game
        next_app_state.set(AppStates::Instructions);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // reset input
        keyboard_input.reset(KeyCode::Return);
        keyboard_input.reset(KeyCode::Space);

        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South,
            });
        }
    }
}

pub fn start_character_selection_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // check for keyboard or gamepad input
    let mut start_input = keyboard_input.just_released(KeyCode::Return)
        || keyboard_input.just_released(KeyCode::Space);

    for gamepad in gamepads.iter() {
        start_input |= gamepad_input.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        });
    }

    // if input read enter the game state
    if start_input {
        // set the state to game
        next_app_state.set(AppStates::CharacterSelection);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // reset input
        keyboard_input.reset(KeyCode::Return);
        keyboard_input.reset(KeyCode::Space);

        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South,
            });
        }
    }
}

/// Quit the game if quit input read
pub fn quit_game_system(
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    // check for input
    let mut quit_input = keyboard_input.just_released(KeyCode::Escape);

    for gamepad in gamepads.iter() {
        quit_input |= gamepad_input.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::East,
        });
    }

    // quit app if input read
    if quit_input {
        app_exit_events.send(AppExit);
    }
}
