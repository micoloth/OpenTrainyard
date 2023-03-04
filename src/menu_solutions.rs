
use std::time::Duration;

use crate::data_saving::SelectedLevelSolvedDataEvent;
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
                .with_system(click_newsolution_button_solution)
                .with_system(scroll_events_solution_touch)
                .with_system(scroll_events_solution_mouse)
                .with_system(click_deletesolution_button_solution)
                .with_system(handle_gesture_mouse)
                .with_system(handle_gesture_touch)
                .with_system(handle_full_click_solution)
                .with_system(make_board_and_title.before(handle_full_click_solution))
                .with_system(advance_tick)
            )
            .add_event::<BoardEvent>()
            .add_event::<RedrawCarouselEvent>()
            // add CarouselState resource:
            .insert_resource(CarouselState::default())
            ;
    }
}


// Resource CarouselState:
#[derive(Debug, Component, Default, Resource)]
pub struct CarouselState {
    pub timer: Timer,
    pub position_offset: Vec3,
    pub position_delta: Vec3,
    // Note that current_index (which is the index of the current map) is stored in the SelectedLevel resource !!
}


#[derive(Component)]
pub struct SolutionsMenuBotton;

#[derive(Component)]
pub struct NextLevelButtonSolutions;

#[derive(Component)]
pub struct PrevLevelButton;

#[derive(Component)]
pub struct CopySolutionButton;

#[derive(Component)]
pub struct NewSolutionButton;

#[derive(Component)]
pub struct DeleteSolutionButton;

#[derive(Component)]
pub struct BackButtonSolutions;

#[derive(Component)]
pub struct CloneButton;

#[derive(Component)]
pub struct LevelNameElem;

#[derive(Component)]
pub struct BestScoreElem;

#[derive(Component)]
pub struct CarouselTextNode;




/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct RedrawCarouselEvent {
    pub maps: Option<Vec<SolutionData>>,
    pub level_name: String,
    pub index: Option<usize>,
}


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
    windows: Res<Windows>,
    selected_level: ResMut<SelectedLevel>, 
    // Resource CarouselState:
    mut redraw_carousel_event_writer: EventWriter<RedrawCarouselEvent>,
) 
{
    let level_name = selected_level.level.clone();
    // Print the game name:
    println!("LAUNCHED: {}", level_name.clone());
    redraw_carousel_event_writer.send(RedrawCarouselEvent { maps: None, level_name: level_name, index: None});
    let font_size = 22.;

    
    let (width, margin, heigh, percent_left_right, left, right, bottom, top) = get_coordinates(&windows);
    let prev_id = make_button("PREVIOUS LEVEL".to_string(), &mut commands, &font_assets, &button_colors, font_size, left, right , top, bottom, PrevLevelButton, Some(SolutionsMenuBotton));
    let next_id = make_button("NEXT LEVEL".to_string(), &mut commands, &font_assets, &button_colors, font_size, width * percent_left_right + margin/2., width - margin , top, bottom, SolutionsMenuBotton, Some(NextLevelButtonSolutions));
    
    
    let ((l1, r1, b1, t1), (l2, r2, b2, t2), (l3, r3, b3, t3)) = get_sol_commands_coordinates(&windows);
    println!("l1: {}, r1: {}, b1: {}, t1: {}", l1, r1, b1, t1);
    println!("l2: {}, r2: {}, b2: {}, t2: {}", l2, r2, b2, t2);
    println!("l3: {}, r3: {}, b3: {}, t3: {}", l3, r3, b3, t3);

    let new_id = make_button("DELETE".to_string(), &mut commands, &font_assets, &button_colors, font_size, l1, r1, t1, b1, SolutionsMenuBotton, Some(DeleteSolutionButton));
    let new_id = make_button("NEW SOLUTION".to_string(), &mut commands, &font_assets, &button_colors, font_size, l2, r2, t2, b2, SolutionsMenuBotton, Some(NewSolutionButton));
    let clone_id = make_button("CLONE".to_string(), &mut commands, &font_assets, &button_colors, font_size, l3, r3, t3, b3, SolutionsMenuBotton, Some(CloneButton));


    // Upper::
    let ((left_, right_, bottom_, top_), _, _) = get_upper_coordinates(&windows);
    let back_id = make_button("BACK".to_string(), &mut commands, &font_assets, &button_colors, 22.*0.8, left_, right_, top_, bottom_, SolutionsMenuBotton, Some(BackButtonSolutions));
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
    mut selected_level: ResMut<SelectedLevel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        let (vx, vy) = match ev.unit {
            MouseScrollUnit::Line => { (ev.x, ev.y) }
            MouseScrollUnit::Pixel => { (ev.x, ev.y) }
        };
        // v = vy if vx==0 else vx
        let v = if vx == 0. { vy } else { vx };
        _scroll_event_solution(v, &mut carousel_state, &mut selected_level, &board_q, &textnode_q, &windows, &mut commands);
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
    mut selected_level: ResMut<SelectedLevel>,
) {
    // for finger in touches.iter() {
    //     *current_vy = Some(finger.delta().y);
    //     let finger_pos = format!("{:?}", finger.position());
    // }
    for ev in scroll_evr.iter() {
        let current_vx = Some(ev.vx);
        if let Some(vx) = current_vx.as_ref() {     
            _scroll_event_solution(*vx, &mut carousel_state, &mut selected_level, &board_q, &textnode_q, &windows, &mut commands);
        }
    }
}



