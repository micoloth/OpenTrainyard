

use bevy::prelude::*;

use crate::simulator::{Tile, parse_map};
use crate::utils::Coordinates;

use crate::tile::TileSpawnEvent;

use crate::logic::TicksInATick;

use std::collections::HashMap;
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
pub struct BoardOptionsDefault {
    // We use serde to allow saving option presets and loading them at runtime
    pub map_size: (u16, u16),    // Tile map size
    pub position: BoardPosition, // Baard world position
    pub tile_size: TileSize,     // Tile world size
}
pub fn get_board_option_default() -> BoardOptionsDefault {
    BoardOptionsDefault {
        map_size: (7, 7),
        tile_size: TileSize::Adaptive,
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
}
#[derive(Debug, Component)]
pub struct BoardEntities {
    pub tiles: HashMap<Coordinates, Entity>,
}
#[derive(Debug, Component)]
pub struct BoardHoverable {
    pub hovered_pos_1: Option<Coordinates>,
    pub hovered_pos_2: Option<Coordinates>,
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
    pub name: Name,
    pub transform: Transform, // This component is required until https://github.com/bevyengine/bevy/pull/2331 is merged
    pub global_transform: GlobalTransform,
    pub tile_map: BoardTileMap,
    pub entities: BoardEntities,
    pub hoverable: BoardHoverable,
    pub options: BoardDimensions,
    // Flattened SpriteBundle #[bundle] : SO NICE!!
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub visibility: Visibility, // User indication of whether an entity is visible
    pub computed_visibility: ComputedVisibility,
}


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


// System to generate the complete board
pub fn create_board(
    mut commands: Commands,
    board_options: Res<BoardOptionsDefault>,
    windows: Res<Windows>,
    mut tick_status: ResMut<TicksInATick>,
    mut spawn_event: EventWriter<TileSpawnEvent>,
) {
    
    let map_s = vec![
        "00 00 00 E0100_g 00 00 00".to_string(),
        "00 00 00 02 00 00 00".to_string(),
        "00 00 06 01 00 00 00".to_string(),
        "Sr_b 05 53 45 05 05 E0010_g".to_string(),
        "00 00 00 23 00 00 00".to_string(),
        "00 00 00 St_y 00 00 00".to_string(),
        "00 00 00 00 00 00 00".to_string(),
        ];
    let tile_map: Vec<Vec<Tile>> = parse_map(map_s);
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
    commands.spawn_bundle(BoardBundle {
        board: Board,
        name: Name::new("Board"),
        transform: Transform::from_translation(board_dimensions.position), // This component is required until
        // global_transform: GlobalTransform::default(),
        tile_map: BoardTileMap {
            map: tile_map.clone(),
        },
        entities: BoardEntities {
            tiles: HashMap::new(),
        },
        hoverable: BoardHoverable {
            hovered_pos_1: None,
            hovered_pos_2: None,
        },
        options: board_dimensions,
        sprite: Sprite{
            color: Color::rgb(0.5, 0.5, 0.5),
            ..default()
        },
        global_transform: GlobalTransform::default(),
        texture: default(),
        visibility: default(),
        computed_visibility: default(),
        
    });

    // Launch event to spawn each tile
    for (y, line) in tile_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            spawn_event.send(TileSpawnEvent {
                x,
                y,
                new_tile: *tile,
            });
        }
    }

    tick_status.is_in_game = true;
    tick_status.current_tick = 1000;
}

pub fn cleanup_board(mut commands: Commands, board_q: Query<Entity, With<Board>>) {
    let board_id = board_q.single();
    commands.entity(board_id).despawn_recursive();
}