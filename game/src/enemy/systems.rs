use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{
    components::{Direction, Enemy},
    timers::EnemyMoveTimer,
};

pub const ENEMIES_ROWS: i32 = 4;
pub const ENEMIES_COLUMNS: i32 = 10;
pub const ENEMY_SPEED: i32 = 10;
pub const ENEMY_WIDTH: i32 = 48;
pub const ENEMY_HEIGHT: i32 = 36;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for i in 0..ENEMIES_ROWS {
        let space_between_enemies = 10;
        let start_x =
            (window.width() as i32 - ENEMIES_COLUMNS * (ENEMY_WIDTH + space_between_enemies)) / 2;
        let start_y = window.height() as i32 - ENEMY_HEIGHT - 20;

        for j in 0..ENEMIES_COLUMNS {
            let enemy_x = start_x + j * (ENEMY_WIDTH + space_between_enemies);
            let enemy_y = start_y as i32 - i * (ENEMY_HEIGHT + space_between_enemies);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(enemy_x as f32, enemy_y as f32, 0.0),
                    texture: asset_server.load("sprites/enemy.png"),
                    ..default()
                },
                Enemy {
                    direction: Direction::Right,
                    previous_x_direction: Direction::Right,
                },
            ));
        }
    }
}

pub fn move_enemy_over_time(
    enemy_move_timer: Res<EnemyMoveTimer>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
) {
    if enemy_move_timer.timer.finished() {
        for (mut transform, mut enemy) in enemy_query.iter_mut() {
            transform.translation += get_vec_from_direction(&enemy.direction) * ENEMY_SPEED as f32;

            if enemy.direction == Direction::Bottom {
                enemy.direction = match enemy.previous_x_direction {
                    Direction::Right => Direction::Left,
                    Direction::Left => Direction::Right,
                    _ => Direction::Right,
                };
            } else {
                enemy.previous_x_direction = enemy.direction.clone();
                enemy.direction = Direction::Bottom;
            }
        }
    }
}

fn get_vec_from_direction(direction: &Direction) -> Vec3 {
    match direction {
        Direction::Right => Vec3::new(1.0, 0.0, 0.0),
        Direction::Bottom => Vec3::new(0.0, -1.0, 0.0),
        Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
    }
}
