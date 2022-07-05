use bevy::ecs::component::Component;
use multiarray::*;

#[derive(Component)]
struct GameGrid {
    grid: Array3D<i32>,
}
