use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Player {
    pub can_shoot: bool,
}
