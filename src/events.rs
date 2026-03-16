use serde::{Deserialize, Serialize};
use crate::bear::AgeStage;
use crate::time::Season;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventTheme {
    // ── Universal (all actions) ───────────────────────────────────────────────
    Thunderstorm,
    OtherBear,
    WolfPackEncounter,
    Photographer,
    Rangers,

    // Universal one-time
    WhaleCarcass,
    ElkCarcass,
    ForestFireSmell,

    // Universal follow-ups (require prerequisite in event log)
    FamiliarBearReturns, // requires: other_bear
    CubsAtPlay,          // requires: other_cubs

    // ── Season-specific universal ─────────────────────────────────────────────
    SpringFloodCrossing,      // spring

    // ── Age-specific universal ────────────────────────────────────────────────

    // Cub
    GetLost,
    MotherBearNearby,
    EscapeMaleBear,     // tense — male bear stalking you
    MotherTeachesFishing, // fish — learning moment
    NeighboringCub,     // playful — cub from nearby litter
    ShoedByStranger,    // big adult chases you off

    // Juvenile
    YoungBearPlayfight,
    RivalEncounter,     // tense standoff with another adolescent bear
    DisplacedFromSpot,  // bigger bear takes your fishing hole
    CuriousFemale,      // young female checks you out
    SubAdultBuddy,      // another sub-adult tries to hang around

    // Adult
    TerritoryIntruder,
    PotentialMate,
    CubsFromOtherMother,
    FamiliarRival,      // fish — bear you know from the salmon run
    MotherWithNewborns, // spring/summer — back away carefully
    RiverStandoff,      // fish — dominance display, no clear winner
    BearFight,          // choice: stand ground vs back down

    // Adult + Elder
    OldScratchMarks,

    // Elder
    OldMemory,
    SlowerNow,
    YoungBearWatching,
    DisplacedByYoung,   // fish — younger bear pushes you off your spot
    DistantBear,        // explore — watching another bear from far away
    BearLikeYouOnce,    // young bear that moves like you used to

    // ── Fish-specific ─────────────────────────────────────────────────────────
    SalmonRun,
    RiverCrossing,
    EagleStealsYourCatch,
    OtterCompetition,
    SalmonFrenzy,  // fall only
    IcyRapids,    // spring only

    // ── Forage-specific ───────────────────────────────────────────────────────
    BerryPatch,
    GrubsUnderLog,
    BeehiveDiscovery,
    HikersWithFood,
    MushroomPatch,
    AbandonedCampsite,
    PorcupineQuills,
    BeeTree,
    WildOnions,      // spring only
    TidepoolShellfish, // summer, fall
    DiggingRoots,    // forage

    // ── Explore-specific ──────────────────────────────────────────────────────
    CaveDiscovery,
    FoxFollowing,
    MotherMooseWarning, // spring, summer
    RavenLeadsToFood,
    FallenTreeBridge,
    StormTossedBeach,   // fall
    BeachedSeal,        // fall — one-time
    LynxSighting,       // rare
    CaribouHerd,        // spring, fall
    DallSheep,          // spring, summer

    // ── Relax-specific ────────────────────────────────────────────────────────
    ScenicRest,
    NapInMeadow,
    WhalesFromShore,    // summer, fall
    Aurora,             // spring, fall
    MigratoryBirds,     // spring, fall
    PreHibernationRestlessness, // fall
    HotSpring,          // bond ≥15
    ViewFromCliff,      // bond ≥40
    Sunbathing,         // summer
    RiverFloat,         // spring, summer
    Swimming,           // spring, summer, fall
    WatchingStorm,      // fall
    Waterfall,
    RainUnderTree,

    // ── Forage + Explore ──────────────────────────────────────────────────────
    MarmotHunt,         // forage, explore
    SnowshoeHare,       // forage, explore — higher weight spring/winter
    WeaselEncounter,    // forage, explore

    // ── Universal (new animals) ────────────────────────────────────────────────
    WolverineEncounter, // explore + universal
    LoneWolf,           // universal
    DogEncounter,       // universal — near humans

    // ── Interact (human present, bond-focused) ─────────────────────────────────

    // Cub
    InteractRolling,     // rolling around in a meadow together
    InteractSleepOnFeet, // cub falls asleep on your feet
    InteractStickGift,   // cub brings you a stick as a gift
    InteractHidesBehind, // cub hides behind you from something scary

    // Juvenile
    InteractPlayWrestle, // mock wrestling that gets slightly too rough
    InteractRiverSit,    // sitting quietly at the river together
    InteractShowsFind,   // juvenile shows you something they found
    InteractTooOld,      // brief moment of closeness before they act too cool

    // Adult
    InteractQuietTime,   // quiet companionship at a favorite spot
    InteractBearGift,    // bear brings you something
    InteractParallel,    // you sit, bear does its thing nearby

    // Elder
    InteractSlowWalk,    // moving slowly through familiar territory together
    InteractRidgeWatch,  // watching the world from a ridge together
    InteractJustPresent, // elder rests close — just being there

    // High bond (70+) — universal, trust fully earned
    InteractSeeksYouOut,  // bear comes looking for you
    InteractGrooming,     // bear grooms/nuzzles you
    InteractLeansAsleep,  // bear falls asleep leaning against you
    InteractFavoriteSpot, // a place you always return to together
    InteractProtects,     // bear stands between you and something
    InteractScratchSpot,  // you find the spot; bear goes completely still
}

