
use crate::GameState;
use crate::data_saving::{SolutionsSavedData, SolutionData};
use crate::loading::TileAssets;
use crate::utils::SelectedLevel;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::loading::FontAssets;
use crate::menu_utils::*;

// Import PuzzlesData:
use crate::all_puzzles_clean::PuzzlesData;



/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

pub struct MenuLevelsPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuLevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::MenuLevels).with_system(setup_menu_levels))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels)
                .with_system(scroll_events_levels_touch)
                .with_system(scroll_events_levels_mouse)
                .with_system(handle_gesture_mouse)
                .with_system(handle_gesture_touch)
                .with_system(handle_full_click)
                .with_system(click_play_button_levels)
                .with_system(click_back_button_levels)
            )
            // ButtonColors resource:
            .insert_resource(MenuLimits{..default()})
            .add_system_set(SystemSet::on_exit(GameState::MenuLevels).with_system(cleanup_menu_levels))
            // Event FullClickHappened:
            .add_event::<FullClickHappened>()
            .add_event::<ScrollHappened>()
            ;
    }
}

#[derive(Component)]
pub struct LevelButton{}

#[derive(Component)]
pub struct Banner{}

#[derive(Component)]
pub struct BackButtonLevels{}


// Resource with fields max_firstbutton_heigh amd min_firstbutton_heigh:
#[derive(Default, Resource, Clone, Copy)]
pub struct MenuLimits {
    pub max_firstbutton_heigh: f32,
    pub min_firstbutton_heigh: f32,
    pub current_firstbutton_heigh: f32,
}


const BANNER_HEIGHT: f32 = 50.;



/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////





/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////



fn setup_menu_levels(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
    windows: Res<Windows>,
    player_solutions_data: Res<SolutionsSavedData>,
    tile_assets: Res<TileAssets>,
    // Resourvce:
    mut menu_limits: ResMut<MenuLimits>,
) {
    println!("YES IM HERE. good...");
    println!("Fingerss?????????");
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();

    // Get name of first level:
    let names = levels.puzzles.iter().map(|x| x.name.clone()).collect::<Vec<String>>();

    // Get index in names of selected_level.level:
    let index_selected_level = if selected_level.level != ""{
        names.iter().position(|x| x == &selected_level.level).unwrap()
    }
    else {
        0
    };

    let rect_width = 320.;
    let rect_height = 40.;

    make_top_banner( &mut commands, &font_assets, &button_colors, 30., width / 2. - rect_width / 2., width / 2. + rect_width / 2., BANNER_HEIGHT);

    // Heigh of the entire menu:
    let menu_height = (names.len() as f32) * rect_height;
    let max_firstbutton_heigh = BANNER_HEIGHT ;
    let min_firstbutton_heigh = BANNER_HEIGHT - menu_height - rect_height - rect_height + height;
    // Set resource:
    menu_limits.max_firstbutton_heigh = max_firstbutton_heigh;
    menu_limits.min_firstbutton_heigh = min_firstbutton_heigh;

    println!("max_firstbutton_heigh: {}", max_firstbutton_heigh);
    println!("min_firstbutton_heigh: {}", min_firstbutton_heigh);


    // LET'S SAY SELECTED_LEVEL = LAST. 
    // # Currently, its rect ('s top) has coordinates:
    // max_firstbutton_heigh + (i as f32) * rect_height
    // While i want it to be:
    // max_firstbutton_heigh

    // so we have to SUBTRACT:
    let starting_offset_for_selected_level = - (index_selected_level as f32) * rect_height + height / 2.;

    println!("starting_offset_for_selected_level: {}", starting_offset_for_selected_level);

    // BUT: it cannot be BIGGER THAN max_firstbutton_heigh
    let starting_offset_for_selected_level = starting_offset_for_selected_level.min(0.);
    // OR SMALLER THAN min_firstbutton_heigh
    let starting_offset_for_selected_level = starting_offset_for_selected_level.max(min_firstbutton_heigh - BANNER_HEIGHT);


    
    // One button per level name, stacked vertically:
    for (i, name) in names.iter().enumerate() {

        // if name is in player_solutions_data.levels, AND the object is of type Solved, then we have solved it, and we can make the button green
        let score = match player_solutions_data.levels.get(name)
        {
            Some(solutions) => {
                // Get the solution (map, tracks, and second tracks) of the object with the MIN number of tracks:
                // Filter the solutions where time > 0:
                let solutions: Vec<&SolutionData> = solutions.iter().filter(|x| x.time > 0).collect();
                if solutions.len() == 0 {
                    "".to_string()
                }
                else {
                    let solution = solutions.iter().min_by_key(|x| x.tracks).unwrap();
                    format!("({}+{})", solution.tracks, solution.second_tracks)
                }
            },
            _ => {"".to_string()}
        };

        let offset_x = 0.;
        // Vertical offset:
        let offset_y = (i as f32) * rect_height + starting_offset_for_selected_level;
        // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
        let left = width / 2. - rect_width / 2. + offset_x;
        let right = width / 2. + rect_width / 2. + offset_x;
        let top = max_firstbutton_heigh + offset_y;
        let bottom = max_firstbutton_heigh - rect_height + offset_y;

        if i == 0 {
            // Set resource:
            menu_limits.current_firstbutton_heigh = top;
        }

        make_menu_elem(name.to_string(), score, i as u16, &mut commands, &font_assets, &button_colors, 25., left, right, top, bottom, &tile_assets);
        selected_level.level = "".to_string();
    }
}