fn click_back_button_solution(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<BackButtonSolutions>)>,
    mut game_state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for (interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let level_name = selected_level.level.clone();
                *selected_level = SelectedLevel::default();
                selected_level.level = level_name;
                game_state.set(GameState::MenuLevels);
            }
            _ => {}
        }
    }
}


fn click_clone_button_solution(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<CloneButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    // SelectedLevelSolvedDataEvent event writer:
    mut selected_level_solved_data_event_writer: EventWriter<SelectedLevelSolvedDataEvent>,
    mut redraw_carousel_event_writer: EventWriter<RedrawCarouselEvent>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let new_solution_data = selected_level.player_maps[selected_level.current_index as usize].clone();
                let new_index = selected_level.current_index.clone()as usize + 1;
                selected_level.player_maps.insert(new_index, new_solution_data);
                selected_level_solved_data_event_writer.send(SelectedLevelSolvedDataEvent{data: None});
                redraw_carousel_event_writer.send(RedrawCarouselEvent { maps: Some(selected_level.player_maps.clone()), level_name: selected_level.level.clone(), index: Some(new_index)});

            }
            _ => {}
        }
    }
}

fn click_newsolution_button_solution(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<NewSolutionButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    // SelectedLevelSolvedDataEvent event writer:
    mut selected_level_solved_data_event_writer: EventWriter<SelectedLevelSolvedDataEvent>,
    mut redraw_carousel_event_writer: EventWriter<RedrawCarouselEvent>,
    levels: Res<PuzzlesData>, 
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let empty_map = levels.puzzles.iter().find(|p| p.name == selected_level.level.clone()).unwrap().parsed_map.clone();
                let new_solution_data = SolutionData::new_from_string(empty_map, 0);
                let new_index = selected_level.current_index.clone()as usize + 1;
                selected_level.player_maps.insert(new_index, new_solution_data);
                selected_level_solved_data_event_writer.send(SelectedLevelSolvedDataEvent{data: None});
                redraw_carousel_event_writer.send(RedrawCarouselEvent { maps: Some(selected_level.player_maps.clone()), level_name: selected_level.level.clone(), index: Some(new_index)});

            }
            _ => {}
        }
    }
}

fn click_deletesolution_button_solution(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<DeleteSolutionButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    // SelectedLevelSolvedDataEvent event writer:
    mut selected_level_solved_data_event_writer: EventWriter<SelectedLevelSolvedDataEvent>,
    mut redraw_carousel_event_writer: EventWriter<RedrawCarouselEvent>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selected_level.player_maps.len() > 1 {
                    let index = selected_level.current_index as usize;
                    selected_level.player_maps.remove(index);
                    // let newindex be the min between (index and selected_level.player_maps.len() - 1);
                    let newindex: Option<usize> = if index <= selected_level.player_maps.len() - 1 { Some(index) } else { None};
                    selected_level_solved_data_event_writer.send(SelectedLevelSolvedDataEvent{data: None});
                    redraw_carousel_event_writer.send(RedrawCarouselEvent { maps: Some(selected_level.player_maps.clone()), level_name: selected_level.level.clone(), index: newindex});
                }
            }
            _ => {}
        }
    }
}



