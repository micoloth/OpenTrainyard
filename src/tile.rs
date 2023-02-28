use bevy::prelude::*;

use crate::simulator::*;
use crate::utils::Coordinates;

use crate::board::*;
use crate::loading::{TileAssets, get_asset};

use partial_application::partial;


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Bundle)]
pub struct TileSpriteBundle {
    pub coordinates: Coordinates, // Tile coordinates
    pub tile: Tile, // Tile type

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
pub struct TileSpawnData {  // It was called Event!
    pub x: usize,
    pub y: usize,
    pub new_tile: Tile,
    pub prev_tile: Option<Tile>,
}
// mut evt: EventReader<TileSpawnEvent>,
// mut spawn_event: EventWriter<TileSpawnEvent>,


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_tile(
    mut commands: Commands,
    board_assets_map: Res<TileAssets>,
    mut board_q: Query<(Entity, &BoardDimensions, &BoardTileMap, &Children), (With<Board>, Changed<BoardTileMap>)>,
    tile_q: Query<(Entity, &Coordinates, &mut Tile)>,
) {
    for (board_id, board_dimensions, board_tilemap, children) in board_q.iter_mut() {
        // `children` is a collection of Entity IDs
        for &child in children.iter() {
            // get the health of each child unit
            if let Ok((tile_entity, coordinates, tile)) = tile_q.get(child)
            {
                if board_tilemap.map[coordinates.y as usize][coordinates.x as usize] != *tile {
                    // Remove parent/child relationship:
                    commands.entity(board_id).remove_children(&[tile_entity]);
                    // despawn tile entity:
                    commands.entity(tile_entity).despawn_recursive();
                    // Create new tile:
                    let size = board_dimensions.tile_size;
                    let coordinates = Coordinates { x: coordinates.x as u16, y: coordinates.y as u16,};
                    let newtile = board_tilemap.map[coordinates.y as usize][coordinates.x as usize];
                    let child_id = make_tile(newtile, &mut commands, &board_assets_map, size, coordinates);
                    // Append to parent/child relationship:
                    commands.entity(board_id).push_children(&[child_id]);// add the child to the parent
                }
            }
        }
    }
}





/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

fn get_poss_minitile_1() -> Vec<usize> {
    vec![11]
}
fn get_poss_minitile_4() -> Vec<usize> {
    vec![11, 25]
}
fn get_poss_minitile_9() -> Vec<usize> {
    vec![11, 20, 29]
}

fn get_dimension_minitile_1() -> u32 {
    27
}
fn get_dimension_minitile_4() -> u32 {
    12
}
fn get_dimension_minitile_9() -> u32 {
    8
}

fn flipmatrix_vertical(mut t: Transform) -> Transform {
    t = t.clone();
    t.scale.y *= -1.;
    return t;
}
fn flipmatrix_horizontal(mut t: Transform) -> Transform {
    t.scale.x *= -1.;
    return t;
}
fn rotate_tile(mut t: Transform, angle: f32) -> Transform {
    t.rotate(Quat::from_rotation_z(angle)); // std::f32::consts::PI / 2.
    return t;
}
fn rotate_tile_90(mut t: Transform, times: i16) -> Transform {
    t.rotate(Quat::from_rotation_z(
        std::f32::consts::PI / 2. * times as f32,
    )); // std::f32::consts::PI / 2.
    return t;
}

fn add_color_minitiles_children(
    child_cmd: &mut ChildBuilder,
    elems: VectorOfColorz,
    orig_len: i8,
    is_start: bool,
    assets: &TileAssets,
    big_tile_size: f32,
) {
    // let scale = big_tile_size / 46.;
    let (n, poss, small_tile_size) = if orig_len == 1 {
        (1, get_poss_minitile_1(), get_dimension_minitile_1())
    } else if orig_len <= 4 {
        (4, get_poss_minitile_4(), get_dimension_minitile_4())
    } else if orig_len <= 9 {
        (9, get_poss_minitile_9(), get_dimension_minitile_9())
    } else {
        panic!("Too many elements in StartTile");
    };
    for (i, y) in poss.iter().enumerate() {
        for (j, x) in poss.iter().enumerate() {
            let n_to_get = i * poss.len() + j;
            if n_to_get < elems.len() {
                let pos_x = -(23. - (*x as f32) - (small_tile_size as f32) / 2. +1.5); // * scale;
                let pos_y = (23. - (*y as f32) - (small_tile_size as f32) / 2. +1.5); // * scale;
                let prefix = if is_start { "s" } else { "e" };
                let minitile = format!(
                    "{}_elem_{}_{}.png",
                    prefix,
                    n,
                    colorz_to_long_str(elems.v[n_to_get].unwrap())
                );
                // let child_asset = assets.get(&minitile).unwrap();
                let child_asset = get_asset(minitile, assets);
                child_cmd.spawn(SpriteBundle {
                    // sprite: Sprite {
                    //     custom_size: Some(Vec2::splat(small_tile_size as f32)),
                    //     ..default()
                    // },
                    transform: Transform::from_xyz(pos_x, pos_y, 5.),
                    texture: child_asset,
                    ..default()
                });
            } else {
                break;
            }
        }
    }
}

fn get_transform_and_texture(
    t: Tile,
    assets: &TileAssets,
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
                    flipmatrix_vertical(rotate_tile_90(transform, 1)),
                ), // get_tile_track_tl1_over_lr5()
                "51" => (
                    "tb_over_br.png".to_string(),
                    flipmatrix_vertical(rotate_tile_90(transform, 1)),
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
                    flipmatrix_vertical(rotate_tile_90(transform, 1)),
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
                    flipmatrix_horizontal(rotate_tile_90(transform, 1)),
                ), // get_tile_track_lr5_over_br6()
                "65" => (
                    "br_over_tb.png".to_string(),
                    flipmatrix_horizontal(rotate_tile_90(transform, 1)),
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
            "D1" => ("scissor_u.png".to_string(), Transform::from_xyz(0., 0., 2.)),
            "D2" => ("scissor_u.png".to_string(), rotate_tile_90(Transform::from_xyz(0., 0., 2.), 2)),
            "D3" => ("scissor_u.png".to_string(), rotate_tile_90(Transform::from_xyz(0., 0., 2.), 1)),
            "D4" => ("scissor_u.png".to_string(), rotate_tile_90(Transform::from_xyz(0., 0., 2.), -1)),
            _ => {
                panic!("Unknown tile combination: {}", print_tile(&t))
            }
        },
        Tile::StartTile { dir: _, elems: _, orig_len:_ } => ("s_base.png".to_string(), Transform::from_xyz(0., 0., 4.)),
        Tile::EndTile {
            t_: _,
            b_: _,
            l_: _,
            r_: _,
            elems: _,
            orig_len:_
        } => ("e_base.png".to_string(), Transform::from_xyz(0., 0., 4.)),
    };
    let texture = get_asset(texture_path, assets);

    return (texture, transform);
}

