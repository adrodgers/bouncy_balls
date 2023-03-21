pub mod events;
mod game;
mod main_menu;
mod systems;
use std::default;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_system(transition_to_game_state)
        .add_system(transition_to_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
}
