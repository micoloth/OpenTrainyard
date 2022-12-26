// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, NonSend, WindowDescriptor};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
// use trainyard::GamePlugin;
use bevy::ecs::system::EntityCommands;
use std::io::Cursor;
use winit::window::Icon;

mod audio;
mod loading;
mod menu;
mod utils;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::utils::ActionsPlugin;

use crate::loading::TextureAssets;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

mod simulator;
use simulator::*;

mod tests;
use tests::test;

use crate::utils::Actions;

use std::ops::{Add, Sub};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl From<(u16, u16)> for Coordinates {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(i16, i16)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i16, i16)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u16;
        let y = ((self.y as i16) + y as i16) as u16;
        Self { x, y }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

use partial_application::partial;

/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Bundle, Default)]
pub struct TileSpriteBundle {
    pub name: Name,               // Tile name
    pub coordinates: Coordinates, // Tile coordinates

    // Flattened SpriteBundle #[bundle] : SO NICE!!
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility, // User indication of whether an entity is visible
    pub computed_visibility: ComputedVisibility,
}


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
            offset: Default::default(),
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
fn get_board_option_default() -> BoardOptionsDefault {
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
struct BoardBundle {
    board: Board,
    name: Name,
    transform: Transform, // This component is required until https://github.com/bevyengine/bevy/pull/2331 is merged
    global_transform: GlobalTransform,
    tile_map: BoardTileMap,
    entities: BoardEntities,
    hoverable: BoardHoverable,
    options: BoardDimensions,
    // Flattened SpriteBundle #[bundle] : SO NICE!!
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub visibility: Visibility, // User indication of whether an entity is visible
    pub computed_visibility: ComputedVisibility,
}

use std::collections::HashMap;



pub struct TicksInATick {
    pub ticks: u32,
    pub is_in_game: bool,
    pub current_tick: u32,
    pub locked_waiting_for_tick_event: bool,
}
fn get_ticks_in_a_tick_default() -> TicksInATick {
    TicksInATick {
        ticks: 200,
        is_in_game: false,
        current_tick: 0,
        locked_waiting_for_tick_event: false,
    }
}


// Button action type
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)] 
pub enum ButtonAction {Clear,Generate}

#[derive(Debug)] pub struct ButtonColors {pub normal: Color,pub hovered: Color,pub pressed: Color}
#[derive(Debug, Clone, Eq, PartialEq, Hash)] pub enum AppState {InGame, Out}

/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct TileSpawnEvent {
    x: usize,
    y: usize,
    new_tile: Tile,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicTickEvent {
    TickBegin,
    TickMiddle,
    TickEnd,
}


/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

pub fn get_poss_minitile_1() -> Vec<usize> {
    vec![11]
}
pub fn get_poss_minitile_4() -> Vec<usize> {
    vec![11, 25]
}
pub fn get_poss_minitile_9() -> Vec<usize> {
    vec![11, 20, 29]
}

pub fn get_dimension_minitile_1() -> u32 {
    27
}
pub fn get_dimension_minitile_4() -> u32 {
    12
}
pub fn get_dimension_minitile_9() -> u32 {
    8
}

pub fn flipmatrix_vertical(mut t: Transform) -> Transform {
    t = t.clone();
    t.scale.y *= -1.;
    return t;
}
pub fn flipmatrix_horizontal(mut t: Transform) -> Transform {
    t.scale.x *= -1.;
    return t;
}
pub fn rotate_tile(mut t: Transform, angle: f32) -> Transform {
    t.rotate(Quat::from_rotation_z(angle)); // std::f32::consts::PI / 2.
    return t;
}
pub fn rotate_tile_90(mut t: Transform, times: i16) -> Transform {
    t.rotate(Quat::from_rotation_z(
        std::f32::consts::PI / 2. * times as f32,
    )); // std::f32::consts::PI / 2.
    return t;
}

pub fn add_color_minitiles_children(
    child_cmd: &mut ChildBuilder,
    elems: VectorOfColorz,
    is_start: bool,
    assets: &HashMap<String, Handle<Image>>,
    big_tile_size: f32,
) {
    let scale = big_tile_size / 46.;
    let (n, poss, small_tile_size) = if elems.len() == 1 {
        (1, get_poss_minitile_1(), get_dimension_minitile_1())
    } else if elems.len() <= 4 {
        (4, get_poss_minitile_4(), get_dimension_minitile_4())
    } else if elems.len() <= 9 {
        (9, get_poss_minitile_9(), get_dimension_minitile_9())
    } else {
        panic!("Too many elements in StartTile");
    };
    for (i, y) in poss.iter().enumerate() {
        for (j, x) in poss.iter().enumerate() {
            let n_to_get = i * poss.len() + j;
            if n_to_get < elems.len() {
                let pos_x = -(23. - (*x as f32) - (small_tile_size as f32) / 2.) * scale;
                let pos_y = (23. - (*y as f32) - (small_tile_size as f32) / 2.) * scale;
                let prefix = if is_start { "s" } else { "e" };
                let minitile = format!(
                    "{}_elem_{}_{}.png",
                    prefix,
                    n,
                    colorz_to_long_str(elems.v[n_to_get].unwrap())
                );
                let child_asset = assets.get(&minitile).unwrap();
                child_cmd.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(small_tile_size as f32)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos_x, pos_y, 1.),
                    texture: (*child_asset).clone(),
                    ..Default::default()
                });
            } else {
                break;
            }
        }
    }
}