impl EventTheme {
    pub fn all() -> Vec<EventTheme> {
        vec![
            EventTheme::Thunderstorm,
            EventTheme::OtherBear,
            EventTheme::WolfPackEncounter,
            EventTheme::Photographer,
            EventTheme::Rangers,
            EventTheme::WhaleCarcass,
            EventTheme::ElkCarcass,
            EventTheme::ForestFireSmell,
            EventTheme::FamiliarBearReturns,
            EventTheme::CubsAtPlay,
            EventTheme::SpringFloodCrossing,
            EventTheme::GetLost,
            EventTheme::MotherBearNearby,
            EventTheme::EscapeMaleBear,
            EventTheme::MotherTeachesFishing,
            EventTheme::NeighboringCub,
            EventTheme::ShoedByStranger,
            EventTheme::YoungBearPlayfight,
            EventTheme::RivalEncounter,
            EventTheme::DisplacedFromSpot,
            EventTheme::CuriousFemale,
            EventTheme::SubAdultBuddy,
            EventTheme::TerritoryIntruder,
            EventTheme::PotentialMate,
            EventTheme::CubsFromOtherMother,
            EventTheme::FamiliarRival,
            EventTheme::MotherWithNewborns,
            EventTheme::RiverStandoff,
            EventTheme::BearFight,
            EventTheme::OldScratchMarks,
            EventTheme::OldMemory,
            EventTheme::SlowerNow,
            EventTheme::YoungBearWatching,
            EventTheme::DisplacedByYoung,
            EventTheme::DistantBear,
            EventTheme::BearLikeYouOnce,
            EventTheme::SalmonRun,
            EventTheme::RiverCrossing,
            EventTheme::EagleStealsYourCatch,
            EventTheme::OtterCompetition,
            EventTheme::SalmonFrenzy,
            EventTheme::IcyRapids,
            EventTheme::BerryPatch,
            EventTheme::GrubsUnderLog,
            EventTheme::BeehiveDiscovery,
            EventTheme::HikersWithFood,
            EventTheme::MushroomPatch,
            EventTheme::AbandonedCampsite,
            EventTheme::PorcupineQuills,
            EventTheme::BeeTree,
            EventTheme::WildOnions,
            EventTheme::TidepoolShellfish,
            EventTheme::DiggingRoots,
            EventTheme::CaveDiscovery,
            EventTheme::HotSpring,
            EventTheme::ViewFromCliff,
            EventTheme::FoxFollowing,
            EventTheme::MotherMooseWarning,
            EventTheme::RavenLeadsToFood,
            EventTheme::FallenTreeBridge,
            EventTheme::StormTossedBeach,
            EventTheme::BeachedSeal,
            EventTheme::LynxSighting,
            EventTheme::CaribouHerd,
            EventTheme::DallSheep,
            EventTheme::ScenicRest,
            EventTheme::NapInMeadow,
            EventTheme::WhalesFromShore,
            EventTheme::Aurora,
            EventTheme::MigratoryBirds,
            EventTheme::PreHibernationRestlessness,
            EventTheme::Sunbathing,
            EventTheme::RiverFloat,
            EventTheme::Swimming,
            EventTheme::WatchingStorm,
            EventTheme::Waterfall,
            EventTheme::RainUnderTree,
            EventTheme::MarmotHunt,
            EventTheme::SnowshoeHare,
            EventTheme::WeaselEncounter,
            EventTheme::WolverineEncounter,
            EventTheme::LoneWolf,
            EventTheme::DogEncounter,
            EventTheme::InteractRolling,
            EventTheme::InteractSleepOnFeet,
            EventTheme::InteractStickGift,
            EventTheme::InteractHidesBehind,
            EventTheme::InteractPlayWrestle,
            EventTheme::InteractRiverSit,
            EventTheme::InteractShowsFind,
            EventTheme::InteractTooOld,
            EventTheme::InteractQuietTime,
            EventTheme::InteractBearGift,
            EventTheme::InteractParallel,
            EventTheme::InteractSlowWalk,
            EventTheme::InteractRidgeWatch,
            EventTheme::InteractJustPresent,
            EventTheme::InteractSeeksYouOut,
            EventTheme::InteractGrooming,
            EventTheme::InteractLeansAsleep,
            EventTheme::InteractFavoriteSpot,
            EventTheme::InteractProtects,
            EventTheme::InteractScratchSpot,
        ]
    }

