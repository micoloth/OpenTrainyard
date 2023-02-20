
use crate::data_saving::LevelSolutionData;
use crate::data_saving::SolutionDataMap;
use crate::loading::FontAssets;
use crate::GameState;
use crate::simulator::parse_map;
use crate::simulator::pretty_print_map;
use crate::simulator::print_map;
use bevy::prelude::*;

use crate::menu_utils::*;

use crate::loading::TileAssets;

use crate::tile::*;
use crate::board::*;
use crate::menu_utils::*;
use crate::all_puzzles_clean::*;

use crate::utils::SelectedLevel;

use crate::data_saving::save_player_data;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 120.0;

/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Component, Default, Resource)]
pub struct GameScreenState {
    pub name: String,
    pub solved_map: String,

}


pub struct MenuSolutionsPlugin;


/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::MenuSolutions`
impl Plugin for MenuSolutionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(get_board_option_default())
            .add_system_set(SystemSet::on_enter(GameState::MenuSolutions).with_system(setup_solutions_menu),)
            .add_system_set(SystemSet::on_exit(GameState::MenuSolutions).with_system(cleanup_solutions_menu),)
            .add_system_set(SystemSet::on_update(GameState::MenuSolutions)
                .with_system(create_board)
                .with_system(handle_click_mouse)
                .with_system(handle_click_touch)
                .with_system(handle_full_click)
                .with_system(click_nextlevel_button)
                .with_system(click_prevlevel_button)
                .with_system(click_back_button)
            )
            .add_event::<BoardEvent>()
            ;
    }
}


#[derive(Component)]
pub struct SolutionsMenuBotton;

#[derive(Component)]
pub struct NextLevelButton;

#[derive(Component)]
pub struct PrevLevelButton;

#[derive(Component)]
pub struct CopySolutionButton;

#[derive(Component)]
pub struct NewSolutionButton;


#[derive(Component)]
pub struct BackButton;


#[derive(Component)]
pub struct LevelNameElem;




/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////



/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////



fn setup_solutions_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TileAssets>,
    windows: Res<Windows>,
    selected_level: Res<SelectedLevel>, 
    levels: Res<PuzzlesData>, 
    board_q: Query<Entity, With<Board>>, 
    mut board_event_writer: EventWriter<BoardEvent>, 
    level_name_query: Query<Entity, With<LevelNameElem>>, 
    solution_data_map: Res<SolutionDataMap>,
) 
{
    let level_name = selected_level.level.clone();
    // Print the game name:
    println!("LAUNCHED: {}", level_name.clone());
    let empty_map = levels.puzzles.iter().find(|p| p.name == *level_name.clone()).unwrap().parsed_map.clone();    
    let solved_data = solution_data_map.levels.get(&level_name);
    let maps = match solved_data {
        Some(LevelSolutionData::Solved(solved_maps)) => solved_maps.iter().map(|s| s.map.clone()).collect(),
        _ => vec![empty_map.clone()],
    };
    _make_board_and_title(board_q, &mut commands, level_name_query, maps, board_event_writer, level_name, &windows, &font_assets, &button_colors);

    
    let (width, margin, heigh, percent_left_right, left, right, bottom, top) = get_coordinates(&windows);
    let erase_id = make_button("Clone".to_string(), &mut commands, &font_assets, &button_colors, 25., left, right, top - heigh - margin, bottom - heigh - margin, SolutionsMenuBotton, Some(NextLevelButton));
    let undo_id = make_button("Previous level".to_string(), &mut commands, &font_assets, &button_colors, 25., left, right , top, bottom, PrevLevelButton, Some(SolutionsMenuBotton));
    let run_id = make_button("Next level".to_string(), &mut commands, &font_assets, &button_colors, 25., width * percent_left_right + margin/2., width - margin , top, bottom, SolutionsMenuBotton, Some(NextLevelButton));
    
    // Upper::
    let (left_, right_, bottom_, top_) = get_upper_coordinates(&windows);
    let back_id = make_button("Back".to_string(), &mut commands, &font_assets, &button_colors, 20., width - right_, width - left_, top_, bottom_, SolutionsMenuBotton, Some(BackButton));
}




fn cleanup_solutions_menu(
        mut commands: Commands, 
        buttons: Query<Entity, With<SolutionsMenuBotton>>,
        board_q: Query<Entity, With<Board>>, 
        mut board_event_writer: EventWriter<BoardEvent>,
) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) { id.despawn_recursive();};
    }
    // Delete boards:
    for board_id in board_q.iter() {
        if let Some(id) = commands.get_entity(board_id) { id.despawn_recursive();}
    }
    board_event_writer.send(BoardEvent::Delete);

}


fn click_back_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<BackButton>)>,
    mut game_state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::MenuLevels);
                selected_level.level = "".to_string();
            }
            _ => {}
        }
    }
}

fn click_nextlevel_button(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<NextLevelButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    windows: Res<Windows>,
    board_q: Query<Entity, With<Board>>, 
    mut board_event_writer: EventWriter<BoardEvent>, 
    level_name_query: Query<Entity, With<LevelNameElem>>, 
    solution_data_map: Res<SolutionDataMap>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(next_puzzle) = get_next_puzzle(selected_level.level.clone(), &levels) {
                    let level_name = next_puzzle.name.clone();
                    selected_level.level = level_name.clone();
                    // let map = levels.puzzles.iter().find(|p| p.name == next_puzzle.name.clone()).unwrap().parsed_map.clone();    
                    // // TODO this is wrong u should read playerdata
                    // selected_level.map = map.clone();
                    // Print the game name:
                    println!("LAUNCHED: {}", level_name.clone());
                    let empty_map = levels.puzzles.iter().find(|p| p.name == *level_name.clone()).unwrap().parsed_map.clone();    
                    let solved_data = solution_data_map.levels.get(&level_name);
                    let maps = match solved_data {
                        Some(LevelSolutionData::Solved(solved_maps)) => solved_maps.iter().map(|s| s.map.clone()).collect(),
                        _ => vec![empty_map.clone()],
                    };
                    _make_board_and_title(board_q, &mut commands, level_name_query, maps, board_event_writer, level_name, &windows, &font_assets, &button_colors);
                    return
                }
            }
            _ => {}
        }
    }
}


fn click_prevlevel_button(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<PrevLevelButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    windows: Res<Windows>,
    board_q: Query<Entity, With<Board>>, 
    mut board_event_writer: EventWriter<BoardEvent>, 
    level_name_query: Query<Entity, With<LevelNameElem>>, 
    solution_data_map: Res<SolutionDataMap>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(next_puzzle) = get_prev_puzzle(selected_level.level.clone(), &levels) {
                    let level_name = next_puzzle.name.clone();
                    selected_level.level = level_name.clone();
                    // let map = levels.puzzles.iter().find(|p| p.name == next_puzzle.name.clone()).unwrap().parsed_map.clone();    
                    // // TODO this is wrong u should read playerdata
                    // selected_level.map = map.clone();
                    // Print the game name:
                    println!("LAUNCHED: {}", level_name.clone());
                    let empty_map = levels.puzzles.iter().find(|p| p.name == *level_name.clone()).unwrap().parsed_map.clone();    
                    let solved_data = solution_data_map.levels.get(&level_name);
                    let maps = match solved_data {
                        Some(LevelSolutionData::Solved(solved_maps)) => solved_maps.iter().map(|s| s.map.clone()).collect(),
                        _ => vec![empty_map.clone()],
                    };
                    _make_board_and_title(board_q, &mut commands, level_name_query, maps, board_event_writer, level_name, &windows, &font_assets, &button_colors);
                    return
                }
            }
            _ => {}
        }
    }
}

// Listen to event:
fn handle_full_click(
    mut full_click_happened_reader: EventReader<FullClickHappened>,
    mut board_q: Query<(&BoardTileMap, &BoardDimensions), With<Board>>,
    mut state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for ev in full_click_happened_reader.iter() {
        // Get the board:
        for (board, board_dimensions) in board_q.iter() {
            // Check if ev.pos is inside the board:
            if in_bounds(ev.pos, board_dimensions.rect) {
                // Get the map:
                let map = board.map_string.clone();
                selected_level.map = map;
                println!("WAIT.. we not passing from here?? {:?}", selected_level.map);
                state.set(GameState::Playing);
                // Write the event:
                // change_level_writer.send(ChangeLevel);
            }
        }
    }
}




// fn change_level(
//     level_name: String, 
//     map: String, 
//     board_q: &Query<Entity, With<Board>>, 
//     commands: &mut Commands, 
//     board_event_writer: &mut EventWriter<BoardEvent>, 
//     level_name_query: &Query<Entity,  With<LevelNameElem>>, 
//     windows: &Windows, 
//     font_assets: &FontAssets, 
//     button_colors: &ButtonColors) {
// // Delete board:
// for board_id in board_q.iter() {
//     if let Some(id) = commands.get_entity(board_id) { id.despawn_recursive();}
// }
// // Set the name of the game:

// // Send the event to create the board:
// println!("LAUNCHED: {}", level_name.clone());
// board_event_writer.send(BoardEvent::Make{map_name: level_name.clone(), map: map.clone(), scale: 1.});
// // Print the game name:
// // Despawn the level name:
// for level_name_id in level_name_query.iter() {
//     if let Some(level_name_ec) = commands.get_entity(level_name_id) {level_name_ec.despawn_recursive();}
// }
// // Spawn the level name BUTTON:
// let (left_, right_, bottom_, top_) = get_upper_coordinates(windows);
// let name_id = make_button(level_name.clone(), commands, font_assets, button_colors, 20., left_ - 110., right_ -110., top_, bottom_, MainGameBotton, Some(LevelNameElem));

// }

/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

fn _make_board_and_title(board_q: Query<Entity, With<Board>>, commands: &mut Commands, level_name_query: Query<Entity, With<LevelNameElem>>, maps: Vec<String>, mut board_event_writer: EventWriter<BoardEvent>, level_name: String, windows: &Res<Windows>, font_assets: &Res<FontAssets>, button_colors: &Res<ButtonColors>) {
    // Delete board:
    for board_id in board_q.iter() {
        if let Some(id) = commands.get_entity(board_id) { id.despawn_recursive();}
    }
    // Despawn the level name:
    for level_name_id in level_name_query.iter() {
        if let Some(level_name_ec) = commands.get_entity(level_name_id) {level_name_ec.despawn_recursive();}
    }
    // Set the name of the game:
    // Get the solved maps, if there are any:
    for map in maps {
        board_event_writer.send(BoardEvent::Make{map_name: level_name.clone(), map: map, scale: 0.7});
    }
    
    let (left_, right_, bottom_, top_) = get_upper_coordinates(windows);
    let name_id = make_button(level_name.clone(), commands, font_assets, button_colors, 20., left_ - 110., right_ -110., top_, bottom_, SolutionsMenuBotton, Some(LevelNameElem));
}


fn get_coordinates(windows: &Windows) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    // Genius plan: I'll assume THE BOARD IS ALWAYS ABOUT AS WIDE AS THE SCREEN, AND ALSO SQUARE.
    // Boundaries (left right top bottom) of a Rectangle that occupies the LEFT HALF of the screen, minus a 20 pixel wide margin all around:
    let margin = 7.;
    let button_height = 40.;
    let percent_left_right = 0.5;
    let left = margin;
    let right = width * percent_left_right - margin/2.;
    // Make the button 40 px high FROM THE BOTTOM:
    let bottom = height / 2. + width / 2. - 1.5 * margin ;
    let top = height / 2. + width / 2. - 1.5 * margin + button_height;
    (width, margin, button_height, percent_left_right, left, right, bottom, top)
}

fn get_upper_coordinates(windows: &Windows) -> (f32, f32, f32, f32) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    // Genius plan: I'll assume THE BOARD IS ALWAYS ABOUT AS WIDE AS THE SCREEN, AND ALSO SQUARE.
    // Boundaries (left right top bottom) of a Rectangle that occupies the RIGHT HALF of the screen, minus a 20 pixel wide margin all around:
    let margin = 7.;
    let button_height = 30.;
    // Position it at the TOP of the screen:
    let percent_left_right = 0.65;
    let left = width * percent_left_right + margin/2.;
    let right = width - margin;
    let bottom = height / 2. - width / 2. - 3.5 * margin - 2.* button_height;
    let top = height / 2. - width / 2. - 3.5 * margin - button_height;
    return (left, right, bottom, top);
}


