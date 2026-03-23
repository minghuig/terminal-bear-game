use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::llm::LlmProvider;

pub fn render_name(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("🐻  Terminal Bear  🐻\n\nWhat will you name your bear?")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(title, chunks[1]);

    let input = Paragraph::new(app.bear_name_input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Bear Name ")
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(input, chunks[2]);
}

pub fn render_api_key_choose_provider(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("🐻  Terminal Bear  🐻\n\nChoose your AI provider:")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(title, chunks[1]);

    let options = [LlmProvider::Anthropic.label(), LlmProvider::OpenAi.label()];
    let lines: Vec<String> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            if i == app.selected_provider {
                format!("  ▶  {}  ◀", opt)
            } else {
                format!("     {}", opt)
            }
        })
        .collect();

    let provider_widget = Paragraph::new(lines.join("\n"))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(provider_widget, chunks[2]);
}

pub fn render_api_key_enter(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

    let provider_name = match app.selected_provider {
        0 => LlmProvider::Anthropic.label(),
        _ => LlmProvider::OpenAi.label(),
    };

    let title = Paragraph::new(format!(
        "🐻  Terminal Bear  🐻\n\nEnter your {} API key:",
        provider_name
    ))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Yellow));
    frame.render_widget(title, chunks[1]);

    // Mask the key as it's typed
    let masked = "*".repeat(app.input_buffer.len());
    let display = format!("{}_", masked);
    let input = Paragraph::new(display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" API Key ")
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(input, chunks[2]);

    let hint = Paragraph::new(" [Enter] Confirm   [Esc] Back")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(hint, chunks[3]);
}

pub fn render_settings(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(area);

    let provider_str = app
        .save
        .llm_provider
        .as_ref()
        .map(|p| p.label())
        .unwrap_or("None");

    let masked_key = app.save.api_key.as_deref().map(|k| {
        if k.len() <= 8 {
            "*".repeat(k.len())
        } else {
            format!("{}...{}", &k[..4], &k[k.len() - 4..])
        }
    }).unwrap_or_else(|| "not set".to_string());

    let content = format!(
        "Provider:  {}\nAPI Key:   {}\n\n[c] Change provider / key\n[n] New game\n[Esc] Back",
        provider_str, masked_key
    );

    let settings = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Settings ")
                .style(Style::default().fg(Color::Blue)),
        )
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White));
    frame.render_widget(settings, chunks[1]);
}

pub fn render_pronoun(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new(format!(
        "What pronouns for {}?",
        app.save.bear.name
    ))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Yellow));
    frame.render_widget(title, chunks[1]);

    let options = ["he/him", "she/her", "they/them"];
    let lines: Vec<String> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            if i == app.selected_pronoun {
                format!("  ▶  {}  ◀", opt)
            } else {
                format!("     {}", opt)
            }
        })
        .collect();

    let pronoun_widget = Paragraph::new(lines.join("\n"))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(pronoun_widget, chunks[2]);
}
