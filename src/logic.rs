
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

use crate::data_saving::{SolutionData, LevelSolvedDataEvent};
use crate::simulator::*;

use bevy::utils::Instant;
// use std::time::Instant;

use crate::utils::{Coordinates, SelectedLevel};
use crate::board::*;


use crate::tile::TileSpawnData;

use crate::menu_utils::ScrollBarLimits;


////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Resource)]
pub struct TicksInATick {
    pub ticks: u32,
}
pub fn get_ticks_in_a_tick_default() -> TicksInATick {
    TicksInATick {
        ticks: 100,
    }
}

#[derive(Default)]
pub struct DoubleClickInstant
{
    pub instant: Option<Instant>,
}


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TickMoment {
    TickBegin,
    TickMiddle,
    TickEnd,
}


#[derive(Debug, Clone, PartialEq)]
pub struct DoubleClickEvent {
    pub pos: Vec2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileHoverEvent {
    Newhover(Vec2),
    Released
}


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////
/// 


pub fn change_tick_speed(
    // Liste to events of type ScrollBarLimits:
    mut scroll_bar_limits_event_reader: EventReader<ScrollBarLimits>,
    mut board_q: Query<&mut BoardTickStatus, With<Board>>,
    mut tick_params: ResMut<TicksInATick>,
){
    // Iter events:
    for scroll_bar_limits_event in scroll_bar_limits_event_reader.iter() {
        let new_nticks = (scroll_bar_limits_event.current as u32).max(4);
        for mut board_tick_status in board_q.iter_mut() {    // Really, there's just 1 board
            // Find the fratction of the tick currently elapsed:
            let fraction = board_tick_status.current_tick as f32 / tick_params.ticks as f32;
            // Set the current tick to the same fraction of the new tick count:
            let current = new_nticks as f32 * fraction;
            // Ceil it:
            board_tick_status.current_tick = current.ceil() as u32;
            // Set the new tick count:
            tick_params.ticks = new_nticks;
            println!("Tick speed changed to {}", tick_params.ticks);
        }
    }
}


pub fn tile_hover_touch(touches: Res<Touches>, windows: Res<Windows>, mut hover_event: EventWriter<TileHoverEvent>,) {
    for finger in touches.iter() {
        if touches.just_released(finger.id()) {
            hover_event.send(TileHoverEvent::Released);
            break;
        }
        else {
            let window = windows.get_primary().expect("no primary window");
            let pos = match window.cursor_position() { None => continue, Some(b) => b, };
            let window_size = Vec2::new(window.width(), window.height());
            let pos = pos - window_size / 2.;            
            hover_event.send(TileHoverEvent::Newhover(pos));
        }
    }
}

pub fn tile_hover_mouse(mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>,mut hover_event: EventWriter<TileHoverEvent>,) {
    if mouse_input.pressed(MouseButton::Left) {
            let window = windows.get_primary().expect("no primary window");
            let pos = match window.cursor_position() { None => return, Some(b) => b, };
            let window_size = Vec2::new(window.width(), window.height());
            let pos = pos - window_size / 2.;            
            hover_event.send(TileHoverEvent::Newhover(pos));
    }
    else if mouse_input.any_just_released([MouseButton::Left, MouseButton::Right]) {
        hover_event.send(TileHoverEvent::Released);
    }
}

pub fn tile_hover_event(
        mut board_q: Query<(&BoardDimensions, &mut BoardHoverable, &mut BoardTileMap, &BoardGameState), With<Board>>, 
        mut hover_event: EventReader<TileHoverEvent>,
    ) {
    for ev in hover_event.iter() {
        // Match the 2 types of event:
        match ev {
            TileHoverEvent::Newhover(pos) => {
                for (board_dimensions, mut hoverable, mut board_tile_map, mut hovering_state) in board_q.iter_mut() { // It's never more than 1, but can very well be 0
                    let pos = hovered_tile(board_dimensions, *pos);
                    let pos = match pos { None => continue, Some(b) => b, };
                    match &hovering_state {
                        BoardGameState::Drawing => {
                            if hoverable.hovered_pos_1.is_some() && hoverable.hovered_pos_2.is_some() && hoverable.hovered_pos_2.unwrap() != pos {
                                let p_old = hoverable.hovered_pos_1.unwrap();
                                let p_central = hoverable.hovered_pos_2.unwrap();
                                let p_new = pos;
                                hoverable.hovered_pos_1 = hoverable.hovered_pos_2;
                                hoverable.hovered_pos_2 = Some(p_new);
                                let track_option = get_track_option_from_3_coordinates(p_old, p_central, p_new);
                                let track_option = match track_option { None => continue, Some(b) => b, };
                                let old_tile = board_tile_map.map[p_central.y as usize][p_central.x as usize];
                                let new_tile = get_new_tile_from_track_option(board_tile_map.map[p_central.y as usize][p_central.x as usize], track_option);
                                if new_tile != old_tile {
                                    // Print it: 
                                    board_tile_map.map[p_central.y as usize][p_central.x as usize] = new_tile;
                                    hoverable.history.push(TileSpawnData{x: p_central.x as usize, y: p_central.y as usize, new_tile, prev_tile: Some(old_tile)});
                                }
                            }
                            else if hoverable.hovered_pos_1.is_none() {hoverable.hovered_pos_1 = Some(pos); }
                            else if hoverable.hovered_pos_2.is_none() && hoverable.hovered_pos_1.unwrap() != pos {hoverable.hovered_pos_2 = Some(pos); }
                        },
                        BoardGameState::Erasing => {
                            let p_new = pos;
                            let old_tile = board_tile_map.map[p_new.y as usize][p_new.x as usize];
                            let new_tile = match old_tile {
                                Tile::SingleTrackTile { track:_ } => Tile::EmptyTile,
                                Tile::TrackTile { toptrack:_, bottrack:_ } => Tile::EmptyTile,
                                _ => old_tile,
                            };
                            if new_tile != old_tile {
                                board_tile_map.map[p_new.y as usize][p_new.x as usize] = new_tile;
                                hoverable.history.push(TileSpawnData{x: p_new.x as usize, y: p_new.y as usize, new_tile, prev_tile: Some(old_tile)});
                            }
                        },
                        BoardGameState::Running(_) => {},
                    }
                }
            },
            TileHoverEvent::Released => {
                for (_, mut hoverable,  _, _) in board_q.iter_mut() {
                    hoverable.hovered_pos_1 = None;
                    hoverable.hovered_pos_2 = None;
                }
            }
        }
    }
}


pub fn double_click_touch(
    touches: Res<Touches>,
    mut double_click_time: Local<DoubleClickInstant>,
    mut event_writer: EventWriter<DoubleClickEvent>,
) {
    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            if let Some(double_click_instant) = double_click_time.instant {
                if double_click_instant.elapsed().as_millis() < 400 && double_click_instant.elapsed().as_millis() > 30 {
                    // println!("DOUBLE CLICK");
                    event_writer.send(DoubleClickEvent{pos: finger.position() });
                }
            }
            *double_click_time = DoubleClickInstant{instant: Some(Instant::now())};
            // println!("SET CURRENT TIME: {:?}",Instant::now());
        }
    }
}

