pub mod home;
pub mod setup;
pub mod screens;

use ratatui::Frame;
use crate::app::{App, ApiKeySetupStep, Screen, SetupStep};

pub fn render(app: &App, frame: &mut Frame) {
    match &app.screen {
        Screen::MainMenu => screens::render_main_menu(app, frame),
        Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider) => {
            setup::render_api_key_choose_provider(app, frame)
        }
        Screen::ApiKeySetup(ApiKeySetupStep::EnterKey) => {
            setup::render_api_key_enter(app, frame)
        }
        Screen::Settings => setup::render_settings(app, frame),
        Screen::Setup(SetupStep::Name) => setup::render_name(app, frame),
        Screen::Setup(SetupStep::Pronoun) => setup::render_pronoun(app, frame),
        Screen::Home => home::render_home(app, frame),
        Screen::Dialogue => screens::render_dialogue(app, frame),
        Screen::Event => screens::render_event(app, frame),
        Screen::Hibernation { success, bond_at_sleep } => screens::render_hibernation(app, frame, *success, *bond_at_sleep),
        Screen::FinalRest => screens::render_final_rest(app, frame),
        Screen::GameOver => screens::render_game_over(app, frame),
    }
}
