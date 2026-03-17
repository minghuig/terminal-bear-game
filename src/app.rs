use anyhow::Result;
use std::sync::{Arc, mpsc};
use std::thread;

use crate::bear::{Bear, Pronoun};
use crate::events::{EventRecord, EventTheme};
use crate::llm::{
    EventChoice, LlmClient, LlmProvider, build_dialogue_prompt, build_event_prompt,
    build_interact_prompt, build_relax_prompt, build_simple_event_prompt, extract_summary,
    find_previous_summary, create_client, parse_dialogue_response, parse_event_response, parse_stat_deltas,
};
use crate::persistence::{DialogueEntry, SaveState};
use crate::time::{DayAdvanceResult, Season, FAT_THRESHOLD_FOR_HIBERNATION};

const RELAX_ENERGY: f32 = 20.0;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    ApiKeySetup(ApiKeySetupStep),
    Settings,
    Setup(SetupStep),
    Home,
    Dialogue,
    Event,
    Hibernation { success: bool, bond_at_sleep: f32 },
    FinalRest,
    GameOver,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiKeySetupStep {
    ChooseProvider,
    EnterKey,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetupStep {
    Name,
    Pronoun,
}

// Result types sent back from background threads
pub struct PendingEventResult {
    pub narrative: String,
    pub choices: Option<[EventChoice; 2]>, // None = simple event, apply stats directly
    pub summary: String,
    pub theme_key: String,
    pub action: String,
    pub season: Season,
    pub year: u32,
    pub day: u32,
}

pub enum PendingResult {
    Event(PendingEventResult),
    Dialogue { player: String, bear: String, bond_delta: f32 },
}

pub struct App {
    pub screen: Screen,
    pub next_screen: Option<Screen>, // deferred transition after event/dialogue is dismissed
    pub save: SaveState,
    pub llm: Arc<dyn LlmClient>,

    // Background LLM state
    pub loading: bool,
    pub tick: u64, // increments each frame, drives spinner
    pending: Option<mpsc::Receiver<Result<PendingResult>>>,

    // UI state
    pub input_buffer: String,
    pub talking: bool,
    pub message: Option<String>,
    pub event_text: Option<String>,
    pub event_choices: Option<[EventChoice; 2]>,  // pending choices for current event
    pub event_choice_made: bool,                   // true = showing outcome, false = showing choices
    pub current_event_action: String,              // action that triggered the current event
    pub selected_pronoun: usize,
    pub selected_provider: usize, // 0 = Anthropic, 1 = OpenAI
    pub bear_name_input: String,
}

impl App {
    pub fn new() -> Self {
        let placeholder_bear = Bear::new("".to_string(), Pronoun::They);
        Self {
            screen: Screen::MainMenu,
            next_screen: None,
            save: SaveState::new(placeholder_bear),
            llm: Arc::new(crate::llm::StubClient),
            loading: false,
            tick: 0,
            pending: None,
            input_buffer: String::new(),
            talking: false,
            message: None,
            event_text: None,
            event_choices: None,
            event_choice_made: false,
            current_event_action: String::new(),
            selected_pronoun: 0,
            selected_provider: 0,
            bear_name_input: String::new(),
        }
    }

    pub fn load_or_new() -> Result<Self> {
        let mut app = App::new();
        if let Some(save) = SaveState::load()? {
            app.save = save;
            if app.save.api_key.is_some() && app.save.llm_provider.is_some() {
                app.rebuild_llm();
                app.screen = Screen::Home;
            } else {
                app.screen = Screen::ApiKeySetup(ApiKeySetupStep::ChooseProvider);
            }
        }
        Ok(app)
    }

    fn rebuild_llm(&mut self) {
        if let (Some(provider), Some(key)) = (&self.save.llm_provider, &self.save.api_key) {
            self.llm = create_client(provider, key);
        }
    }

    // ── API key setup ─────────────────────────────────────────────────────────

    pub fn confirm_provider_choice(&mut self) {
        self.input_buffer.clear();
        self.screen = Screen::ApiKeySetup(ApiKeySetupStep::EnterKey);
    }

    pub fn confirm_api_key(&mut self) {
        let key = self.input_buffer.trim().trim_matches('"').trim_matches('\'').to_string();
        if key.is_empty() {
            return;
        }
        let provider = match self.selected_provider {
            0 => LlmProvider::Anthropic,
            _ => LlmProvider::OpenAi,
        };
        self.save.llm_provider = Some(provider);
        self.save.api_key = Some(key);
        self.rebuild_llm();
        self.input_buffer.clear();
        self.auto_save();

        if self.save.bear.name.is_empty() {
            self.screen = Screen::Setup(SetupStep::Name);
        } else {
            self.screen = Screen::Home;
        }
    }

    pub fn reset_for_new_game(&mut self) {
        let provider = self.save.llm_provider.clone();
        let api_key = self.save.api_key.clone();
        let placeholder = crate::bear::Bear::new("".to_string(), crate::bear::Pronoun::They);
        self.save = crate::persistence::SaveState::new(placeholder);
        self.save.llm_provider = provider;
        self.save.api_key = api_key;
        self.event_text = None;
        self.event_choices = None;
        self.event_choice_made = false;
        self.message = None;
        self.input_buffer.clear();
        self.bear_name_input.clear();
        self.talking = false;
        self.loading = false;
        self.pending = None;
    }

    pub fn go_to_settings(&mut self) {
        self.screen = Screen::Settings;
    }

    pub fn exit_settings(&mut self) {
        if self.save.bear.name.is_empty() {
            self.screen = Screen::MainMenu;
        } else {
            self.screen = Screen::Home;
        }
    }

    /// Called every frame — checks if a background LLM call has finished.
    pub fn poll_pending(&mut self) {
        let result = if let Some(rx) = &self.pending {
            rx.try_recv().ok()
        } else {
            return;
        };

        if let Some(result) = result {
            self.pending = None;
            self.loading = false;
            match result {
                Ok(PendingResult::Event(e)) => self.apply_event_result(e),
                Ok(PendingResult::Dialogue { player, bear, bond_delta }) => {
                    self.apply_dialogue_result(player, bear, bond_delta)
                }
                Err(e) => {
                    self.message = Some(format!("Error: {e}"));
                }
            }
        }
    }

    // ── Free actions ──────────────────────────────────────────────────────────

    fn season_fat_multiplier(&self) -> f32 {
        match self.save.time.season {
            Season::Spring => 0.5, // walking hibernation — slow to rebuild
            Season::Summer => 1.0,
            Season::Fall => 2.0,   // hyperphagia — body absorbs fat rapidly
            Season::Winter => 1.0,
        }
    }

    /// If the bear was missing, clear the flag and notify. Returns true if the
    /// bear just returned (caller should show message and skip the action).
    fn check_bear_returns(&mut self) -> bool {
        if self.save.bear_missing {
            self.save.bear_missing = false;
            self.message = Some(format!(
                "{} has returned. They seem distant.",
                self.save.bear.name
            ));
            self.auto_save();
            return true;
        }
        false
    }

    pub fn action_feed(&mut self) {
        if self.check_bear_returns() { return; }
        if self.save.food_inventory == 0 {
            self.message = Some("No food left. Go fish or forage!".to_string());
            return;
        }
        self.save.food_inventory -= 1;
        self.save.bear.feed(20.0, self.season_fat_multiplier());
        self.save.bear.bond = (self.save.bear.bond + 1.0).min(100.0);
        self.message = Some(format!(
            "{} eats hungrily. ({} food left)",
            self.save.bear.name, self.save.food_inventory
        ));
        self.auto_save();
    }

    pub fn action_interact(&mut self) {
        if self.check_bear_returns() { return; }
        if self.save.bear.is_exhausted() {
            self.message = Some(format!("{} is too exhausted. Take a nap first.", self.save.bear.name));
            return;
        }
        let pool = EventTheme::pool_for_interact(&self.save.bear.age_stage(), self.save.bear.bond);
        let recent: Vec<String> = self.save.event_log
            .iter().rev().take(4)
            .map(|e| e.theme_key.clone())
            .collect();
        let theme = EventTheme::pick(pool, &recent);
        let Some(theme) = theme else {
            self.message = Some("Nothing much happened.".to_string());
            self.advance_day_and_check();
            self.auto_save();
            return;
        };

        let is_choice = self.save.event_log.len() % 2 == 0;
        let theme_key = theme.key().to_string();
        let follow_up = find_previous_summary(&self.save.event_log, &theme_key).map(|s| s.to_string());
        let prompt = build_interact_prompt(&self.save.bear, &theme_key, is_choice, follow_up.as_deref());

        let season = self.save.time.season;
        let year = self.save.time.year;
        let day = self.save.time.day;
        let theme_key_clone = theme_key.clone();
        let llm = Arc::clone(&self.llm);

        self.loading = true;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);

        thread::spawn(move || {
            let result = (|| -> Result<PendingResult> {
                let raw = llm.complete(prompt)?;
                let (raw_no_summary, summary) = extract_summary(&raw);
                let (narrative, choices) = if is_choice {
                    let parsed = parse_event_response(&raw_no_summary);
                    (parsed.narrative, parsed.choices)
                } else {
                    (raw_no_summary, None)
                };
                Ok(PendingResult::Event(PendingEventResult {
                    narrative,
                    choices,
                    summary,
                    theme_key: theme_key_clone,
                    action: "interact".to_string(),
                    season,
                    year,
                    day,
                }))
            })();
            let _ = tx.send(result);
        });
    }

    pub fn begin_talk(&mut self) {
        self.talking = true;
        self.input_buffer.clear();
        self.message = None;
    }

    pub fn cancel_talk(&mut self) {
        self.talking = false;
        self.input_buffer.clear();
    }

    pub fn action_talk(&mut self, player_message: String) {
        self.talking = false;
        self.loading = true;

        let llm = Arc::clone(&self.llm);
        let prompt = build_dialogue_prompt(&self.save.bear, &player_message);

        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);

        thread::spawn(move || {
            let result = llm.complete(prompt).map(|response| {
                let (bear_text, bond_delta) = parse_dialogue_response(&response);
                PendingResult::Dialogue {
                    player: player_message,
                    bear: bear_text,
                    bond_delta,
                }
            });
            let _ = tx.send(result);
        });
    }

    fn apply_dialogue_result(&mut self, player: String, bear_response: String, bond_delta: f32) {
        self.save.dialogue_log.push(DialogueEntry {
            player,
            bear: bear_response,
            year: self.save.time.year,
            season: self.save.time.season.label().to_string(),
        });
        self.save.bear.bond += bond_delta;
        self.save.bear.clamp_stats();
        self.screen = Screen::Dialogue;
        self.auto_save();
    }

    // ── Day actions ───────────────────────────────────────────────────────────

    pub fn action_fish(&mut self) {
        if self.save.bear.is_exhausted() {
            self.message = Some(format!("{} is too exhausted. Take a nap first.", self.save.bear.name));
            return;
        }
        self.run_day_action("fish");
    }

    pub fn action_forage(&mut self) {
        if self.save.bear.is_exhausted() {
            self.message = Some(format!("{} is too exhausted. Take a nap first.", self.save.bear.name));
            return;
        }
        self.run_day_action("forage");
    }

    pub fn action_explore(&mut self) {
        if self.save.bear.is_exhausted() {
            self.message = Some(format!("{} is too exhausted. Take a nap first.", self.save.bear.name));
            return;
        }
        self.run_day_action("explore");
    }



    pub fn action_relax(&mut self) {
        if self.check_bear_returns() { return; }
        let pool = EventTheme::pool_for_relax(self.save.time.season, self.save.bear.bond);
        let recent: Vec<String> = self.save.event_log
            .iter().rev().take(4)
            .map(|e| e.theme_key.clone())
            .collect();
        let theme = EventTheme::pick(pool, &recent);
        let Some(theme) = theme else {
            self.message = Some("Nothing much happened.".to_string());
            self.advance_day_and_check();
            self.auto_save();
            return;
        };

        let theme_key = theme.key().to_string();
        let follow_up = find_previous_summary(&self.save.event_log, &theme_key).map(|s| s.to_string());
        let prompt = build_relax_prompt(&self.save.bear, self.save.time.season, self.save.time.year, &theme_key, follow_up.as_deref());
        let season = self.save.time.season;
        let year = self.save.time.year;
        let day = self.save.time.day;
        let theme_key_clone = theme_key.clone();
        let llm = Arc::clone(&self.llm);

        self.loading = true;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);

        thread::spawn(move || {
            let result = (|| -> Result<PendingResult> {
                let raw = llm.complete(prompt)?;
                let (narrative, summary) = extract_summary(&raw);
                Ok(PendingResult::Event(PendingEventResult {
                    narrative,
                    choices: None,
                    summary,
                    theme_key: theme_key_clone,
                    action: "relax".to_string(),
                    season,
                    year,
                    day,
                }))
            })();
            let _ = tx.send(result);
        });
    }

    pub fn action_nap(&mut self) {
        if self.check_bear_returns() { return; }
        self.save.bear.energy += 40.0;
        self.save.bear.clamp_stats();
        self.message = Some(format!(
            "{} naps peacefully. (day passes)",
            self.save.bear.name
        ));
        self.advance_day_and_check();
        self.auto_save();
    }

    fn run_day_action(&mut self, action: &str) {
        if self.check_bear_returns() { return; }
        let recent: Vec<String> = self.save.event_log
            .iter().rev().take(4)
            .map(|e| e.theme_key.clone())
            .collect();
        let pool = EventTheme::pool_for(
            self.save.time.season, action,
            &self.save.bear.age_stage(), &self.save.event_log,
            self.save.bear.bond,
        );
        let theme = EventTheme::pick(pool, &recent);

        let Some(theme) = theme else {
            self.message = Some("Nothing much happened.".to_string());
            self.advance_day_and_check();
            self.auto_save();
            return;
        };

        let past = self.save.event_log.clone();
        let follow_up = find_previous_summary(&past, theme.key()).map(|s| s.to_string());

        // 50/50 choice vs simple, alternating based on event count + day
        let is_choice = self.save.event_log.len() % 2 == 0;

        let prompt = if is_choice {
            build_event_prompt(
                &self.save.bear, self.save.time.season, self.save.time.year,
                action, theme.key(), &past, follow_up.as_deref(),
            )
        } else {
            build_simple_event_prompt(
                &self.save.bear, self.save.time.season, self.save.time.year,
                action, theme.key(), &past, follow_up.as_deref(),
            )
        };

        let theme_key = theme.key().to_string();
        let action = action.to_string();
        let season = self.save.time.season;
        let year = self.save.time.year;
        let day = self.save.time.day;
        let llm = Arc::clone(&self.llm);

        self.loading = true;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);

        thread::spawn(move || {
            let result = (|| -> Result<PendingResult> {
                let raw = llm.complete(prompt)?;
                let (raw_no_summary, summary) = extract_summary(&raw);
                let (narrative, choices) = if is_choice {
                    let parsed = parse_event_response(&raw_no_summary);
                    (parsed.narrative, parsed.choices)
                } else {
                    (raw_no_summary, None)
                };
                Ok(PendingResult::Event(PendingEventResult {
                    narrative,
                    choices,
                    summary,
                    theme_key,
                    action,
                    season,
                    year,
                    day,
                }))
            })();
            let _ = tx.send(result);
        });
    }

    fn apply_event_result(&mut self, e: PendingEventResult) {
        // Log the event for memory
        let theme = EventTheme::all().into_iter().find(|t| t.key() == e.theme_key);
        if let Some(theme) = theme {
            self.save.event_log.push(EventRecord::new(
                &theme,
                e.summary,
                e.season,
                e.year,
                e.day,
            ));
        }

        // Check hibernation readiness before event stats are applied, so that
        // events which drain fat don't prevent a bear that entered the day at
        // threshold from being marked ready.
        if self.save.time.season == Season::Fall
            && self.save.bear.fat_reserves >= FAT_THRESHOLD_FOR_HIBERNATION
        {
            self.save.hibernation_ready = true;
        }

        if e.action == "fish" {
            self.save.bear.fishing_skill += 0.02;
            self.save.bear.clamp_stats();
        }
        if e.action == "relax" {
            self.save.bear.energy += RELAX_ENERGY;
            self.save.bear.clamp_stats();
        }

        if e.choices.is_none() {
            // Simple event or failed choice parse — apply stats from narrative bracket now
            let deltas = parse_stat_deltas(&e.narrative);
            let fat_mult = match e.season {
                Season::Spring => 0.5,
                Season::Fall => 2.0,
                _ => 1.0,
            };
            let fat = deltas.fat * fat_mult;
            let food = if e.action == "fish" {
                (deltas.food as f32 * (1.0 + self.save.bear.fishing_skill)).round() as u32
            } else {
                deltas.food
            };
            self.save.bear.bond += deltas.bond;
            self.save.bear.energy += deltas.energy;
            self.save.bear.fat_reserves += fat;
            self.save.food_inventory = (self.save.food_inventory + food).min(20);
            self.save.bear.clamp_stats();

            let mut effects: Vec<String> = Vec::new();
            if e.action == "relax"  { effects.push(format!("Energy +{}", RELAX_ENERGY as i32)); }
            if deltas.bond != 0.0   { effects.push(format!("Bond {:+.0}", deltas.bond)); }
            if deltas.energy != 0.0 { effects.push(format!("Energy {:+.0}", deltas.energy)); }
            if fat != 0.0           { effects.push(format!("Fat {:+.0}", fat)); }
            if food > 0             { effects.push(format!("+{} food", food)); }
            let effects_line = if effects.is_empty() {
                String::new()
            } else {
                format!("\n\n{}", effects.join("  ·  "))
            };
            self.event_text = Some(format!("{}{}", strip_stat_line(&e.narrative), effects_line));
        } else {
            self.event_text = Some(e.narrative);
        }
        let is_choice = e.choices.is_some();
        self.event_choices = e.choices;
        self.event_choice_made = false;
        self.current_event_action = e.action.clone();
        self.screen = Screen::Event;

        // For choice events, defer advance_day_and_check until after the player
        // picks — so choice stats (bond, fat, etc.) are applied before daily
        // decay and hibernation logic runs.
        if !is_choice {
            self.advance_day_and_check();

            if self.screen != Screen::Event {
                self.next_screen = Some(self.screen.clone());
                self.screen = Screen::Event;
            }
        }

        self.auto_save();
    }

    /// Called when the player picks choice 0 or 1 on the event screen.
    pub fn make_choice(&mut self, index: usize) {
        let Some(choices) = self.event_choices.take() else { return };
        let choice = choices[index].clone();

        let bond = choice.deltas.bond;
        let energy = choice.deltas.energy;
        let fat_mult = self.season_fat_multiplier();
        let fat = choice.deltas.fat * fat_mult;
        let food_base = choice.deltas.food;

        self.save.bear.bond += bond;
        self.save.bear.energy += energy;
        self.save.bear.fat_reserves += fat;
        self.save.bear.clamp_stats();

        let food = if self.current_event_action == "fish" {
            (food_base as f32 * (1.0 + self.save.bear.fishing_skill)).round() as u32
        } else {
            food_base
        };
        self.save.food_inventory = (self.save.food_inventory + food).min(20);

        // Build a visible summary of what changed
        let mut effects: Vec<String> = Vec::new();
        if bond != 0.0   { effects.push(format!("Bond {:+.0}", bond)); }
        if energy != 0.0 { effects.push(format!("Energy {:+.0}", energy)); }
        if fat != 0.0    { effects.push(format!("Fat {:+.0}", fat)); }
        if food > 0      { effects.push(format!("+{} food", food)); }

        let effects_line = if effects.is_empty() {
            String::new()
        } else {
            format!("\n\n{}", effects.join("  ·  "))
        };

        self.event_text = Some(format!("{}{}", choice.outcome_text, effects_line));
        self.event_choice_made = true;

        self.advance_day_and_check();

        if self.screen != Screen::Event {
            self.next_screen = Some(self.screen.clone());
            self.screen = Screen::Event;
        }

        self.auto_save();
    }

    fn advance_day_and_check(&mut self) {
        // Record whether the bear has achieved hibernation-ready fat this fall
        if self.save.time.season == Season::Fall
            && self.save.bear.fat_reserves >= FAT_THRESHOLD_FOR_HIBERNATION
        {
            self.save.hibernation_ready = true;
        }

        self.save.bear.daily_decay(self.save.time.season);

        if self.save.bear.is_gone() {
            self.save.bear.hunger = 20.0;
            self.save.bear.bond = (self.save.bear.bond - 30.0).max(0.0);
            self.save.bear.clamp_stats();
            self.save.bear_missing = true;
            self.message = Some(format!(
                "{} wandered off to find food. Bond suffered.",
                self.save.bear.name
            ));
            return;
        }

        let result = self.save.time.advance_day();
        match result {
            DayAdvanceResult::HibernationBegins => {
                // Succeed if bear hit threshold at any point this fall
                let success = self.save.hibernation_ready;
                self.do_hibernation(success);
            }
            DayAdvanceResult::NewYear => {
                self.save.bear.age_years += 1;
            }
            _ => {}
        }
    }

    fn do_hibernation(&mut self, success: bool) {
        // At age 25, the bear doesn't wake up
        if self.save.bear.age_years >= 20 {
            self.save.hibernation_ready = false;
            self.auto_save();
            self.screen = Screen::FinalRest;
            return;
        }

        // Capture bond before decay so hibernation farewell reflects the real relationship
        let bond_before_sleep = self.save.bear.bond;

        if success {
            self.save.bear.fat_reserves = 20.0;
            self.save.bear.hunger = 40.0;
            self.save.bear.energy = 80.0;
        } else {
            self.save.bear.fat_reserves = 5.0;
            self.save.bear.hunger = 20.0;
            self.save.bear.energy = 40.0;
            self.save.bear.bond -= 30.0;
        }

        // Six months of sleep — bond and skills fade
        self.save.bear.bond = (self.save.bear.bond - 20.0).max(0.0);
        self.save.bear.fishing_skill = (self.save.bear.fishing_skill - 0.1).max(0.0);
        self.save.hibernation_ready = false;

        self.save.bear.clamp_stats();
        self.save.time.season = Season::Spring;
        self.save.time.day = 1;
        self.save.time.year += 1;
        self.save.bear.age_years += 1;
        self.screen = Screen::Hibernation { success, bond_at_sleep: bond_before_sleep };
    }

    fn auto_save(&mut self) {
        let _ = self.save.save();
    }

    // ── Setup helpers ─────────────────────────────────────────────────────────

    pub fn confirm_bear_name(&mut self) {
        let name = self.bear_name_input.trim().to_string();
        if !name.is_empty() {
            self.save.bear.name = name;
            self.screen = Screen::Setup(SetupStep::Pronoun);
        }
    }

    pub fn confirm_pronoun(&mut self) {
        let pronoun = match self.selected_pronoun {
            0 => Pronoun::He,
            1 => Pronoun::She,
            _ => Pronoun::They,
        };
        self.save.bear.pronoun = pronoun;
        self.screen = Screen::Home;
        self.auto_save();
    }
}

fn strip_stat_line(text: &str) -> String {
    // Remove any trailing [...] bracket that looks like a stat line,
    // whether it's on its own line or appended to the end of the narrative.
    let text = text.trim();
    let stripped = if let Some(bracket_start) = text.rfind('[') {
        let tail = &text[bracket_start..];
        if tail.ends_with(']') && (tail == "[]" || tail.contains("Energy") || tail.contains("Fat") || tail.contains("Food") || tail.contains("Bond")) {
            text[..bracket_start].trim()
        } else {
            text
        }
    } else {
        text
    };
    // Also filter any remaining lines that are purely a stat bracket
    stripped.lines()
        .filter(|l| !l.trim().starts_with('[') || !l.trim().ends_with(']'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
