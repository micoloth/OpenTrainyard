use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::NamedField;

use crate::menu_utils::*;

use crate::loading::TileAssets;
use bevy::time::FixedTimestep;  // 0.9: Thi is in Time, not in core


use crate::train::*;
use crate::tile::*;
use crate::board::*;
use crate::logic::*;
use crate::logic::TicksInATick;
use crate::menu_utils::*;
use crate::all_puzzles_clean::*;


// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 120.0;

/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Component, Default, Resource)]
pub struct GameScreenState {
    pub name: String,
    pub solved_map: String,

}


pub struct MainGamePlugin;


/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(get_board_option_default())
            .insert_resource(get_ticks_in_a_tick_default())
            .insert_resource(GameScreenState::default())
            // .insert_resource(GamePlayingState())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(init_gmae),)
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup_board),)
            .add_system_set(
                //////////// MAIN LOGIC:
                SystemSet::on_update(GameState::Playing)
                .with_system(spawn_tile)
                .with_system(create_board)
                .with_system(logic_tick_event)
                .with_system(change_tick_speed)
                .with_system(listen_to_game_state_changes)
                //////////// INTERACTIONS:
                .with_system(tile_hover_mouse)
                .with_system(tile_hover_touch)
                .with_system(tile_hover_event)
                .with_system(double_click_mouse)
                .with_system(double_click_touch)
                .with_system(double_click_event)
                //////////// OTHERS/COSMETICS:
                .with_system(add_borders)
                .with_system(style_run_button)
            )
            /////////////// MOVE TRAINS:
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_trains)   
            )
            .add_event::<TileSpawnEvent>()
            .add_event::<LogicTickEvent>()
            .add_event::<DoubleClickEvent>()
            .add_event::<TileHoverEvent>()
            .add_event::<ScrollBarLimits>()
            .add_event::<BoardEvent>()
            ;
    }
}



pub struct MenuMainGame;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuMainGame {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_game_menu))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(click_undo_button))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(click_erase_button))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(click_run_button))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(click_nextlevel_button))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(scrollbar_input_handler))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(scrollbar_dragging_handler))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(add_borders))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(cleanup_menu))
            ;
    }
}



#[derive(Component)]
pub struct MainGameBotton;

#[derive(Component)]
pub struct EraseStateButton;

#[derive(Component)]
pub struct RunButton;

#[derive(Component)]
pub struct UndoButton;

#[derive(Component)]
pub struct NextLevelButton;

#[derive(Component)]
pub struct LevelNameElem;

/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////






/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////



fn setup_game_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TileAssets>,
    windows: Res<Windows>,
) {
    let (width, margin, heigh, percent_left_right, left, right, bottom, top) = get_coordinates(&windows);

    let erase_id = make_button("Erase".to_string(), &mut commands, &font_assets, &button_colors, 35., left, right, top - heigh - margin, bottom - heigh - margin, EraseStateButton, Some(MainGameBotton));
    let undo_id = make_button("Undo".to_string(), &mut commands, &font_assets, &button_colors, 35., left, right , top, bottom, UndoButton, Some(MainGameBotton));
    let run_id = make_button("Run!".to_string(), &mut commands, &font_assets, &button_colors, 35., width * percent_left_right + margin/2., width - margin , top, bottom, RunButton, Some(MainGameBotton));
    let scrollbar_id = make_scrollbar(&mut commands, &textures, 
        ScrollBarLimits { max: 80./120., min: 0.05/120., current: 3./120., step: 0.01 / 120.},
        &button_colors,
        width * percent_left_right + margin/2., width - margin , top - heigh - margin, bottom - heigh - margin,
        MainGameBotton);
    // Next level:
    let (left_, right_, bottom_, top_) = get_upper_coordinates(&windows);
    let next_level_id = make_button("Next level".to_string(), &mut commands, &font_assets, &button_colors, 20., left_, right_, top_, bottom_, MainGameBotton, Some(NextLevelButton));
}

fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, (With<Button>, With<MainGameBotton>)>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) { id.despawn_recursive();};
    }
}




fn init_gmae(
    mut commands: Commands,
    mut game_screen_state: ResMut<GameScreenState>,
    // BoardEvent event writer:
    mut board_event_writer: EventWriter<BoardEvent>,
    // Query existing boards:
    board_q: Query<Entity, With<Board>>,
    levels: Res<PuzzlesData>,
    // Query all elems with the LevelNameElem component:
    level_name_query: Query<Entity, With<LevelNameElem>>,
    // Windows:
    windows: Res<Windows>,
    // Fonts:
    font_assets: Res<FontAssets>,
    // Button colors:
    button_colors: Res<ButtonColors>,
) {
    change_level("Boomerang".to_string(), &mut game_screen_state, &levels, &board_q, &mut commands, &mut board_event_writer, &level_name_query, &windows, &font_assets, &button_colors);
}