fn get_transform_and_texture(
    t: Tile,
    assets: &HashMap<String, Handle<Image>>,
) -> (Handle<Image>, Transform) {
    let mut transform = Transform::from_xyz(0., 0., 1.);
    let texture_path: String;
    // Print the tile:
    let (texture_path, transform): (String, Transform) = match t {
        Tile::SingleTrackTile { track: _ }
        | Tile::TrackTile {
            toptrack: _,
            bottrack: _,
        } => {
            match &print_tile(&t)[..] {
                "01" => ("br.png".to_string(), rotate_tile_90(transform, 2)), // get_tile_track_1_tl()
                "02" => ("tb.png".to_string(), transform), // get_tile_track_2_tb()
                "03" => ("br.png".to_string(), rotate_tile_90(transform, 1)), // get_tile_track_3_tr()
                "04" => ("br.png".to_string(), rotate_tile_90(transform, 3)), // get_tile_track_4_lb()
                "05" => ("tb.png".to_string(), rotate_tile_90(transform, 1)), // get_tile_track_5_lr()
                "06" => ("br.png".to_string(), transform), // get_tile_track_6_br()
                "31" => ("tr_over_tl.png".to_string(), transform), // get_tile_track_tr3_over_tl1()
                "13" => (
                    "tr_over_tl.png".to_string(),
                    flipmatrix_horizontal(transform),
                ), // get_tile_track_tl1_over_tr3()
                "12" => ("br_over_tb.png".to_string(), rotate_tile_90(transform, 2)), // get_tile_track_tl1_over_tb2()
                "21" => ("tb_over_br.png".to_string(), rotate_tile_90(transform, 2)), // get_tile_track_tb2_over_tl1()
                "14" => ("tr_over_tl.png".to_string(), rotate_tile_90(transform, 1)), // get_tile_track_tl1_over_lb4()
                "41" => (
                    "tr_over_tl.png".to_string(),
                    rotate_tile_90(flipmatrix_horizontal(transform), 1),
                ), // get_tile_track_lb4_over_tl1()
                "15" => (
                    "br_over_tb.png".to_string(),
                    flipmatrix_horizontal(rotate_tile_90(transform, 1)),
                ), // get_tile_track_tl1_over_lr5()
                "51" => (
                    "tb_over_br.png".to_string(),
                    flipmatrix_horizontal(rotate_tile_90(transform, 1)),
                ), // get_tile_track_lr5_over_tl1()
                "16" => (
                    "track_funny_tr_bl.png".to_string(),
                    flipmatrix_horizontal(transform),
                ), // get_tile_track_tl1_over_br6()
                "61" => (
                    "track_funny_tr_bl.png".to_string(),
                    flipmatrix_horizontal(transform),
                ), // get_tile_track_br6_over_tl1()
                "23" => ("tb_over_br.png".to_string(), flipmatrix_vertical(transform)), // get_tile_track_tb2_over_tr3()
                "32" => ("br_over_tb.png".to_string(), flipmatrix_vertical(transform)), // get_tile_track_tr3_over_tb2()
                "24" => (
                    "tb_over_br.png".to_string(),
                    flipmatrix_horizontal(transform),
                ), // get_tile_track_tb2_over_lb4()
                "42" => (
                    "br_over_tb.png".to_string(),
                    flipmatrix_horizontal(transform),
                ), // get_tile_track_lb4_over_tb2()
                "25" => ("lr_over_tb.png".to_string(), rotate_tile_90(transform, 1)), // get_tile_track_tb2_over_lr5()
                "52" => ("lr_over_tb.png".to_string(), transform), // get_tile_track_lr5_over_tb2()
                "26" => ("tb_over_br.png".to_string(), transform), // get_tile_track_tb2_over_br6()
                "62" => ("br_over_tb.png".to_string(), transform), // get_tile_track_br6_over_tb2()
                "34" => ("track_funny_tr_bl.png".to_string(), transform), // get_tile_track_tr3_over_lb4()
                "43" => ("track_funny_tr_bl.png".to_string(), transform), // get_tile_track_lb4_over_tr3()
                "35" => ("br_over_tb.png".to_string(), rotate_tile_90(transform, 1)), // get_tile_track_tr3_over_lr5()
                "53" => ("tb_over_br.png".to_string(), rotate_tile_90(transform, 1)), // get_tile_track_lr5_over_tr3()
                "36" => (
                    "tr_over_tl.png".to_string(),
                    flipmatrix_vertical(rotate_tile_90(transform, -1)),
                ), // get_tile_track_tr3_over_br6()
                "63" => ("tr_over_tl.png".to_string(), rotate_tile_90(transform, -1)), // get_tile_track_br6_over_tr3()
                "45" => ("br_over_tb.png".to_string(), rotate_tile_90(transform, -1)), // get_tile_track_lb4_over_lr5()
                "54" => ("tb_over_br.png".to_string(), rotate_tile_90(transform, -1)), // get_tile_track_lr5_over_lb4()
                "46" => (
                    "tr_over_tl.png".to_string(),
                    flipmatrix_horizontal(flipmatrix_vertical(transform)),
                ), // get_tile_track_lb4_over_br6()
                "64" => ("tr_over_tl.png".to_string(), flipmatrix_vertical(transform)), // get_tile_track_br6_over_lb4()
                "56" => (
                    "tb_over_br.png".to_string(),
                    rotate_tile_90(flipmatrix_vertical(rotate_tile_90(transform, -1)), 2),
                ), // get_tile_track_lr5_over_br6()
                "65" => (
                    "br_over_tb.png".to_string(),
                    rotate_tile_90(flipmatrix_vertical(rotate_tile_90(transform, -1)), 2),
                ), // get_tile_track_br6_over_lr5()
                _ => {
                    panic!("Unknown tile combination: {}", print_tile(&t))
                }
            }
        }
        Tile::PaintTile { track, c } => {
            if track.b_ && track.t_ {
                (
                    "p_outer_str_lr.png".to_string(),
                    rotate_tile_90(transform, 1),
                )
            } else if track.l_ && track.r_ {
                ("p_outer_str_lr.png".to_string(), transform)
            } else if track.b_ && track.r_ {
                ("p_outer_cur_br.png".to_string(), transform)
            } else if track.b_ && track.l_ {
                (
                    "p_outer_cur_br.png".to_string(),
                    rotate_tile_90(transform, -1),
                )
            } else if track.t_ && track.r_ {
                (
                    "p_outer_cur_br.png".to_string(),
                    rotate_tile_90(transform, 1),
                )
            } else if track.t_ && track.l_ {
                (
                    "p_outer_cur_br.png".to_string(),
                    rotate_tile_90(transform, 2),
                )
            } else {
                panic!("Unknown tile combination: {}", print_tile(&t))
            }
        }
        Tile::EmptyTile => ("empty.png".to_string(), transform),
        Tile::RockTile => ("rock.png".to_string(), transform),
        Tile::SplitTile { side_in } => match &print_tile(&t)[..] {
            "D1" => ("scissor_u.png".to_string(), transform),
            "D2" => ("scissor_u.png".to_string(), rotate_tile_90(transform, 2)),
            "D3" => ("scissor_u.png".to_string(), rotate_tile_90(transform, 1)),
            "D4" => ("scissor_u.png".to_string(), rotate_tile_90(transform, -1)),
            _ => {
                panic!("Unknown tile combination: {}", print_tile(&t))
            }
        },
        Tile::StartTile { dir: _, elems: _ } => ("s_base.png".to_string(), transform),
        Tile::EndTile {
            t_: _,
            b_: _,
            l_: _,
            r_: _,
            elems: _,
        } => ("e_base.png".to_string(), transform),
    };
    let texture = assets.get(&texture_path).unwrap().clone();
    return (texture, transform);
}

