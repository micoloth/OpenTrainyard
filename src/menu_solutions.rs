
use std::time::Duration;

use crate::data_saving::LevelSolutionData;
use crate::data_saving::SolutionData;
use crate::data_saving::SolutionsSavedData;
use crate::loading::FontAssets;
use crate::GameState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_tweening::*;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::lens::UiPositionLens;

use crate::menu_utils::*;
use crate::board::Rect;
use crate::loading::TileAssets;

use crate::board::*;
use crate::all_puzzles_clean::*;

use crate::utils::SelectedLevel;


// Defines the amount of time that should elapse between each physics step.

/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


const ANIMATION_TIME_STEP: f32 = 600.;


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
                .with_system(handle_gesture_mouse)
                .with_system(handle_gesture_touch)
                .with_system(handle_full_click_solution)
                .with_system(click_nextlevel_button_solution)
                .with_system(click_prevlevel_button_solution)
                .with_system(click_back_button_solution)
                .with_system(click_clone_button_solution)
                .with_system(scroll_events_solution_touch)
                .with_system(scroll_events_solution_mouse)
                .with_system(handle_gesture_mouse)
                .with_system(handle_gesture_touch)
                .with_system(handle_full_click_solution)
                .with_system(advance_tick)
            )
            .add_event::<BoardEvent>()
            // add CarouselState resource:
            .insert_resource(CarouselState::default())
            ;
    }
}


// Resource CarouselState:
#[derive(Debug, Component, Default, Resource)]
pub struct CarouselState {
    pub maps: Vec<SolutionData>,
    pub current_index: u16,
    pub timer: Timer,
    pub position_offset: Vec3,
    pub position_delta: Vec3,
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
pub struct CloneButton;

#[derive(Component)]
pub struct LevelNameElem;

#[derive(Component)]
pub struct CarouselTextNode;




/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////



/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////



fn advance_tick(
    mut carousel_state: ResMut<CarouselState>,
    time: Res<Time>,
) {
    carousel_state.timer.tick(time.delta());
}


fn setup_solutions_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TileAssets>,
    windows: Res<Windows>,
    selected_level: Res<SelectedLevel>, 
    board_q: Query<Entity, With<Board>>, 
    mut board_event_writer: EventWriter<BoardEvent>, 
    level_name_query: Query<Entity, With<LevelNameElem>>, 
    levels: Res<PuzzlesData>, 
    solution_data_map: Res<SolutionsSavedData>,
    // Resource CarouselState:
    mut carousel_state: ResMut<CarouselState>,
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
    _make_board_and_title(board_q, &mut commands, level_name_query, maps, board_event_writer, level_name, &windows, &font_assets, &button_colors, &mut carousel_state);

    
    let (width, margin, heigh, percent_left_right, left, right, bottom, top) = get_coordinates(&windows);
    let erase_id = make_button("Clone".to_string(), &mut commands, &font_assets, &button_colors, 25., left, right, top - heigh - margin, bottom - heigh - margin, SolutionsMenuBotton, Some(CloneButton));
    let undo_id = make_button("Previous level".to_string(), &mut commands, &font_assets, &button_colors, 25., left, right , top, bottom, PrevLevelButton, Some(SolutionsMenuBotton));
    let run_id = make_button("Next level".to_string(), &mut commands, &font_assets, &button_colors, 25., width * percent_left_right + margin/2., width - margin , top, bottom, SolutionsMenuBotton, Some(NextLevelButton));
    
