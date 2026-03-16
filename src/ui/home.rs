use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::art::bear_art;

pub fn render_home(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // Top bar: time info
    // Main: left = bear art, right = stats + actions
    // Bottom: message bar
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // time bar
            Constraint::Min(0),     // main
            Constraint::Length(3),  // message bar
        ])
        .split(area);

    // ── Time bar ─────────────────────────────────────────────────────────────
    let time_text = format!(
        " {}  {}",
        app.save.time.season.symbol(),
        app.save.time.display(),
    );
    let season_color = match app.save.time.season {
        crate::time::Season::Spring => Color::Green,
        crate::time::Season::Summer => Color::Yellow,
        crate::time::Season::Fall => Color::Red,
        crate::time::Season::Winter => Color::Cyan,
    };
    let time_bar = Paragraph::new(time_text)
        .style(Style::default().fg(season_color).add_modifier(Modifier::BOLD));
    frame.render_widget(time_bar, rows[0]);

    // ── Main area ─────────────────────────────────────────────────────────────
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(rows[1]);

    // Bear art panel
    let art = bear_art(&app.save.bear.age_stage());
    let bear_panel = Paragraph::new(art)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} the {} ", app.save.bear.name, app.save.bear.age_stage().label()))
                .style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Center);
    frame.render_widget(bear_panel, cols[0]);

    // Right panel: stats + actions
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[1]);

    render_stats(app, frame, right[0]);
    render_actions(frame, right[1]);

    // ── Message / talk input bar ──────────────────────────────────────────────
    if app.loading {
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let spinner_char = spinner[(app.tick / 2) as usize % spinner.len()];
        let loading_bar = Paragraph::new(format!(" {} thinking...", spinner_char))
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(loading_bar, rows[2]);
    } else if app.talking {
        let input = Paragraph::new(format!(" > {}_", app.input_buffer))
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .title(" Say something... (Enter to send, Esc to cancel) ")
                    .style(Style::default().fg(Color::Cyan)),
            );
        frame.render_widget(input, rows[2]);
    } else {
        let msg = app
            .message
            .as_deref()
            .unwrap_or("Use number keys to act.");
        let msg_bar = Paragraph::new(format!(" {}", msg))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(msg_bar, rows[2]);
    }
}

fn render_stats(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let bear = &app.save.bear;
    let time = &app.save.time;

    let season_hint = season_goal_line(bear.fat_reserves, time.season, time.day, app.save.hibernation_ready);

    let stats = format!(
        "Hunger:       {}\nEnergy:       {}\nFat reserves: {}\nBond:         {}\n\nFood:          {} / 20\nFishing skill: {:.0}%\n\n{}",
        stat_bar(bear.hunger, 100.0),
        stat_bar(bear.energy, 100.0),
        stat_bar(bear.fat_reserves, 200.0),
        stat_bar(bear.bond, 100.0),
        app.save.food_inventory,
        bear.fishing_skill * 100.0,
        season_hint,
    );

    let stats_widget = Paragraph::new(stats)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Stats ")
                .style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(stats_widget, area);
}

fn season_goal_line(fat: f32, season: crate::time::Season, day: u32, hibernation_ready: bool) -> String {
    use crate::time::{Season, FAT_THRESHOLD_FOR_HIBERNATION, DAYS_PER_SEASON};
    let days_left = DAYS_PER_SEASON - day + 1;
    match season {
        Season::Fall => {
            if hibernation_ready {
                format!("🍂 Ready for winter! ({} days left)", days_left)
            } else {
                let needed = (FAT_THRESHOLD_FOR_HIBERNATION - fat).max(0.0);
                format!("🍂 Winter in {} days — need {:.0} more fat", days_left, needed)
            }
        }
        Season::Spring => format!("🌿 Lean times — {} days of spring left", days_left),
        Season::Summer => "☀️  Build fat reserves before fall".to_string(),
        Season::Winter => "❄️  Hibernating...".to_string(),
    }
}

fn render_actions(frame: &mut Frame, area: ratatui::layout::Rect) {
    let actions = "\
Actions:\n\
  [1] Feed *     [2] Talk *\n\
  [3] Fish       [4] Forage\n\
  [5] Explore    [6] Interact\n\
  [7] Relax      [8] Nap\n\
\n\
* no time passes\n\
  [s] Settings   [q] Quit";

    let actions_widget = Paragraph::new(actions)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Actions ")
                .style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    frame.render_widget(actions_widget, area);
}

fn stat_bar(value: f32, max: f32) -> String {
    let filled = ((value / max) * 10.0).round() as usize;
    let filled = filled.min(10);
    let empty = 10 - filled;
    format!(
        "[{}{}] {:>5.1}",
        "█".repeat(filled),
        "░".repeat(empty),
        value
    )
}