    pub fn key(&self) -> &str {
        match self {
            EventTheme::Thunderstorm => "thunderstorm",
            EventTheme::OtherBear => "other_bear",
            EventTheme::WolfPackEncounter => "wolf_pack",
            EventTheme::Photographer => "photographer",
            EventTheme::Rangers => "rangers",
            EventTheme::WhaleCarcass => "whale_carcass",
            EventTheme::ElkCarcass => "elk_carcass",
            EventTheme::ForestFireSmell => "forest_fire",
            EventTheme::FamiliarBearReturns => "familiar_bear",
            EventTheme::CubsAtPlay => "cubs_at_play",
            EventTheme::SpringFloodCrossing => "spring_flood",
            EventTheme::GetLost => "get_lost",
            EventTheme::MotherBearNearby => "mother_bear_nearby",
            EventTheme::EscapeMaleBear => "escape_male_bear",
            EventTheme::MotherTeachesFishing => "mother_teaches_fishing",
            EventTheme::NeighboringCub => "neighboring_cub",
            EventTheme::ShoedByStranger => "shoed_by_stranger",
            EventTheme::YoungBearPlayfight => "young_bear_playfight",
            EventTheme::RivalEncounter => "rival_encounter",
            EventTheme::DisplacedFromSpot => "displaced_from_spot",
            EventTheme::CuriousFemale => "curious_female",
            EventTheme::SubAdultBuddy => "sub_adult_buddy",
            EventTheme::TerritoryIntruder => "territory_intruder",
            EventTheme::PotentialMate => "potential_mate",
            EventTheme::CubsFromOtherMother => "other_cubs",
            EventTheme::FamiliarRival => "familiar_rival",
            EventTheme::MotherWithNewborns => "mother_with_newborns",
            EventTheme::RiverStandoff => "river_standoff",
            EventTheme::BearFight => "bear_fight",
            EventTheme::OldScratchMarks => "old_scratch_marks",
            EventTheme::OldMemory => "old_memory",
            EventTheme::SlowerNow => "slower_now",
            EventTheme::YoungBearWatching => "young_bear_watching",
            EventTheme::DisplacedByYoung => "displaced_by_young",
            EventTheme::DistantBear => "distant_bear",
            EventTheme::BearLikeYouOnce => "bear_like_you_once",
            EventTheme::SalmonRun => "salmon_run",
            EventTheme::RiverCrossing => "river_crossing",
            EventTheme::EagleStealsYourCatch => "eagle_steals_catch",
            EventTheme::OtterCompetition => "otter_competition",
            EventTheme::SalmonFrenzy => "salmon_frenzy",
            EventTheme::IcyRapids => "icy_rapids",
            EventTheme::BerryPatch => "berry_patch",
            EventTheme::GrubsUnderLog => "grubs_under_log",
            EventTheme::BeehiveDiscovery => "beehive",
            EventTheme::HikersWithFood => "hikers",
            EventTheme::MushroomPatch => "mushroom_patch",
            EventTheme::AbandonedCampsite => "abandoned_campsite",
            EventTheme::PorcupineQuills => "porcupine_quills",
            EventTheme::BeeTree => "bee_tree",
            EventTheme::WildOnions => "wild_onions",
            EventTheme::TidepoolShellfish => "tidepool_shellfish",
            EventTheme::DiggingRoots => "digging_roots",
            EventTheme::CaveDiscovery => "cave_discovery",
            EventTheme::FoxFollowing => "fox_following",
            EventTheme::MotherMooseWarning => "mother_moose_warning",
            EventTheme::RavenLeadsToFood => "raven_leads_to_food",
            EventTheme::FallenTreeBridge => "fallen_tree_bridge",
            EventTheme::StormTossedBeach => "storm_tossed_beach",
            EventTheme::BeachedSeal => "beached_seal",
            EventTheme::LynxSighting => "lynx_sighting",
            EventTheme::CaribouHerd => "caribou_herd",
            EventTheme::DallSheep => "dall_sheep",
            EventTheme::ScenicRest => "scenic_rest",
            EventTheme::NapInMeadow => "nap_in_meadow",
            EventTheme::WhalesFromShore => "whales_from_shore",
            EventTheme::Aurora => "aurora",
            EventTheme::MigratoryBirds => "migratory_birds",
            EventTheme::PreHibernationRestlessness => "pre_hibernation_restlessness",
            EventTheme::HotSpring => "hot_spring",
            EventTheme::ViewFromCliff => "view_from_cliff",
            EventTheme::Sunbathing => "sunbathing",
            EventTheme::RiverFloat => "river_float",
            EventTheme::Swimming => "swimming",
            EventTheme::WatchingStorm => "watching_storm",
            EventTheme::Waterfall => "waterfall",
            EventTheme::RainUnderTree => "rain_under_tree",
            EventTheme::MarmotHunt => "marmot_hunt",
            EventTheme::SnowshoeHare => "snowshoe_hare",
            EventTheme::WeaselEncounter => "weasel_encounter",
            EventTheme::WolverineEncounter => "wolverine_encounter",
            EventTheme::LoneWolf => "lone_wolf",
            EventTheme::DogEncounter => "dog_encounter",
            EventTheme::InteractRolling => "interact_rolling",
            EventTheme::InteractSleepOnFeet => "interact_sleep_on_feet",
            EventTheme::InteractStickGift => "interact_stick_gift",
            EventTheme::InteractHidesBehind => "interact_hides_behind",
            EventTheme::InteractPlayWrestle => "interact_play_wrestle",
            EventTheme::InteractRiverSit => "interact_river_sit",
            EventTheme::InteractShowsFind => "interact_shows_find",
            EventTheme::InteractTooOld => "interact_too_old",
            EventTheme::InteractQuietTime => "interact_quiet_time",
            EventTheme::InteractBearGift => "interact_bear_gift",
            EventTheme::InteractParallel => "interact_parallel",
            EventTheme::InteractSlowWalk => "interact_slow_walk",
            EventTheme::InteractRidgeWatch => "interact_ridge_watch",
            EventTheme::InteractJustPresent => "interact_just_present",
            EventTheme::InteractSeeksYouOut => "interact_seeks_you_out",
            EventTheme::InteractGrooming => "interact_grooming",
            EventTheme::InteractLeansAsleep => "interact_leans_asleep",
            EventTheme::InteractFavoriteSpot => "interact_favorite_spot",
            EventTheme::InteractProtects => "interact_protects",
            EventTheme::InteractScratchSpot => "interact_scratch_spot",
        }
    }