fn add_arrow_minitile_children(
    child_cmd: &mut ChildBuilder,
    dir: Side,
    assets: &TileAssets,
    big_tile_size: f32,
) {
    // let scale = big_tile_size / 46.;
    let arrow = get_asset("s_arrow_elem_rigth.png".to_string(), assets);
    let pos_x: f32;
    let pos_y: f32;
    let mut t = Transform::from_xyz(0., 0., 0.5);
    if dir == Side::R_ {
        pos_x = (23. - 6. / 2.); // * scale;
        pos_y = 0.;
    } else if dir == Side::T_ {
        t = rotate_tile(t, std::f32::consts::PI / 2.);
        pos_x = 0.;
        pos_y = (23. - 6. + 6. / 2.); // * scale;
    } else if dir == Side::B_ {
        t = rotate_tile(t, -std::f32::consts::PI / 2.);
        pos_x = 0.;
        pos_y = ( - 23. + 6. / 2.); // * scale;
    } else {
        t = flipmatrix_horizontal(t);
        pos_x = -(23. - 6. / 2.); // * scale;
        pos_y = 0.;
    }
    // Translate t to the right position:
    t.translation.x = pos_x;
    t.translation.y = pos_y;
    child_cmd.spawn(SpriteBundle {
        transform: t,
        texture: arrow,
        ..default()
    });
}


