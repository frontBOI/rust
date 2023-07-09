use bevy::prelude::*;

use systems::*;
use timers::*;

pub mod components;
pub mod systems;
mod timers;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyMoveTimer>()
            .add_startup_system(spawn_enemies)
            .add_system(tick_enemy_move_timer)
            .add_system(move_enemy_over_time);
    }
}