    // Upper::
    let ((left_, right_, bottom_, top_), _, _) = get_upper_coordinates(&windows);
    let back_id = make_button("Back".to_string(), &mut commands, &font_assets, &button_colors, 20., left_, right_, top_, bottom_, SolutionsMenuBotton, Some(BackButton));
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



// Listen to scrollwheenl events:
pub fn scroll_events_solution_mouse(
    mut scroll_evr: EventReader<MouseWheel>,
    board_q: Query<(Entity, &Transform), With<Board>>, 
    textnode_q: Query<(Entity, &Transform, &Style), With<CarouselTextNode>>, 
    windows: Res<Windows>, 
    mut carousel_state: ResMut<CarouselState>,
    mut commands: Commands,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        let (vx, vy) = match ev.unit {
            MouseScrollUnit::Line => { (ev.x, ev.y) }
            MouseScrollUnit::Pixel => { (ev.x, ev.y) }
        };
        // v = vy if vx==0 else vx
        let v = if vx == 0. { vy } else { vx };
        if v<0. && carousel_state.timer.finished() && carousel_state.current_index < carousel_state.maps.len() as u16 - 1 {
            _start_animation(true, &board_q, &textnode_q, &windows, &mut commands, &mut carousel_state);
            carousel_state.current_index += 1;
        } else if v>0. && carousel_state.timer.finished() && carousel_state.current_index > 0
        {
            _start_animation(false, &board_q, &textnode_q, &windows, &mut commands, &mut carousel_state);
            carousel_state.current_index -= 1;
        }
    }
}

// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const TOUCH_SWIPE_SPEED_DECAY: f32 = 0.04;


// Listen to scrollwheenl events:
pub fn scroll_events_solution_touch(
    board_q: Query<(Entity, &Transform), With<Board>>, 
    textnode_q: Query<(Entity, &Transform, &Style), With<CarouselTextNode>>, 
    mut scroll_evr: EventReader<ScrollHappened>,
    // touches: Res<Touches>, 
    mut carousel_state: ResMut<CarouselState>,
    mut commands: Commands,
    windows: Res<Windows>, 
) {
    // for finger in touches.iter() {
    //     *current_vy = Some(finger.delta().y);
    //     let finger_pos = format!("{:?}", finger.position());
    // }
    for ev in scroll_evr.iter() {
        let current_vx = Some(ev.vx);
        if let Some(vx) = current_vx.as_ref() {     
            if *vx < 0. && carousel_state.timer.finished() && carousel_state.current_index < carousel_state.maps.len() as u16 - 1 {
                _start_animation(true, &board_q, &textnode_q, &windows, &mut commands, &mut carousel_state);
                carousel_state.current_index += 1;
            } else if *vx > 0. && carousel_state.timer.finished() && carousel_state.current_index > 0
            {
                _start_animation(false, &board_q,  &textnode_q,&windows, &mut commands, &mut carousel_state);
                carousel_state.current_index -= 1;
            }
        }
    }
}



fn click_back_button_solution(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<BackButton>)>,
    mut game_state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for (interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                selected_level.level = "".to_string();
                game_state.set(GameState::MenuLevels);
            }
            _ => {}
        }
    }
}


fn click_clone_button_solution(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<CloneButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    windows: Res<Windows>,
    board_q: Query<(Entity, &Transform), With<Board>>, 
    textnode_q: Query<(Entity, &Transform, &Style), With<CarouselTextNode>>, 
    mut carousel_state: ResMut<CarouselState>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // Iter over boards:
                _start_animation(true, &board_q, &textnode_q, &windows, &mut commands, &mut carousel_state);
            }
            _ => {}
        }
    }
}



fn click_nextlevel_button_solution(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<NextLevelButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    windows: Res<Windows>,
    board_q: Query<Entity, With<Board>>, 
    board_event_writer: EventWriter<BoardEvent>, 
    level_name_query: Query<Entity, With<LevelNameElem>>, 
    solution_data_map: Res<SolutionsSavedData>,
    mut carousel_state: ResMut<CarouselState>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(next_puzzle) = get_next_puzzle(selected_level.level.clone(), &levels) {
                    let level_name = next_puzzle.name.clone();
                    selected_level.level = level_name.clone();
                    println!("LAUNCHED: {}", level_name.clone());
                    let empty_map = levels.puzzles.iter().find(|p| p.name == *level_name.clone()).unwrap().parsed_map.clone();    
                    let solved_data = solution_data_map.levels.get(&level_name);
                    let maps = match solved_data {
                        Some(LevelSolutionData::Solved(solved_maps)) => solved_maps.iter().map(|s| s.map.clone()).collect(),
                        _ => vec![empty_map.clone()],
                    };
                    _make_board_and_title(board_q, &mut commands, level_name_query, maps, board_event_writer, level_name, &windows, &font_assets, &button_colors, &mut carousel_state);
                    return
                }
            }
            _ => {}
        }
    }
}




