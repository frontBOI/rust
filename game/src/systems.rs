use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::Victory;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn handle_victory(
    mut victory_event: EventReader<Victory>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for _ in victory_event.iter() {
        let laser_sound = asset_server.load("audio/victory.ogg");
        audio.play(laser_sound);
        println!("Victoire !");
    }
}
