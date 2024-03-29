use bevy::{prelude::*};

use log::info;

const COLOR_NAME_OK: Color = Color::rgb(0.0, 0.6, 0.2);
const COLOR_NAME_BAD: Color = Color::rgb(0.6, 0.2, 0.0);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn name_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut player: ResMut<crate::resources::player::Player>,
    cloud: ResMut<crate::resources::cloud::CloudClient>,
) {
    let mut modified = false;
    for ev in char_evr.iter() {
        if ev.char.is_alphanumeric() && (player.name.len() < 8) {
            player.name.push(ev.char);
            modified = true;
        }
    }

    for key in keys.get_just_pressed() {
        match key {
            KeyCode::Return => {}
            KeyCode::Backslash | KeyCode::Delete | KeyCode::Back => {
                player.name.pop();
                modified = true;
            }
            _ => {}
        }
    }

    if modified {
        cloud.get_name_available(player.name.clone());
        info!("Player name : {}", player.name);
    }
}

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _player: ResMut<crate::resources::player::Player>,
) {
    info!("setup menu!");

    let exit_button = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Name",
                    TextStyle {
                        font: asset_server.load("ARCADE.TTF"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                ..default()
            });
        })
        .insert(crate::components::main_menu::ExitButton)
        .id();

    let menu = commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
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

    commands.entity(menu).push_children(&[exit_button]);
}

#[allow(clippy::type_complexity)]
pub fn main_menu_ui_system(
    _commands: Commands,
    player: ResMut<crate::resources::player::Player>,
    cloud: Res<crate::resources::cloud::CloudClient>,
    // _ev: EventWriter<AppExit>,
    // mut interaction_query: Query<
    //     (&Interaction, &mut BackgroundColor, &Children),
    //     (Changed<Interaction>, With<Button>),
    // >,
    mut update_name_query: Query<(&mut BackgroundColor, &Children), With<Button>>,
    mut text_query: Query<&mut Text>,
) {
    // for (interaction, mut color, children) in interaction_query.iter_mut() {
    //     let mut text = text_query.get_mut(children[0]).unwrap();
    //     match *interaction {
    //         Interaction::Clicked => {
    //             text.sections[0].value = "Press".to_string();
    //             *color = PRESSED_BUTTON.into();
    //             commands.insert_resource(NextState(crate::GameState::Connecting));
    //         }
    //         Interaction::Hovered => {
    //             text.sections[0].value = "Hover".to_string();
    //             *color = HOVERED_BUTTON.into();
    //         }
    //         Interaction::None => {
    //             text.sections[0].value = "Button".to_string();
    //             *color = NORMAL_BUTTON.into();
    //         }
    //     }
    // }

    for (mut color, children) in update_name_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value = player.name.clone();
        let data = cloud.data.get_name_available.read().unwrap();
        *color = match *data {
            true => COLOR_NAME_OK.into(),
            false => COLOR_NAME_BAD.into(),
        };
    }
}
