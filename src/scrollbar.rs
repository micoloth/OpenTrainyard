

use bevy::ui::widget::ImageMode;
use bevy::ui::FocusPolicy;
use bevy::prelude::*;

use crate::menu::ButtonColors;

use crate::loading::TileAssets;

/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


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
}

pub fn getScrollBarHandleBundle(img: Handle<Image>, limits: ScrollBarLimits) -> ScrollBarHandleBundle {
    let mut sbh = ScrollBarHandleBundle{
        texture: UiImage(img),
        transform: Transform::from_translation(Vec3::new(0., 0., 3.)).with_scale(Vec3::splat(5.)),
        ..Default::default()
    };

    return sbh;
}


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////



/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////



pub fn scrollbar_input_handler(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<(&Interaction, &mut UiColor, &mut ScrollBarPosition, &mut ScrollBarLimits, &mut ScrollBarStatus),(Changed<Interaction>)>,
) {
    for (interaction, mut color, mut sbpos, mut sblimits, mut sbstatus) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // Set the image transparency to 0.5:
                println!("NICE!!");
                if let Color::Rgba { red, green, blue, alpha } = color.0.as_rgba(){  // Surprise surprise!
                    *color = Color::rgba(red, green, blue, 0.5).into();
                }
                sbstatus.dragging = true;
            }
            Interaction::Hovered => {
                if let Color::Rgba { red, green, blue, alpha } = color.0.as_rgba(){  // Surprise surprise!
                    *color = Color::rgba(red, green, blue, 0.1).into();
                }
            }
            Interaction::None => {
                *color = button_colors.normal.into();
                sbstatus.dragging = false;
            }
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
                println!("THANKSSS, {:?}", pos);
                let position = pos - window_size / 2.;
                let pos_x = position.x;
                sbpos.current_x = pos_x;

                // print current x position of the image from transform:
                // transform.translation.x = pos_x;
                // let mut old_pos:Rect<Val> = style.position;
                // old_pos.left = Val::Px(pos_x as f32);
                // old_pos.right = Val::Px((pos_x as f32) + 20.);
                style.position.left = Val::Px(pos_x);
                
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
) {
    let arrow = assets.s_arrow_elem_rigth.clone();
    commands
        .spawn_bundle(
            ScrollBarHandleBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                    //     // margin: UiRect::all(Val::Auto),
                    //     // justify_content: JustifyContent::Center,
                    //     // align_items: AlignItems::Center,
                    position: UiRect {
                        top: Val::Px(50.0),
                        left: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                texture: UiImage(arrow),
                transform: Transform::from_translation(Vec3::new(0., -200., 3.)).with_scale(Vec3::splat(5.)),
                ..Default::default()
        })
        // .with_children(|builder| {
        //     builder.spawn_bundle(
        //         getScrollBarHandleBundle(arrow, ScrollBarLimits { max: 100., min: 0., current: 0., step: 1.})
        //     );
        // })
        ;
}