fn click_back_button_levels(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<BackButtonLevels>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::MenuTitle);
            }
            _ => {}
        }
    }
}



fn click_play_button_levels(
    mut selected_level: ResMut<SelectedLevel>,
    mut interaction_query: Query<
        (&Interaction, &ButtonData),
        (Changed<Interaction>, With<Button>, With<LevelButton>),
    >,
) {
    for (interaction, button_data) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                selected_level.level = button_data.text.clone();
            }
            _ => {}
        }
    }
}


// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const SCROLLWHEEL_SPEED_MULTIPLIER: f32 = 0.03;
const TRACKPAD_SPEED_MULTIPLIER: f32 = 0.8;

// Listen to scrollwheenl events:
fn scroll_events_levels_mouse(
    mut scroll_evr: EventReader<MouseWheel>,
    mut button_query: Query<(&mut Style, &LevelButton),(With<Button>, With<LevelButton>),>,
    // resource:
    mut menu_limits: ResMut<MenuLimits>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        let vy = match ev.unit {
            MouseScrollUnit::Line => {ev.y * SCROLLWHEEL_SPEED_MULTIPLIER}
            MouseScrollUnit::Pixel => {ev.y * TRACKPAD_SPEED_MULTIPLIER}
        };
        let delta = vy.clamp(menu_limits.min_firstbutton_heigh - menu_limits.current_firstbutton_heigh, menu_limits.max_firstbutton_heigh - menu_limits.current_firstbutton_heigh);
        if delta != 0. {
            menu_limits.current_firstbutton_heigh += delta;
            for (mut style, _) in button_query.iter_mut() {
                style.position.top.try_add_assign(Val::Px(delta));
            }
        }

    }
}

// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const TOUCH_SWIPE_SPEED_DECAY: f32 = 0.04;


// Listen to scrollwheenl events:
fn scroll_events_levels_touch(
    mut current_vy: Local<Option<f32>>,
    mut button_query: Query<(&mut Style, &LevelButton),(With<Button>, With<LevelButton>),>,
    mut scroll_evr: EventReader<ScrollHappened>,
    // touches: Res<Touches>, 
    mut menu_limits: ResMut<MenuLimits>,
) {
    
    if let Some(vy) = current_vy.as_ref() {
        let new_vy = vy * (1. - TOUCH_SWIPE_SPEED_DECAY);
        if new_vy.abs() > 0.1 { *current_vy = Some(new_vy);} 
        else {*current_vy = None;}
    }
    // for finger in touches.iter() {
    //     *current_vy = Some(finger.delta().y);
    //     let finger_pos = format!("{:?}", finger.position());
    // }
    for ev in scroll_evr.iter() {
        *current_vy = Some(ev.vy);
    }
    if let Some(vy) = current_vy.as_ref() {
        let vy = -vy;
        let delta = vy.clamp(menu_limits.min_firstbutton_heigh - menu_limits.current_firstbutton_heigh, menu_limits.max_firstbutton_heigh - menu_limits.current_firstbutton_heigh);
        if delta != 0. {
            menu_limits.current_firstbutton_heigh += delta;
            for (mut style, _) in button_query.iter_mut() {
                style.position.top.try_add_assign(Val::Px(delta));
            }
        }
    }
}




// Listen to event:
fn handle_full_click(
    mut full_click_happened_reader: EventReader<FullClickHappened>,
    mut state: ResMut<State<GameState>>,
    selected_level: Res<SelectedLevel>,
    windows: Res<Windows>,
) {
    for ev in full_click_happened_reader.iter() {
        info!("YEEEE Successfull Click!!! : ");
        let height = windows.get_primary().unwrap().height();
        if selected_level.level != "" && ev.pos.y < height / 2. - BANNER_HEIGHT
        {
            state.set(GameState::MenuSolutions).unwrap();
        }
    }
}



