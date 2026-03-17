use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Trait so the LLM backend can be swapped out.
pub trait LlmClient: Send + Sync {
    fn complete(&self, prompt: String) -> Result<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LlmProvider {
    Anthropic,
    OpenAi,
}

impl LlmProvider {
    pub fn label(&self) -> &str {
        match self {
            LlmProvider::Anthropic => "Anthropic (Claude)",
            LlmProvider::OpenAi => "OpenAI (GPT)",
        }
    }
}

pub fn create_client(provider: &LlmProvider, key: &str) -> Arc<dyn LlmClient> {
    match provider {
        LlmProvider::Anthropic => Arc::new(ClaudeClient::new(key.to_string())),
        LlmProvider::OpenAi => Arc::new(OpenAiClient::new(key.to_string())),
    }
}

// --- Claude implementation ---

pub struct ClaudeClient {
    api_key: String,
    model: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "claude-haiku-4-5-20251001".to_string(),
        }
    }
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
    system: String,
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Deserialize)]
struct ClaudeContent {
    text: String,
}

impl LlmClient for ClaudeClient {
    fn complete(&self, prompt: String) -> Result<String> {
        let client = reqwest::blocking::Client::new();

        let request = ClaudeRequest {
            model: self.model.clone(),
            max_tokens: 512,
            system: "You are a bear in a cozy terminal game. Respond only as instructed by the prompt.".to_string(),
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            return Err(anyhow::anyhow!("Anthropic API error {}: {}", status, body));
        }

        let response = resp.json::<ClaudeResponse>()?;

        let text = response
            .content
            .into_iter()
            .next()
            .map(|c| c.text)
            .unwrap_or_default();

        Ok(text)
    }
}

// --- OpenAI implementation ---

pub struct OpenAiClient {
    api_key: String,
    model: String,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gpt-4.1-2025-04-14".to_string(),
        }
    }
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    max_completion_tokens: u32,
    messages: Vec<OpenAiMessage>,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiChoiceMessage,
}

#[derive(Deserialize)]
struct OpenAiChoiceMessage {
    content: String,
}

impl LlmClient for OpenAiClient {
    fn complete(&self, prompt: String) -> Result<String> {
        let client = reqwest::blocking::Client::new();

        let request = OpenAiRequest {
            model: self.model.clone(),
            max_completion_tokens: 512,
            messages: vec![
                OpenAiMessage {
                    role: "system".to_string(),
                    content: "You are a bear in a cozy terminal game. Respond only as instructed by the prompt.".to_string(),
                },
                OpenAiMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
        };

        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&request)
            .send()?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            return Err(anyhow::anyhow!("OpenAI API error {}: {}", status, body));
        }

        let response = resp.json::<OpenAiResponse>()?;

        let text = response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .unwrap_or_default();

        Ok(text)
    }
}

// --- Stub for testing without an API key ---

pub struct StubClient;

impl LlmClient for StubClient {
    fn complete(&self, _prompt: String) -> Result<String> {
        Ok("Salmon. Good.".to_string())
    }
}

// --- Prompt builders ---

use crate::bear::{AgeStage, Bear};
use crate::events::EventRecord;
use crate::time::Season;

pub fn build_relax_prompt(bear: &Bear, season: Season, year: u32, theme_key: &str, follow_up: Option<&str>, relax_bond: bool) -> String {
    let season_desc = match season {
        Season::Spring => "Spring: the world is thawing, cold and fresh.",
        Season::Summer => "Summer: warm, unhurried, the days are long.",
        Season::Fall => "Fall: the air is turning. There's a restlessness underneath the stillness.",
        Season::Winter => "",
    };

    let follow_up_line = match follow_up {
        Some(prev) => format!("This is a returning scene — the bear has been here before. Previously: {prev}. Write a natural continuation, not a repeat.\n"),
        None => String::new(),
    };

    let benefit_instruction = if relax_bond {
        "Your person is quietly present with the bear during this moment. The scene includes both of you — unhurried, no agenda, just sharing the space.\n\
        End with: [Bond +X] where X is 2–5 depending on how connected the moment feels.\n\
        Only use Bond. Do not include Fat or Energy."
    } else {
        "The bear grazes or nibbles on something small during this moment. End with: [Fat +X] where X is 2–5.\n\
        Only use Fat. Do not include Energy or Bond."
    };

    format!(
        "You are narrating a cozy bear game set in coastal Alaska wilderness.\n\
        The bear's name is {name}. It is {season} of year {year}.\n\
        {season_desc}\n\
        The bear is taking a moment to rest and just exist.\n\
        THIS scene is about ONLY this one thing: {theme_key}.\n\
        {follow_up_line}\
        Do not include a title, heading, or label. Begin directly with the narrative.\n\n\
        Write a 3-5 sentence scene of this peaceful moment.\n\
        Second person (\"you\"). Calm, sensory, unhurried — focus on what the bear sees, smells, feels.\n\
        No tension, no decisions, no drama. Just the bear being.\n\
        {benefit_instruction}\n\
        Then on the very last line: [SUMMARY: one sentence in past tense describing what happened]",
        name = bear.name,
        season = season.label(),
    )
}

