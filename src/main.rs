mod app;
mod art;
mod bear;
mod events;
mod llm;
mod persistence;
mod time;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;

use app::{App, ApiKeySetupStep, Screen, SetupStep};

fn main() -> Result<()> {
    let mut app = App::load_or_new()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        app.poll_pending();
        app.tick = app.tick.wrapping_add(1);

        terminal.draw(|frame| ui::render(app, frame))?;

        let timeout = if app.loading {
            Duration::from_millis(80)
        } else {
            Duration::from_millis(200)
        };

        if !event::poll(timeout)? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Release {
            continue;
        }

        match &app.screen.clone() {
            Screen::MainMenu => handle_main_menu(app, key.code),
            Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider) => {
                handle_api_key_choose_provider(app, key.code)
            }
            Screen::ApiKeySetup(ApiKeySetupStep::EnterKey) => {
                handle_api_key_enter(app, key.code)
            }
            Screen::Settings => handle_settings(app, key.code),
            Screen::Setup(SetupStep::Name) => handle_setup_name(app, key.code),
            Screen::Setup(SetupStep::Pronoun) => handle_setup_pronoun(app, key.code),
            Screen::Home => handle_home(app, key.code),
            Screen::Dialogue => {
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ')) {
                    app.screen = Screen::Home;
                }
            }
            Screen::Event => {
                if app.event_choices.is_some() {
                    match key.code {
                        KeyCode::Char('1') => app.make_choice(0),
                        KeyCode::Char('2') => app.make_choice(1),
                        _ => {}
                    }
                } else if matches!(key.code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ')) {
                    app.screen = app.next_screen.take().unwrap_or(Screen::Home);
                }
            }
            Screen::Hibernation { success: _, bond_at_sleep: _ } => {
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ')) {
                    app.screen = Screen::Home;
                }
            }
            Screen::FinalRest => match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('n') => {
                    app.reset_for_new_game();
                    if app.save.api_key.is_none() {
                        app.screen = Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider);
                    } else {
                        app.screen = Screen::Setup(SetupStep::Name);
                    }
                }
                _ => {}
            },
            Screen::GameOver => match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('n') => {
                    app.reset_for_new_game();
                    if app.save.api_key.is_none() {
                        app.screen = Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider);
                    } else {
                        app.screen = Screen::Setup(SetupStep::Name);
                    }
                }
                _ => {}
            },
        }
    }
}

fn handle_main_menu(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') => std::process::exit(0),
        KeyCode::Char('n') => {
            app.reset_for_new_game();
            if app.save.api_key.is_none() {
                app.screen = Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider);
            } else {
                app.screen = Screen::Setup(SetupStep::Name);
            }
        }
        _ => {}
    }
}

fn handle_api_key_choose_provider(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_provider > 0 {
                app.selected_provider -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_provider < 1 {
                app.selected_provider += 1;
            }
        }
        KeyCode::Enter => app.confirm_provider_choice(),
        KeyCode::Esc => {
            if !app.save.bear.name.is_empty() {
                app.screen = Screen::Settings;
            } else {
                app.screen = Screen::MainMenu;
            }
        }
        _ => {}
    }
}

fn handle_api_key_enter(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => app.confirm_api_key(),
        KeyCode::Esc => {
            app.input_buffer.clear();
            if !app.save.bear.name.is_empty() {
                app.screen = Screen::Settings;
            } else {
                app.screen = Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider);
            }
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Char(c) => {
            if app.input_buffer.len() < 200 {
                app.input_buffer.push(c);
            }
        }
        _ => {}
    }
}

fn handle_settings(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('c') => {
            app.input_buffer.clear();
            app.screen = Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider);
        }
        KeyCode::Char('n') => {
            app.reset_for_new_game();
            app.screen = Screen::Setup(SetupStep::Name);
        }
        KeyCode::Esc | KeyCode::Char('q') => app.exit_settings(),
        _ => {}
    }
}

fn handle_setup_name(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => app.confirm_bear_name(),
        KeyCode::Backspace => {
            app.bear_name_input.pop();
        }
        KeyCode::Char(c) => {
            if app.bear_name_input.len() < 20 {
                app.bear_name_input.push(c);
            }
        }
        _ => {}
    }
}

fn handle_setup_pronoun(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_pronoun > 0 {
                app.selected_pronoun -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_pronoun < 2 {
                app.selected_pronoun += 1;
            }
        }
        KeyCode::Enter => app.confirm_pronoun(),
        _ => {}
    }
}

fn handle_home(app: &mut App, key: KeyCode) {
    if app.loading {
        return;
    }

    if app.talking {
        match key {
            KeyCode::Enter => {
                let msg = app.input_buffer.trim().to_string();
                if !msg.is_empty() {
                    app.action_talk(msg);
                } else {
                    app.cancel_talk();
                }
            }
            KeyCode::Esc => app.cancel_talk(),
            KeyCode::Backspace => {
                app.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                if app.input_buffer.len() < 200 {
                    app.input_buffer.push(c);
                }
            }
            _ => {}
        }
        return;
    }

    app.message = None;

    // While bear is missing, only [w] and [q]/[s] work
    if app.save.bear_missing_turns > 0 {
        match key {
            KeyCode::Char('w') => app.action_wait(),
            KeyCode::Char('q') => std::process::exit(0),
            KeyCode::Char('s') => app.go_to_settings(),
            _ => {
                app.message = Some(format!(
                    "{} is gone. Press [w] to wait a day.",
                    app.save.bear.name
                ));
            }
        }
        return;
    }

    match key {
        KeyCode::Char('q') => std::process::exit(0),

        KeyCode::Char('1') => app.action_feed(),
        KeyCode::Char('2') => app.begin_talk(),
        KeyCode::Char('3') => app.action_fish(),
        KeyCode::Char('4') => app.action_forage(),
        KeyCode::Char('5') => app.action_explore(),
        KeyCode::Char('6') => app.action_interact(),
        KeyCode::Char('7') => app.action_relax(),
        KeyCode::Char('8') => app.action_nap(),

        KeyCode::Char('s') => app.go_to_settings(),

        _ => {}
    }
}