    /// Build a weighted pool of themes for the given context.
    /// `event_log` is used to check prerequisites and recent themes for cooldown.
    pub fn pool_for(
        season: Season,
        action: &str,
        age: &AgeStage,
        event_log: &[EventRecord],
        bond: f32,
    ) -> Vec<(EventTheme, u32)> {
        let mut pool: Vec<(EventTheme, u32)> = Vec::new();

        let has_seen = |key: &str| event_log.iter().any(|e| e.theme_key == key);

        // ── Universal ────────────────────────────────────────────────────────
        pool.push((EventTheme::Thunderstorm, 3));
        pool.push((EventTheme::OtherBear, 5));
        pool.push((EventTheme::WolfPackEncounter, 4));
        pool.push((EventTheme::Photographer, 3));
        pool.push((EventTheme::Rangers, 2));
        pool.push((EventTheme::WhaleCarcass, 1));
        pool.push((EventTheme::ElkCarcass, 2));
        pool.push((EventTheme::ForestFireSmell, 1));
        pool.push((EventTheme::LoneWolf, 3));
        pool.push((EventTheme::DogEncounter, 2));
        pool.push((EventTheme::WolverineEncounter, 3));

        // Follow-ups
        if has_seen("other_bear") {
            pool.push((EventTheme::FamiliarBearReturns, 4));
        }
        if has_seen("other_cubs") {
            pool.push((EventTheme::CubsAtPlay, 4));
        }

        // ── Season-specific universal ─────────────────────────────────────────
        if season == Season::Spring {
            pool.push((EventTheme::SpringFloodCrossing, 3));
        }

        // ── Age-specific universal ────────────────────────────────────────────
        match age {
            AgeStage::Cub => {
                pool.push((EventTheme::GetLost, 4));
                pool.push((EventTheme::MotherBearNearby, 4));
                pool.push((EventTheme::EscapeMaleBear, 3));
                pool.push((EventTheme::NeighboringCub, 4));
                pool.push((EventTheme::ShoedByStranger, 3));
            }
            AgeStage::Adolescent => {
                pool.push((EventTheme::YoungBearPlayfight, 5));
                pool.push((EventTheme::RivalEncounter, 4));
                pool.push((EventTheme::DisplacedFromSpot, 4));
                pool.push((EventTheme::CuriousFemale, 3));
                pool.push((EventTheme::SubAdultBuddy, 4));
            }
            AgeStage::Adult => {
                pool.push((EventTheme::TerritoryIntruder, 4));
                pool.push((EventTheme::PotentialMate, 3));
                pool.push((EventTheme::CubsFromOtherMother, 3));
                pool.push((EventTheme::OldScratchMarks, 3));
                pool.push((EventTheme::RiverStandoff, 4));
                pool.push((EventTheme::BearFight, 3));
                if matches!(season, Season::Spring | Season::Summer) {
                    pool.push((EventTheme::MotherWithNewborns, 3));
                }
            }
            AgeStage::Elder => {
                pool.push((EventTheme::OldScratchMarks, 3));
                pool.push((EventTheme::DisplacedByYoung, 4));
                pool.push((EventTheme::BearLikeYouOnce, 3));
                // Bond-gated elder events
                if bond >= 40.0 {
                    pool.push((EventTheme::YoungBearWatching, 4));
                    pool.push((EventTheme::DistantBear, 3));
                }
                if bond >= 70.0 {
                    pool.push((EventTheme::OldMemory, 5));
                    pool.push((EventTheme::SlowerNow, 4));
                }
            }
        }

        // ── Fish-specific ─────────────────────────────────────────────────────
        if action == "fish" {
            let salmon_weight = match season {
                Season::Fall => 20,
                Season::Summer => 12,
                _ => 5,
            };
            pool.push((EventTheme::SalmonRun, salmon_weight));
            pool.push((EventTheme::RiverCrossing, 6));
            pool.push((EventTheme::EagleStealsYourCatch, 5));
            pool.push((EventTheme::OtterCompetition, 5));

            if season == Season::Fall {
                pool.push((EventTheme::SalmonFrenzy, 8));
            }
            if season == Season::Spring {
                pool.push((EventTheme::IcyRapids, 5));
            }
            if matches!(age, AgeStage::Cub) {
                pool.push((EventTheme::MotherTeachesFishing, 5));
            }
            if matches!(age, AgeStage::Adult) {
                pool.push((EventTheme::FamiliarRival, 4));
            }
            if matches!(age, AgeStage::Elder) {
                pool.push((EventTheme::DisplacedByYoung, 4));
            }
        }

        // ── Forage-specific ───────────────────────────────────────────────────
        if action == "forage" {
            let berry_weight = match season {
                Season::Summer | Season::Fall => 15,
                Season::Spring => 6,
                Season::Winter => 0,
            };
            if berry_weight > 0 {
                pool.push((EventTheme::BerryPatch, berry_weight));
            }
            pool.push((EventTheme::GrubsUnderLog, 8));
            pool.push((EventTheme::BeehiveDiscovery, 4));
            pool.push((EventTheme::HikersWithFood, 3));
            pool.push((EventTheme::MushroomPatch, 5));
            pool.push((EventTheme::AbandonedCampsite, 3));
            pool.push((EventTheme::PorcupineQuills, 4));
            pool.push((EventTheme::BeeTree, 3));

            if season == Season::Spring {
                pool.push((EventTheme::WildOnions, 6));
            }
            if matches!(season, Season::Summer | Season::Fall) {
                pool.push((EventTheme::TidepoolShellfish, 4));
            }
            pool.push((EventTheme::MarmotHunt, 4));
            pool.push((EventTheme::WeaselEncounter, 4));
            pool.push((EventTheme::DiggingRoots, 6));
            let hare_weight = match season {
                Season::Spring => 6,
                _ => 3,
            };
            pool.push((EventTheme::SnowshoeHare, hare_weight));
        }

        // ── Explore-specific ──────────────────────────────────────────────────
        if action == "explore" {
            pool.push((EventTheme::BeehiveDiscovery, 3));
            pool.push((EventTheme::HikersWithFood, 4));
            pool.push((EventTheme::RiverCrossing, 4));
            pool.push((EventTheme::CaveDiscovery, 4));
            pool.push((EventTheme::FallenTreeBridge, 3));
            pool.push((EventTheme::PorcupineQuills, 3));
            pool.push((EventTheme::TidepoolShellfish, 3));
            pool.push((EventTheme::LynxSighting, 1));
            pool.push((EventTheme::MarmotHunt, 3));
            pool.push((EventTheme::WeaselEncounter, 3));

            if bond >= 15.0 {
                pool.push((EventTheme::FoxFollowing, 4));
            }
            if bond >= 40.0 {
                pool.push((EventTheme::RavenLeadsToFood, 4));
            }

            if matches!(season, Season::Spring | Season::Summer) {
                pool.push((EventTheme::MotherMooseWarning, 4));
            }
            if season == Season::Fall {
                pool.push((EventTheme::StormTossedBeach, 4));
                pool.push((EventTheme::BeachedSeal, 2));
            }
            let hare_weight = match season {
                Season::Spring => 5,
                _ => 2,
            };
            pool.push((EventTheme::SnowshoeHare, hare_weight));
            if matches!(season, Season::Spring | Season::Fall) {
                pool.push((EventTheme::CaribouHerd, 5));
            }
            if matches!(season, Season::Spring | Season::Summer) {
                pool.push((EventTheme::DallSheep, 3));
            }
        }

        pool
    }

