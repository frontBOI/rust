use bevy::{
    prelude::*,
    time::{Timer, TimerMode},
};

pub const SHOOT_DELAY: f32 = 0.01;

#[derive(Resource)]
pub struct LastShootTimer {
    pub timer: Timer,
}

impl Default for LastShootTimer {
    fn default() -> LastShootTimer {
        LastShootTimer {
            timer: Timer::from_seconds(SHOOT_DELAY, TimerMode::Repeating),
        }
    }
}

pub fn tick_last_shoot_timer(mut last_shoot_timer: ResMut<LastShootTimer>, time: Res<Time>) {
    last_shoot_timer.timer.tick(time.delta());
}
