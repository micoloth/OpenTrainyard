// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_tweening::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use data_saving::{SolutionsSavedData, SelectedLevelSolvedDataEvent, save_player_data};
use menu_credits::MenuCredits;
use menu_solutions::MenuSolutionsPlugin;
// use trainyard::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;

// mod audio;
mod loading;
mod menu_title;
mod menu_levels;
mod utils;
mod tile;
mod train;
mod board;
mod logic;
mod menu_game_screen;
mod menu_utils;
mod all_puzzles_clean;
mod data_saving;
mod menu_solutions;
mod menu_credits;

// use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu_title::MenuPlugin;
use crate::menu_levels::MenuLevelsPlugin;
use crate::menu_utils::button_color_handler;
use crate::utils::SelectedLevel;

use crate::all_puzzles_clean::load_puzzles_data;

use crate::menu_game_screen::{MainGamePlugin, MenuMainGame};



mod simulator;

mod tests;
use tests::test;

use bevy_pkv::PkvStore;



// Button action type
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)] 
pub enum ButtonAction {Clear,Generate}

#[derive(Debug)] pub struct ButtonColors {pub normal: Color,pub hovered: Color,pub pressed: Color}
#[derive(Debug, Clone, Eq, PartialEq, Hash)] pub enum AppState {InGame, Out}







// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // Here the menu is drawn and waiting for player interaction
    MenuTitle,
    MenuLevels,
    MenuSolutions,
    MenuCredits,
    // During this State the actual game logic is executed
    Playing,
}



fn setup_camera(mut commands: Commands) {
    // commands.spawn(OrthographicCameraBundle::new_2d());  // 2D orthographic camera
    // commands.spawn(UiCameraBundle::default());  // UI Camera
    commands.spawn(Camera2dBundle::default());
}


fn main() {
    // test();
    App::new()
        // .insert_resource(Msaa { samples: 0 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(PkvStore::new("OpenTrainyard", "OpenTrainyard"))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {width: 320.,height: 550.,title: "Trainyard".to_string(), canvas: Some("#bevy".to_owned()), ..default()},
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(TweeningPlugin)
        .add_startup_system(setup_camera) // Startup system (cameras)
        .add_startup_system(set_window_icon)
        .insert_resource(load_puzzles_data())
        .insert_resource(SolutionsSavedData::default())
        .insert_resource(SelectedLevel::default())
        .add_plugin(LoadingPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(MenuLevelsPlugin)
        .add_plugin(MenuSolutionsPlugin)
        .add_plugin(MenuMainGame)
        .add_plugin(MenuCredits)
        // .add_plugin(InternalAudioPlugin)
        .add_plugin(MainGamePlugin)
        .add_state(GameState::Loading)
        .add_system(button_color_handler)
        .add_system(save_player_data)
        .add_event::<SelectedLevelSolvedDataEvent>()
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/samples/icon_crop.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
