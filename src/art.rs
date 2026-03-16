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

// ── CUB ──────────────────────────────────────────────────────────────────────
pub const CUB: &str = "\
\n\
     _     _\n\
    /_\\───/_\\\n\
   /         \\\n\
  (  ◉     ◉  )\n\
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
  (  ◉       ◉  )\n\
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
