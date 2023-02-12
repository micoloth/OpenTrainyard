use crate::loading::FontAssets;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::widget::ImageMode;
use bevy::ui::FocusPolicy;

use crate::loading::TileAssets;

/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Resource)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
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

#[derive(Debug, Component, Clone, PartialEq, Copy, Default)]
pub struct ScrollBarLimits {
    pub max: f32,
    pub min: f32,
    pub current: f32,
    pub step: f32,
}
#[derive(Debug, Component, Clone, PartialEq, Copy, Default)]
pub struct ScrollBarPosition {
    max_x: f32,
    min_x: f32,
    current_x: f32,
    step_x: f32,
}
#[derive(Debug, Component, Clone, PartialEq, Copy, Default)]
pub struct ScrollBarStatus {
    dragging: bool,
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct ScrollBarHandleBundle {
    pub scroll_bar_limits: ScrollBarLimits,
    pub scroll_bar_position: ScrollBarPosition,
    pub scroll_bar_status: ScrollBarStatus,

    pub sprite: Sprite,

    // Flattened components of ImageBundle AND ButtonBundle - Nice!   (kinda .....)
    pub node: Node,
    pub button: Button,
    pub style: Style,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    pub background_color: BackgroundColor,
    pub texture: UiImage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub calculated_size: CalculatedSize,
    pub image_mode: ImageMode,
    pub z_index: ZIndex,
}

#[derive(Component)]
pub struct BorderElem;

#[derive(Component)]
pub struct ButtonData {
    pub text: String
}

/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////

pub fn button_color_handler(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
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
    // Listen to mouse inputs:
    mouse_button_input: Res<Input<MouseButton>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut ScrollBarPosition,
            &mut ScrollBarLimits,
            &mut ScrollBarStatus,
        ),
        (Changed<Interaction>),
    >,
) {
    for (interaction, mut color, mut sbpos, mut sblimits, mut sbstatus) in
        interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                sbstatus.dragging = true;
            }
            Interaction::Hovered => {}
            Interaction::None => {
                sbstatus.dragging = false;
            }
        }
        // If mouse just release left button:
        if mouse_button_input.just_released(MouseButton::Left) {
            sbstatus.dragging = false;
        }
    }
}