fn add_funnels_minitile_children(
    child_cmd: &mut ChildBuilder,
    t_: bool,
    b_: bool,
    l_: bool,
    r_: bool,
    assets: &TileAssets,
    big_tile_size: f32,
) {
    // let scale = big_tile_size / 45.;
    let funnel = get_asset("e_funnel_elem_rigth.png".to_string(), assets);
    let mut t = Transform::from_xyz(0., 0., 0.5);
    let pos_x: f32;
    let pos_y: f32;
    if r_ {
        pos_x = (23. - 8. / 2.); // * scale;
        pos_y = 0.;
    } else if l_ {
        t = flipmatrix_horizontal(t);
        pos_x = -(23. - 8. / 2.); // * scale;
        pos_y = 0.;
    } else if t_ {
        t = rotate_tile(t, std::f32::consts::PI / 2.);
        pos_x = 0.;
        pos_y = (23. - 8. + 8. / 2.); // * scale;
    } else {
        t = rotate_tile(t, -std::f32::consts::PI / 2.);
        pos_x = 0.;
        pos_y = (- 23. + 8. / 2.); // * scale;
    }
    // Translate t to the right position:
    t.translation.x = pos_x;
    t.translation.y = pos_y;
    child_cmd.spawn(SpriteBundle {
        transform: t,
        texture: funnel,
        ..default()
    });
}

const START_POSS: [i32; 7] = [0, 45, 90, 135, 180, 225, 270];  // In case it's not clear, this is starts = [((0:6) .* (46 - 1))...]


pub fn make_tile(
    t: Tile,
    commands: &mut Commands,
    assets: &TileAssets,
    big_tile_size: f32,
    coordinates: Coordinates,
) -> Entity {
    // Translate the tile to the right position:
    // let (transl_x, transl_y) = ((coordinates.x as f32 * big_tile_size) + (big_tile_size / 2.), ((6 - coordinates.y) as f32 * big_tile_size) + (big_tile_size / 2.));
    let (transl_x, transl_y) =  (START_POSS[coordinates.x as usize] as f32 + (big_tile_size / 2.), START_POSS[6 - coordinates.y as usize] as f32 + (big_tile_size / 2.));
    let (texture, transform) = get_transform_and_texture(t, assets);
    let mut child = commands.spawn(TileSpriteBundle {
        coordinates, // Tile coordinates
        texture: texture,
        transform: transform.with_translation(Vec3::new(transl_x, transl_y, 2.)),
        tile: t,
        sprite: default(),
        global_transform: default(),
        visibility: default(),
        computed_visibility: default(),
    });
    if let Tile::StartTile { dir, elems , orig_len} = t {
        child
            .with_children(|parent| 
            {
                let inner = get_asset("e_base_inner.png".to_string(), assets);
                parent.spawn(SpriteBundle {texture: inner,transform: Transform::from_xyz(0., 0., 4.),..default()});
            })
            .with_children(
                partial!(add_color_minitiles_children => _, elems, orig_len, true, assets, big_tile_size),
            )
            .with_children(partial!(add_arrow_minitile_children => _, dir, assets, big_tile_size));
    } else if let Tile::EndTile { t_, b_, l_, r_, elems, orig_len} = t
    {
        child
            .with_children(|parent| 
            {
                let inner = get_asset("e_base_inner.png".to_string(), assets);
                parent.spawn(SpriteBundle {texture: inner,transform: Transform::from_xyz(0., 0., 4.),..default()});
            })
            .with_children(
                partial!(add_color_minitiles_children => _, elems, orig_len, false, assets, big_tile_size),
            )
            .with_children(
                partial!(add_funnels_minitile_children => _, t_, b_, l_, r_, assets, big_tile_size),
            );
    } else if let Tile::PaintTile { track, c } = t {
        child.with_children(|parent| {
            let inner = get_asset(format!("p_{}.png", colorz_to_long_str(c)), assets);
            parent.spawn(SpriteBundle {
                texture: inner,
                transform: Transform::from_xyz(0., 0., 4.),
                ..default()
            });
        });
    } else if let Tile::SplitTile { side_in: _ } = t{
        child.with_children(|parent| {
            parent.spawn(SpriteBundle {
            texture: get_asset("scissor_u_inner.png".to_string(), assets),
            transform: Transform::from_xyz(0., 0., 4.),
            ..default()
        });
    });
    }
    return child.id();
}


