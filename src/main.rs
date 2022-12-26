// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, NonSend, WindowDescriptor};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
// use trainyard::GamePlugin;
use bevy::ecs::system::EntityCommands;
use std::io::Cursor;
use winit::window::Icon;

mod audio;
mod loading;
mod menu;
mod utils;
mod tile;
mod train;
mod board;
mod logic;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::loading::BoardAssetsMap;
use crate::loading::TrainAssets;
use crate::menu::MenuPlugin;


use crate::train::*;
use crate::tile::*;
use crate::board::*;
use crate::logic::*;


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

mod simulator;

mod tests;
use tests::test;


use crate::board::*;






// Button action type
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)] 
pub enum ButtonAction {Clear,Generate}

#[derive(Debug)] pub struct ButtonColors {pub normal: Color,pub hovered: Color,pub pressed: Color}
#[derive(Debug, Clone, Eq, PartialEq, Hash)] pub enum AppState {InGame, Out}

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;



use bevy::time::FixedTimestep;  // 0.9: Thi is in Time, not in core


/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(check_mouse_action))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_tile))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(create_board),)
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup_board),)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(logic_tick_event))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_mouse_action)
                .with_system(move_trains)   
            )
            ;
    }
}



// use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
215+


fn main() {
    // test();
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Trainyard".to_string(), // ToDo
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(get_board_option_default())
        .insert_resource(get_ticks_in_a_tick_default())
        .add_plugin(GamePlugin)
        .add_startup_system(set_window_icon)
        .add_event::<TileSpawnEvent>()
        .add_event::<LogicTickEvent>()
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