fn add_arrow_minitile_children(
    child_cmd: &mut ChildBuilder,
    dir: Side,
    assets: &HashMap<String, Handle<Image>>,
    big_tile_size: f32,
) {
    let scale = big_tile_size / 46.;
    let arrow = assets.get("s_arrow_elem_rigth.png").unwrap();
    let pos_x: f32;
    let pos_y: f32;
    let mut t = Transform::from_xyz(0., 0., 1.);
    if dir == Side::R_ {
        t = flipmatrix_horizontal(t);
        pos_x = -(23. - 6. - 6. / 2.) * scale;
        pos_y = (23. - 23. - 6. / 2.) * scale;
    } else if dir == Side::T_ {
        t = rotate_tile(t, std::f32::consts::PI / 2.);
        pos_x = -(23. - 23. - 6. / 2.) * scale;
        pos_y = (23. - 6. - 6. / 2.) * scale;
    } else if dir == Side::B_ {
        t = rotate_tile(t, -std::f32::consts::PI / 2.);
        pos_x = -(23. - 23. - 6. / 2.) * scale;
        pos_y = (23. - 46. - 6. / 2.) * scale;
    } else {
        pos_x = -(23. - 46. + 6. - 6. / 2.) * scale;
        pos_y = (23. - 23. - 6. / 2.) * scale;
    }
    // Translate t to the right position:
    t.translation.x = pos_x;
    t.translation.y = pos_y;
    child_cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(6., 46. - 26.)),
            ..Default::default()
        },
        transform: t,
        texture: (*arrow).clone(),
        ..Default::default()
    });
}