fn click_nextlevel_button_solution(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<NextLevelButtonSolutions>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
    mut redraw_carousel_event_writer: EventWriter<RedrawCarouselEvent>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(next_puzzle) = get_next_puzzle(selected_level.level.clone(), &levels) {
                    let level_name = next_puzzle.name.clone();
                    *selected_level = SelectedLevel::default();
                    selected_level.level = level_name.clone();
                    println!("LAUNCHED: {}", level_name.clone());
                    redraw_carousel_event_writer.send(RedrawCarouselEvent { maps: None, level_name: selected_level.level.clone(), index: None});
                    return
                }
            }
            _ => {}
        }
    }
}




fn click_prevlevel_button_solution(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<PrevLevelButton>, With<SolutionsMenuBotton>)>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
    mut redraw_carousel_event_writer: EventWriter<RedrawCarouselEvent>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(prev_puzzle) = get_prev_puzzle(selected_level.level.clone(), &levels) {
                    let level_name = prev_puzzle.name.clone();
                    *selected_level = SelectedLevel::default();
                    selected_level.level = level_name.clone();
                    redraw_carousel_event_writer.send(RedrawCarouselEvent { maps: None, level_name: selected_level.level.clone(), index: None });
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
            println!("UHHH.. Why in finished??");
            let map = selected_level.player_maps[selected_level.current_index as usize].clone();
            selected_level.current_map = map.map;
            state.set(GameState::Playing);
            // Write the event:
            // change_level_writer.send(ChangeLevel);
        }
    }
}


fn make_board_and_title(
    windows: Res<Windows>, 
    mut commands: Commands,
    board_q: Query<Entity, With<Board>>,
    mut carousel_state: ResMut<CarouselState>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>, 
    solution_data_map: Res<SolutionsSavedData>,
    level_name_query: Query<Entity, With<LevelNameElem>>, 
    // best_score_text_query: Query<Entity, With<BestScoreElem>>, 
    mut board_event_writer: EventWriter<BoardEvent>, 
    mut redraw_carousel_event_reader: EventReader<RedrawCarouselEvent>,
    font_assets: Res<FontAssets>, 
    button_colors: Res<ButtonColors>, 
    texts: Query<Entity, With<CarouselTextNode>>,
) {
    for ev in redraw_carousel_event_reader.iter() {
        // Get the window:
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
        // For button in query:
        for text in texts.iter() { // It's never more than 1, but can very well be 0
            if let Some(id) = commands.get_entity(text) { id.despawn_recursive();};
        }
        // For button in query:
        // for text in texts.iter() { // It's never more than 1, but can very well be 0
        //     if let Some(id) = commands.get_entity(text) { id.despawn_recursive();};
        // }

        // Get the maps:
        let empty_map = levels.puzzles.iter().find(|p| p.name == ev.level_name.clone()).unwrap().parsed_map.clone();
        let empty_map_data = SolutionData::new_from_string(empty_map.clone(), 0);
        let solved_data = solution_data_map.get(&ev.level_name);
        let maps = match &ev.maps {
            Some(maps) => maps.clone(),
            None => {
                if selected_level.player_maps.len() != 0 {selected_level.player_maps.clone()} else if solved_data.len() == 0 { vec![empty_map_data] }  else { solved_data }
            }
        };
        let index = match &ev.index {
            Some(index) => *index as u16,
            None => maps.len() as u16 - 1,
        };
        *selected_level = SelectedLevel{
            level: ev.level_name.clone(),
            player_maps: maps.clone(),
            current_index: index,
            current_map: maps[index as usize].map.clone(),
            vanilla_map: empty_map,
            city: "".to_string(),
        };
        // print index:
        println!("Index: {}", selected_level.current_index);

        carousel_state.timer = Timer::new(Duration::from_millis(500), TimerMode::Once);
        carousel_state.position_delta = Vec3::new(width * 0.6, 0., 0.);
        carousel_state.position_offset = Vec3::new((1.-SCALE) * width/2. * 1.5  -width * 0.61, - width * SCALE / 2. + 25., 0.);


        // Spawn the level name BUTTON:
        let (_, (left_, right_, bottom_, top_), _) = get_upper_coordinates(&windows);
        let name_id = make_text(ev.level_name.clone(), &mut commands, &font_assets, &button_colors, 20., left_, right_, top_, bottom_, SolutionsMenuBotton, Some(LevelNameElem));

        // Spawn the "pick solution" text:
        let text_id = make_text("PICK A SOLUTION".to_string(), &mut commands, &font_assets, &button_colors, 20., left_, right_, top_  + width * SCALE * 1.5 + 50., bottom_  + width * SCALE * 1.5 + 50., SolutionsMenuBotton, Some(BestScoreElem));


        // Set the name of the game:
        // Get the solved maps, if there are any:
        // let n_maps = maps.len();
        for (i, map_data) in selected_level.player_maps.iter().enumerate() {
            let ii = - (selected_level.current_index as i16) + i as i16;
            let pos = carousel_state.position_offset + carousel_state.position_delta * ii as f32;
            let boardpos = Some(BoardPosition::Custom(pos));
            board_event_writer.send(BoardEvent::Make{map_name: ev.level_name.clone(), map: map_data.map.clone(), scale: SCALE, position: boardpos, index: Some(i as u32)});

            // Make the text:
            let duration = if map_data.time == 0 {String::from("Unsolved")} else {format!("steps: {}", map_data.time)};
            let text = format!("{} / {}  ({})", map_data.tracks, map_data.second_tracks, duration);
            make_text(text, &mut commands, &font_assets, &button_colors, 20., left_ + carousel_state.position_delta.x * ii as f32, right_ + carousel_state.position_delta.x * ii as f32, top_+75., bottom_+75., SolutionsMenuBotton, Some(CarouselTextNode));
        }
    }
}    



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

