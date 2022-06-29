use bevy::ecs::component::Component;
use bevy::transform::components::Transform;

#[derive(Component)]
struct Position(Transform);
