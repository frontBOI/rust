mod enemy;
mod events;
mod laser;
mod player;
mod score;
mod systems;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use events::Victory;
use laser::LaserPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::{handle_victory, spawn_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(LaserPlugin)
        .add_plugin(ScorePlugin)
        .add_event::<Victory>()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_startup_system(spawn_camera)
        .add_system(handle_victory)
        .run();
}
