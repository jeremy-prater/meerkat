use bevy::ecs::component::Component;
use nalgebra::Point3;

#[derive(Component)]
struct Position(Point3::<f32>);
