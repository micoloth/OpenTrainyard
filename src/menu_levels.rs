use crate::GameState;
use crate::utils::SelectedLevel;
use bevy::prelude::*;

use crate::loading::FontAssets;
use crate::menu_utils::*;

// Import PuzzlesData:
use crate::all_puzzles_clean::PuzzlesData;

use crate::loading::TileAssets;

use crate::menu_utils::make_button;
use crate::menu_utils::ButtonColors;


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

pub struct MenuLevelsPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuLevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::MenuLevels).with_system(setup_menu_levels))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(click_play_button_levels))
            .add_system_set(SystemSet::on_exit(GameState::MenuLevels).with_system(cleanup_menu_levels));
    }
}

#[derive(Component)]
pub struct StartGameBotton;









/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


fn setup_menu_levels(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    levels: Res<PuzzlesData>,
    windows: Res<Windows>,
) {
    println!("YES IM HERE. good...");
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();

    // Get name of first level:
    let name = levels.puzzles[0].name.clone();
    let names = levels.puzzles.iter().map(|x| x.name.clone()).collect::<Vec<String>>();

    let rect_width = 320.;
    let rect_height = 40.;
    
    // One button per level name, stacked vertically:
    for (i, name) in names.iter().enumerate() {

        let offset_x = 0.;
        // Vertical offset:
        let offset_y = -(i as f32) * rect_height;
        // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
        let left = width / 2. - rect_width / 2. + offset_x;
        let right = width / 2. + rect_width / 2. + offset_x;
        let top = height / 2. - rect_height / 2. + offset_y;
        let bottom = height / 2. - rect_height * 1.5 + offset_y;

        let startbutton_id = make_button(name.to_string(), &mut commands, &font_assets, &button_colors, 25., left, right, top, bottom, StartGameBotton, Option::<StartGameBotton>::None);
    }
    // make_scrollbar(&mut commands, &textures, 50., 250., 50., 25.);
}




fn click_play_button_levels(
    mut selected_level: ResMut<SelectedLevel>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonData),
        (Changed<Interaction>, With<Button>, With<StartGameBotton>),
    >,
) {
    for (interaction, button_data) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
                selected_level.level = button_data.text.clone();
            }
            _ => {}
        }
    }
}

fn cleanup_menu_levels(mut commands: Commands, buttons: Query<Entity, With<Button>>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