// state.set(GameState::Playing).unwrap();



fn cleanup_menu_levels(
    mut commands: Commands, 
    buttons: Query<Entity, (With<Button>, With<LevelButton>)>,
    banners: Query<Entity, With<Banner>>,
) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
    // For button in query:
    for button in banners.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////
/// 


pub fn make_top_banner(
    commands: &mut Commands,
    font_assets: &FontAssets,
    button_colors: &ButtonColors,
    font_size: f32,
    pleft: f32,
    pright: f32,
    height : f32,
) -> Entity {
    // Make a top banner that is always at the top 100 pixels of the window:
    let mut ec = commands.spawn(ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(pright - pleft), Val::Px(height)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, // Baseline, // FlexEnd, // Stretch, // Center, // I have to say, this was cool ....
            position: UiRect {
                top: Val::Px(0.),
                left: Val::Px(pleft),
                ..default()
            },

            ..default()
        },
        background_color: BackgroundColor(button_colors.hovered),
        transform: Transform::from_xyz(0., 0., 2.),
        ..default()
    });
    ec.insert(Banner{});
    let ec_id = ec.id();
    // Add a child TextBundle that says "Pick level:"
    let text_id = commands.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Pick level:".to_string(),
                style: TextStyle {
                    font: font_assets.fira_sans.clone(),
                    font_size: font_size,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            }],
            alignment: TextAlignment{
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
        },
        ..default()
    },
    ..default()
    }).id();
    commands.entity(ec_id) .push_children(&[text_id]);

    // Make a "Back" button at the left of  the banner, centered vertically:
    let margin = 10.; // Margin around the "Back" button
    let button_width = 50.;
    let back_bottom = margin *3.  - height;
    let back_top =  margin;
    let back_left = margin;
    let back_right = margin + button_width;
    let back_id = make_button("BACK".to_string(), commands, &font_assets, &button_colors, 20., back_left, back_right, back_top, back_bottom, BackButtonLevels{}, None::<Banner>);
    commands.entity(ec_id) .push_children(&[back_id]);
    ec_id
}


pub fn make_menu_elem(
    name: String,
    score: String,
    index: u16,
    commands: &mut Commands,
    font_assets: &FontAssets,
    button_colors: &ButtonColors,
    font_size: f32,
    pleft: f32,
    pright: f32,
    ptop: f32,
    pbottom: f32,
    tile_assets: &TileAssets,
) -> Entity {
    let mut ec = commands.spawn((ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(pright - pleft), Val::Px(ptop - pbottom)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center, // Baseline, // FlexEnd, // Stretch, // Center, // I have to say, this was cool ....
            position: UiRect {
                top: Val::Px(ptop),
                left: Val::Px(pleft),
                ..default()
            },
            ..default()
        },
        background_color: button_colors.normal.into(),
        ..default()
    },
    ButtonData{text: name.clone()})
    );
    ec.with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: name,
                    style: TextStyle {
                        font: font_assets.fira_sans.clone(),
                        font_size: font_size,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                }],
                alignment: TextAlignment{
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
            },
            style: Style{margin: UiRect{left: Val::Px(20.), ..default()}, ..default()},
            ..default()
        });
        if score != "".to_string() {
            parent.spawn(
                // NodeBundle{..default()}).with_children(|parent| {parent.spawn(
                ImageBundle {
                    image: UiImage(tile_assets.tick.clone()),
                    // transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                    style: Style { 
                        //display: (), position_type: (), direction: (), flex_direction: (), flex_wrap: (), align_items: (), align_self: (), align_content: (), justify_content: (), position: (), margin: (), padding: (), border: (), flex_grow: (), flex_shrink: (), flex_basis: (), size: (), min_size: (), max_size: (), aspect_ratio: (), overflow: () }
                        // Center vertically and put at 66% of the width:
                        position_type: PositionType::Relative,
                        margin: UiRect{right: Val::Px(2.), left: Val::Px(70.), ..default()},
                        // Align to the RIGHT of the parent object:
                        align_items: AlignItems::FlexEnd,
                        
                        ..default()
                        
                    },
                    // Scale down to 50% of the width:
                    transform: Transform{..default()}.with_scale(Vec3::splat(0.45)),
                    ..default()
            });  
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: score,
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: font_size,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: TextAlignment{
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Right,
                    },
                },
                style: Style{
                    align_items: AlignItems::FlexEnd,
                    margin: UiRect{left: Val::Px(2.), right: Val::Px(20.), ..default()}, ..default()
                },
                ..default()
            });
        // }); // HERE
        };
    });
    ec.insert(LevelButton{});

    return ec.id();
}