// Julia code:
// function add_funnels_minitile!(base, t_, b_, l_, r_)
//     funnel = samples["e_funnel_elem_rigth.png"]
//     # Add the funnel:
//     if r_ base[:, (46-8+2):46] = funnel end
//     if l_ base[:, 1:7] = flipmatrix_horizontal(funnel) end
//     if t_ base[1:7, :] = rotate_tile(funnel, 1) end
//     if b_ base[(46-8+2):46, :] = rotate_tile(funnel, -1) end
// end
// Turn Julia code into Rust code:
// Julia to Rust:

fn add_funnels_minitile_children(
    child_cmd: &mut ChildBuilder,
    t_: bool,
    b_: bool,
    l_: bool,
    r_: bool,
    assets: &HashMap<String, Handle<Image>>,
    big_tile_size: f32,
) {
    let scale = big_tile_size / 46.;
    let funnel = assets.get("e_funnel_elem_rigth.png").unwrap();
    let mut t = Transform::from_xyz(0., 0., 1.);
    let pos_x: f32;
    let pos_y: f32;
    if r_ {
        pos_x = -(23. - 8. - 8. / 2.); // * scale;
        pos_y = (23. - 23. - 8. / 2.); // * scale;
    } else if l_ {
        t = flipmatrix_horizontal(t);
        pos_x = -(23. - 8. - 8. / 2.); // * scale;
        pos_y = (23. - 23. - 8. / 2.); // * scale;
    } else if t_ {
        t = rotate_tile(t, std::f32::consts::PI / 2.);
        pos_x = -(23. - 23. - 8. / 2.); // * scale;
        pos_y = (23. - 8. - 8. / 2.); // * scale;
    } else {
        t = rotate_tile(t, -std::f32::consts::PI / 2.);
        pos_x = -(23. - 23. - 8. / 2.); // * scale;
        pos_y = (23. - 46. - 8. / 2.); // * scale;
    }
    // Translate t to the right position:
    t.translation.x = pos_x;
    t.translation.y = pos_y;
    child_cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(7., 46.)),
            ..Default::default()
        },
        transform: t,
        texture: (*funnel).clone(),
        ..Default::default()
    });
}

