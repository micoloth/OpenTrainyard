use crate::GameState;
use bevy::prelude::*;

use crate::loading::FontAssets;

use crate::loading::TileAssets;

use crate::menu_utils::make_button;
use crate::menu_utils::ButtonColors;


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::MenuTitle).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::MenuTitle).with_system(click_play_button))
            .add_system_set(SystemSet::on_exit(GameState::MenuTitle).with_system(cleanup_menu));
    }
}

#[derive(Component)]
pub struct StartGameBotton;









/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TileAssets>,
    windows: Res<Windows>,
    tile_assets: Res<TileAssets>,
) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    println!("YES IM HERE. good...");
    commands.spawn(
        // NodeBundle{..default()}).with_children(|parent| {parent.spawn(
        SpriteBundle {
            texture: tile_assets.background_solutions.clone(),
            // transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            // Scale down to 50% of the width:
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
    });

    let rect_width = 120.;
    let rect_height = 50.;
    let offset_x = 0.;
    let offset_y = 0.;
    // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
    let left = width / 2. - rect_width / 2. + offset_x;
    let right = width / 2. + rect_width / 2. + offset_x;
    let top = height / 2. - rect_height / 2. + offset_y;
    let bottom = height / 2. - rect_height * 1.5 + offset_y;

    let startbutton_id = make_button("Play".to_string(), &mut commands, &font_assets, &button_colors, 35., left, right, top, bottom, StartGameBotton, Option::<StartGameBotton>::None);
    // make_scrollbar(&mut commands, &textures, 50., 250., 50., 25.);
}




fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<StartGameBotton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::MenuLevels).unwrap();
            }
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, With<Button>>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

