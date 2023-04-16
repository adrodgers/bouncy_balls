use bevy::{app::AppExit, prelude::*};

use crate::{
    main_menu::{
        components::{PlayButton, QuitButton},
        styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOUR, PRESSED_BUTTON_COLOR},
    },
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_colour)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_colour = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game)
            }
            Interaction::Hovered => *background_colour = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_colour = NORMAL_BUTTON_COLOUR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_colour)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_colour = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit)
            }
            Interaction::Hovered => *background_colour = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_colour = NORMAL_BUTTON_COLOUR.into(),
        }
    }
}
