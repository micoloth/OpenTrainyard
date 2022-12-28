use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;

use crate::scrollbar::*;

use crate::loading::TileAssets;


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(scrollbar_input_handler))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(scrollbar_dragging_handler))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
    }
}

pub struct ButtonColors {
    pub normal: UiColor,
    pub hovered: UiColor,
    pub pressed: UiColor
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            pressed: Color::rgb(0.35, 0.35, 0.35).into(),
        }
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
) {
    commands.spawn_bundle(Camera2dBundle::default());
    make_button("Play".to_string(), &mut commands, &font_assets, &button_colors);
    make_scrollbar(&mut commands, &textures);
}




fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<StartGameBotton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, With<Button>>) {
    // For button in query:
    for button in buttons.iter() { // It's never more than 1, but can very well be 0
        commands.entity(button).despawn_recursive();
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////



fn make_button(text: String, mut commands: &mut Commands, font_assets: &FontAssets, button_colors: &ButtonColors) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: button_colors.normal,
            ..default()
        }).insert(StartGameBotton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text,
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: default(),
                },
                ..default()
            });
        });
}


// // fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, asset_shapes: Res<BoardAssetsMap>) {
//     fn setup_ui(mut commands: &mut Commands, asset_server: &AssetServer, asset_shapes: &BoardAssetsMap) {
//         let button_materials = ButtonColors {
//             normal: Color::GRAY,
//             hovered: Color::DARK_GRAY,
//             pressed: Color::BLACK,
//         };
//         commands
//             .spawn_bundle(NodeBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(100.), Val::Px(50.)),
//                     align_items: AlignItems::Center,
//                     justify_content: JustifyContent::Center,
    
//                     ..default()
//                 },
//                 color: Color::WHITE.into(),
//                 ..default()
//             })
//             .insert(Name::new("UI"))
//             .with_children(|parent| {
//                 let font = asset_server.load("fonts/pixeled.ttf");
//                 setup_single_menu(
//                     parent,
//                     "CLEAR",
//                     button_materials.normal.into(),
//                     font.clone(),
//                     ButtonAction::Clear,
//                 );
//                 // setup_scrollbar(
//                 //     parent,
//                 //     "GENERATE",
//                 //     button_materials.normal.into(),
//                 //     font,
//                 //     ButtonAction::Generate,
//                 //     &asset_shapes.assets
//                 // );
//             });
//         commands.insert_resource(button_materials);
//     }
    
//     fn setup_single_menu(
//         parent: &mut ChildBuilder,
//         text: &str,
//         color: UiColor,
//         font: Handle<Font>,
//         action: ButtonAction,
//     ) {
//         parent
//             .spawn_bundle(ButtonBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(95.), Val::Auto),
//                     margin: Rect::all(Val::Px(10.)),
//                     // horizontally center child text
//                     justify_content: JustifyContent::Center,
//                     // vertically center child text
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 color,
//                 ..default()
//             })
//             .insert(action)
//             .insert(Name::new(text.to_string()))
//             .with_children(|builder| {
//                 builder.spawn_bundle(TextBundle {
//                     text: Text {
//                         sections: vec![TextSection {
//                             value: text.to_string(),
//                             style: TextStyle {
//                                 font,
//                                 font_size: 30.,
//                                 color: Color::WHITE,
//                             },
//                         }],
//                         alignment: TextAlignment {
//                             vertical: VerticalAlign::Center,
//                             horizontal: HorizontalAlign::Center,
//                         },
//                     },
//                     ..default()
//                 });
//             });
//     }
    
    