fn click_prevlevel_button_solution(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<PrevLevelButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    windows: Res<Windows>,
    board_q: Query<Entity, With<Board>>, 
    board_event_writer: EventWriter<BoardEvent>, 
    level_name_query: Query<Entity, With<LevelNameElem>>, 
    solution_data_map: Res<SolutionsSavedData>,
    mut carousel_state: ResMut<CarouselState>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(prev_puzzle) = get_prev_puzzle(selected_level.level.clone(), &levels) {
                    let level_name = prev_puzzle.name.clone();
                    selected_level.level = level_name.clone();
                    let empty_map = levels.puzzles.iter().find(|p| p.name == *level_name.clone()).unwrap().parsed_map.clone();    
                    let solved_data = solution_data_map.levels.get(&level_name);
                    let mut maps = match solved_data {
                        Some(LevelSolutionData::Solved(solved_maps)) => {solved_maps.iter().map(|s| s.map.clone()).collect()},
                        _ => vec![empty_map.clone()],
                    };
                    // Push empty_map.clone() to maps (add 1 element to the vec):
                    maps.push(empty_map.clone());
                    _make_board_and_title(board_q, &mut commands, level_name_query, maps, board_event_writer, level_name, &windows, &font_assets, &button_colors, &mut carousel_state);
                    return
                }
            }
            _ => {}
        }
    }
}

// Listen to event:
fn handle_full_click_solution(
    mut full_click_happened_reader: EventReader<FullClickHappened>,
    mut state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
    carousel_state: ResMut<CarouselState>,
    windows: Res<Windows>,
) {
    for ev in full_click_happened_reader.iter() {
        // Get the board:
        // Check if ev.pos is inside the board:
        let window = windows.get_primary().unwrap();
        let width = window.width() as f32;
        let height = window.height() as f32;
        let rect = Rect{left:0. - width / 2., top:height / 2. - width / 2.+25. - height / 2. , right:width - width / 2.,  bottom:height / 2. +width / 2. + 25. - height / 2. };
        if carousel_state.timer.finished() && in_bounds(ev.pos, rect) {
            // Get the map:
            let map = carousel_state.maps[carousel_state.current_index as usize].map.clone();
            selected_level.map = map;
            state.set(GameState::Playing);
            // Write the event:
            // change_level_writer.send(ChangeLevel);
        }
    }
}





/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

const SCALE: f32 = 0.5;



fn _start_animation(
    go_left: bool,
    board_q: &Query<(Entity, &Transform), With<Board>>, 
    textnode_q: &Query<(Entity, &Transform, &Style), With<CarouselTextNode>>, 
    windows: &Res<Windows>, 
    commands: &mut Commands,
    carousel_state: &mut ResMut<CarouselState>,
) {
    // Get the window:
    let window = windows.get_primary().unwrap();
    let width = window.width() as f32;
    let delta = if go_left { - carousel_state.position_delta.x } else { carousel_state.position_delta.x };
    for (board_id, transform) in board_q.iter() {
        let board_pos = transform.translation;
        let new_transform = Vec3::new(board_pos.x + delta, board_pos.y, board_pos.z);
        let tween = Tween::new(
            EaseFunction::QuadraticInOut, Duration::from_millis(ANIMATION_TIME_STEP as u64),
            TransformPositionLens {start: transform.translation, end: new_transform,},
        );
        commands.entity(board_id).insert(Animator::new(tween),);
    }
    // Same for text nodes:
    for (textnode_id, transform, style) in textnode_q.iter() {
        let textnode_pos = style.position;
        let newright = textnode_pos.right.try_add(Val::Px(delta)).unwrap();  // , width).unwrap();
        let newleft = textnode_pos.left.try_add(Val::Px(delta)).unwrap();  // , width).unwrap();
        // let new_pos = UiRect{left: Val::Px(newleft), top: textnode_pos.top, right: Val::Px(newright), bottom: textnode_pos.bottom};
        let new_pos = UiRect{left: newleft, top: textnode_pos.top, right: newright, bottom: textnode_pos.bottom};
        let tween = Tween::new(
            EaseFunction::QuadraticInOut, Duration::from_millis(ANIMATION_TIME_STEP as u64),
            UiPositionLens {start: textnode_pos, end: new_pos,},
        );
        commands.entity(textnode_id).insert(Animator::new(tween),);
    }

    // Restart the timer in the carousel state:
    carousel_state.timer = Timer::new(Duration::from_millis(ANIMATION_TIME_STEP as u64), TimerMode::Once)
}