pub fn double_click_mouse(
    mouse_input: Res<Input<MouseButton>>, 
    windows: Res<Windows>, 
    mut double_click_time: Local<DoubleClickInstant>,
    mut event_writer: EventWriter<DoubleClickEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(double_click_instant) = double_click_time.instant {
            if double_click_instant.elapsed().as_millis() < 400 && double_click_instant.elapsed().as_millis() > 30 {
                // println!("DOUBLE CLICK");
                let pos = windows.get_primary().unwrap().cursor_position();
                if let Some(pos) = pos {
                    event_writer.send(DoubleClickEvent{pos: pos});
                }
            }
        }
        *double_click_time = DoubleClickInstant{instant: Some(Instant::now())};
        // println!("SET CURRENT TIME: {:?}",Instant::now());
    }
}

pub fn double_click_event(
    windows: Res<Windows>, 
    mut board_q: Query<(&BoardDimensions, &mut BoardTileMap, &mut BoardHoverable, &BoardGameState), With<Board>>, 
    mut event_reader: EventReader<DoubleClickEvent>,
) {
    for (board_dimensions, mut board_tile_map, mut board_hoverable, hovering_state) in board_q.iter_mut() { // It's never more than 1, but can very well be 0
        for ev in event_reader.iter() {
            let window = windows.get_primary().expect("no primary window");
            let window_size = Vec2::new(window.width(), window.height());
            let pos = ev.pos - window_size / 2.;
            if *hovering_state != BoardGameState::Drawing {continue;}
            let pos = hovered_tile(board_dimensions, pos);
            // println!("  >>CLICKED {:?}", pos);
            let pos = match pos { None => break, Some(b) => b, };
            let tile = board_tile_map.map[pos.y as usize][pos.x as usize];
            // println!("  >>Old tile: {:?}", tile);
            let newtile = get_new_tile_from_flipping(tile);
            // println!("  >>FLIPPED {:?}", newtile);
            if let Some(tile_) = newtile {
                board_tile_map.map[pos.y as usize][pos.x as usize] = tile_;
                board_hoverable.history.push(TileSpawnData { x: pos.x as usize, y: pos.y as usize, new_tile: tile_, prev_tile: Some(tile) });

            }
        }
    }
}