pub fn build_interact_prompt(bear: &Bear, theme_key: &str, choice_style: bool, follow_up: Option<&str>) -> String {
    let age_desc = match bear.age_stage() {
        AgeStage::Cub => "a tiny, clumsy bear cub — chaotic and adorable",
        AgeStage::Adolescent => "a gangly young bear — enthusiastic, a bit too rough, testing limits",
        AgeStage::Adult => "a large adult bear — calm, grounded, not easily impressed",
        AgeStage::Elder => "an old bear — slow and deliberate, a quiet dignity",
    };

    let bond_desc = if bear.bond >= 70.0 {
        "The bear is deeply comfortable with you — familiar, trusting, at ease in your presence."
    } else if bear.bond >= 40.0 {
        "The bear knows you well and is settled around you, though still very much its own creature."
    } else if bear.bond >= 15.0 {
        "The bear is warming up to you — cautious but curious, not yet fully at ease."
    } else {
        "The bear is still wary of you. It tolerates your presence but keeps its distance."
    };

    let ending = if choice_style {
        "Write a 2-3 sentence scene that ends at a moment of decision.\n\
        Second person (\"you\"). Warm, physical, grounded — no sentimentality.\n\n\
        Then output EXACTLY this block:\n\
        What do you do?\n\
        A) <one short action>\n\
        B) <one short action>\n\
        [A_RESULT: <one sentence outcome> | Bond +8, Energy -5]\n\
        [B_RESULT: <one sentence outcome> | Bond +4, Energy 0]\n\n\
        Rules:\n\
        - Bond is the primary stat. Range +3 to +15. Energy cost optional (-5 to -15).\n\
        - Only use: Bond, Energy\n\
        - After the B_RESULT line, add: [SUMMARY: one sentence in past tense describing what happened]\n\
        - Do not write anything after the SUMMARY line"
    } else {
        "Write a 3-5 sentence scene of this moment.\n\
        Second person (\"you\"). Warm, physical, grounded — no sentimentality.\n\
        End with a single bracketed line like: [Bond +8, Energy -5]\n\
        Bond is the primary stat. Range +5 to +15. Energy cost optional (-5 to -15).\n\
        Only use: Bond, Energy. No other stats.\n\
        Then on the very last line: [SUMMARY: one sentence in past tense describing what happened]"
    };

    let follow_up_line = match follow_up {
        Some(prev) => format!("This is a returning moment — you have shared something like this before. Previously: {prev}. Write a natural continuation, not a repeat.\n"),
        None => String::new(),
    };

    format!(
        "You are narrating a cozy bear game set in coastal Alaska wilderness.\n\
        The bear's name is {name}. It is {age_desc}.\n\
        {bond_desc}\n\
        This is a quiet moment of interaction between the bear and its person.\n\
        THIS scene is about ONLY this one thing: {theme_key}.\n\
        {follow_up_line}\
        No other animals or events. Stay focused on the bear and the moment.\n\
        Do not include a title, heading, or label. Begin directly with the narrative.\n\n\
        {ending}",
        name = bear.name,
    )
}

