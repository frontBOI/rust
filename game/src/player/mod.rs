use bevy::prelude::*;

mod components;
mod systems;
mod timers;

use systems::*;
use timers::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            .init_resource::<LastShootTimer>()
            .add_startup_system(spawn_player)
            .add_system(player_movement.in_set(PlayerSystemSet::Movement))
            .add_system(confine_player_movement.in_set(PlayerSystemSet::Confinement))
            .add_system(player_shoot)
            .add_system(tick_last_shoot_timer)
            .add_system(reload_shoot_over_time);
    }
}
