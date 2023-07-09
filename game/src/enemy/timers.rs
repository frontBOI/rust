use bevy::{
    prelude::*,
    time::{Timer, TimerMode},
};

pub const ENEMY_MOVE_DELAY: f32 = 1.0;

#[derive(Resource)]
pub struct EnemyMoveTimer {
    pub timer: Timer,
}

impl Default for EnemyMoveTimer {
    fn default() -> EnemyMoveTimer {
        EnemyMoveTimer {
            timer: Timer::from_seconds(ENEMY_MOVE_DELAY, TimerMode::Repeating),
        }
    }
}

pub fn tick_enemy_move_timer(mut enemy_move_timer: ResMut<EnemyMoveTimer>, time: Res<Time>) {
    enemy_move_timer.timer.tick(time.delta());
}