pub fn build_dialogue_prompt(bear: &Bear, player_message: &str) -> String {
    let stage = bear.age_stage();
    let voice = match stage {
        AgeStage::Cub => "You are a very young bear cub. Speak in very short, simple sentences. Almost like a toddler bear. Very food-focused.",
        AgeStage::Adolescent => "You are a young bear, still learning. Speak simply but with a bit more awareness than a cub.",
        AgeStage::Adult => "You are a mature adult bear. Calm and direct. Not verbose. Very grounded.",
        AgeStage::Elder => "You are an old wise bear. Still simple thoughts, but a quiet dignity. Occasional weary humor.",
    };

    let hunger_note = if bear.is_hungry() {
        " You are quite hungry and it is on your mind."
    } else {
        ""
    };

    let tired_note = if bear.is_tired() {
        " You are tired."
    } else {
        ""
    };

    let bond_note = if bear.bond >= 70.0 {
        " You are deeply bonded with this human. You trust them completely and feel warmth toward them."
    } else if bear.bond >= 40.0 {
        " You know this human well and trust them."
    } else if bear.bond >= 15.0 {
        " This human is familiar to you. You are warming up to them."
    } else {
        " This human is still somewhat new to you. You are cautious but curious."
    };

    format!(
        "{voice}{hunger_note}{tired_note}{bond_note}\n\
        Your name is {}. Respond to what your human companion says. \
        Keep it to 1-3 short sentences. Bear-brained: think about food, sleep, smells, territory, your human. \
        Not philosophical. Not clever. Just bear thoughts.\n\n\
        After your response, on a new line write exactly one of:\n\
        [interest: high] — topic is something a bear genuinely cares about (food, salmon, berries, sleep, smells, nature, territory, your human)\n\
        [interest: low] — topic makes no sense to you (technology, abstract ideas, human concepts)\n\
        [interest: neutral] — everything else\n\n\
        Human says: \"{player_message}\"\n\
        {}'s response:",
        bear.name, bear.name
    )
}

/// Simple (non-choice) event prompt — just a narrative with a stat bracket at the end.
pub fn build_simple_event_prompt(
    bear: &Bear,
    season: Season,
    year: u32,
    action: &str,
    theme_key: &str,
    past_events: &[EventRecord],
    follow_up: Option<&str>,
) -> String {
    build_event_prompt_with_style(bear, season, year, action, theme_key, past_events, false, follow_up)
}

pub fn build_event_prompt(
    bear: &Bear,
    season: Season,
    year: u32,
    action: &str,
    theme_key: &str,
    past_events: &[EventRecord],
    follow_up: Option<&str>,
) -> String {
    build_event_prompt_with_style(bear, season, year, action, theme_key, past_events, true, follow_up)
}

