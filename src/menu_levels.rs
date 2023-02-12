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
            .init_resource::<ClickPosition>()
            .add_system_set(SystemSet::on_enter(GameState::MenuLevels).with_system(setup_menu_levels))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(click_play_button_levels))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(scroll_events_levels_mouse))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(scroll_events_levels_touch))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(click_play_button_levels))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(release_mouse))
            .add_system_set(SystemSet::on_update(GameState::MenuLevels).with_system(release_touch))
            .add_system_set(SystemSet::on_exit(GameState::MenuLevels).with_system(cleanup_menu_levels));
    }
}

#[derive(Component)]
pub struct LevelButton;

//Resource "ClickPosition", with a (x,y) tuple:
#[derive(Default, Resource, Debug, Copy, Clone, PartialEq)]
pub struct ClickPosition { // This is EXCLUDIVELY used because we want to record a click as a click-RELERASE in the SAME POSITION, so that a Touch-Swipe DON't trigger a click.
    pub pos: Vec2
}
// Impl euclidean distance:
pub fn distance(one: &Vec2, other: &Vec2) -> f32 {
    let x = one.x - other.x;
    let y = one.y - other.y;
    (x * x + y * y).sqrt()
}


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
    levels: Res<PuzzlesData>,
    windows: Res<Windows>,
) {
    println!("YES IM HERE. good...");
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
    // make_scrollbar(&mut commands, &textures, 50., 250., 50., 25.);
}

// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const SCROLLWHEEL_SPEED_MULTIPLIER: f32 = 5.;

// Listen to scrollwheenl events:
fn scroll_events_levels_mouse(
    mut scroll_evr: EventReader<MouseWheel>,
    mut button_query: Query<&mut Style,(With<Button>, With<LevelButton>),>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        for mut style in button_query.iter_mut() {
            let vy = match ev.unit {
                MouseScrollUnit::Line => {ev.y * SCROLLWHEEL_SPEED_MULTIPLIER}
                MouseScrollUnit::Pixel => {ev.y}
            };
            style.position.top = style.position.top.try_add(Val::Px(vy)).unwrap();
        }
    }
}

// Set constan SCROLLWHEEL_SPEED_MULTIPLIER:
const TOUCH_SWIPE_SPEED_DECAY: f32 = 0.05;


// Listen to scrollwheenl events:
fn scroll_events_levels_touch(
    touches: Res<Touches>, 
    mut current_vy: Local<Option<f32>>,
    mut button_query: Query<&mut Style,(With<Button>, With<LevelButton>),>,
) {
    if let Some(vy) = current_vy.as_ref() {
        let new_vy = vy * (1. - TOUCH_SWIPE_SPEED_DECAY);
        if new_vy > 0.1 { *current_vy = Some(new_vy);} 
        else {*current_vy = None;}
    }
    for finger in touches.iter() {
        // Get finger movement delta:
        *current_vy = Some(finger.delta().y);
    }
    if let Some(vy) = current_vy.as_ref() {
        for mut style in button_query.iter_mut() {
            style.position.top = style.position.top.try_add(Val::Px(*vy)).unwrap();
        }
    }
}


fn click_play_button_levels(
    mut selected_level: ResMut<SelectedLevel>,
    mut interaction_query: Query<
        (&Interaction, &ButtonData),
        (Changed<Interaction>, With<Button>, With<LevelButton>),
    >,
    windows: Res<Windows>,
    mut click_position: ResMut<ClickPosition>,
) {
    for (interaction, button_data) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                selected_level.level = button_data.text.clone();
                let window = windows.get_primary().expect("no primary window");
                let pos = match window.cursor_position() { None => return, Some(b) => b, };
                let window_size = Vec2::new(window.width(), window.height());
                click_position.pos = pos - window_size / 2.;  
            }
            _ => {}
        }
    }
}

fn release_mouse(
    mouse_input: Res<Input<MouseButton>>, 
    windows: Res<Windows>,
    click_position: Res<ClickPosition>,
    mut state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    if mouse_input.any_just_released([MouseButton::Left, MouseButton::Right]) {
        let window = windows.get_primary().expect("no primary window");
        let pos = match window.cursor_position() { None => return, Some(b) => b, };  // In touches, it would be continue
        let window_size = Vec2::new(window.width(), window.height());
        let pos = pos - window_size / 2.;

        if distance(&click_position.pos, &pos) < 5. && selected_level.level != "".to_string() { 
            state.set(GameState::Playing).unwrap();
        } // If the click was released in a different position, it was a swipe, not a click.
        else {
            selected_level.level = "".to_string();
        }
    }
}

fn release_touch(
    touches: Res<Touches>, 
    click_position: Res<ClickPosition>,
    mut state: ResMut<State<GameState>>,
    mut selected_level: ResMut<SelectedLevel>,
) {
    for finger in touches.iter() {
        if touches.just_released(finger.id()) {
            // Get last finger pos:
            let pos = finger.position();
            
            if distance(&click_position.pos, &pos) < 5. && selected_level.level != "".to_string(){ 
                state.set(GameState::Playing).unwrap();
            } // If the click was released in a different position, it was a swipe, not a click.
            else {
                selected_level.level = "".to_string();
            }
        }
    }
}


// pub fn tile_hover_touch(touches: Res<Touches>, windows: Res<Windows>, mut hover_event: EventWriter<TileHoverEvent>,) {
//     for finger in touches.iter() {
//         if touches.just_released(finger.id()) {
//             hover_event.send(TileHoverEvent::Released);
//             break;
//         }
//         else {
//             let window = windows.get_primary().expect("no primary window");
//             let pos = match window.cursor_position() { None => continue, Some(b) => b, };
//             let window_size = Vec2::new(window.width(), window.height());
//             let pos = pos - window_size / 2.;            
//             hover_event.send(TileHoverEvent::Newhover(pos));
//         }
//     }
// }


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

