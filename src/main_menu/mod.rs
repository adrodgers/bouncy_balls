use bevy::prelude::*;

use crate::AppState;

use self::systems::{
    interactions::{interact_with_play_button, interact_with_quit_button},
    layout::{despawn_main_menu, spawn_main_menu},
};
mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems((interact_with_play_button, interact_with_quit_button))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
