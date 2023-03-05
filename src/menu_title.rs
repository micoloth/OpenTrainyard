use crate::GameState;
use crate::all_puzzles_clean::PuzzlesData;
use crate::data_saving::SolutionData;
use crate::data_saving::{SolutionsSavedData, LOCAL_STORAGE_DATA_KEY};
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

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::MenuTitle).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MenuTitle)
                .with_system(click_play_button)
                .with_system(click_button_credits))
            .add_system_set(SystemSet::on_exit(GameState::MenuTitle).with_system(cleanup_menu));
    }
}

#[derive(Component)]
pub struct StartGameBotton;


#[derive(Component)]
pub struct ButtonCredits;

#[derive(Component)]
pub struct MainMenuElem;







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
    mut pkv: ResMut<PkvStore>,

    mut player_solutions_data: ResMut<SolutionsSavedData>,

) {
    // pkv.set(LOCAL_STORAGE_DATA_KEY, &SolutionsSavedData::default()).expect("failed to store level data");

    if let Ok(all_solutions) = pkv.get::<SolutionsSavedData>(LOCAL_STORAGE_DATA_KEY) {
        // Set resource:
        *player_solutions_data = all_solutions;
    } else {
        // Setplayer_solutions_data.levels to  an empty hashmap
        *player_solutions_data = SolutionsSavedData::default();
    }

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

    // Spawn UIImage icon_crop from assets above the button:
    commands.spawn(
        (SpriteBundle {
            texture: tile_assets.icon_crop.clone(),
            // transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(width / 2., width / 2.)),
                ..default()
            },
            // Scale down to 50% of the width:
            transform: Transform::from_xyz(0., height * 0.11, 0.),
            ..default()
    },
    MainMenuElem{}));
        
    //Write "Trainyard" at the top of the page:
    let mut ec = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, // Baseline, // FlexEnd, // Stretch, // Center, // I have to say, this was cool ....
            position: UiRect { top: Val::Px(height * 0.11), left: Val::Px(width / 2.), right: Val::Px(width / 2.), ..default() },
            ..default()
        },
        background_color: BackgroundColor(button_colors.hovered),
        transform: Transform::from_xyz(0., 0., 4.),
        ..default()
    });
    ec.insert(MainMenuElem{});
    let ec_id = ec.id();
    let text_id = commands.spawn(TextBundle {
        style: Style { position_type: PositionType::Absolute, margin: UiRect::all(Val::Auto), ..default() },
        text: Text {
            sections: vec![TextSection {
                value: "Trainyard".to_string(),
                style: TextStyle { font: font_assets.fira_sans.clone(), font_size: 45., color: Color::rgb(0.9, 0.9, 0.9), },
            }],
            alignment: TextAlignment{ vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center, },
        ..default()
    },
    ..default()
    }).id();
    commands.entity(ec_id) .push_children(&[text_id]);

    let rect_width = 120.;
    let rect_height = 50.;
    let offset_x = 0.;
    let offset_y = height * 0.15;
    // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
    let left = width / 2. - rect_width / 2. + offset_x;
    let right = width / 2. + rect_width / 2. + offset_x;
    let top = height / 2. - rect_height / 2. + offset_y;
    let bottom = height / 2. - rect_height * 1.5 + offset_y;
    let startbutton_id = make_button("PLAY".to_string(), &mut commands, &font_assets, &button_colors, 35., left, right, top, bottom, MainMenuElem, Some(StartGameBotton));


    let rect_width = 100.;
    let rect_height = 45.;
    let offset_x = 0.;
    let offset_y = height * 0.34;
    // Boundaries (left right top bottom) of a Rectangle that is Centered in the window:
    let left = width / 2. - rect_width / 2. + offset_x;
    let right = width / 2. + rect_width / 2. + offset_x;
    let top = height / 2. - rect_height / 2. + offset_y;
    let bottom = height / 2. - rect_height * 1.5 + offset_y;
    let startbutton_id = make_button("CREDITS".to_string(), &mut commands, &font_assets, &button_colors, 25., left, right, top, bottom, MainMenuElem, Some(ButtonCredits));
}




fn click_play_button(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<StartGameBotton>),
    >,
    player_solutions_data: Res<SolutionsSavedData>,
    mut selected_level: ResMut<SelectedLevel>,
    levels: Res<PuzzlesData>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if !player_solutions_data.just_begun() {
                    state.set(GameState::MenuLevels).unwrap();
                } else {
                    let level_name = levels.puzzles[0].name.clone();
                    let empty_map = levels.puzzles[0].parsed_map.clone();
                    let maps =  vec![SolutionData::new_from_string(empty_map.clone(), 0)];
                    *selected_level = SelectedLevel{
                        level: level_name.clone(),
                        player_maps: maps.clone(),
                        current_index: 0,
                        current_map: empty_map.clone(),
                        vanilla_map: empty_map,
                        city: "".to_string(),
                    };
                    state.set(GameState::Playing).unwrap();
                }
            }
            _ => {}
        }
    }
}

fn click_button_credits(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<Button>, With<ButtonCredits>),>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &mut interaction_query {
        println!("click_play_button");
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::MenuCredits);
            }
            _ => {}
        }
    }
}


fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, With<MainMenuElem>>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        if let Some(id) = commands.get_entity(button) {id.despawn_recursive();}
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

