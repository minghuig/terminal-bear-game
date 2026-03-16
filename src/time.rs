use serde::{Deserialize, Serialize};

pub const DAYS_PER_SEASON: u32 = 10;
pub const SEASONS_PER_YEAR: u32 = 4;
pub const FAT_THRESHOLD_FOR_HIBERNATION: f32 = 200.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    pub fn label(&self) -> &str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Fall => "Fall",
            Season::Winter => "Winter",
        }
    }

    pub fn next(self) -> Season {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Fall,
            Season::Fall => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }

    /// Emoji/symbol for the season.
    pub fn symbol(&self) -> &str {
        match self {
            Season::Spring => "🌿",
            Season::Summer => "☀️",
            Season::Fall => "🍂",
            Season::Winter => "❄️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTime {
    pub year: u32,
    pub season: Season,
    pub day: u32, // 1-10 within the season
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            year: 1,
            season: Season::Spring,
            day: 1,
        }
    }

    /// Advance one day. Returns true if a new year begins.
    pub fn advance_day(&mut self) -> DayAdvanceResult {
        self.day += 1;
        if self.day > DAYS_PER_SEASON {
            self.day = 1;
            let prev_season = self.season;
            self.season = self.season.next();
            if self.season == Season::Spring && prev_season == Season::Winter {
                self.year += 1;
                return DayAdvanceResult::NewYear;
            }
            if self.season == Season::Winter {
                return DayAdvanceResult::HibernationBegins;
            }
            return DayAdvanceResult::NewSeason(self.season);
        }
        DayAdvanceResult::Normal
    }

    pub fn is_last_fall_day(&self) -> bool {
        self.season == Season::Fall && self.day == DAYS_PER_SEASON
    }

    pub fn display(&self) -> String {
        format!(
            "Year {} | {} | Day {}",
            self.year,
            self.season.label(),
            self.day
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum DayAdvanceResult {
    Normal,
    NewSeason(Season),
    HibernationBegins,
    NewYear,
}
