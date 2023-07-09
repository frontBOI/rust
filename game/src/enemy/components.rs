use bevy::prelude::Component;

#[derive(Component)]
pub struct Enemy {
    pub direction: Direction,
    pub previous_x_direction: Direction,
}

#[derive(Component, PartialEq, Clone)]
pub enum Direction {
    Right,
    Bottom,
    Left,
}
