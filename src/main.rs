// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, NonSend, WindowDescriptor};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
// use trainyard::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;
use bevy::ecs::system::EntityCommands;

mod audio;
mod loading;
mod menu;
mod utils;

use crate::utils::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

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
pub struct TileSpriteBundle{
    pub name: Name, // Tile name
    pub coordinates: Coordinates, // Tile coordinates
    
    // Flattened SpriteBundle #[bundle] : SO NICE!!
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility, // User indication of whether an entity is visible
    pub computed_visibility: ComputedVisibility,
}

/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Clone)]
pub struct TileSpawnEvent{
    x: usize,
    y: usize,
    new_tile: Tile,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicTickEvent{
    TickBegin,
    TickMiddle,
    TickEnd,
}


use std::collections::HashMap;

/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////


pub fn get_poss_minitile_1() -> Vec<usize> {vec![11,]}
pub fn get_poss_minitile_4() -> Vec<usize> {vec![11, 25,]}
pub fn get_poss_minitile_9() -> Vec<usize> {vec![11, 20, 29,]}

pub fn get_dimension_minitile_1() -> u32 {27}
pub fn get_dimension_minitile_4() -> u32 {12}
pub fn get_dimension_minitile_9() -> u32 {8}


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
    t.rotate(Quat::from_rotation_z(angle));  // std::f32::consts::PI / 2.
    return t;
}
pub fn rotate_tile_90(mut t: Transform, times: i16) -> Transform {
    t.rotate(Quat::from_rotation_z(std::f32::consts::PI / 2. * times as f32));  // std::f32::consts::PI / 2.
    return t;
}


