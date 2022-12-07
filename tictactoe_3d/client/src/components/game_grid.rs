use bevy::ecs::component::Component;
use multiarray::*;

#[derive(Component)]
struct GameGrid(Array3D<i32>);
