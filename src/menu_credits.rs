use crate::GameState;
use crate::all_puzzles_clean::PuzzlesData;
use crate::data_saving::SolutionData;
use crate::data_saving::SolutionsSavedData;
use crate::utils::SelectedLevel;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::loading::FontAssets;

use crate::loading::TileAssets;

use crate::menu_utils::make_button;
use crate::menu_utils::ButtonColors;


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

pub struct MenuCredits;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuCredits {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::MenuCredits).with_system(setup_menu_credits))
            .add_system_set(SystemSet::on_update(GameState::MenuCredits).with_system(click_back_button_credits))
            .add_system_set(SystemSet::on_exit(GameState::MenuCredits).with_system(cleanup_menu));
    }
}

#[derive(Component)]
pub struct BackButtonCredits;









/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////


fn setup_menu_credits(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TileAssets>,
    windows: Res<Windows>,
    tile_assets: Res<TileAssets>,
) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();
    //Write "TrainYard" at the top of the page:
    let mut ec = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, // Baseline, // FlexEnd, // Stretch, // Center, // I have to say, this was cool ....
            position: UiRect { top: Val::Px(height * 0.5), left: Val::Px(width / 2.), right: Val::Px(width / 2.), ..default() },
            ..default()
        },
        background_color: BackgroundColor(button_colors.hovered),
        transform: Transform::from_xyz(0., 0., 4.),
        ..default()
    });
    ec.insert(BackButtonCredits{});
    let ec_id = ec.id();
    let text_id = commands.spawn(TextBundle {
        style: Style { position_type: PositionType::Absolute, margin: UiRect::all(Val::Auto), ..default() },
        text: Text {
            sections: vec![
            TextSection {
                value: "Credits go entirely to the original creator of\nthis great puzzle:\n".to_string(),
                style: TextStyle { font: font_assets.fira_sans.clone(), font_size:22., color: Color::rgb(0.9, 0.9, 0.9), },
            },
            TextSection {
                value: "Matt Rix".to_string(),
                style: TextStyle { font: font_assets.fira_sans.clone(), font_size:35., color: Color::rgb(0.9, 0.9, 0.9), },
            },
            TextSection {
                value: "\n(@MattRix on Twitter)\n\n".to_string(),
                style: TextStyle { font: font_assets.fira_sans.clone(), font_size:22., color: Color::rgb(0.9, 0.9, 0.9), },
            },
            TextSection {
                value: "Reimplemented using the Bevy game engine\nby Micoloth\n(https://github.com/micoloth)".to_string(),
                style: TextStyle { font: font_assets.fira_sans.clone(), font_size:22., color: Color::rgb(0.9, 0.9, 0.9), },
            }
            ],
            alignment: TextAlignment{ vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center, },
        ..default()
    },
    ..default()
    }).id();
    commands.entity(ec_id) .push_children(&[text_id]);


    let margin_left = 13.;
    let rect_height = 35.;
    let offset_x = 0.;
    let offset_y = - height * 0.38;
    // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
    let left = margin_left;
    let right = margin_left + 100.;
    let top = height / 2. - rect_height / 2. + offset_y;
    let bottom = height / 2. - rect_height * 1.5 + offset_y;
    let startbutton_id = make_button("BACK".to_string(), &mut commands, &font_assets, &button_colors, 22. * 0.8, left, right, top, bottom, BackButtonCredits, Option::<BackButtonCredits>::None);
}




fn click_back_button_credits(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<Button>, With<BackButtonCredits>),>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::MenuTitle);
            }
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, With<BackButtonCredits>>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

