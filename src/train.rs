use bevy::prelude::*;

use crate::simulator::*;


use crate::board::*;
use crate::logic::*;
use crate::loading::TrainAssets;


////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Bundle)]
pub struct TrainBundle{
    pub train: Train,

    // Flattened SpriteBundle #[bundle] : SO NICE!!
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility, // User indication of whether an entity is visible
    pub computed_visibility: ComputedVisibility,
}
impl Default for TrainBundle {
    fn default() -> Self {
        Self {
            train: Train { c: Colorz::BROWN_, pos: Pos { px: 0, py: 0, side: Side::T_, going_in: true, towards_side: Some(Side::B_) } },
            sprite: default(),
            transform: default(),
            global_transform: default(),
            texture: default(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


pub fn move_trains(
    mut trains_q: Query<(&mut Train, &mut Transform)>, 
    // windows: Res<Windows>,
    mut board_q: Query<(&mut BoardTileMap, &BoardDimensions, &BoardGameState, &mut BoardTickStatus), With<Board>>,
    tick_params: ResMut<TicksInATick>,
    mut logic_tick_event: EventWriter<RedrawTrainsEvent>) {
        
    for (mut board_tilemap, board_dimensions, game_state, mut tick_status) in board_q.iter_mut() {    // Really, there's just 1 board
        // If board_hoverable.game_state is NOT running, continue:
        match game_state { BoardGameState::Running(_) => {}, _ => {continue;}}
        tick_status.current_tick += 1;
        if tick_status.current_tick >= tick_params.ticks {
            tick_status.current_tick = 0;
            tick_status.first_half = true;
            let (new_tilemap, new_trains) = logic_tick_core(&board_tilemap, TickMoment::TickEnd, *game_state);
            board_tilemap.map = new_tilemap.clone();
            board_tilemap.current_trains = new_trains.clone();
            logic_tick_event.send(RedrawTrainsEvent {tiles: new_tilemap, trains: new_trains});
        } else if tick_status.current_tick >= ((tick_params.ticks as f32 / 2.) as u32)  && tick_status.first_half {
            let (new_tilemap, new_trains) = logic_tick_core(&mut board_tilemap, TickMoment::TickMiddle, *game_state);
            board_tilemap.map = new_tilemap.clone();
            board_tilemap.current_trains = new_trains.clone();
            logic_tick_event.send(RedrawTrainsEvent {tiles: new_tilemap, trains: new_trains});
            tick_status.first_half = false;
        }
        for (train, mut transform) in trains_q.iter_mut() {
            *transform = get_train_transform(*train, board_dimensions, (tick_status.current_tick as f32) / (tick_params.ticks as f32));
        }
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////


fn get_train_image(train_assets: &TrainAssets, color: Colorz) -> Handle<Image> {
    match color {
        Colorz::RED_ => train_assets.train_red.clone(),
        Colorz::BLUE_ => train_assets.train_blue.clone(),
        Colorz::YELLOW_ => train_assets.train_yellow.clone(),
        Colorz::ORANGE_ => train_assets.train_orange.clone(),
        Colorz::GREEN_ => train_assets.train_green.clone(),
        Colorz::PURPLE_ => train_assets.train_purple.clone(),
        Colorz::BROWN_ => train_assets.train_brown.clone(),
    }
}


fn get_train_transform(t:Train, board: &BoardDimensions, tick_rateo: f32) -> Transform {
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 3.0));
    let in_side: Side = t.pos.side;
    let out_side: Side = t.pos.towards_side.unwrap();

    let angle = tick_rateo * 0.5 * std::f32::consts::PI;

    let (x, y, train_angle) =  match (in_side, out_side) {
        // For STRAIGHT tracks (in/out is right/left or top/bottom), the train should go from left to rigth or right to left. Use tick_rateo to get the fraction of the way.
        // ASSUME THAT the tile has side 1, and the origin is in the TOP LEFT corner ( x=0 is leftmost, Y=0 is topmost)
        (Side::L_, Side::R_) => {(tick_rateo, 0.5, 0.)},
        (Side::R_, Side::L_) => {(1. - tick_rateo, 0.5, - std::f32::consts::PI)},
        (Side::T_, Side::B_) => {(0.5, 1. - tick_rateo, - std::f32::consts::PI / 2.)},
        (Side::B_, Side::T_) => {(0.5, tick_rateo, std::f32::consts::PI / 2.)},
        // For CURVED tracks, the train should do a CURVED arc from one side of the tile to the other, PIVOTING AROUND THE CONER, that is a CONCAVE arc towards the center of the tile.
        (Side::R_, Side::T_) => {( 1.- 0.5*angle.sin(), 1. - 0.5 * angle.cos(), - angle + std::f32::consts::PI)},
        (Side::T_, Side::L_) => {(0.5* angle.cos(), 1. - 0.5 * angle.sin(), -angle - std::f32::consts::PI /2.)},
        (Side::L_, Side::B_) => {( 0.5*angle.sin(), 0.5 * angle.cos(), - angle)},
        (Side::B_, Side::R_) => {(1. - 0.5* angle.cos(), 0.5*angle.sin(), - angle + std::f32::consts::PI /2.)},
        (Side::T_, Side::R_) => {(1. - 0.5* angle.cos(), 1.- 0.5 * angle.sin(), angle - std::f32::consts::PI /2.)},
        (Side::R_, Side::B_) => {(1. - 0.5*angle.sin(), 0.5 * angle.cos(), angle + std::f32::consts::PI)},
        (Side::B_, Side::L_) => {(0.5* angle.cos(), 0.5 * angle.sin(), angle + std::f32::consts::PI /2.)},
        (Side::L_, Side::T_) => {( 0.5*angle.sin(), 1. - 0.5 * angle.cos(), angle)},
        _ => {panic!("WTF")}
        };
        transform.translation.x = (x + t.pos.px as f32) * board.tile_size;
        transform.translation.y = (y + (6 - t.pos.py)as f32) * board.tile_size;
        transform.rotation = Quat::from_rotation_z( train_angle);
        
        transform.scale = Vec3::splat(1.);
        
        return transform;
}


pub fn make_train(train: Train, commands: &mut Commands, train_assets: &TrainAssets, board_dimensions: &BoardDimensions, tick_rateo: f32) -> Entity {
    let child = commands.spawn(TrainBundle {
        train: train,
        texture: get_train_image(train_assets, train.c),
        transform: get_train_transform(train, board_dimensions, tick_rateo),
        // sprite: Sprite { custom_size: Some(Vec2::splat(board_dimensions.tile_size)), color: Color::WHITE, ..default()},
        ..default()
    });
    return child.id();
}

