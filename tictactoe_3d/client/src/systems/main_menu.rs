use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use log::info;

const COLOR_NAME_OK: Color = Color::srgb(0.0, 0.6, 0.2);
const COLOR_NAME_BAD: Color = Color::srgb(0.6, 0.2, 0.0);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn name_input(
    mut char_evr: EventReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<crate::resources::player::Player>,
    cloud: ResMut<crate::resources::cloud::CloudClient>,
) {
    let mut modified = false;
    for ev in char_evr.read() {
        if player.name.len() < 8 && ev.state == ButtonState::Pressed {
            if let bevy::input::keyboard::Key::Character(char) = &ev.logical_key {
                player.name.push_str(char);
                modified = true;
            }
        }
    }

    for key in keys.get_just_pressed() {
        match key {
            KeyCode::Enter => {}
            KeyCode::Delete | KeyCode::Backspace => {
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

    commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        ))
        .insert(crate::components::main_menu::MainMenu)
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON.into()),
                ))
                .with_child((
                    Text::new("Name"),
                    TextFont {
                        font: asset_server.load("ARCADE.TTF"),
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ))
                .insert(crate::components::main_menu::ExitButton);
        });
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
        text.0 = player.name.clone();
        let data = cloud.data.get_name_available.read().unwrap();
        *color = match *data {
            true => COLOR_NAME_OK.into(),
            false => COLOR_NAME_BAD.into(),
        };
    }
}