pub fn listen_to_game_state_changes(
    mut board_q: Query<(&mut BoardTileMap, &mut BoardGameState, &mut BoardTickStatus), With<Board>>,
    selected_level_name: Res<SelectedLevel>,
    mut change_board_game_state_event_reader: EventReader<ChangeGameStateEvent>,
    mut level_solved_data_event_writer: EventWriter<LevelSolvedDataEvent>,
) {
    // For each event:
    for ev in change_board_game_state_event_reader.iter() {
        for (mut board_tilemap, mut hovering_state, mut tick_status) in board_q.iter_mut() {
            match *ev {
                ChangeGameStateEvent{old_state: BoardGameState::Running(RunningState::Started), new_state: BoardGameState::Running(RunningState::Won)} => {
                    let solution_data = SolutionData::new_from_tiles(&board_tilemap.submitted_map);
                    level_solved_data_event_writer.send(LevelSolvedDataEvent{level_name: selected_level_name.level.clone(), solution_data: solution_data});

                },
                ChangeGameStateEvent{old_state: _, new_state: BoardGameState::Running(_) }=> {
                    board_tilemap.current_trains = Vec::new();
                    board_tilemap.submitted_map = board_tilemap.map.clone();
                    tick_status.current_tick = 0;
                    tick_status.first_half = Section::NotEvenBegun;
                    *hovering_state = ev.new_state;

                },
                ChangeGameStateEvent{old_state: BoardGameState::Running(_), new_state: _  }=> {
                    board_tilemap.current_trains = Vec::new();
                    // RESET the board to Solved_map if it exists:
                    println!(">>>>>Current trains: {:?}", board_tilemap.current_trains.len());
                    board_tilemap.map = board_tilemap.submitted_map.clone();
                    *hovering_state = ev.new_state;
                },
                _ => {
                    *hovering_state = ev.new_state;
                }
            }
        }
    }
}




pub fn logic_tick(
    // windows: Res<Windows>,
    mut board_q: Query<(&mut BoardTileMap, &mut BoardGameState, &mut BoardTickStatus), With<Board>>,
    tick_params: ResMut<TicksInATick>,
    mut change_gamestate_event_writer: EventWriter<ChangeGameStateEvent>,
    ) {
        
    for (mut board_tilemap, mut game_state, mut tick_status) in board_q.iter_mut() {    // Really, there's just 1 board
        // If board_hoverable.game_state is NOT running, continue:
        match *game_state { BoardGameState::Running(_) => {}, _ => {continue;}}
        if (tick_status.current_tick >= tick_params.ticks -1 && tick_status.first_half == Section::Second) || (tick_status.first_half == Section::NotEvenBegun && tick_status.current_tick == 0) {
            (board_tilemap.map, board_tilemap.current_trains) = logic_tick_core(&board_tilemap, TickMoment::TickEnd, &mut game_state, &mut change_gamestate_event_writer).clone();
            tick_status.current_tick = 0;
            // println!("Tick now 0");
            tick_status.first_half = Section::First;
        } else if tick_status.current_tick >= ((tick_params.ticks as f32 / 2.) as u32)  && tick_status.first_half == Section::First {
            tick_status.first_half = Section::Second;
            (board_tilemap.map, board_tilemap.current_trains) = logic_tick_core(&mut board_tilemap, TickMoment::TickMiddle, &mut game_state, &mut change_gamestate_event_writer);
        }
        tick_status.current_tick += 1;
    }
}


/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////


pub fn logic_tick_core(
        board_tilemap: &BoardTileMap, 
        trigger_event: TickMoment, 
        mut hovering_state: &mut BoardGameState,
        mut change_gamestate_event_writer: &mut EventWriter<ChangeGameStateEvent>,
    ) -> (Vec<Vec<Tile>>, Vec<Train>){ 

    let crashed;
    let completed;
    let mut new_tilemap: Vec<Vec<Tile>>;
    let mut new_trains: Vec<Train>;
    
    (new_tilemap, new_trains) = (board_tilemap.map.clone(), board_tilemap.current_trains.clone());
    
    if trigger_event == TickMoment::TickEnd  || trigger_event == TickMoment::TickBegin {
        (new_tilemap, new_trains) = go_to_towards_side(new_trains, new_tilemap);
        (new_tilemap, new_trains) = add_beginnings(new_trains, new_tilemap);
        (new_tilemap, new_trains) = flip_exchanges(new_trains, new_tilemap);
        (new_tilemap, new_trains) = check_merges(new_trains, new_tilemap);
        (new_tilemap, new_trains) = check_border_collisions(new_trains, new_tilemap);
        (crashed, completed, new_tilemap, new_trains) = check_arrived_or_crashed(new_trains, new_tilemap);
        (new_tilemap, new_trains) = set_towards_side(new_trains, new_tilemap);

        // If there is a crash or a completion, set the state:
        if crashed && (*hovering_state != BoardGameState::Running(RunningState::Crashed)) {  // This is bc res mut trigger is fired always
            *hovering_state = BoardGameState::Running(RunningState::Crashed);
        }
        else if completed && (*hovering_state != BoardGameState::Running(RunningState::Won) && (*hovering_state != BoardGameState::Running(RunningState::Crashed))) {
            change_gamestate_event_writer.send(ChangeGameStateEvent{old_state: *hovering_state, new_state: BoardGameState::Running(RunningState::Won)});  // Here I'm ASSUMING 
            *hovering_state = BoardGameState::Running(RunningState::Won);
        }
    }
    else if trigger_event == TickMoment::TickMiddle {
        (new_tilemap, new_trains) = check_center_colliding(new_trains, new_tilemap);
        (new_tilemap, new_trains) = do_center_coloring_things(new_trains, new_tilemap);
    }
    else{
        panic!("Unknown RedrawEvent: for now we dont use {:?}", trigger_event);
    }
    return (new_tilemap, new_trains)

}



