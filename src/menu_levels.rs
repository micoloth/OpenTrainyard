use std::ops::ControlFlow;

use crate::GameState;
use crate::utils::Coordinates;
use crate::utils::SelectedLevel;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::loading::FontAssets;
use crate::menu_utils::*;

// Import PuzzlesData:
use crate::all_puzzles_clean::PuzzlesData;

use crate::menu_utils::make_button;
use crate::menu_utils::ButtonColors;


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
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(scroll_events_levels_touch)
                .with_system(scroll_events_levels_mouse)
                .with_system(click_play_button_levels)
                .with_system(click_play_button_levels)
                .with_system(handle_full_click_mouse)
                .with_system(handle_full_click_touch)
                .with_system(handle_full_click)
            )
            .add_system_set(SystemSet::on_exit(GameState::MenuLevels).with_system(cleanup_menu_levels))
            // Event FullClickHappened:
            .add_event::<FullClickHappened>()
            .add_event::<ScrollHappened>()
            ;
    }
}

#[derive(Component)]
pub struct LevelButton;

//Resource "ClickPosition", with a (x,y) tuple:
#[derive(Default, Resource, Debug, Copy, Clone, PartialEq)]
pub struct ClickPosition { // This is EXCLUDIVELY used because we want to record a click as a click-RELERASE in the SAME POSITION, so that a Touch-Swipe DON't trigger a click.
    pub clicked_pos: Option<Vec2>,
    pub last_hovered_pos: Option<Vec2>,

}
// Impl euclidean distance:
pub fn distance(one: &Vec2, other: &Vec2) -> f32 {
    let x = one.x - other.x;
    let y = one.y - other.y;
    (x * x + y * y).sqrt()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ClickState {JustClicked, Hovering, JustReleased}


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Copy, Clone)]
pub struct FullClickHappened {
    pub pos: Vec2
}

#[derive(Debug, Copy, Clone)]
pub struct ScrollHappened {
    pub vy: f32
}



/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


fn setup_menu_levels(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    levels: Res<PuzzlesData>,
    windows: Res<Windows>,
) {
    println!("YES IM HERE. good...");
    println!("Fingerss?????????");
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();

    // Get name of first level:
    let name = levels.puzzles[0].name.clone();
    let names = levels.puzzles.iter().map(|x| x.name.clone()).collect::<Vec<String>>();

    let rect_width = 320.;
    let rect_height = 40.;
    
    // One button per level name, stacked vertically:
    for (i, name) in names.iter().enumerate() {

        let offset_x = 0.;
        // Vertical offset:
        let offset_y = (i as f32) * rect_height;
        // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
        let left = width / 2. - rect_width / 2. + offset_x;
        let right = width / 2. + rect_width / 2. + offset_x;
        let top = height / 2. - rect_height / 2. + offset_y;
        let bottom = height / 2. - rect_height * 1.5 + offset_y;

        let startbutton_id = make_button(name.to_string(), &mut commands, &font_assets, &button_colors, 25., left, right, top, bottom, LevelButton, Option::<LevelButton>::None);
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


fn handle_full_click_mouse(
    mouse_input: Res<Input<MouseButton>>, 
    windows: Res<Windows>,
    mut click_position: Local<ClickPosition>,
    mut full_click_happened_writer: EventWriter<FullClickHappened>,
    mut scroll_happened_writer: EventWriter<ScrollHappened>,
) {
    if mouse_input.any_just_released([MouseButton::Left, MouseButton::Right]) {
        _touch_event_handler(&windows, &mut click_position, ClickState::JustReleased, &mut full_click_happened_writer, &mut scroll_happened_writer);
    }
    else if mouse_input.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        _touch_event_handler(&windows, &mut click_position, ClickState::JustClicked, &mut full_click_happened_writer, &mut scroll_happened_writer);
    }
    else if mouse_input.any_pressed([MouseButton::Left, MouseButton::Right]) {
        _touch_event_handler(&windows, &mut click_position, ClickState::Hovering, &mut full_click_happened_writer, &mut scroll_happened_writer);
    }
}


fn handle_full_click_touch(
    touches: Res<Touches>, 
    mut click_position: Local<ClickPosition>,
    windows: Res<Windows>,
    mut full_click_happened_writer: EventWriter<FullClickHappened>,
    mut scroll_happened_writer: EventWriter<ScrollHappened>,
) {
    for finger in touches.iter() {
        if touches.just_released(finger.id()) {
            _touch_event_handler(&windows, &mut click_position, ClickState::JustReleased, &mut full_click_happened_writer, &mut scroll_happened_writer);
        }
        else if touches.just_pressed(finger.id()) {
            _touch_event_handler(&windows, &mut click_position, ClickState::JustClicked, &mut full_click_happened_writer, &mut scroll_happened_writer);
        }
        else {
            _touch_event_handler(&windows, &mut click_position, ClickState::Hovering, &mut full_click_happened_writer, &mut scroll_happened_writer);
        }
        return;
    }
}



// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const SCROLLWHEEL_SPEED_MULTIPLIER: f32 = 3.;
const TRACKPAD_SPEED_MULTIPLIER: f32 = 0.8;

// Listen to scrollwheenl events:
fn scroll_events_levels_mouse(
    mut commands: Commands,
    mut scroll_evr: EventReader<MouseWheel>,
    mut button_query: Query<&mut Style,(With<Button>, With<LevelButton>),>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        let vy = match ev.unit {
            MouseScrollUnit::Line => {ev.y * SCROLLWHEEL_SPEED_MULTIPLIER}
            MouseScrollUnit::Pixel => {ev.y * TRACKPAD_SPEED_MULTIPLIER}
        };
        for mut style in button_query.iter_mut() {
            style.position.top = style.position.top.try_add(Val::Px(vy)).unwrap();
        }
    }
}

// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const TOUCH_SWIPE_SPEED_DECAY: f32 = 0.04;


// Listen to scrollwheenl events:
fn scroll_events_levels_touch(
    mut commands: Commands,
    mut current_vy: Local<Option<f32>>,
    mut button_query: Query<&mut Style,(With<Button>, With<LevelButton>),>,
    mut scroll_evr: EventReader<ScrollHappened>,
    // touches: Res<Touches>, 
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
        for mut style in button_query.iter_mut() {
            let v = style.position.top.try_sub_assign(Val::Px(*vy));
        }
    }
}




// Listen to event:
fn handle_full_click(
    mut full_click_happened_reader: EventReader<FullClickHappened>,
    mut state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for _ in full_click_happened_reader.iter() {
        info!("YEEEE Successfull Click!!! : ");
        if selected_level.level != ""
        {
            state.set(GameState::Playing).unwrap();
        }
    }
}



// state.set(GameState::Playing).unwrap();



fn cleanup_menu_levels(mut commands: Commands, buttons: Query<Entity, (With<Button>, With<LevelButton>)>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////


// ClickState {JustClicked, Hovering, JustReleased}

fn _touch_event_handler(
    windows: &Windows, 
    click_position: &mut ClickPosition, 
    state: ClickState,
    full_click_happened_writer: &mut EventWriter<FullClickHappened>,
    scroll_happened_writer: &mut EventWriter<ScrollHappened>
) {
let window = windows.get_primary().expect("no primary window");
let pos = window.cursor_position();
let window_size = Vec2::new(window.width(), window.height());
// If Some(Vec2), substract Window size: 
let clicked_pos = match pos {
    Some(pos) => Some(pos - window_size / 2.),
    None => None,
};

match state {
    ClickState::JustClicked => {
        click_position.clicked_pos = clicked_pos;
        click_position.last_hovered_pos = clicked_pos;
    }
    ClickState::Hovering => {
        if click_position.last_hovered_pos.is_some() && clicked_pos.is_some() {
            let last_pos = click_position.last_hovered_pos.unwrap();
            let new_pos = clicked_pos.unwrap();
            let delta = new_pos - last_pos;
            scroll_happened_writer.send(ScrollHappened{vy: delta.y});
        }
        click_position.last_hovered_pos = clicked_pos;
    }
    ClickState::JustReleased => {
        if click_position.clicked_pos.is_some() && click_position.last_hovered_pos.is_some() && distance(&click_position.clicked_pos.unwrap(), &click_position.last_hovered_pos.unwrap()) < 5.
        {
            info!("YEEEE Successfull Click!!! : pos{:?}", clicked_pos);
            full_click_happened_writer.send(FullClickHappened{pos: click_position.clicked_pos.unwrap()});
        }
        else{
            info!("NOO Aborted Click!!! : pos{:?}", clicked_pos);
        }
        click_position.clicked_pos = None;
        click_position.last_hovered_pos = None;
    }
}
}
