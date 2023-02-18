
use crate::GameState;
use crate::data_saving::LevelSolutionData;
use crate::data_saving::SolutionDataMap;
use crate::utils::SelectedLevel;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::loading::FontAssets;
use crate::menu_utils::*;

// Import PuzzlesData:
use crate::all_puzzles_clean::PuzzlesData;

use crate::menu_utils::make_button;


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
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(scroll_events_levels_touch)
                .with_system(scroll_events_levels_mouse)
                .with_system(click_play_button_levels)
                .with_system(click_play_button_levels)
                .with_system(handle_click_mouse)
                .with_system(handle_click_touch)
                .with_system(handle_full_click)
            )
            .add_system_set(SystemSet::on_exit(GameState::MenuLevels).with_system(cleanup_menu_levels))
            // Event FullClickHappened:
            .add_event::<FullClickHappened>()
            .add_event::<ScrollHappened>()
            ;
    }
}

#[derive(Component)]
pub struct LevelButton;



/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////





/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


fn setup_menu_levels(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    levels: Res<PuzzlesData>,
    windows: Res<Windows>,
    pkv: Res<PkvStore>,
    mut solution_data_map: ResMut<SolutionDataMap>,
) {

    if let Ok(all_solutions) = pkv.get::<SolutionDataMap>("solved_levels") {
        // Set resource:
        *solution_data_map = all_solutions;
    } else {
        // Setsolution_data_map.levels to  an empty hashmap
        *solution_data_map = SolutionDataMap::default();
    }

    println!("YES IM HERE. good...");
    println!("Fingerss?????????");
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();

    // Get name of first level:
    let names = levels.puzzles.iter().map(|x| x.name.clone()).collect::<Vec<String>>();

    let rect_width = 320.;
    let rect_height = 40.;
    
    // One button per level name, stacked vertically:
    for (i, name) in names.iter().enumerate() {

        // if name is in solution_data_map.levels, AND the object is of type Solved, then we have solved it, and we can make the button green
        let name2 = match solution_data_map.levels.get(name)
        {
            Some(LevelSolutionData::Solved(_)) => {
                format!("{} (SOLVED)", name)
            },
            _ => {name.clone()}
        };

        let offset_x = 0.;
        // Vertical offset:
        let offset_y = (i as f32) * rect_height;
        // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
        let left = width / 2. - rect_width / 2. + offset_x;
        let right = width / 2. + rect_width / 2. + offset_x;
        let top = height / 2. - rect_height / 2. + offset_y;
        let bottom = height / 2. - rect_height * 1.5 + offset_y;

        make_button(name2.to_string(), &mut commands, &font_assets, &button_colors, 25., left, right, top, bottom, LevelButton, Option::<LevelButton>::None);
    }
}




fn click_play_button_levels(
    mut selected_level: ResMut<SelectedLevel>,
    mut interaction_query: Query<
        (&Interaction, &ButtonData),
        (Changed<Interaction>, With<Button>, With<LevelButton>),
    >,
) {
    for (interaction, button_data) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                selected_level.level = button_data.text.clone();
            }
            _ => {}
        }
    }
}


// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const SCROLLWHEEL_SPEED_MULTIPLIER: f32 = - 0.03;
const TRACKPAD_SPEED_MULTIPLIER: f32 = - 0.8;

// Listen to scrollwheenl events:
fn scroll_events_levels_mouse(
    mut scroll_evr: EventReader<MouseWheel>,
    mut button_query: Query<&mut Style,(With<Button>, With<LevelButton>),>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        let vy = match ev.unit {
            MouseScrollUnit::Line => {ev.y * SCROLLWHEEL_SPEED_MULTIPLIER}
            MouseScrollUnit::Pixel => {info!("{:?}", ev.y); ev.y * TRACKPAD_SPEED_MULTIPLIER}
        };
        for mut style in button_query.iter_mut() {
            style.position.top.try_sub_assign(Val::Px(vy));
        }
    }
}

// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const TOUCH_SWIPE_SPEED_DECAY: f32 = 0.04;


// Listen to scrollwheenl events:
fn scroll_events_levels_touch(
    mut current_vy: Local<Option<f32>>,
    mut button_query: Query<&mut Style,(With<Button>, With<LevelButton>),>,
    mut scroll_evr: EventReader<ScrollHappened>,
    // touches: Res<Touches>, 
) {
    
    if let Some(vy) = current_vy.as_ref() {
        let new_vy = vy * (1. - TOUCH_SWIPE_SPEED_DECAY);
        if new_vy.abs() > 0.1 { *current_vy = Some(new_vy);} 
        else {*current_vy = None;}
    }
    // for finger in touches.iter() {
    //     *current_vy = Some(finger.delta().y);
    //     let finger_pos = format!("{:?}", finger.position());
    // }
    for ev in scroll_evr.iter() {
        *current_vy = Some(ev.vy);
    }
    if let Some(vy) = current_vy.as_ref() {
        for mut style in button_query.iter_mut() {
            style.position.top.try_sub_assign(Val::Px(*vy));
        }
    }
}




// Listen to event:
fn handle_full_click(
    mut full_click_happened_reader: EventReader<FullClickHappened>,
    mut state: ResMut<State<GameState>>,
    selected_level: Res<SelectedLevel>,
) {
    for _ in full_click_happened_reader.iter() {
        info!("YEEEE Successfull Click!!! : ");
        if selected_level.level != ""
        {
            state.set(GameState::MenuSolutions).unwrap();
        }
    }
}



// state.set(GameState::Playing).unwrap();



fn cleanup_menu_levels(mut commands: Commands, buttons: Query<Entity, (With<Button>, With<LevelButton>)>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