fn hovered_tile(board: &BoardDimensions, position: Vec2) -> Option<Coordinates> {
    if !in_bounds(position, board.rect) {return None;}
    // Get vec2 with x and y out of the vec3 position:
    let boardpospos: Vec2 = Vec2::new(board.position.x, board.position.y);
    let coordinates = position - boardpospos;
    let coords = Coordinates {
        x: (coordinates.x / board.tile_size) as u16,
        y: 6 - ((coordinates.y / board.tile_size) as u16),
    };

    Some(coords)
}

fn get_track_option_from_3_coordinates(p_before: Coordinates, p_central: Coordinates, p_after: Coordinates) -> Option<TrackOptions> {
    let delta_before = (p_central.x as i8 - p_before.x as i8  , (p_central.y as i8) - p_before.y as i8);
    let delta_after = (p_central.x as i8 - p_after.x as i8, (p_central.y as i8) - p_after.y as i8);
    // get the sides where the deltas point to:
    let side_before = match delta_before {
        (0, 1) => Side::T_, (0, -1) => Side::B_, (1, 0) => Side::L_, (-1, 0) => Side::R_,
        _ => return None,
    };
    let side_after = match delta_after {
        (0, 1) => Side::T_, (0, -1) => Side::B_, (1, 0) => Side::L_, (-1, 0) => Side::R_,
        _ => return None,
    };
    // get the track option from the sides:
    match (side_before, side_after) {
        (Side::T_, Side::L_) => Some(TrackOptions::TL), (Side::T_, Side::B_) => Some(TrackOptions::TB), (Side::T_, Side::R_) => Some(TrackOptions::TR), (Side::L_, Side::B_) => Some(TrackOptions::LB), (Side::L_, Side::R_) => Some(TrackOptions::LR), (Side::B_, Side::R_) => Some(TrackOptions::BR), 
        (Side::L_, Side::T_) => Some(TrackOptions::TL), (Side::B_, Side::T_) => Some(TrackOptions::TB), (Side::R_, Side::T_) => Some(TrackOptions::TR), (Side::B_, Side::L_) => Some(TrackOptions::LB), (Side::R_, Side::L_) => Some(TrackOptions::LR), (Side::R_, Side::B_) => Some(TrackOptions::BR),
        _ => return None,
    }
}


fn get_new_tile_from_track_option(old_tile: Tile, new_track_option: TrackOptions) -> Tile {
    // Move the old toptrack to bottom track, and add a new toptrack from the new track option.
    // If the new track option is the same as the old toptrack, then the tile becomes a SingleTrackTile.
    match old_tile {
        Tile::TrackTile{toptrack, bottrack: _} => {
            if new_track_option == get_track_option(toptrack) { Tile::SingleTrackTile{track: toptrack}}
            else { Tile::TrackTile{toptrack: get_track(new_track_option), bottrack: toptrack}}
        },
        Tile::SingleTrackTile{track} => {
            if new_track_option == get_track_option(track) { Tile::SingleTrackTile{track}}
            else { Tile::TrackTile{toptrack: get_track(new_track_option), bottrack: track}}
        },
        Tile::EmptyTile => Tile::SingleTrackTile{track: get_track(new_track_option)},
        _ => {old_tile}
    }
}

fn get_new_tile_from_flipping(old_tile: Tile) -> Option<Tile> {
    match old_tile {
        Tile::TrackTile{toptrack, bottrack} => {
            Some(Tile::TrackTile{toptrack: bottrack, bottrack: toptrack})
        },
        _ => {None}
    }
}

