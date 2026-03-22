use crate::bear::AgeStage;

/// Returns the ASCII art for the bear at a given life stage.
pub fn bear_art(stage: &AgeStage) -> &'static str {
    match stage {
        AgeStage::Cub => CUB,
        AgeStage::Adolescent => JUVENILE,
        AgeStage::Adult => ADULT,
        AgeStage::Elder => ELDER,
    }
}

/// Returns the sleeping bear art (closed eyes, z's) for the hibernation screen.
pub fn bear_sleep_art(stage: &AgeStage) -> &'static str {
    match stage {
        AgeStage::Cub => CUB_SLEEP,
        AgeStage::Adolescent => JUVENILE_SLEEP,
        AgeStage::Adult => ADULT_SLEEP,
        AgeStage::Elder => ELDER_SLEEP,
    }
}

// ── CUB ──────────────────────────────────────────────────────────────────────
pub const CUB: &str = "\
\n\
       _     _\n\
      /_\\───/_\\\n\
     /         \\\n\
    (   ◉   ◉   )\n\
    │     ▲     │\n\
    │    ( )    │\n\
     ╲   '─'   ╱\n\
      '───────'\n\
      ╱│     │╲\n\
     ╱ │     │ ╲\n\
    ╱/-|     |-\\╲\n\
";

// ── JUVENILE ─────────────────────────────────────────────────────────────────
pub const JUVENILE: &str = "\
\n\
       __     __\n\
      /__\\───/__\\\n\
     /           \\\n\
    (   ◉     ◉   )\n\
    │      ▲      │\n\
    │    (   )    │\n\
     ╲   '───'   ╱\n\
      '─────────'\n\
      ╱│       │╲\n\
     ╱ │       │ ╲\n\
    ╱/-|       |-\\╲\n\
";

// ── ADULT ────────────────────────────────────────────────────────────────────
pub const ADULT: &str = "\
\n\
        __         __\n\
       /__\\───────/__\\\n\
      /               \\\n\
     (    ◉       ◉    )\n\
     │        ▲        │\n\
     │      (   )      │\n\
      ╲     '───'     ╱\n\
       '─────────────'\n\
       ╱│           │╲\n\
      ╱ │           │ ╲\n\
    ╱─/-|           |-\\─╲\n\
";

// ── ELDER ─────────────────────────────────────────────────────────────────────
pub const ELDER: &str = "\
\n\
          __         __\n\
         /__\\───────/__\\\n\
        /               \\\n\
       (    ◉       ◉    )\n\
       │   ░    ▲    ░   │\n\
       │  ░   (   )   ░  │\n\
        ╲  ░  '───'  ░  ╱\n\
         '─────────────'\n\
         ╱│           │╲\n\
        ╱ │           │ ╲\n\
      ╱─/-|           |-\\─╲\n\
";

// ── SLEEPING VARIANTS (hibernation screen) ────────────────────────────────────

pub const CUB_SLEEP: &str = "\
\n\
       _     _\n\
      /_\\───/_\\\n\
     /         \\\n\
    (   ─   ─   )\n\
    │     ▲     │\n\
    │    ( )    │\n\
     ╲   '─'   ╱\n\
      '───────'\n\
      ╱│     │╲\n\
     ╱ │     │ ╲\n\
    ╱/-|     |-\\╲\n\
";

pub const JUVENILE_SLEEP: &str = "\
\n\
       __     __\n\
      /__\\───/__\\\n\
     /           \\\n\
    (   ─     ─   )\n\
    │      ▲      │\n\
    │    (   )    │\n\
     ╲   '───'   ╱\n\
      '─────────'\n\
      ╱│       │╲\n\
     ╱ │       │ ╲\n\
    ╱/-|       |-\\╲\n\
";

pub const ADULT_SLEEP: &str = "\
\n\
        __         __\n\
       /__\\───────/__\\\n\
      /               \\\n\
     (    ─       ─    )\n\
     │        ▲        │\n\
     │      (   )      │\n\
      ╲     '───'     ╱\n\
       '─────────────'\n\
       ╱│           │╲\n\
      ╱ │           │ ╲\n\
    ╱─/-|           |-\\─╲\n\
";

pub const ELDER_SLEEP: &str = "\
\n\
          __         __\n\
         /__\\───────/__\\\n\
        /               \\\n\
       (    ─       ─    )\n\
       │   ░    ▲    ░   │\n\
       │  ░   (   )   ░  │\n\
        ╲  ░  '───'  ░  ╱\n\
         '─────────────'\n\
         ╱│           │╲\n\
        ╱ │           │ ╲\n\
      ╱─/-|           |-\\─╲\n\
";