fn click_nextlevel_button(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<NextLevelButton>)>,
    mut game_screen_state: ResMut<GameScreenState>,
    // BoardEvent event writer:
    mut board_event_writer: EventWriter<BoardEvent>,
    // Query existing boards:
    board_q: Query<Entity, With<Board>>,
    levels: Res<PuzzlesData>,
    // Query all elems with the LevelNameElem component:
    level_name_query: Query<Entity, With<LevelNameElem>>,
    // Windows:
    windows: Res<Windows>,
    // Fonts:
    font_assets: Res<FontAssets>,
    // Button colors:
    button_colors: Res<ButtonColors>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(next_puzzle) = get_next_puzzle(game_screen_state.name.clone(), &levels) {
                    change_level(next_puzzle.name, &mut game_screen_state, &levels, &board_q, &mut commands, &mut board_event_writer, &level_name_query, &windows, &font_assets, &button_colors);

                }
            }
            _ => {}
        }
    }
}




// Button action: Listen to EraseStateButton and print Erased to console when clicked:
fn click_erase_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<EraseStateButton>),
        >,
    mut board_q: Query<(Entity, &mut BoardHoverable, &mut BoardGameState), With<Board>>,
) {
    for (interaction, _) in &mut interaction_query {
        for (_, _, mut hovering_state) in board_q.iter_mut() {
            match *interaction {
                Interaction::Clicked => {
                    match *hovering_state {
                        BoardGameState::Erasing =>{
                            *hovering_state = BoardGameState::Drawing;
                        },
                        BoardGameState::Drawing => {
                            *hovering_state = BoardGameState::Erasing;
                        },
                        _ => {}
                    };
                }
                _ => {}
            }
        }
    }
}