    /// Weighted pool of interact themes for the given age stage and bond level.
    pub fn pool_for_interact(age: &AgeStage, bond: f32) -> Vec<(EventTheme, u32)> {
        let mut pool = match age {
            AgeStage::Cub => vec![
                (EventTheme::InteractRolling, 5),
                (EventTheme::InteractSleepOnFeet, 5),
                (EventTheme::InteractStickGift, 4),
                (EventTheme::InteractHidesBehind, 4),
            ],
            AgeStage::Adolescent => vec![
                (EventTheme::InteractPlayWrestle, 5),
                (EventTheme::InteractRiverSit, 4),
                (EventTheme::InteractShowsFind, 4),
                (EventTheme::InteractTooOld, 4),
            ],
            AgeStage::Adult => vec![
                (EventTheme::InteractQuietTime, 5),
                (EventTheme::InteractBearGift, 4),
                (EventTheme::InteractParallel, 5),
            ],
            AgeStage::Elder => vec![
                (EventTheme::InteractSlowWalk, 5),
                (EventTheme::InteractRidgeWatch, 5),
                (EventTheme::InteractJustPresent, 5),
            ],
        };

        if bond >= 70.0 {
            pool.push((EventTheme::InteractSeeksYouOut, 5));
            pool.push((EventTheme::InteractGrooming, 4));
            pool.push((EventTheme::InteractLeansAsleep, 5));
            pool.push((EventTheme::InteractFavoriteSpot, 4));
            pool.push((EventTheme::InteractProtects, 3));
            pool.push((EventTheme::InteractScratchSpot, 4));
        }

        pool
    }

