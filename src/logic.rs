use bevy::prelude::*;
use crate::loading::TrainAssets;

#[derive(Component)]
pub struct Player;

use crate::simulator::*;

use bevy::utils::Instant;
// use std::time::Instant;

use crate::utils::Coordinates;
use crate::board::*;

use crate::game_screen::GameScreenState;

use crate::tile::TileSpawnEvent;
use crate::train::make_train;

use crate::menu_utils::ScrollBarLimits;



////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug)]
pub struct TicksInATick {
    pub ticks: u32,
    pub current_tick: u32,
    pub first_half: bool,
    pub locked_waiting_for_tick_event: bool,
}
pub fn get_ticks_in_a_tick_default() -> TicksInATick {
    TicksInATick {
        ticks: 100,
        current_tick: 0,
        first_half: true,
        locked_waiting_for_tick_event: false,
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
pub enum LogicTickEvent {
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

#[derive(Debug, Clone)]
pub enum RunEvent{
    Start,
    Stop,
}


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////
/// 


pub fn change_tick_speed(
    // Liste to events of type ScrollBarLimits:
    mut scroll_bar_limits_event_reader: EventReader<ScrollBarLimits>,
    mut tick_status: ResMut<TicksInATick>,
){
    // Iter events:
    for scroll_bar_limits_event in scroll_bar_limits_event_reader.iter() {
        // Find the fratction of the tick currently elapsed:
        let fraction = tick_status.current_tick as f32 / tick_status.ticks as f32;
        tick_status.ticks = ((1. / scroll_bar_limits_event.current) as u32).max(3);
        // Set the current tick to the same fraction of the new tick count:
        tick_status.current_tick = (tick_status.ticks as f32 * fraction) as u32;
        println!("Tick speed changed to {}", tick_status.ticks);
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
        mut board_q: Query<(&BoardDimensions, &mut BoardHoverable, &mut BoardTileMap), With<Board>>, 
        mut hover_event: EventReader<TileHoverEvent>,
        mut spawn_event: EventWriter<TileSpawnEvent>,
    ) {
    for ev in hover_event.iter() {
        // Match the 2 types of event:
        match ev {
            TileHoverEvent::Newhover(pos) => {
                for (board_dimensions, mut hoverable, mut board_tile_map) in board_q.iter_mut() { // It's never more than 1, but can very well be 0
                    let pos = hovered_tile(board_dimensions, *pos);
                    let pos = match pos { None => continue, Some(b) => b, };
                    match &hoverable.hovering_state {
                        HoveringState::Drawing => {
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
                                // Print it: 
                                if new_tile != old_tile {
                                    board_tile_map.map[p_central.y as usize][p_central.x as usize] = new_tile;
                                    let event = TileSpawnEvent{x: p_central.x as usize, y: p_central.y as usize, new_tile, prev_tile: Some(old_tile)};
                                    spawn_event.send(event.clone());
                                    hoverable.history.push(event);
                                }
                            }
                            else if hoverable.hovered_pos_1.is_none() {hoverable.hovered_pos_1 = Some(pos); }
                            else if hoverable.hovered_pos_2.is_none() && hoverable.hovered_pos_1.unwrap() != pos {hoverable.hovered_pos_2 = Some(pos); }
                        },
                        HoveringState::Erasing => {
                            let p_new = pos;
                            let old_tile = board_tile_map.map[p_new.y as usize][p_new.x as usize];
                            let new_tile = Tile::EmptyTile;
                            if new_tile != old_tile {
                                board_tile_map.map[p_new.y as usize][p_new.x as usize] = new_tile;
                                let event = TileSpawnEvent{x: p_new.x as usize, y: p_new.y as usize, new_tile, prev_tile: Some(old_tile)};
                                spawn_event.send(event.clone());
                                hoverable.history.push(event);
                            }
                        },
                        HoveringState::Running => {},
                    }
                }
            },
            TileHoverEvent::Released => {
                for (_, mut hoverable,  _) in board_q.iter_mut() {
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
    mut board_q: Query<(&BoardDimensions, &BoardTileMap, &mut BoardHoverable), With<Board>>, 
    mut event_reader: EventReader<DoubleClickEvent>,
    mut spawn_event: EventWriter<TileSpawnEvent>
) {
    for (board_dimensions, board_tile_map, mut board_hoverable) in board_q.iter_mut() { // It's never more than 1, but can very well be 0
        for ev in event_reader.iter() {
            let window = windows.get_primary().expect("no primary window");
            let window_size = Vec2::new(window.width(), window.height());
            let pos = ev.pos - window_size / 2.;
            if board_hoverable.hovering_state != HoveringState::Drawing {continue;}
            let pos = hovered_tile(board_dimensions, pos);
            // println!("  >>CLICKED {:?}", pos);
            let pos = match pos { None => break, Some(b) => b, };
            let tile = board_tile_map.map[pos.y as usize][pos.x as usize];
            // println!("  >>Old tile: {:?}", tile);
            let newtile = get_new_tile_from_flipping(tile);
            // println!("  >>FLIPPED {:?}", newtile);
            if let Some(tile_) = newtile {
                let event = TileSpawnEvent { x: pos.x as usize, y: pos.y as usize, new_tile: tile_, prev_tile: Some(tile) };
                spawn_event.send(event.clone());
                board_hoverable.history.push(event);

            }
        }
    }
}


pub fn listen_to_game_run_events(
    mut commands: Commands,
    mut board_q: Query<(Entity, &BoardDimensions, &mut BoardTileMap,  &mut BoardHoverable), With<Board>>,
    mut trains_q: Query<(Entity, &Train)>,
    mut tick_status: ResMut<TicksInATick>,
    mut evt: EventReader<RunEvent>,
    mut spawn_event: EventWriter<TileSpawnEvent>,
    //  LogicTickEvent Writer:
    mut logic_tick_event_reader: EventWriter<LogicTickEvent>,

) {
    for (board_id, board_dimensions, mut board_tilemap, mut board_hoverable) in board_q.iter_mut() {
        for trigger_event in evt.iter() {
            match trigger_event {
                RunEvent::Start => {
                    // If hoversble state is already Running, continue:
                    if let HoveringState::Running = board_hoverable.hovering_state {continue;}
                    // Despawn all trains sprites: (ACTUALLY THERE SHOULD BE NONE)
                    for (train_id, _) in trains_q.iter_mut() {
                        commands.entity(train_id).despawn_recursive();
                    }
                    // Set solved_tilemap  to a clone of the current tilemap:
                    board_tilemap.solved_map = Some(board_tilemap.map.clone());
                    // Set tick state to 0:
                    tick_status.current_tick = 0;
                    tick_status.first_half = true;
                    tick_status.locked_waiting_for_tick_event = true;
                    // Go on and fire a LogicTickEvent immediatly:
                    logic_tick_event_reader.send(LogicTickEvent::TickBegin);
                    // Set the board to Running:
                    board_hoverable.hovering_state = HoveringState::Running;
                },
                RunEvent::Stop => {
                    // If hoversble state is already Erasing OR Drawing, continue:
                    if let HoveringState::Erasing = board_hoverable.hovering_state {continue;}
                    if let HoveringState::Drawing = board_hoverable.hovering_state {continue;}

                    // Despawn all trains sprites: 
                    for (train_id, _) in trains_q.iter_mut() {
                        commands.entity(train_id).despawn_recursive();
                    }
                    // RESET the board to Solved_map if it exists:
                    if let Some(solved_map) = board_tilemap.solved_map.clone() {
                        for (y, line) in solved_map.iter().enumerate() {
                            for (x, tile) in line.iter().enumerate() {
                                if tile != &board_tilemap.map[y][x] {
                                    spawn_event.send(TileSpawnEvent { x, y, new_tile: *tile, prev_tile: Some(board_tilemap.map[y][x]) });
                                }
                            }
                        }
                    }
                    // Set the board to Drawing:
                    board_hoverable.hovering_state = HoveringState::Drawing;
                },
            }
        }
    }
}








pub fn logic_tick_event(
    mut commands: Commands,
    train_assets: Res<TrainAssets>,
    mut board_q: Query<(Entity, &BoardDimensions, &BoardTileMap), With<Board>>,
    trains_q: Query<(Entity, &Train)>,
    mut tick_status: ResMut<TicksInATick>,
    mut evt: EventReader<LogicTickEvent>,
    mut spawn_event: EventWriter<TileSpawnEvent>,
    //GameScreenState resource:
    mut game_playing_state: ResMut<GameScreenState>,
) {
    for (board_id, board_dimensions, board_tilemap) in board_q.iter_mut() {
        for trigger_event in evt.iter() {
        // if board is not None:
        
        // Despawn all trains sprites and save the train in current_trains: 
        let mut current_trains: Vec<Train> = Vec::new();
        for (train_entity, train) in trains_q.iter() {
            let mut board_entity = commands.entity(board_id);  // Get entity by id:
            current_trains.push(*train);
            board_entity.remove_children(&[train_entity]);
            commands.entity(train_entity).despawn_recursive();
        }

        let mut crashed;
        let mut  completed;
        let mut new_tilemap: Vec<Vec<Tile>>;
        let mut new_trains: Vec<Train>;
        (new_tilemap, new_trains) = (board_tilemap.map.clone(), current_trains);

        if *trigger_event == LogicTickEvent::TickEnd  || *trigger_event == LogicTickEvent::TickBegin {
            (new_tilemap, new_trains) = go_to_towards_side(new_trains, new_tilemap);
            (new_tilemap, new_trains) = add_beginnings(new_trains, new_tilemap);
            (new_tilemap, new_trains) = flip_exchanges(new_trains, new_tilemap);
            (new_tilemap, new_trains) = check_merges(new_trains, new_tilemap);
            (new_tilemap, new_trains) = check_border_collisions(new_trains, new_tilemap);
            (crashed, completed, new_tilemap, new_trains) = check_arrived_or_crashed(new_trains, new_tilemap);
            (new_tilemap, new_trains) = set_towards_side(new_trains, new_tilemap);
            
            // pretty_print_map(&new_tilemap);
            // Send an event to spawn all changed tiles:
            for (y, line) in new_tilemap.iter().enumerate() {
                for (x, tile) in line.iter().enumerate() {
                    if tile != &board_tilemap.map[y][x] {
                        spawn_event.send(TileSpawnEvent { x, y, new_tile: *tile, prev_tile: Some(board_tilemap.map[y][x]) });
                    }
                }
            }

            // If there is a crash or a completion, set the state:
            if crashed {
                game_playing_state.state.crashed = true;
            }
            if completed {
                game_playing_state.state.won = true;
            }
        }
        else if *trigger_event == LogicTickEvent::TickMiddle {
            (new_tilemap, new_trains) = check_center_colliding(new_trains, new_tilemap);
            (new_tilemap, new_trains) = do_center_coloring_things(new_trains, new_tilemap);
            // println!("");
            // println!("");
            // println!("");
            // println!("");
            // println!("");
        }
        else{
            panic!("Unknown LogicTickEvent: for now we dont use {:?}", trigger_event);
        }
        
        // spawnn all trains:
        for train in new_trains {
            let child_id = make_train(train, &mut commands, &train_assets, &board_dimensions, tick_status.current_tick as f32 / tick_status.ticks as f32);
            let mut board_entity = commands.entity(board_id);  // Get entity by id:
            board_entity.push_children(&[child_id]);// add the child to the parent
        };

        break; // Never ever 2 logic ticks should happen sincronously anyway
    }
    break; // Never ever 2 logic ticks should happen sincronously anyway
}
tick_status.locked_waiting_for_tick_event = false;
}




/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////



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

