

use bevy::prelude::*;

use crate::simulator::{Tile, parse_map};
use crate::utils::Coordinates;

use crate::tile::{TileSpawnData, make_tile};

use crate::simulator::Train;

use crate::all_puzzles_clean::*;
use crate::logic::TicksInATick;

use crate::loading::{TrainAssets, TileAssets};
use crate::train::make_train;


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Clone)]
pub enum TileSize {
    // Tile size options
    Fixed(f32), // Fixed tile size
    Adaptive,   // Window adaptative tile size
}
impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive
    }
}


#[derive(Debug, Clone)]
pub enum BoardPosition {
    // Board position customization options
    Centered { offset: Vec3 }, // Centered board
    Custom(Vec3),              // Custom position
}
impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: default(),
        }
    }
}

// #[derive(Debug, Resource, Serialize, Deserialize)]
#[derive(Resource)]
pub struct BoardOptionsDefault {
    // We use serde to allow saving option presets and loading them at runtime
    pub map_size: (u16, u16),    // Tile map size
    pub position: BoardPosition, // Baard world position
    pub tile_size: TileSize,     // Tile world size
}
pub fn get_board_option_default() -> BoardOptionsDefault {
    BoardOptionsDefault {
        map_size: (7, 7),
        // tile_size: TileSize::Adaptive,
        tile_size: TileSize::Fixed(46.),
        position: BoardPosition::Centered {
            offset: Vec3::new(0., 25., 0.),
        },
    }
}

#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Rect {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}
pub fn in_bounds(point: Vec2, rect: Rect) -> bool {
    point.x > rect.left && point.x < rect.right && point.y > rect.top && point.y < rect.bottom
}

#[derive(Debug, Component)]
pub struct Board;
#[derive(Debug, Component)]
pub struct BoardTileMap {
    pub map: Vec<Vec<Tile>>,
    pub submitted_map: Vec<Vec<Tile>>,
    pub map_name: String,
    pub current_trains: Vec<Train>
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RunningState {
    // Used to track the hovering_state of the mouse hovering over a tile
    Started,
    Won,
    Crashed,
}
#[derive(Debug, PartialEq, Eq, Component, Copy, Clone)]
pub enum BoardGameState {
    // Used to track the hovering_state of the mouse hovering over a tile
    Erasing,
    Drawing,
    Running(RunningState),
}
// Implement default as Drawing:
impl Default for BoardGameState {
    fn default() -> Self {
        Self::Drawing
    }
}
#[derive(Debug, Default)]
pub struct History {
    pub history: Vec<TileSpawnData>,
}
// Implement the Push function:
// When receiving a TileSpawnData, we push it to the history, and REMOVE the oldest one if the history is OVER 50 items
impl History {
    pub fn push(&mut self, item: TileSpawnData) {
        self.history.push(item);
        if self.history.len() > 50 {
            self.history.remove(0);
        }
    }
}
#[derive(Debug, Component)]
pub struct BoardHoverable {
    pub hovered_pos_1: Option<Coordinates>,
    pub hovered_pos_2: Option<Coordinates>,
    pub history: History
}

// Enum First half, second half or None:
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Section {
    First,
    Second,
    NotEvenBegun,  
}

#[derive(Debug, Component)]
pub struct BoardTickStatus {
    pub current_tick: u32,
    pub first_half: Section,
}
// impl default:
impl Default for BoardTickStatus {
    fn default() -> Self {
        Self {
            current_tick: 0,
            first_half: Section::NotEvenBegun,
        }
    }
}
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct BoardDimensions {
    // We use serde to allow saving option presets and loading them at runtime
    pub tile_size: f32, // Tile world size
    pub position: Vec3,
    pub rect: Rect,
}
#[derive(Bundle)]
pub struct BoardBundle {
    pub board: Board,
    pub tile_map: BoardTileMap,
    // pub entities: BoardEntities,
    pub hoverable: BoardHoverable,
    pub options: BoardDimensions,
    pub hovering_state: BoardGameState,
    pub board_tick_status: BoardTickStatus,
    // Flattened SpriteBundle #[bundle] : SO NICE!!
    pub transform: Transform, // This component is required until https://github.com/bevyengine/bevy/pull/2331 is merged
    pub global_transform: GlobalTransform,
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub visibility: Visibility, // User indication of whether an entity is visible
    pub computed_visibility: ComputedVisibility,
}


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum BoardEvent {
    Make(String),
    Delete,
}


// ChangeGameStateEvent:
#[derive(Debug, Clone)]
pub struct ChangeGameStateEvent {
    pub new_state: BoardGameState,
    pub old_state: BoardGameState,
}



/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Clone)]
pub struct PuzzleData {
    pub name: String, 
    pub city: String, 
    pub parsed_map: String, 
    pub type_: String, 
    pub track_count: String, 
}