    /// Weighted pool of relax themes — peaceful, contemplative moments.
    pub fn pool_for_relax(season: Season, bond: f32) -> Vec<(EventTheme, u32)> {
        let mut pool = vec![
            (EventTheme::ScenicRest, 5),
            (EventTheme::NapInMeadow, 5),
            (EventTheme::Waterfall, 4),
            (EventTheme::RainUnderTree, 4),
        ];

        match season {
            Season::Spring => {
                pool.push((EventTheme::MigratoryBirds, 4));
                pool.push((EventTheme::RiverFloat, 3));
                pool.push((EventTheme::Swimming, 4));
                pool.push((EventTheme::Aurora, 2));
            }
            Season::Summer => {
                pool.push((EventTheme::Sunbathing, 6));
                pool.push((EventTheme::RiverFloat, 5));
                pool.push((EventTheme::Swimming, 6));
                pool.push((EventTheme::WhalesFromShore, 3));
            }
            Season::Fall => {
                pool.push((EventTheme::MigratoryBirds, 4));
                pool.push((EventTheme::PreHibernationRestlessness, 5));
                pool.push((EventTheme::WatchingStorm, 5));
                pool.push((EventTheme::Swimming, 2));
                pool.push((EventTheme::WhalesFromShore, 3));
                pool.push((EventTheme::Aurora, 3));
            }
            Season::Winter => {}
        }

        if bond >= 15.0 {
            pool.push((EventTheme::HotSpring, 4));
        }
        if bond >= 40.0 {
            pool.push((EventTheme::ViewFromCliff, 5));
        }

        pool
    }

