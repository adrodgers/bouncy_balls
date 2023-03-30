use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct MovementSystemSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // .add_startup_system(spawn_player)
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // .add_systems(
            //     (
            //         player_movement,
            //         confine_player_movement.after(player_movement),
            //     )
            //         .chain(),
            // )
            // .add_system(player_movement)
            // .add_system(confine_player_movement.after(player_movement))
            // .add_system(
            //     player_movement
            //         .in_set(MovementSystemSet)
            //         .run_if(in_state(AppState::Game))
            //         .run_if(in_state(SimulationState::Running)),
            // )
            // .add_system(
            //     confine_player_movement
            //         .in_set(ConfinementSystemSet)
            //         .run_if(in_state(AppState::Game))
            //         .run_if(in_state(SimulationState::Running)),
            // )
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // .add_system(enemy_hit_player)
            // .add_system(player_hit_star)
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