// System to generate the complete board
pub fn create_board(
    mut commands: Commands,
    board_assets_map: Res<TileAssets>,
    board_options: Res<BoardOptionsDefault>,
    windows: Res<Windows>,
    levels: Res<PuzzlesData>,
    mut board_event_reader: EventReader<BoardEvent>,
) {
    for event in board_event_reader.iter() {
        match event {
            BoardEvent::Make(map_name) => {
            let map = levels.puzzles.iter().find(|p| p.name == *map_name).unwrap().parsed_map.clone();    
            // Split map on '\n':
            let map: Vec<String> = map.split('\n').map(|s| s.to_string()).collect();
            let tile_map: Vec<Vec<Tile>> = parse_map(map);
            let n_width_ = tile_map.len();
            let n_height_ = tile_map.len();
            let tile_size = match board_options.tile_size {
                TileSize::Fixed(v) => v,
                TileSize::Adaptive =>  (
                    windows.get_primary().unwrap().width() / n_width_ as f32).min(
                        windows.get_primary().unwrap().height() / n_height_ as f32) * 0.92
            };
            let board_position = match board_options.position {
                BoardPosition::Centered { offset } => {
                    // offset
                    Vec3::new(-(n_width_ as f32 * tile_size / 2.), -(n_height_ as f32 * tile_size / 2.), 0.) + offset
                }
                BoardPosition::Custom(p) => p,
            };
            // log::info!("board size: {}", board_size);

            // Init BoardDimensions component
            let board_dimensions = BoardDimensions {
                tile_size,
                position: board_position,
                rect: Rect{
                    top: board_position.y,
                    bottom: board_position.y + n_height_ as f32 * tile_size,
                    left: board_position.x,
                    right: board_position.x + n_width_ as f32 * tile_size,
                }
            };
            // Println board_dimensions.position:
            // println!("board_dimensions.position: {:?}", board_dimensions.position);

            // We add the main resource of the game, the board
            let board_entity = commands.spawn(BoardBundle {
                board: Board,
                transform: Transform::from_translation(board_dimensions.position), // This component is required until
                // global_transform: GlobalTransform::default(),
                tile_map: BoardTileMap {
                    map: tile_map.clone(),
                    map_name: map_name.to_string(),
                    submitted_map: tile_map.clone(),
                    current_trains: Vec::new(),
                },
                hoverable: BoardHoverable {
                    hovered_pos_1: None,
                    hovered_pos_2: None,
                    history: History{ ..default()},
                },
                options: board_dimensions,
                sprite: Sprite{
                    color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                },
                hovering_state: BoardGameState::Drawing,
                global_transform: GlobalTransform::default(),
                texture: default(),
                visibility: default(),
                computed_visibility: default(),
                board_tick_status: default(),
            }).id();

            
            // Send an event to spawn all tiles:
            for (y, line) in tile_map.iter().enumerate() {
                for (x, tile) in line.iter().enumerate() {
                    let coordinates = Coordinates { x: x as u16, y: y as u16,};
                    let child_id = make_tile(*tile, &mut commands, &board_assets_map, board_dimensions.tile_size, coordinates);
                    
                    commands.entity(board_entity).push_children(&[child_id]);// add the child to the parent
                }
            }
        },
        _ => {}
    }
}
}

pub fn cleanup_board(
    mut commands: Commands, 
    board_q: Query<Entity, With<Board>>,
    // Read event:
    mut board_event_reader: EventReader<BoardEvent>,
) {
    for event in board_event_reader.iter() {
        match event {
            BoardEvent::Delete => {
                // Delete all boards:
                for board_id in board_q.iter() {
                    if let Some(entity) = commands.get_entity(board_id) { 
                        entity.despawn_recursive();
                    }
                }
            },
            _ => {}
        }
    }
}


/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////


