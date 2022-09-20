use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;
use log::info;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<crate::resources::player::Player>,
) {
    info!("setup menu!");

    player.name = "Jones".to_string();

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(crate::components::main_menu::Camera);
    
    // note that we have to include the `Scene0` label
    let my_gltf = asset_server.load("o.gltf#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a TransformBundle
    // and spawn our gltf as a scene under it
    let o_model = commands
        .spawn_bundle(TransformBundle {
            local: Transform::identity(),
            global: GlobalTransform::identity(),
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: my_gltf,
                ..default()
            });
        })
        .insert(crate::components::main_menu::OModel);
        

    // const HALF_SIZE: f32 = 1.0;
    // let light = commands
    //     .spawn_bundle(DirectionalLightBundle {
    //         directional_light: DirectionalLight {
    //             shadow_projection: OrthographicProjection {
    //                 left: -HALF_SIZE,
    //                 right: HALF_SIZE,
    //                 bottom: -HALF_SIZE,
    //                 top: HALF_SIZE,
    //                 near: -10.0 * HALF_SIZE,
    //                 far: 10.0 * HALF_SIZE,
    //                 ..default()
    //             },
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(crate::components::main_menu::Light)
    //     .id();

    let exit_button = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("ARCADE.TTF"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                transform: Transform::from_xyz(20.0, 20.0, 0.0),
                ..default()
            });
        })
        .insert(crate::components::main_menu::ExitButton)
        .id();

    let menu = commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::rgb(0.5, 0.5, 0.5)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                //align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(crate::components::main_menu::MainMenu)
        .id();

    commands
        .entity(menu)
        .push_children(&[exit_button]);
}

pub fn main_menu_ui_system(
    mut commands: Commands,
    mut player: ResMut<crate::resources::player::Player>,
    mut ev: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                commands.insert_resource(NextState(crate::GameState::Connecting));
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}