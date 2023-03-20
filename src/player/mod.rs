use bevy::prelude::*;
use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct MovementSystemSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)
            // .add_systems(
            //     (
            //         player_movement,
            //         confine_player_movement.after(player_movement),
            //     )
            //         .chain(),
            // )
            // .add_system(player_movement)
            // .add_system(confine_player_movement.after(player_movement))
            .add_system(player_movement.in_set(MovementSystemSet))
            .add_system(confine_player_movement.in_set(ConfinementSystemSet))
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
