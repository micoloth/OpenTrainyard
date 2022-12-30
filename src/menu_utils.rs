
use crate::loading::FontAssets;

use bevy::ui::widget::ImageMode;
use bevy::ui::FocusPolicy;
use bevy::prelude::*;

use crate::loading::TileAssets;

/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

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



#[derive(Debug, Component, Clone, PartialEq, Copy, Default)] pub struct ScrollBarLimits { max: f32, min: f32, current: f32, step: f32,}
#[derive(Debug, Component, Clone, PartialEq, Copy, Default)] pub struct ScrollBarPosition { max_x: f32, min_x: f32, current_x: f32, step_x: f32,}
#[derive(Debug, Component, Clone, PartialEq, Copy, Default)] pub struct ScrollBarStatus { dragging: bool }
#[derive(Bundle, Clone, Debug, Default)]
pub struct ScrollBarHandleBundle {
    pub scrollBarLimits: ScrollBarLimits,
    pub scrollBarPosition: ScrollBarPosition,
    pub scrollBarStatus: ScrollBarStatus,

    pub sprite: Sprite,

    // Flattened components of ImageBundle AND ButtonBundle - Nice!   (kinda .....)
    pub node: Node,
    pub button: Button,
    pub style: Style,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    pub color: UiColor,
    pub texture: UiImage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub calculated_size: CalculatedSize,
    pub image_mode: ImageMode,
}

#[derive(Component)]
pub struct BorderElem;


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum BorderEvent{
    Spawn,
    Despawn,
}


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////



pub fn button_color_handler(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<(&Interaction, &mut UiColor),(Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = button_colors.pressed.into();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}


pub fn scrollbar_input_handler(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<(&Interaction, &mut UiColor, &mut ScrollBarPosition, &mut ScrollBarLimits, &mut ScrollBarStatus),(Changed<Interaction>)>,
) {
    for (interaction, mut color, mut sbpos, mut sblimits, mut sbstatus) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {println!("NICE!!"); sbstatus.dragging = true;}
            Interaction::Hovered => {}
            Interaction::None => {sbstatus.dragging = false;}
        }
    }
}


pub fn scrollbar_dragging_handler(
    button_colors: Res<ButtonColors>,
    windows: Res<Windows>,
    mut interaction_query: Query<(&mut Transform, &mut GlobalTransform, &mut Style, &mut ScrollBarPosition, &mut ScrollBarLimits, &mut ScrollBarStatus)>,
) {
    for (mut transform, mut gltr, mut style, mut sbpos, mut sblimits, mut sbstatus) in interaction_query.iter_mut() {
        if sbstatus.dragging {
            let window = windows.get_primary().expect("no primary window");
            let window_size = Vec2::new(window.width(), window.height());
            if let Some(pos) = window.cursor_position() {
                let handle_x = (sbpos.max_x - sbpos.min_x) * 0.07;

                let relposx = pos.x - sbpos.min_x - handle_x/2.;
                let relposx = relposx.clamp(0., sbpos.max_x - sbpos.min_x - handle_x);
                // let position = pos - window_size / 2.;
                let fraction = relposx / (sbpos.max_x - sbpos.min_x - handle_x);
                println!("THANKSSS, {:?}", fraction);

                // print current x position of the image from transform:
                // transform.translation.x = pos_x;
                // let mut old_pos:Rect<Val> = style.position;
                // old_pos.left = Val::Px(pos_x as f32);
                // old_pos.right = Val::Px((pos_x as f32) + 20.);
                sbpos.current_x = relposx;
                style.position.left = Val::Px(relposx );

            }

        }
    }
}