fn make_tile(
    t: Tile,
    commands: &mut Commands,
    assets: &HashMap<String, Handle<Image>>,
    big_tile_size: f32,
    coordinates: Coordinates,
) -> Entity {
    // let (transl_x, transl_y) = ((coordinates.x as f32 * big_tile_size) + (big_tile_size / 2.), ((6 - coordinates.y) as f32 * big_tile_size) + (big_tile_size / 2.));
    let (transl_x, transl_y) = (
        (coordinates.x as f32 * big_tile_size),
        (coordinates.y as f32 * big_tile_size),
    );

    let (texture, transform) = get_transform_and_texture(t, assets);
    // Translate the tile to the right position:
    let transform = transform.with_translation(Vec3::new(transl_x, transl_y, 2.));
    let mut child = commands.spawn_bundle(TileSpriteBundle {
        coordinates, // Tile coordinates
        texture: texture,
        transform: transform,
        name: Name::new(format!("Tile ({}, {})", coordinates.x, coordinates.y)), // Tile name
        sprite: Sprite {
            custom_size: Some(Vec2::splat(big_tile_size)),
            color: Color::WHITE,
            ..Default::default()
        },
        ..Default::default()
    });
    if let Tile::StartTile { dir, elems } = t {
        child
            .with_children(
                partial!(add_color_minitiles_children => _, elems, true, assets, big_tile_size),
            )
            .with_children(partial!(add_arrow_minitile_children => _, dir, assets, big_tile_size));
    } else if let Tile::EndTile {
        t_,
        b_,
        l_,
        r_,
        elems,
    } = t
    {
        child
            .with_children(
                partial!(add_color_minitiles_children => _, elems, false, assets, big_tile_size),
            )
            .with_children(
                partial!(add_funnels_minitile_children => _, t_, b_, l_, r_, assets, big_tile_size),
            );
    } else if let Tile::PaintTile { track, c } = t {
        child.with_children(|parent| {
            let size = ((40 - 6) as f32) / 46. * big_tile_size;
            let inner = assets
                .get(&format!("p_{}.png", colorz_to_long_str(c)))
                .unwrap()
                .clone();
            parent.spawn_bundle(SpriteBundle {
                texture: inner,
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(big_tile_size)),
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
    return child.id();
}

pub fn hovered_tile(board: &BoardDimensions, window: &Window) -> Option<Coordinates> {
    let window_size = Vec2::new(window.width(), window.height());
    let position = window.cursor_position()? - window_size / 2.;
    if !in_bounds(position, board.rect) {return None;}
    // Get vec2 with x and y out of the vec3 position:
    let boardpospos: Vec2 = Vec2::new(board.position.x, board.position.y);
    let offset = Vec2{x: board.tile_size / 2., y: board.tile_size / 2.};
    let coordinates = position - boardpospos + offset;
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




/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////
///

pub fn check_mouse_action(mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>, mut board_q: Query<(&BoardDimensions, &mut BoardHoverable, &mut BoardTileMap), With<Board>>, mut spawn_event: EventWriter<TileSpawnEvent>,) {
    if mouse_input.pressed(MouseButton::Left) {
        for (board_dimensions, mut hoverable, mut boardTileMap) in board_q.iter_mut() { // It's never more than 1, but can very well be 0
            let window = windows.get_primary().expect("no primary window");
            let pos = hovered_tile(board_dimensions, window);
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
            }
            else if hoverable.hovered_pos_1.is_none() {hoverable.hovered_pos_1 = Some(pos);}
            else if hoverable.hovered_pos_2.is_none() && hoverable.hovered_pos_1.unwrap() != pos {hoverable.hovered_pos_2 = Some(pos);}
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



// System to generate the complete board
pub fn create_board(
    mut commands: Commands,
    board_options: Res<BoardOptionsDefault>,
    windows: Res<Windows>,
    mut tick_status: ResMut<TicksInATick>,
    mut spawn_event: EventWriter<TileSpawnEvent>,
) {
    
    // let map_s = vec![
    //     "00 00 00 E0100_g 00 00 00".to_string(),
    //     "00 00 00 02 00 00 00".to_string(),
    //     "00 00 06 01 00 00 00".to_string(),
    //     "Sr_b 05 53 45 05 05 E0010_g".to_string(),
    //     "00 00 00 23 00 00 00".to_string(),
    //     "00 00 00 St_y 00 00 00".to_string(),
    //     "00 00 00 00 00 00 00".to_string(),
    //     ];
    let map_s = vec![
        "06 06 06 06 06 06 06".to_string(),
        "06 06 06 06 06 06 06".to_string(),
        "06 06 06 06 06 06 06".to_string(),
        "06 06 06 06 06 06 06".to_string(),
        "06 06 06 06 06 06 06".to_string(),
        "06 06 06 06 06 06 06".to_string(),
        "06 06 06 06 06 06 06".to_string(),
        ];
    let tile_map: Vec<Vec<Tile>> = parse_map(map_s);
    let n_width_ = tile_map.len();
    let n_height_ = tile_map.len();
    // let tile_size = match board_options.tile_size {
    //     TileSize::Fixed(v) => v,
    //     TileSize::Adaptive =>  (
    //         windows.get_primary().unwrap().width() / n_width_ as f32).min(
    //             windows.get_primary().unwrap().height() / n_height_ as f32) //.clamp(min, max)
    // };
    let tile_size = 50.0;
    let board_position = match board_options.position {
        BoardPosition::Centered { offset } => {
            // offset
            Vec3::new(-(n_width_ as f32 * tile_size / 2.), -(n_height_ as f32 * tile_size / 2.), 0.) + offset
        }
        BoardPosition::Custom(p) => p,
    };
    // log::info!("board size: {}", board_size);

    // x va spostato a sx (-) di ts/2
    // y va spostato in giu (-) di ts/2
    let offset_ts2 = tile_size / 2.;

    // Init BoardDimensions component
    let board_dimensions = BoardDimensions {
        tile_size,
        position: board_position,
        rect: Rect{
            top: board_position.y - offset_ts2,
            bottom: board_position.y + n_height_ as f32 * tile_size - offset_ts2,
            left: board_position.x - offset_ts2,
            right: board_position.x + n_width_ as f32 * tile_size - offset_ts2,
        }
    };
    // Println board_dimensions.position:
    println!("board_dimensions.position: {:?}", board_dimensions.position);

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
            ..Default::default()
        },
        global_transform: GlobalTransform::default(),
        texture: Default::default(),
        visibility: Default::default(),
        computed_visibility: Default::default(),
        
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

fn cleanup_board(mut commands: Commands, board_q: Query<Entity, With<Board>>) {
    let board_id = board_q.single();
    commands.entity(board_id).despawn_recursive();
}

fn spawn_tile(
    mut commands: Commands,
    // board_assets_map: Res<BoardAssetsMap>,
    mut board_q: Query<(Entity, &BoardDimensions, &mut BoardEntities), With<Board>>,
    mut evt: EventReader<TileSpawnEvent>,
    textures: Res<TextureAssets>,
) {
    for trigger_event in evt.iter() {
        for (board_id, board_dimensions, mut board_entities) in board_q.iter_mut() {
            let mut board_entity = commands.entity(board_id);  // Get entity by id:
            // if board is not None:
            let (x, y) = (trigger_event.x, trigger_event.y);
            let coordinates = Coordinates {
                x: x as u16,
                y: y as u16,
            };
            let size = board_dimensions.tile_size;
            let t = trigger_event.new_tile;
            
            // get entity by coordinates using hashmap
            let old_entity: Option<Entity> = None; // board_entities.tiles.get(&coordinates).cloned();
            if let Some(old_entity) = old_entity {
                board_entity.remove_children(&[old_entity]);
                board_entities.tiles.remove(&coordinates);
                commands.entity(old_entity).despawn_recursive();
            }
            
            if let Tile::StartTile { dir, elems } = t {
                println!(">> Happening !! {:?}", t);
            }
            
            // Make a &HashMap<String, Handle<Image>> from "06" => to br
            let mut asset_map: HashMap<String, Handle<Image>> = HashMap::new();
            asset_map.insert("br.png".to_string(), textures.br.clone());
            
            // let child_id=  make_tile(t, &mut commands, &board_assets_map.assets, size, coordinates);
            let child_id = make_tile(t, &mut commands, &asset_map, size, coordinates);
            
            let mut board_entity = commands.entity(board_id);  // Get entity by id:
            board_entity.push_children(&[child_id]);// add the child to the parent
            board_entities.tiles.insert(coordinates, child_id); // add the child to the hashmap:
            
        }
    }
}


fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(check_mouse_action))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_tile))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(create_board),)
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup_board),)
            ;
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut tile_spawn_event_writer: EventWriter<TileSpawnEvent>,
) {
    let mut x: EntityCommands = commands.spawn_bundle(SpriteBundle {
        texture: textures.e_elem_4_green.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..Default::default()
    });
    x.insert(Player);

    // Spawn an event creating a Tile at (0,0):
    // let tile_spawn_event = TileSpawnEvent {
    //     x: 0,
    //     y: 0,
    //     new_tile: Tile::SingleTrackTile {
    //         track: get_track(TrackOptions::BR),
    //     },
    // };
    // tile_spawn_event_writer.send(tile_spawn_event);
    // println!(">> Spawned player");
}

// use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

fn main() {
    // test();
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Trainyard".to_string(), // ToDo
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(get_board_option_default())
        .insert_resource(get_ticks_in_a_tick_default())
        .add_plugin(GamePlugin)
        .add_startup_system(set_window_icon)
        .add_event::<TileSpawnEvent>()
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