const SCALE: f32 = 0.5;

fn _scroll_event_solution(
        v: f32,
        carousel_state: &mut ResMut<CarouselState>,
        mut selected_level: &mut ResMut<SelectedLevel>,
        board_q: &Query<(Entity, &Transform), With<Board>>,
        textnode_q: &Query<(Entity, &Transform, &Style), With<CarouselTextNode>>,
        windows: &Res<Windows>,
        commands: &mut Commands) {
    if v<0. && carousel_state.timer.finished() && selected_level.current_index < selected_level.player_maps.len() as u16 - 1 {
        _start_animation(true, board_q, textnode_q, windows, commands, carousel_state);
        selected_level.current_index += 1;
        selected_level.current_map = selected_level.player_maps[selected_level.current_index as usize].map.clone();
    } else if v>0. && carousel_state.timer.finished() && selected_level.current_index > 0
    {
        _start_animation(false, board_q, textnode_q, windows, commands, carousel_state);
        selected_level.current_index -= 1;
        selected_level.current_map = selected_level.player_maps[selected_level.current_index as usize].map.clone();
    }
}

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
    let bottom = height / 2. + width / 2. - 1.5 * margin +3.;
    let top = height / 2. + width / 2. - 1.5 * margin + button_height +3.;
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




fn get_sol_commands_coordinates(windows: &Windows) -> ((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32)) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    // Genius plan: I'll assume THE BOARD IS ALWAYS ABOUT AS WIDE AS THE SCREEN, AND ALSO SQUARE.
    // Boundaries (left right top bottom) of a Rectangle that occupies the RIGHT HALF of the screen, minus a 20 pixel wide margin all around:
    let margin = 7.;
    let button_height = 40.;
    let percent_left_right = 0.3;
    let left = margin;
    let right = width * percent_left_right + margin/2.;
    // Make the button 40 px high FROM THE BOTTOM:
    let bottom = height / 2. + width / 2. - 1.5 * margin  - button_height - margin + 3.;
    let top = height / 2. + width / 2. - 1.5 * margin + button_height - button_height - margin + 3.;
    return ((left, right - margin, bottom, top), (right, width - right, bottom, top), (width - right + margin, width - left, bottom, top));
}