pub fn scrollbar_dragging_handler(
    windows: Res<Windows>,
    mut interaction_query: Query<(
        &mut Transform,
        &mut GlobalTransform,
        &mut Style,
        &mut ScrollBarPosition,
        &mut ScrollBarLimits,
        &mut ScrollBarStatus,
    )>,
    mut dragged_event_writer: EventWriter<ScrollBarLimits>,
) {
    for (mut transform, mut gltr, mut style, mut sbpos, mut sblimits, mut sbstatus) in
        interaction_query.iter_mut()
    {
        if sbstatus.dragging {
            let window = windows.get_primary().expect("no primary window");
            if let Some(pos) = window.cursor_position() {
                let handle_x = (sbpos.max_x - sbpos.min_x) * 0.12;

                let relposx = pos.x - sbpos.min_x - handle_x / 2.;
                let relposx = relposx.clamp(0., sbpos.max_x - sbpos.min_x - handle_x);
                // let window_size = Vec2::new(window.width(), window.height());
                // let position = pos - window_size / 2.;
                let fraction = relposx / (sbpos.max_x - sbpos.min_x - handle_x);

                // New value using the fraction with  ScrollBarLimits { pub max: f32, pub min: f32, pub current: f32, pub step: f32,}:
                let newval = sblimits.min + (sblimits.max - sblimits.min) * fraction;
                // Round newval to the nearest step:
                let newval = (newval / sblimits.step).round() * sblimits.step;
                // println!("THANKSSS, {:?}", newval);
                // Update ScrollBarPosition:
                sbpos.current_x = relposx;
                style.position.left = Val::Px(relposx);
                // launch event:
                if newval != sblimits.current {
                    sblimits.current = newval;
                    dragged_event_writer.send(*sblimits);
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
    scroll_bar_limits: ScrollBarLimits,
    button_colors: &ButtonColors,
    pleft: f32,
    pright: f32,
    ptop: f32,
    pbottom: f32,
    type_: impl Bundle,
) -> Entity {
    // let arrow = assets.s_arrow_elem_rigth.clone();
    let back = commands
        .spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(pright - pleft), Val::Px(ptop - pbottom)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center, // I have to say, this was cool ....
                position: UiRect {
                    top: Val::Px(ptop),
                    left: Val::Px(pleft),
                    ..default()
                },
                ..default()
            },
            background_color: Color::rgb(1.0, 1.0, 1.0).into(),
            ..default()
        })
        .insert(type_)
        .id();
    // get the fraction from (scroll_bar_limits.current - min) / (scroll_bar_limits.max- min)
    // and apply it to ScrollBarPosition{ max_x: pright, min_x: pleft} to get the current_x:
    let fraction = (scroll_bar_limits.current - scroll_bar_limits.min)
        / (scroll_bar_limits.max - scroll_bar_limits.min);
    let current_x = fraction * (pright - pleft);
    let handle = commands
        .spawn(ScrollBarHandleBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(12.), Val::Percent(100.)),
                position: UiRect {
                    top: Val::Px(0.),
                    left: Val::Px(current_x),
                    ..default()
                },
                ..default()
            },
            // texture: UiImage(arrow),
            background_color: BackgroundColor(Color::rgb(1., 1.0, 1.0)),
            scroll_bar_limits: scroll_bar_limits,
            scroll_bar_position: ScrollBarPosition {
                max_x: pright,
                min_x: pleft,
                current_x: current_x,
                step_x: 1.,
            },
            ..default()
        })
        .id();
    commands.entity(back).push_children(&[handle]); // add the child to the parent
    return back;
}

pub fn make_button(
    text: String,
    commands: &mut Commands,
    font_assets: &FontAssets,
    button_colors: &ButtonColors,
    font_size: f32,
    pleft: f32,
    pright: f32,
    ptop: f32,
    pbottom: f32,
    type1: impl Bundle,
    type2: Option<impl Bundle>,
) -> Entity {
    let mut ec = commands.spawn((ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(pright - pleft), Val::Px(ptop - pbottom)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, // I have to say, this was cool ....
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
    ButtonData{text: text.clone()})
    );
    ec.with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        font: font_assets.fira_sans.clone(),
                        font_size: font_size,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                }],
                alignment: default(),
            },
            ..default()
        });
    });
    ec.insert(type1);
    if let Some(type2) = type2 {
        ec.insert(type2);
    }
    return ec.id();
}





pub fn make_border(commands: &mut Commands, color: Color) {
    // Draw a UiImage that has only a border, no fill. The border is yellow. It should be AS BIG AS THE SCREEN.
    // Do it by creating 4 different narrow rectangles, at each side of the screen:
    // Left rectangle:
    commands
        .spawn(ImageBundle {
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
            background_color: color.into(),
            ..default()
        })
        .insert(BorderElem);
    // Right rectangle:
    commands
        .spawn(ImageBundle {
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
            background_color: color.into(),
            ..default()
        })
        .insert(BorderElem);
    // Top rectangle:
    commands
        .spawn(ImageBundle {
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
            background_color: color.into(),
            ..default()
        })
        .insert(BorderElem);
    // Bottom rectangle:
    commands
        .spawn(ImageBundle {
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
            background_color: color.into(),
            ..default()
        })
        .insert(BorderElem);
}

// fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, (With<Button>, With<MainGameBotton>)>) {
//     // For button in query:
//     for button in buttons.iter() { // It's never more than 1, but can very well be 0
//         commands.entity(button).despawn_recursive();
//     }
// }