fn build_event_prompt_with_style(
    bear: &Bear,
    season: Season,
    year: u32,
    action: &str,
    theme_key: &str,
    past_events: &[EventRecord],
    choice_style: bool,
    follow_up: Option<&str>,
) -> String {
    let recent: Vec<String> = past_events
        .iter()
        .rev()
        .take(3)
        .map(|e| format!("- {} ({}): {}", e.theme_key, e.season, e.summary))
        .collect();
    let past_context = if recent.is_empty() {
        "No past events yet.".to_string()
    } else {
        format!(
            "Past events (for continuity only — do NOT incorporate these into this event):\n{}",
            recent.join("\n")
        )
    };
    let action_desc = match action {
        "fish" => "fishing at the river",
        "forage" => "foraging in the forest/meadow",
        "explore" => "exploring the territory",
        _ => action,
    };
    let season_desc = match season {
        Season::Spring => "Spring: the bear just woke from hibernation — thin, hungry, cautiously optimistic. The world is thawing. Rivers are running cold and fast.",
        Season::Summer => "Summer: easy living. Food is abundant, the days are long and warm. The bear is relaxed and unhurried.",
        Season::Fall => "Fall: urgency is setting in. The bear must fatten up before winter. Every meal matters. The salmon are running. There's an edge to everything.",
        Season::Winter => "Winter: the bear should be hibernating. Something has stirred it.",
    };
    let age_desc = match bear.age_stage() {
        AgeStage::Cub => "a small bear cub (years 1-2) — clumsy, inexperienced, easily intimidated",
        AgeStage::Adolescent => "an adolescent bear (years 3-5) — growing but still cautious, not yet dominant",
        AgeStage::Adult => "a prime adult bear — large, confident, willing to assert dominance",
        AgeStage::Elder => "an old elder bear — experienced and wise, picks fights carefully, commands respect",
    };

    let bond_desc = if bear.bond >= 70.0 {
        "The bear is calm and confident — deeply at ease in its territory, unhurried, comfortable in its own skin."
    } else if bear.bond >= 40.0 {
        "The bear is settled and secure — curious more than cautious, generally at peace."
    } else if bear.bond >= 15.0 {
        "The bear is cautious but warming up — still watchful, but less skittish than before."
    } else {
        "The bear is wary and unsettled — easily startled, quick to bristle."
    };

    let ending = if choice_style {
        "Write a 2-3 sentence narrative that sets the scene and ends at a moment of decision. \
        Second person (\"you\"). Cozy, grounded, nature-focused.\n\n\
        Then output EXACTLY this block (no extra text, no markdown):\n\
        What do you do?\n\
        A) <one short action>\n\
        B) <one short action>\n\
        [A_RESULT: <one sentence outcome> | Energy -10, Fat +5, Food +2]\n\
        [B_RESULT: <one sentence outcome> | Energy +5, Fat +15, Food +0]\n\n\
        Rules:\n\
        - Choices must have meaningfully different outcomes (any combo of good/neutral/bad is fine)\n\
        - Food: 0–8 items. 0 if no food gathered. Be generous on successful foraging/fishing.\n\
        - Fat: 0–15. Use for food-related events. 0 for non-food events.\n\
        - Energy: -15 to +5. Actions are tiring but not punishing.\n\
        - Only use: Energy, Fat, Food. Do NOT include Bond.\n\
        - After the B_RESULT line, add: [SUMMARY: one sentence in past tense describing what happened]\n\
        - Do not write anything after the SUMMARY line"
    } else {
        "Write a 3-5 sentence narrative of this event. \
        Second person (\"you\"). Cozy, grounded, nature-focused. \
        End with a single bracketed line like: [Energy -10, Fat +5, Food +3]\n\
        Food: 0–8 items. 0 if no food gathered. Be generous on successful foraging/fishing.\n\
        Fat: 0–15. Use for food-related events. 0 for non-food events.\n\
        Energy: -15 to +5. Actions are tiring but not punishing.\n\
        Only use: Energy, Fat, Food. Do NOT include Bond or Hunger.\n\
        Then on the very last line: [SUMMARY: one sentence in past tense describing what happened]"
    };

    let follow_up_line = match follow_up {
        Some(prev) => format!("This is a follow-up encounter — the bear has experienced this before. Previously: {prev}. Write a natural continuation, not a repeat.\n"),
        None => String::new(),
    };

    format!(
        "You are narrating a cozy bear game set in coastal Alaska/Kamchatka wilderness.\n\
        The bear's name is {name}. It is {season} of year {year}. The bear is {action_desc}.\n\
        The bear is {age_desc}.\n\
        {season_desc}\n\
        {bond_desc}\n\n\
        THIS event is about ONLY this one thing: {theme_key}.\n\
        {follow_up_line}\
        Do not mention any other animals, people, or events. Stay focused on this single encounter.\n\
        Do not include a title, heading, or label. Begin directly with the narrative.\n\n\
        {past_context}\n\n\
        {ending}",
        name = bear.name,
        season = season.label(),
        year = year,
    )
}

/// Find the most recent summary for a given theme key in the event log.
pub fn find_previous_summary<'a>(event_log: &'a [crate::events::EventRecord], theme_key: &str) -> Option<&'a str> {
    event_log.iter().rev().find(|e| e.theme_key == theme_key).map(|e| e.summary.as_str())
}

/// Extract and strip the [SUMMARY: ...] line from LLM output.
/// Returns (text_without_summary, summary).
pub fn extract_summary(raw: &str) -> (String, String) {
    let fallback = "Something happened.".to_string();
    for line in raw.lines().rev() {
        let trimmed = line.trim();
        if let Some(inner) = trimmed.strip_prefix("[SUMMARY:").and_then(|s| s.strip_suffix(']')) {
            let summary = inner.trim().to_string();
            let text = raw[..raw.rfind(trimmed).unwrap_or(raw.len())].trim_end().to_string();
            return (text, if summary.is_empty() { fallback } else { summary });
        }
    }
    (raw.trim().to_string(), fallback)
}

#[derive(Debug, Default, Clone)]
pub struct StatDeltas {
    pub bond: f32,
    pub energy: f32,
    pub fat: f32,
    pub food: u32,
}

#[derive(Debug, Clone)]
pub struct EventChoice {
    pub text: String,
    pub outcome_text: String,
    pub deltas: StatDeltas,
}

#[derive(Debug)]
pub struct ParsedEvent {
    pub narrative: String,
    pub choices: Option<[EventChoice; 2]>,
}