// Cleanup all the elems with Border component:
pub fn handle_border(
    mut commands: Commands, 
    elems: Query<Entity, With<BorderElem>>,
    // Listen to the event:
    mut events: EventReader<BorderEvent>,
) {
    // Iterate events:
    for event in events.iter() {
        match event {
            BorderEvent::Spawn => {
                // Spawn the border:
                make_border(&mut commands);
            }
            BorderEvent::Despawn => {
                // Despawn the border:
                for elem in elems.iter() {
                    commands.entity(elem).despawn_recursive();
                }
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////


pub fn make_scrollbar(
    mut commands: &mut Commands,
    assets: &TileAssets,
    pleft: f32,
    pright: f32,
    ptop: f32,
    pbottom: f32,
)-> Entity {
    // let arrow = assets.s_arrow_elem_rigth.clone();
    let back = commands.spawn_bundle(
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(pright - pleft), Val::Px(ptop-pbottom)),
                //     // margin: UiRect::all(Val::Auto),
                //     // justify_content: JustifyContent::Center,
                //     // align_items: AlignItems::Center,
                position: UiRect {
                    top: Val::Px(pleft),
                    left: Val::Px(ptop),
                    ..default()
                },
                ..default()
            },
            color: Color::rgb(1.0, 1.0, 1.0).into(),
            ..default()
        }).id();
    let handle = commands.spawn_bundle(
        ScrollBarHandleBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(7.), Val::Percent(100.)),
                position: UiRect {
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            // texture: UiImage(arrow),
            color: UiColor(Color::rgb(1., 1.0, 1.0)),
            scrollBarLimits: ScrollBarLimits { max: 100., min: 0., current: 0., step: 1.},
            scrollBarPosition: ScrollBarPosition{ max_x: pright, min_x: pleft, current_x: pright, step_x: 1.},
            ..default()
    }).id();
    commands.entity(back).push_children(&[handle]);// add the child to the parent
    return back;

}



pub fn make_button(
        text: String,
        mut commands: &mut Commands,
        font_assets: &FontAssets,
        button_colors: &ButtonColors,
        pleft: f32,
        pright: f32,
        ptop: f32,
        pbottom: f32,
    ) -> Entity {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(pright - pleft), Val::Px(ptop-pbottom)),
                margin: UiRect::all(Val::Auto), 
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,  // I have to say, this was cool ....
                position: UiRect {
                    top: Val::Px(ptop),
                    left: Val::Px(pleft),
                    ..default()
                },
                ..default()
            },
            color: button_colors.normal,
            ..default()
        })
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
        }).id()
}





fn make_border(
    mut commands: &mut Commands,
) {
    // Draw a UiImage that has only a border, no fill. The border is yellow. It should be AS BIG AS THE SCREEN.
    // Do it by creating 4 different narrow rectangles, at each side of the screen:
    // Left rectangle:
    commands.spawn_bundle(
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(4.), Val::Percent(100.)),
                position: UiRect {
                    // Left of the screen, using percentages:
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            // yellow color:
            color: Color::rgb(1.0, 1.0, 0.0).into(),
            ..default()
        }).insert(BorderElem);
    // Right rectangle:
    commands.spawn_bundle(
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(4.), Val::Percent(100.)),
                position: UiRect {
                    // Right of the screen, using percentages:
                    top: Val::Px(0.),
                    right: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            // yellow color:
            color: Color::rgb(1.0, 1.0, 0.0).into(),
            ..default()
        }).insert(BorderElem);
    // Top rectangle:
    commands.spawn_bundle(
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.), Val::Px(4.)),
                position: UiRect {
                    // Top of the screen, using percentages:
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            // yellow color:
            color: Color::rgb(1.0, 1.0, 0.0).into(),
            ..default()
        }).insert(BorderElem);
    // Bottom rectangle:
    commands.spawn_bundle(
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.), Val::Px(4.)),
                position: UiRect {
                    // Bottom of the screen, using percentages:
                    bottom: Val::Px(0.),
                    left: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            // yellow color:
            color: Color::rgb(1.0, 1.0, 0.0).into(),
            ..default()
        }).insert(BorderElem);
}


// fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, (With<Button>, With<MainGameBotton>)>) {
//     // For button in query:
//     for button in buttons.iter() { // It's never more than 1, but can very well be 0
//         commands.entity(button).despawn_recursive();
//     }
// }

