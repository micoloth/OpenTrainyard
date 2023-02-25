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

#[derive(Component)]
pub struct TextElem;  // Use this to qury text elems


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Copy, Clone)]
pub struct FullClickHappened {
    pub pos: Vec2
}

#[derive(Debug, Copy, Clone)]
pub struct ScrollHappened {
    pub vx: f32,
    pub vy: f32
}



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
                // Fraction is now in [0,1]. 
                // Tranform it by (1-x)^3:
                let newval = _get_scrollbar_value(fraction, &sblimits);
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






pub fn handle_gesture_mouse(
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


pub fn handle_gesture_touch(
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
                scroll_happened_writer.send(ScrollHappened{vx: delta.x, vy: delta.y});
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


pub fn make_scrollbar(
    mut commands: &mut Commands,
    assets: &TileAssets,
    scroll_bar_limits: ScrollBarLimits,
    button_colors: &ButtonColors,
    pleft: f32,
    pright: f32,
    ptop: f32,
    pbottom: f32,
    position_fraction: f32,
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
    let current_x = position_fraction * (pright - pleft);
    let current_val = _get_scrollbar_value(position_fraction, &scroll_bar_limits);
    let scroll_bar_limits = ScrollBarLimits {
        min: scroll_bar_limits.min,
        max: scroll_bar_limits.max,
        current: current_val,
        step: scroll_bar_limits.step,
    };
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


pub fn make_text(
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
    let mut ec = commands.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(pright - pleft), Val::Px(ptop - pbottom)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, // I have to say, this was cool ....
            position: UiRect {
                top: Val::Px(ptop),
                left: Val::Px(pleft),
                right: Val::Px(pright),
                bottom: Val::Px(pbottom),
            },
            ..default()
        },
        // background_color: button_colors.normal.into(),
        ..default()
    },
    ButtonData{text: text.clone()})
    );
    ec.with_children(|parent| {
        parent.spawn((TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        font: font_assets.fira_sans.clone(),
                        font_size: font_size,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                }],
                alignment: TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
            },
            ..default()
        }, TextElem{}));
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


fn _get_scrollbar_value(fraction: f32, sblimits: &ScrollBarLimits) -> f32 {
    let fraction = (1. - fraction).powf(6.);
                
    // New value using the fraction with  ScrollBarLimits { pub max: f32, pub min: f32, pub current: f32, pub step: f32,}:
    let newval = sblimits.min + (sblimits.max - sblimits.min) * fraction;
    // Round newval to the nearest step:
    let newval = (newval / sblimits.step).round() * sblimits.step;
    newval
}