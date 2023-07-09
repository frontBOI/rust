use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(laser_movement)
            .add_system(laser_hit_detection)
            .add_system(laser_detect_outside_screen);
    }
}
