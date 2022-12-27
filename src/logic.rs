use bevy::prelude::*;
use crate::loading::TrainAssets;


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

use crate::simulator::*;

use bevy::utils::Instant;
// use std::time::Instant;

use crate::utils::Coordinates;
use crate::board::*;

use crate::tile::TileSpawnEvent;
use crate::train::make_train;


////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


pub struct TicksInATick {
    pub ticks: u32,
    pub is_in_game: bool,
    pub current_tick: u32,
    pub locked_waiting_for_tick_event: bool,
}
pub fn get_ticks_in_a_tick_default() -> TicksInATick {
    TicksInATick {
        ticks: 200,
        is_in_game: false,
        current_tick: 0,
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
    pub pos_x: f32,
    pub pos_y: f32,
}

/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


pub fn check_mouse_action(mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>, mut board_q: Query<(&BoardDimensions, &mut BoardHoverable, &mut BoardTileMap), With<Board>>, mut spawn_event: EventWriter<TileSpawnEvent>,) {
    if mouse_input.pressed(MouseButton::Left) {
        for (board_dimensions, mut hoverable, mut boardTileMap) in board_q.iter_mut() { // It's never more than 1, but can very well be 0
            let window = windows.get_primary().expect("no primary window");
            let pos = hovered_tile(board_dimensions, window, window.cursor_position());
            let pos = match pos { None => continue, Some(b) => b, };
            if hoverable.hovered_pos_1.is_some() && hoverable.hovered_pos_2.is_some() && hoverable.hovered_pos_2.unwrap() != pos {
                let p_old = hoverable.hovered_pos_1.unwrap();
                let p_central = hoverable.hovered_pos_2.unwrap();
                let p_new = pos;
                hoverable.hovered_pos_1 = hoverable.hovered_pos_2;
                hoverable.hovered_pos_2 = Some(p_new);
                let track_option = get_track_option_from_3_coordinates(p_old, p_central, p_new);
                let track_option = match track_option { None => continue, Some(b) => b, };
                let new_tile = get_new_tile_from_track_option(boardTileMap.map[p_central.y as usize][p_central.x as usize], track_option);
                boardTileMap.map[p_central.y as usize][p_central.x as usize] = new_tile;
                spawn_event.send(TileSpawnEvent{x: p_central.x as usize, y: p_central.y as usize, new_tile});
                // print p_central.y and p_central.x:
            }
            else if hoverable.hovered_pos_1.is_none() {hoverable.hovered_pos_1 = Some(pos); }
            else if hoverable.hovered_pos_2.is_none() && hoverable.hovered_pos_1.unwrap() != pos {hoverable.hovered_pos_2 = Some(pos); }
            // println!("CURRENTLY click at {:?}, old tile: {:?}", pos, boardTileMap.map[pos.y as usize][pos.x as usize]);
        }
    }
    else if mouse_input.any_just_released([MouseButton::Left, MouseButton::Right]) {
        for (_, mut hoverable,  _) in board_q.iter_mut() {
            hoverable.hovered_pos_1 = None;
            hoverable.hovered_pos_2 = None;
        }
    }
}



use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;



pub fn double_click_mouse(
    //mouse_input: Res<Input<MouseButton>>, 
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    windows: Res<Windows>, 
    mut double_click_time: Local<DoubleClickInstant>,
    mut board_q: Query<(&BoardDimensions, &mut BoardHoverable, &mut BoardTileMap), With<Board>>, 
    mut eventWriter: EventWriter<DoubleClickEvent>,
    mut spawn_event: EventWriter<TileSpawnEvent>
) {
    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if let Some(double_click_instant) = double_click_time.instant {
                    if double_click_instant.elapsed().as_millis() < 400 && double_click_instant.elapsed().as_millis() > 30 {
                        println!("Time: {}", double_click_instant.elapsed().as_millis());
                        println!("DOUBLE CLICK");
                        for (board_dimensions, mut hoverable, mut boardTileMap) in board_q.iter_mut() { // It's never more than 1, but can very well be 0
                            let window = windows.get_primary().expect("no primary window");
                            let pos = hovered_tile(board_dimensions, window, window.cursor_position());
                            println!("  >>CLICKED {:?}", pos);
                            let pos = match pos { None => break, Some(b) => b, };
                            eventWriter.send(DoubleClickEvent{pos});
                            // println!("CURRENTLY click at {:?}, old tile: {:?}", pos, boardTileMap.map[pos.y as usize][pos.x as usize]);
                            let tile = boardTileMap.map[pos.y as usize][pos.x as usize];
                            println!("  >>Old tile: {:?}", tile);
                            let newtile = get_new_tile_from_flipping(tile);
                            println!("  >>FLIPPED {:?}", newtile);
                            if let Some(tile_) = newtile {
                                spawn_event.send(TileSpawnEvent { x: pos.x as usize, y: pos.y as usize, new_tile: tile_ });

                            }
                        }
                    }
                }
                *double_click_time = DoubleClickInstant{instant: Some(Instant::now())};
                println!("SET CURRENT TIME: {:?}",Instant::now());
        },

        ButtonState::Released => {}
    }
}
}


pub fn logic_tick_event(
    mut commands: Commands,
    train_assets: Res<TrainAssets>,
    mut board_q: Query<(Entity, &BoardDimensions, &mut BoardEntities, &mut BoardTileMap), With<Board>>,
    trains_q: Query<(Entity, &Train)>,
    mut tick_status: ResMut<TicksInATick>,
    mut evt: EventReader<LogicTickEvent>,
    mut spawn_event: EventWriter<TileSpawnEvent>
) {
for trigger_event in evt.iter() {
    for (board_id, board_dimensions, mut board_entities, mut board_tilemap) in board_q.iter_mut() {
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

        if *trigger_event == LogicTickEvent::TickEnd {
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
                        spawn_event.send(TileSpawnEvent { x, y, new_tile: *tile});
                    }
                }
            }
        }
        else if *trigger_event == LogicTickEvent::TickMiddle {
            (new_tilemap, new_trains) = check_center_colliding(new_trains, new_tilemap);
            (new_tilemap, new_trains) = do_center_coloring_things(new_trains, new_tilemap);
            println!("");
            println!("");
            println!("");
            println!("");
            println!("");
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



fn hovered_tile(board: &BoardDimensions, window: &Window, cursor_pos: Option<Vec2>) -> Option<Coordinates> {
    let window_size = Vec2::new(window.width(), window.height());
    let position = cursor_pos? - window_size / 2.;
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
        Tile::TrackTile{toptrack, bottrack} => {
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