pub fn add_sprite_on_sprite(mut sprit_parent: EntityCommands, child_asset: Handle<Image>, x: f32, y: f32, big_tile_size: f32, small_tile_size: f32) {

}
pub fn add_color_minitiles_children(child_cmd: &mut ChildBuilder, elems: VectorOfColorz, is_start: bool, assets: &HashMap<String, Handle<Image>>, big_tile_size: f32) {
    let scale = big_tile_size / 46.;
    let (N, poss, small_tile_size) = if elems.len() == 1 { (1, get_poss_minitile_1(), get_dimension_minitile_1()) }
        else if elems.len() <= 4 { (4, get_poss_minitile_4(), get_dimension_minitile_4()) }
        else if elems.len() <= 9 { (9, get_poss_minitile_9(), get_dimension_minitile_9()) }
        else { panic!("Too many elements in StartTile"); };
    for (i, y) in poss.iter().enumerate() {
        for (j, x) in poss.iter().enumerate() {
            let n_to_get = i*poss.len() + j;
            if n_to_get < elems.len() {
                let pos_x = - (23. - (*x as f32) - (small_tile_size as f32)/2.) * scale;
                let pos_y = (23. - (*y as f32) - (small_tile_size as f32)/2.) * scale;
                let prefix = if is_start {"s"} else {"e"};
                let minitile = format!("{}_elem_{}_{}.png", prefix, N, colorz_to_long_str(elems.v[n_to_get].unwrap()));
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

fn get_transform_and_texture(t :Tile, assets: &HashMap<String, Handle<Image>>) -> (Handle<Image>, Transform){
    let mut transform = Transform::from_xyz(0., 0., 1.);
    let texture_path: String;
    // Print the tile:
    println!(">>>>>>>>>>>>Tile: {}", print_tile(&t));
    let (texture_path, transform): (String, Transform) = match t {
        Tile::SingleTrackTile{track:_} | Tile::TrackTile { toptrack:_, bottrack:_ } => { 
            match &print_tile(&t)[..] { 
                "01" => {("br.png".to_string(), rotate_tile_90(transform, 2)) } // get_tile_track_1_tl() 
                "02" => {("tb.png".to_string(), transform) } // get_tile_track_2_tb() 
                "03" => {("br.png".to_string(), rotate_tile_90(transform, 1)) } // get_tile_track_3_tr() 
                "04" => {("br.png".to_string(), rotate_tile_90(transform, 3)) } // get_tile_track_4_lb() 
                "05" => {("tb.png".to_string(), rotate_tile_90(transform, 1)) } // get_tile_track_5_lr() 
                "06" => {("br.png".to_string(), transform) } // get_tile_track_6_br()
                "31" => {("tr_over_tl.png".to_string(), transform) } // get_tile_track_tr3_over_tl1()
                "13" => {("tr_over_tl.png".to_string(), flipmatrix_horizontal(transform)) } // get_tile_track_tl1_over_tr3()
                "12" => {("br_over_tb.png".to_string(), rotate_tile_90(transform, 2)) } // get_tile_track_tl1_over_tb2()
                "21" => {("tb_over_br.png".to_string(), rotate_tile_90(transform, 2)) } // get_tile_track_tb2_over_tl1()
                "14" => {("tr_over_tl.png".to_string(), rotate_tile_90(transform, 1)) } // get_tile_track_tl1_over_lb4()
                "41" => {("tr_over_tl.png".to_string(), rotate_tile_90(flipmatrix_horizontal(transform), 1)) } // get_tile_track_lb4_over_tl1()
                "15" => {("br_over_tb.png".to_string(), flipmatrix_horizontal(rotate_tile_90(transform, 1))) } // get_tile_track_tl1_over_lr5()
                "51" => {("tb_over_br.png".to_string(), flipmatrix_horizontal(rotate_tile_90(transform, 1))) } // get_tile_track_lr5_over_tl1()
                "16" => {("track_funny_tr_bl.png".to_string(), flipmatrix_horizontal(transform)) } // get_tile_track_tl1_over_br6()
                "61" => {("track_funny_tr_bl.png".to_string(), flipmatrix_horizontal(transform)) } // get_tile_track_br6_over_tl1()
                "23" => {("tb_over_br.png".to_string(), flipmatrix_vertical(transform)) } // get_tile_track_tb2_over_tr3()
                "32" => {("br_over_tb.png".to_string(), flipmatrix_vertical(transform)) } // get_tile_track_tr3_over_tb2()
                "24" => {("tb_over_br.png".to_string(), flipmatrix_horizontal(transform)) } // get_tile_track_tb2_over_lb4()
                "42" => {("br_over_tb.png".to_string(), flipmatrix_horizontal(transform)) } // get_tile_track_lb4_over_tb2()
                "25" => {("lr_over_tb.png".to_string(), rotate_tile_90(transform, 1)) } // get_tile_track_tb2_over_lr5()
                "52" => {("lr_over_tb.png".to_string(), transform) } // get_tile_track_lr5_over_tb2()
                "26" => {("tb_over_br.png".to_string(), transform) } // get_tile_track_tb2_over_br6()
                "62" => {("br_over_tb.png".to_string(), transform) } // get_tile_track_br6_over_tb2()
                "34" => {("track_funny_tr_bl.png".to_string(), transform) } // get_tile_track_tr3_over_lb4()
                "43" => {("track_funny_tr_bl.png".to_string(), transform) } // get_tile_track_lb4_over_tr3()
                "35" => {("br_over_tb.png".to_string(), rotate_tile_90(transform, 1)) } // get_tile_track_tr3_over_lr5()
                "53" => {("tb_over_br.png".to_string(), rotate_tile_90(transform, 1)) } // get_tile_track_lr5_over_tr3()
                "36" => {("tr_over_tl.png".to_string(), flipmatrix_vertical(rotate_tile_90(transform, -1))) } // get_tile_track_tr3_over_br6()
                "63" => {("tr_over_tl.png".to_string(), rotate_tile_90(transform, -1)) } // get_tile_track_br6_over_tr3()
                "45" => {("br_over_tb.png".to_string(), rotate_tile_90(transform, -1)) } // get_tile_track_lb4_over_lr5()
                "54" => {("tb_over_br.png".to_string(), rotate_tile_90(transform, -1)) } // get_tile_track_lr5_over_lb4()
                "46" => {("tr_over_tl.png".to_string(), flipmatrix_horizontal(flipmatrix_vertical(transform))) } // get_tile_track_lb4_over_br6()
                "64" => {("tr_over_tl.png".to_string(), flipmatrix_vertical(transform)) } // get_tile_track_br6_over_lb4()
                "56" => {("tb_over_br.png".to_string(), rotate_tile_90(flipmatrix_vertical(rotate_tile_90(transform, -1)), 2)) } // get_tile_track_lr5_over_br6()
                "65" => {("br_over_tb.png".to_string(), rotate_tile_90(flipmatrix_vertical(rotate_tile_90(transform, -1)), 2)) }// get_tile_track_br6_over_lr5()
                _ => {panic!("Unknown tile combination: {}", print_tile(&t)) }
            }
        },
        Tile::PaintTile { track, c } => {
            if track.b_ && track.t_ { ("p_outer_str_lr.png".to_string(), rotate_tile_90(transform, 1))}
            else if track.l_ && track.r_ { ("p_outer_str_lr.png".to_string(), transform)}
            else if track.b_ && track.r_ { ("p_outer_cur_br.png".to_string(), transform)}
            else if track.b_ && track.l_ { ("p_outer_cur_br.png".to_string(), rotate_tile_90(transform, -1))}
            else if track.t_ && track.r_ { ("p_outer_cur_br.png".to_string(), rotate_tile_90(transform, 1))}
            else if track.t_ && track.l_ { ("p_outer_cur_br.png".to_string(), rotate_tile_90(transform, 2))}
            else { panic!("Unknown tile combination: {}", print_tile(&t)) }
        },
        Tile::EmptyTile => {
            ("empty.png".to_string(), transform)
        },
        Tile::RockTile => {
            ("rock.png".to_string(), transform)
        },
        Tile::SplitTile { side_in } => {
            match  &print_tile(&t)[..] {
                "D1" => {("scissor_u.png".to_string(), transform) }
                "D2" => {("scissor_u.png".to_string(), rotate_tile_90(transform, 2)) }
                "D3" => {("scissor_u.png".to_string(), rotate_tile_90(transform, 1)) }
                "D4" => {("scissor_u.png".to_string(), rotate_tile_90(transform, -1)) }
                _ => {panic!("Unknown tile combination: {}", print_tile(&t)) }
            }
        },
        Tile::StartTile{ dir: _, elems: _ } => {
            ("s_base.png".to_string(), transform)
        },
        Tile::EndTile{ t_: _, b_: _, l_: _, r_: _, elems: _ } => {
            ("e_base.png".to_string(), transform)
        },
    };
    let texture = assets.get(&texture_path).unwrap().clone();
    return (texture, transform);
}


fn add_arrow_minitile_children(child_cmd: &mut ChildBuilder, dir: Side, assets: &HashMap<String, Handle<Image>>, big_tile_size: f32) {
    let scale = big_tile_size / 46.;
    let arrow = assets.get("s_arrow_elem_rigth.png").unwrap();
    let pos_x: f32;
    let pos_y: f32;
    let mut t = Transform::from_xyz(0., 0., 1.);
    if dir == Side::R_ {
        t = flipmatrix_horizontal(t);
        pos_x = - (23. - 6. - 6./2.) * scale;
        pos_y = (23. - 23. - 6./2.) * scale;
    } else if dir == Side::T_ {
        t = rotate_tile(t, std::f32::consts::PI / 2.);
        pos_x = - (23. - 23. - 6./2.) * scale;
        pos_y = (23. - 6. - 6./2.) * scale;
    } else if dir == Side::B_ {
        t = rotate_tile(t, -std::f32::consts::PI / 2.);
        pos_x = - (23. - 23. - 6./2.) * scale;
        pos_y = (23. - 46. - 6./2.) * scale;
    }
    else {
        pos_x = - (23. - 46. + 6. - 6./2.) * scale;
        pos_y = (23. - 23. - 6./2.) * scale;
    }
    // Translate t to the right position:
    t.translation.x = pos_x;
    t.translation.y = pos_y;
    child_cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(6., 46.-26.)),
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

fn add_funnels_minitile_children(child_cmd: &mut ChildBuilder, t_: bool, b_: bool, l_: bool, r_: bool, assets: &HashMap<String, Handle<Image>>, big_tile_size: f32) {
    let scale = big_tile_size / 46.;
    let funnel = assets.get("e_funnel_elem_rigth.png").unwrap();
    let mut t = Transform::from_xyz(0., 0., 1.);
    let pos_x: f32;
    let pos_y: f32;
    if r_ {
        pos_x = - (23. - 8. - 8./2.); // * scale;
        pos_y = (23. - 23. - 8./2.); // * scale;
    } else if l_ {
        t = flipmatrix_horizontal(t);
        pos_x = - (23. - 8. - 8./2.); // * scale;
        pos_y = (23. - 23. - 8./2.); // * scale;
    } else if t_ {
        t = rotate_tile(t, std::f32::consts::PI / 2.);
        pos_x = - (23. - 23. - 8./2.); // * scale;
        pos_y = (23. - 8. - 8./2.); // * scale;
    } else {
        t = rotate_tile(t, -std::f32::consts::PI / 2.);
        pos_x = - (23. - 23. - 8./2.); // * scale;
        pos_y = (23. - 46. - 8./2.); // * scale;
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



fn make_tile(t :Tile, commands: &mut Commands, assets: &HashMap<String, Handle<Image>>, big_tile_size: f32, coordinates: Coordinates) -> Entity{
    let (transl_x, transl_y) = ((coordinates.x as f32 * big_tile_size) + (big_tile_size / 2.), ((6 - coordinates.y) as f32 * big_tile_size) + (big_tile_size / 2.));

    let (texture, transform) = get_transform_and_texture(t, assets);
    // Translate the tile to the right position:
    let transform = transform.with_translation(Vec3::new(transl_x, transl_y, 0.));
    let mut child = commands.spawn_bundle(TileSpriteBundle {
        coordinates, // Tile coordinates
        texture: texture,
        transform: transform,
        name: Name::new(format!("Tile ({}, {})", coordinates.x, coordinates.y)), // Tile name
        sprite: Sprite { custom_size: Some(Vec2::splat(big_tile_size)), color: Color::WHITE, ..Default::default()
        },
        ..Default::default()
    });
    if let Tile::StartTile{dir, elems} = t {
            child.with_children(partial!(add_color_minitiles_children => _, elems, true, assets, big_tile_size))
            .with_children(partial!(add_arrow_minitile_children => _, dir, assets, big_tile_size));
    }
    else if let Tile::EndTile{t_, b_, l_, r_, elems} = t {
        child.with_children(partial!(add_color_minitiles_children => _, elems, false, assets, big_tile_size))
            .with_children(partial!(add_funnels_minitile_children => _, t_, b_, l_, r_, assets, big_tile_size));
    }
    else if let Tile::PaintTile{track, c} = t {
        child.with_children( | parent | {
            let size = ((40-6) as f32) / 46. * big_tile_size;
            let inner = assets.get(&format!("p_{}.png", colorz_to_long_str(c))).unwrap().clone();
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


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////

fn spawn_tile(
    mut commands: Commands,
    // board_assets_map: Res<BoardAssetsMap>,
    // mut board_q: Query<(Entity, &BoardDimensions, &mut BoardEntities), With<Board>>,
    mut evt: EventReader<TileSpawnEvent>,
    textures: Res<TextureAssets>) {
    for trigger_event in evt.iter() {
    println!("Received!!!!!");
    // for (board_id, board_dimensions, mut board_entities) in board_q.iter_mut() {
        // let mut board_entity = commands.entity(board_id);  // Get entity by id:
        // if board is not None:
        let (x, y) = (trigger_event.x, trigger_event.y);
        let coordinates = Coordinates { x: x as u16, y: y as u16, };
        let size = 25.; //board_dimensions.tile_size;
        let t = trigger_event.new_tile;

        // get entity by coordinates using hashmap
        let old_entity: Option<Entity> = None; // board_entities.tiles.get(&coordinates).cloned();
        if let Some(old_entity) = old_entity {
            // board_entity.remove_children(&[old_entity]);
            // board_entities.tiles.remove(&coordinates);
            commands.entity(old_entity).despawn_recursive();
        }

        if let Tile::StartTile { dir, elems } = t {
            println!(">> Happening !! {:?}", t);
        }

        // Make a &HashMap<String, Handle<Image>> from "06" => to br
        let mut asset_map: HashMap<String, Handle<Image>> = HashMap::new();
        asset_map.insert("br.png".to_string(), textures.br.clone());
        
        // let child_id=  make_tile(t, &mut commands, &board_assets_map.assets, size, coordinates);
        let child_id=  make_tile(t, &mut commands, &asset_map, size, coordinates);

        // let mut board_entity = commands.entity(board_id);  // Get entity by id:
        // board_entity.push_children(&[child_id]);// add the child to the parent
        // board_entities.tiles.insert(coordinates, child_id); // add the child to the hashmap:

    // }
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
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_tile));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>, 
    mut tile_spawn_event_writer: EventWriter<TileSpawnEvent>
) {
    let mut x: EntityCommands = commands
        .spawn_bundle(SpriteBundle {
            texture: textures.e_elem_4_green.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        });
        x.insert(Player);

        // Spawn an event creating a Tile at (0,0):
        let tile_spawn_event = TileSpawnEvent {
            x: 0,
            y: 0,
            new_tile: Tile::SingleTrackTile { track: get_track(TrackOptions::BR), },
        };
        tile_spawn_event_writer.send(tile_spawn_event);
        println!(">> Spawned player");
    
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