/// Parse the structured event response into narrative + choices.
/// Falls back to plain narrative if the format isn't found.
pub fn parse_event_response(raw: &str) -> ParsedEvent {
    let raw = raw.trim();

    // Split on "What do you do?"
    let Some((narrative_part, choices_part)) = raw.split_once("What do you do?") else {
        return ParsedEvent { narrative: raw.to_string(), choices: None };
    };

    let narrative = narrative_part.trim().to_string();

    // Extract choice texts: lines starting with "A)" and "B)"
    let mut choice_a_text = String::new();
    let mut choice_b_text = String::new();
    let mut result_a_raw = String::new();
    let mut result_b_raw = String::new();

    for line in choices_part.lines() {
        let line = line.trim();
        if line.starts_with("A)") {
            choice_a_text = line[2..].trim().trim_start_matches('<').trim_end_matches('>').trim().to_string();
        } else if line.starts_with("B)") {
            choice_b_text = line[2..].trim().trim_start_matches('<').trim_end_matches('>').trim().to_string();
        } else if line.starts_with("[A_RESULT:") {
            result_a_raw = line.to_string();
        } else if line.starts_with("[B_RESULT:") {
            result_b_raw = line.to_string();
        }
    }

    if choice_a_text.is_empty() || choice_b_text.is_empty()
        || result_a_raw.is_empty() || result_b_raw.is_empty()
    {
        return ParsedEvent { narrative, choices: None };
    }

    let Some(choice_a) = parse_result_line(&result_a_raw, choice_a_text) else {
        return ParsedEvent { narrative, choices: None };
    };
    let Some(choice_b) = parse_result_line(&result_b_raw, choice_b_text) else {
        return ParsedEvent { narrative, choices: None };
    };

    ParsedEvent { narrative, choices: Some([choice_a, choice_b]) }
}

fn parse_result_line(line: &str, choice_text: String) -> Option<EventChoice> {
    // Format: [A_RESULT: outcome text. | Stat +N, Stat -N]
    let inner = line.trim_start_matches('[').trim_end_matches(']');
    let colon_pos = inner.find(':')?;
    let after_colon = inner[colon_pos + 1..].trim();

    let (outcome_text, stats_str) = if let Some((t, s)) = after_colon.split_once('|') {
        (t.trim().to_string(), s.trim())
    } else {
        (after_colon.to_string(), "")
    };

    let deltas = parse_stat_str(stats_str);

    Some(EventChoice { text: choice_text, outcome_text, deltas })
}

pub fn parse_stat_str(s: &str) -> StatDeltas {
    let mut deltas = StatDeltas::default();
    for part in s.split(',') {
        let part = part.trim();
        if part.starts_with("Bond") {
            deltas.bond = parse_delta(part);
        } else if part.starts_with("Energy") {
            deltas.energy = parse_delta(part);
        } else if part.starts_with("Fat") {
            deltas.fat = parse_delta(part);
        } else if part.starts_with("Food") {
            deltas.food = parse_delta(part).max(0.0) as u32;
        }
    }
    deltas
}

/// Parse `[interest: high/neutral/low]` from a dialogue response.
/// Returns (cleaned response text, bond delta).
pub fn parse_dialogue_response(raw: &str) -> (String, f32) {
    let mut interest = 0.0_f32; // default neutral → small gain
    let mut lines: Vec<&str> = Vec::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed == "[interest: high]" {
            interest = 10.0;
        } else if trimmed == "[interest: neutral]" {
            interest = 3.0;
        } else if trimmed == "[interest: low]" {
            interest = 0.0;
        } else {
            lines.push(line);
        }
    }

    let text = lines.join("\n").trim().to_string();
    // If the LLM didn't include the tag at all, give a small default bond gain
    let bond = if interest == 0.0 && !raw.contains("[interest:") {
        3.0
    } else {
        interest
    };
    (text, bond)
}

// Keep this for any legacy use
pub fn parse_stat_deltas(text: &str) -> StatDeltas {
    if let Some(start) = text.rfind('[') {
        if let Some(end) = text.rfind(']') {
            if end > start {
                return parse_stat_str(&text[start + 1..end]);
            }
        }
    }
    StatDeltas::default()
}

fn parse_delta(s: &str) -> f32 {
    if let Some(plus) = s.find('+') {
        s[plus + 1..].trim().parse().unwrap_or(0.0)
    } else if let Some(minus) = s.rfind('-') {
        -s[minus + 1..].trim().parse::<f32>().unwrap_or(0.0)
    } else {
        0.0
    }
}
