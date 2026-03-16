use serde::{Deserialize, Serialize};
use crate::time::Season;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Pronoun {
    He,
    She,
    They,
}

impl Pronoun {
    pub fn subject(&self) -> &str {
        match self {
            Pronoun::He => "he",
            Pronoun::She => "she",
            Pronoun::They => "they",
        }
    }

    pub fn object(&self) -> &str {
        match self {
            Pronoun::He => "him",
            Pronoun::She => "her",
            Pronoun::They => "them",
        }
    }

    pub fn possessive(&self) -> &str {
        match self {
            Pronoun::He => "his",
            Pronoun::She => "her",
            Pronoun::They => "their",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgeStage {
    Cub,        // years 1-2
    Adolescent, // years 3-5
    Adult,      // years 6-16
    Elder,      // years 17-20
}

impl AgeStage {
    pub fn label(&self) -> &str {
        match self {
            AgeStage::Cub => "Cub",
            AgeStage::Adolescent => "Adolescent",
            AgeStage::Adult => "Adult",
            AgeStage::Elder => "Elder",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bear {
    pub name: String,
    pub pronoun: Pronoun,
    pub age_years: u32,

    // Stats: all 0-100 except fat_reserves which can go above 100 in fall
    pub hunger: f32,
    pub energy: f32,
    pub fat_reserves: f32,

    // Relationship
    #[serde(default)]
    pub bond: f32, // 0.0-100.0, grows through pet/talk/play

    // Skills improve with practice
    pub fishing_skill: f32, // 0.0-1.0
}

impl Bear {
    pub fn new(name: String, pronoun: Pronoun) -> Self {
        Self {
            name,
            pronoun,
            age_years: 1,
            hunger: 70.0,
            energy: 80.0,
            fat_reserves: 30.0,
            bond: 0.0,
            fishing_skill: 0.0,
        }
    }

    pub fn age_stage(&self) -> AgeStage {
        match self.age_years {
            1..=2 => AgeStage::Cub,
            3..=5 => AgeStage::Adolescent,
            6..=16 => AgeStage::Adult,
            _ => AgeStage::Elder,
        }
    }

    pub fn is_hungry(&self) -> bool {
        self.hunger < 30.0
    }

    pub fn is_tired(&self) -> bool {
        self.energy < 25.0
    }

    /// Returns true if hunger has hit 0 — bear leaves.
    pub fn is_gone(&self) -> bool {
        self.hunger <= 0.0
    }

    /// Clamp all stats to valid ranges.
    pub fn clamp_stats(&mut self) {
        self.hunger = self.hunger.clamp(0.0, 100.0);
        self.energy = self.energy.clamp(0.0, 100.0);
        self.fat_reserves = self.fat_reserves.clamp(0.0, 200.0);
        self.bond = self.bond.clamp(0.0, 100.0);
        self.fishing_skill = self.fishing_skill.clamp(0.0, 1.0);
    }

    /// Called each day — passive stat decay.
    pub fn daily_decay(&mut self, season: Season) {
        // Exhausted bears burn more energy just staying alive
        let exhausted_drain = if self.energy < 25.0 { 3.0 } else { 0.0 };
        self.hunger -= 8.0 + exhausted_drain;
        self.energy -= 5.0;

        // Fat drains slowly each day, faster when hungry (body compensates)
        let base_fat_drain = match season {
            Season::Spring => 2.5, // lean times, burning reserves
            Season::Summer => 2.0, // still burning through the season
            Season::Fall => 1.0,   // building up, but reserves still cost something
            Season::Winter => 0.0, // handled by hibernation reset
        };
        let hunger_drain = if self.hunger < 30.0 { 2.0 } else { 0.0 };
        self.fat_reserves -= base_fat_drain + hunger_drain;

        // Bond fades without interaction
        self.bond -= 2.0;

        self.clamp_stats();
    }

    pub fn feed(&mut self, nutrition: f32, fat_multiplier: f32) {
        self.hunger += nutrition;
        self.fat_reserves += nutrition * 0.3 * fat_multiplier;
        self.clamp_stats();
    }


    pub fn is_exhausted(&self) -> bool {
        self.energy <= 0.0
    }
}