fn click_undo_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<UndoButton>),
        >,
    mut spawn_event: EventWriter<TileSpawnEvent>,
    mut board_q: Query<(Entity, &mut BoardHoverable, &BoardGameState), With<Board>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                for (_, mut board_hoverable, hovering_state) in board_q.iter_mut() {
                    // if hovering_state is Running, continue:
                    if let BoardGameState::Running(_) = *hovering_state {continue;}
                    println!("TRIGGERED UNDO! {:?}", board_hoverable.history.history);
                    let last_event = board_hoverable.history.history.pop();
                    if let Some(TileSpawnEvent { x, y, new_tile, prev_tile }) = last_event {
                        if let Some(prev_tile) = prev_tile {
                            spawn_event.send(TileSpawnEvent { x, y, new_tile: prev_tile, prev_tile: None });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn click_run_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<RunButton>),>,
    mut board_q: Query<(Entity, &mut BoardHoverable, &mut BoardGameState), With<Board>>,
) {
    for interaction in &mut interaction_query {
        for (_, _, mut hovering_state) in board_q.iter_mut() {
            match *interaction {
                Interaction::Clicked => {
                    println!("TRIGGERED RUN!");
                    match *hovering_state {
                        BoardGameState::Erasing | BoardGameState::Drawing =>{
                            *hovering_state = BoardGameState::Running(RunningState::Started);
                        },
                        BoardGameState::Running(_) => {
                            *hovering_state = BoardGameState::Drawing;
                        },
                    };
                }
                _ => {}
            }
        }
    }
}

pub fn style_run_button(
    mut commands: Commands,
    //Listen to the RunButton and the HoveringState:
    mut interaction_query: Query<Entity, (With<Button>, With<RunButton>)>,
    mut board_q: Query<&BoardGameState, (With<Board>, Changed<BoardGameState>)>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    windows: Res<Windows>,
) {
    for hovering_state in board_q.iter() {
        match *hovering_state {
            BoardGameState::Running(_) => {
                // Despawn the button:
                for entity in interaction_query.iter_mut() {
                    if let Some(id) = commands.get_entity(entity) { id.despawn_recursive();}
                }
                // Rebuild:
                let (width, margin, heigh, percent_left_right, left, right, bottom, top) = get_coordinates(&windows);
                let run_id = make_button("Stop".to_string(), &mut commands, &font_assets, &button_colors, 35., width * percent_left_right + margin/2., width - margin , top, bottom, RunButton, Some(MainGameBotton));
                    
            },
            _ => {
                // Despawn the button:
                for entity in interaction_query.iter_mut() {
                    if let Some(id) = commands.get_entity(entity) { id.despawn_recursive();}
                }
                // Rebuild:
                let (width, margin, heigh, percent_left_right, left, right, bottom, top) = get_coordinates(&windows);
                let run_id = make_button("Run!".to_string(), &mut commands, &font_assets, &button_colors, 35., width * percent_left_right + margin/2., width - margin , top, bottom, RunButton, Some(MainGameBotton));
            }
        }
    }
}

// Listen to CHANGES in the GameScreenState:
fn add_borders(
    mut commands: Commands,
    // use Option, not to panic if the resource doesn't exist yet
    board_q: Query<&BoardGameState, (With<Board>, Changed<BoardGameState>)>,
    // Query borders:
    elems: Query<Entity, With<BorderElem>>,
) {
    for hovering_state in board_q.iter() {
        println!("TRIGGEREDDDDDDD, {:?}", hovering_state);
        // Despawn all the borders:
        for elem in elems.iter() {
            if let Some(id) = commands.get_entity(elem) { id.despawn_recursive();}
        }
        // Make new ones:
        match *hovering_state {
            BoardGameState::Running(RunningState::Crashed) => {
                make_border(&mut commands,  Color::rgb(130./255., 9./255., 0.));
            },
            BoardGameState::Running(RunningState::Won) => {
                make_border(&mut commands,  Color::rgb(0., 130./255., 0.));
            },
            BoardGameState::Erasing =>
                make_border(&mut commands,  Color::rgb(1., 1., 0.)),

            _ => {}
        };           
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////


fn change_level(
        level_name: String, 
        game_screen_state: &mut GameScreenState, 
        levels: &PuzzlesData, 
        board_q: &Query<Entity, With<Board>>, 
        commands: &mut Commands, 
        board_event_writer: &mut EventWriter<BoardEvent>, 
        level_name_query: &Query<Entity,  With<LevelNameElem>>, 
        windows: &Windows, 
        font_assets: &FontAssets, 
        button_colors: &ButtonColors) {
    // Delete board:
    for board_id in board_q.iter() {
        if let Some(id) = commands.get_entity(board_id) { id.despawn_recursive();}
    }
    // Set the name of the game:
    game_screen_state.name = level_name;
    // Send the event to create the board:
    board_event_writer.send(BoardEvent::Make(game_screen_state.name.clone()));
    // Print the game name:
    println!("LAUNCHED: {}", game_screen_state.name.clone());
    let map = levels.puzzles.iter().find(|p| p.name == *game_screen_state.name.clone()).unwrap().parsed_map.clone();    
    println!(">> map:: {}", map);
    // Despawn the level name:
    for level_name_id in level_name_query.iter() {
        if let Some(level_name_ec) = commands.get_entity(level_name_id) {level_name_ec.despawn_recursive();}
    }
    // Spawn the level name BUTTON:
    let (left_, right_, bottom_, top_) = get_upper_coordinates(windows);
    let name_id = make_button(game_screen_state.name.clone(), commands, font_assets, button_colors, 20., left_ - 110., right_ -110., top_, bottom_, MainGameBotton, Some(LevelNameElem));

}


fn get_coordinates(windows: &Windows) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    // Boundaries (left right top bottom) of a Rectangle that occupies the LEFT HALF of the screen, minus a 20 pixel wide margin all around:
    let margin = 20.;
    let heigh = 40.;
    let percent_left_right = 0.35;
    let left = margin;
    let right = width * percent_left_right - margin/2.;
    // Make the button 40 px high FROM THE BOTTOM:
    let bottom = height - margin - 2.*40.;
    let top = height - margin - heigh;
    (width, margin, heigh, percent_left_right, left, right, bottom, top)
}

fn get_upper_coordinates(windows: &Windows) -> (f32, f32, f32, f32) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    // Boundaries (left right top bottom) of a Rectangle that occupies the RIGHT HALF of the screen, minus a 20 pixel wide margin all around:
    let margin = 20.;
    let heigh = 30.;
    // Position it at the TOP of the screen:
    let percent_left_right = 0.65;
    let left = width * percent_left_right + margin/2.;
    let right = width - margin;
    let top = margin;
    let bottom = margin - heigh;
    return (left, right, bottom, top);
}