    /// Pick a theme from the weighted pool.
    /// Recent themes (last 4) have their weight reduced to 1 instead of being
    /// hard-banned, so small pools don't hit "nothing happened."
    pub fn pick(
        pool: Vec<(EventTheme, u32)>,
        recent: &[String],
    ) -> Option<EventTheme> {
        let filtered: Vec<(EventTheme, u32)> = pool
            .into_iter()
            .map(|(theme, weight)| {
                let key = theme.key();
                let w = if recent.contains(&key.to_string()) { 1 } else { weight };
                (theme, w)
            })
            .collect();

        let total: u32 = filtered.iter().map(|(_, w)| w).sum();
        if total == 0 {
            return None;
        }

        let mut roll = fastrand_range(total);
        for (theme, weight) in filtered {
            if roll < weight {
                return Some(theme);
            }
            roll -= weight;
        }
        None
    }
}

fn fastrand_range(n: u32) -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    // Mix seconds and nanoseconds so we don't hit macOS clock granularity issues
    let seed = (d.as_secs() as u32).wrapping_mul(2654435761) ^ d.subsec_nanos();
    seed % n
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub theme_key: String,
    pub summary: String,
    pub season: String,
    pub year: u32,
    pub day: u32,
}

impl EventRecord {
    pub fn new(theme: &EventTheme, summary: String, season: Season, year: u32, day: u32) -> Self {
        Self {
            theme_key: theme.key().to_string(),
            summary,
            season: season.label().to_string(),
            year,
            day,
        }
    }
}
