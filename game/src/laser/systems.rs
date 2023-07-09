use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

use crate::enemy::components::Enemy;
use crate::enemy::systems::{ENEMY_HEIGHT, ENEMY_WIDTH};
use crate::events::Victory;

use super::components::Laser;

pub const LASER_SPEED: f32 = 600.0;

pub fn spawn_laser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_transform: &Transform,
) {
    commands.spawn((
        SpriteBundle {
            transform: player_transform.clone(),
            texture: asset_server.load("sprites/laser.png"),
            ..default()
        },
        Laser {},
    ));
}

pub fn laser_movement(mut laser_query: Query<&mut Transform, With<Laser>>, time: Res<Time>) {
    for mut transform in laser_query.iter_mut() {
        transform.translation += Vec3::new(0.0, 1.0, 0.0) * LASER_SPEED * time.delta_seconds();
    }
}

pub fn laser_hit_detection(
    mut commands: Commands,
    mut victory_event: EventWriter<Victory>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let enemy_size = Vec2::new(ENEMY_WIDTH as f32, ENEMY_HEIGHT as f32);
    let laser_size = Vec2::new(26.0, 46.0);
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        for (laser_entity, laser_transform) in laser_query.iter() {
            if collide(
                enemy_transform.translation,
                enemy_size,
                laser_transform.translation,
                laser_size,
            )
            .is_some()
            {
                if enemy_query.iter().len() == 1 {
                    victory_event.send(Victory);
                } else {
                    // play audio
                    let laser_sound = asset_server.load("audio/boom.ogg");
                    audio.play(laser_sound);
                }

                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}

pub fn laser_detect_outside_screen(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (laser_entity, laser_transform) in laser_query.iter() {
        if laser_transform.translation.y >= window.height() {
            commands.entity(laser_entity).despawn();
        }
    }
}