fn _make_board_and_title(board_q: Query<Entity, With<Board>>, commands: &mut Commands, level_name_query: Query<Entity, With<LevelNameElem>>, maps: Vec<String>, mut board_event_writer: EventWriter<BoardEvent>, level_name: String, windows: &Res<Windows>, font_assets: &Res<FontAssets>, button_colors: &Res<ButtonColors>, mut carousel_state: &mut CarouselState,) {
    let w = windows.get_primary().unwrap();
    // Get width:
    let width = w.width();
    let height = w.height();
    // Delete board:
    for board_id in board_q.iter() {
        if let Some(id) = commands.get_entity(board_id) { id.despawn_recursive();}
    }
    // Despawn the level name:
    for level_name_id in level_name_query.iter() {
        if let Some(level_name_ec) = commands.get_entity(level_name_id) {level_name_ec.despawn_recursive();}
    }

    let map_datas: Vec<SolutionData> = maps.iter().map(|map| SolutionData::new_from_string(map.clone())).collect();
    carousel_state.maps = map_datas.clone();
    carousel_state.current_index = 0;
    carousel_state.timer = Timer::new(Duration::from_millis(100), TimerMode::Once);
    carousel_state.position_delta = Vec3::new(width * 0.6, 0., 0.);
    carousel_state.position_offset = Vec3::new((1.-SCALE) * width/2. * 1.5  -width * 0.61, - width * SCALE / 2. + 25., 0.);

    
    // Spawn the level name BUTTON:
    let (_, (left_, right_, bottom_, top_), _) = get_upper_coordinates(&windows);
    let name_id = make_text(level_name.clone(), commands, &font_assets, &button_colors, 20., left_, right_, top_, bottom_, SolutionsMenuBotton, Some(LevelNameElem));

    // Set the name of the game:
    // Get the solved maps, if there are any:
    // let n_maps = maps.len();
    for (i, map_data) in map_datas.iter().enumerate() {
        let pos = carousel_state.position_offset + carousel_state.position_delta * i as f32;
        let boardpos = Some(BoardPosition::Custom(pos));
        board_event_writer.send(BoardEvent::Make{map_name: level_name.clone(), map: map_data.map.clone(), scale: SCALE, position: boardpos, index: Some(i as u32)});

        // Make the text:
        let text = format!("{} / {}", map_data.tracks, map_data.second_tracks);
        make_text(text, commands, &font_assets, &button_colors, 20., left_, right_, top_+75., bottom_+75., SolutionsMenuBotton, Some(CarouselTextNode));
    }
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

fn get_upper_coordinates(windows: &Windows) -> ((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32)) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    // Genius plan: I'll assume THE BOARD IS ALWAYS ABOUT AS WIDE AS THE SCREEN, AND ALSO SQUARE.
    // Boundaries (left right top bottom) of a Rectangle that occupies the RIGHT HALF of the screen, minus a 20 pixel wide margin all around:
    let margin = 7.;
    let button_height = 30.;
    // Position it at the TOP of the screen:
    let percent_left_right = 0.3;
    let left = margin;
    let right = width * percent_left_right + margin/2.;
    let bottom = height / 2. - width / 2. - 3.5 * margin - 2.* button_height;
    let top = height / 2. - width / 2. - 3.5 * margin - button_height;
    return ((left, right - margin, bottom, top), (right, width - right, bottom, top), (width - right + margin, width - left, bottom, top));
}



