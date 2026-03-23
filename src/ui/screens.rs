use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;
use crate::art::bear_sleep_art;

pub fn render_main_menu(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let art = crate::art::ADULT;

    let menu = Paragraph::new(format!("{}\n  TERMINAL BEAR\n\n[n] New Game    [q] Quit", art))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    frame.render_widget(menu, area);
}

pub fn render_dialogue(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    let last = app.save.dialogue_log.last();
    let content = if let Some(entry) = last {
        format!("You: {}\n\n{}: {}", entry.player, app.save.bear.name, entry.bear)
    } else {
        "...".to_string()
    };

    let dialogue = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Talking with {} ", app.save.bear.name))
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White));
    frame.render_widget(dialogue, chunks[1]);

    let hint_text = if app.last_talk_capped {
        " [Enter / Esc] Back    (talked twice today — no bond boost)"
    } else {
        " [Enter / Esc] Back"
    };
    let hint = Paragraph::new(hint_text)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(hint, chunks[2]);
}

pub fn render_event(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let has_choices = app.event_choices.is_some();
    let hint_height = if has_choices { 8 } else { 2 };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Min(0),
            Constraint::Length(hint_height),
        ])
        .split(area);

    let text = app.event_text.as_deref().unwrap_or("...");
    let event = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " {} | {} ",
                    app.save.time.season.label(),
                    app.save.bear.name
                ))
                .style(Style::default().fg(Color::Green)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White));
    frame.render_widget(event, chunks[1]);

    if let Some(choices) = &app.event_choices {
        // Show the two choices
        let choice_text = format!(
            " [1] {}\n [2] {}",
            choices[0].text, choices[1].text
        );
        let choice_widget = Paragraph::new(choice_text)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::TOP))
            .wrap(Wrap { trim: false });
        frame.render_widget(choice_widget, chunks[2]);
    } else {
        // Outcome shown or no choices — just dismiss hint
        let hint = Paragraph::new(" [Enter / Esc] Continue")
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(hint, chunks[2]);
    }
}

pub fn render_hibernation(app: &App, frame: &mut Frame, success: bool, bond_at_sleep: f32) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    let name = &app.save.bear.name;
    let age = app.save.bear.age_years;
    let sleep_art = bear_sleep_art(&app.save.bear.age_stage());

    let (season_text, msg) = if success {
        let bond_note = if bond_at_sleep >= 70.0 {
            format!("Before settling in, {} turned and pressed {} nose against your hand.\nThen curled up and was still.", name, app.save.bear.pronoun.possessive())
        } else if bond_at_sleep >= 40.0 {
            format!("{} settled in slowly, glancing toward you once before closing {} eyes.", name, app.save.bear.pronoun.possessive())
        } else {
            format!("{} found a hollow and disappeared inside without ceremony.", name)
        };
        (
            "~ Winter passes ~\n  ~ Spring arrives ~",
            format!(
                "{}\n\nWoke up a little hungry, but ready for spring.\n\nYear {} begins.",
                bond_note, age
            ),
        )
    } else {
        (
            "~ A hard winter ~\n  ~ Spring arrives ~",
            format!(
                "{} didn't have enough fat for a good hibernation.\nWoke up thin and weak. This spring will be tough.\n\nYear {} begins.",
                name, age
            ),
        )
    };

    let color = if success { Color::Cyan } else { Color::Red };

    let zzz = if success { "z  z  z" } else { "z . . ." };
    let screen = Paragraph::new(format!("{}\n{}\n\n  {}\n\n{}", sleep_art, zzz, season_text, msg))
        .alignment(Alignment::Center)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD));
    frame.render_widget(screen, chunks[0]);

    let hint = Paragraph::new(" [Enter / Esc] Wake up")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(hint, chunks[1]);
}

pub fn render_final_rest(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
        ])
        .split(area);

    let bear = &app.save.bear;

    let bond_line = if bear.bond >= 70.0 {
        format!("You were with {} at the end of fall, the way you always were.\n{} settled in, and was still.", bear.name, bear.name)
    } else if bear.bond >= 40.0 {
        format!("{} found a good hollow, the same one as always.\nYou watched from a distance as {} settled in for the last time.", bear.name, bear.pronoun.subject())
    } else {
        format!("{} went off alone to find a place to sleep.\nAs {} always had.", bear.name, bear.pronoun.subject())
    };

    let msg = format!(
        "{}\n\n\
        Some bears just don't wake up.\n\
        The old ones especially — they go quietly, in their sleep,\n\
        when the world is cold and still.\n\n\
        {} lived {} years.\n\n\
        [q] Quit   [n] New Game",
        bond_line,
        bear.name,
        bear.age_years,
    );

    use crate::bear::AgeStage;
    let art = crate::art::bear_sleep_art(&AgeStage::Elder);

    let screen = Paragraph::new(format!("{}\n  ~ A good long life ~\n\n{}", art, msg))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: false });
    frame.render_widget(screen, chunks[0]);
}

pub fn render_game_over(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(area);

    let bear = &app.save.bear;
    let bond_line = if bear.bond >= 70.0 {
        format!("{} paused at the treeline. Looked back at you for a long moment.\nThen was gone.", bear.name)
    } else if bear.bond >= 40.0 {
        format!("{} glanced back once before disappearing into the trees.", bear.name)
    } else {
        format!("{} didn't look back.", bear.name)
    };

    let msg = format!(
        "{} waited as long as {} could.\n\
        But the hunger got to be too much.\n\
        One morning, {} just... wandered off.\n\n\
        {}\n\n\
        Year {}, {} {}.\n\n\
        [q] Quit   [n] New Game",
        bear.name,
        bear.pronoun.subject(),
        bear.name,
        bond_line,
        app.save.time.year,
        app.save.time.season.label(),
        app.save.time.day,
    );

    let screen = Paragraph::new(msg)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: false });
    frame.render_widget(screen, chunks[1]);
}
