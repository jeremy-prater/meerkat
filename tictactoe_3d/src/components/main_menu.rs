use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component, Debug)]
pub struct OModel {
    pub rotate_deg_s: Vec3,
}

#[derive(Component)]
pub struct Light;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct Camera;