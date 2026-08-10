//! The founding genomes. Hand-written wit, one statement per line (the
//! LINE is the gene — see genome.rs). Nobody here can solve an oracle: the
//! world starts dumb. Intelligence must evolve, or be injected.

/// (name, genome, founding count). Lineage = index in this table.
pub const SEED_POP: &[(&str, &str, usize)] = &[
    (
        "sessile",
        "harvest();\n\
         if energy() > 700 { spawn(); }",
        12,
    ),
    (
        "grazer",
        "if light(0,0) > 15 { harvest(); }\n\
         if light(0,0) <= 15 { let b = light(2,0); let bx = 1; let by = 0; if light(0,2) > b { b = light(0,2); bx = 0; by = 1; } if light(-2,0) > b { b = light(-2,0); bx = -1; by = 0; } if light(0,-2) > b { b = light(0,-2); bx = 0; by = -1; } step(bx,by); harvest(); }\n\
         if energy() > 650 { spawn(); }",
        12,
    ),
    (
        "drifter",
        "step(roll(3) - 1, roll(3) - 1);\n\
         harvest();\n\
         if energy() > 600 { spawn(); }",
        8,
    ),
    (
        "wolf",
        "if occupied(1,0) == 1 { bite(1,0); }\n\
         if occupied(-1,0) == 1 { bite(-1,0); }\n\
         if occupied(0,1) == 1 { bite(0,1); }\n\
         if occupied(0,-1) == 1 { bite(0,-1); }\n\
         if occupied(1,1) == 1 { bite(1,1); }\n\
         if occupied(-1,-1) == 1 { bite(-1,-1); }\n\
         if occupied(2,0) == 1 { step(1,0); }\n\
         if occupied(-2,0) == 1 { step(-1,0); }\n\
         if occupied(0,2) == 1 { step(0,1); }\n\
         if occupied(0,-2) == 1 { step(0,-1); }\n\
         if light(3,3) > light(0,0) + 30 { step(1,1); }\n\
         if light(-3,-3) > light(0,0) + 30 { step(-1,-1); }\n\
         if roll(3) == 0 { step(roll(3) - 1, roll(3) - 1); }\n\
         harvest();\n\
         if energy() > 800 { spawn(); }",
        8,
    ),
    (
        "clan",
        "if light(0,0) > 15 { harvest(); }\n\
         if light(0,0) <= 15 { let b = light(2,0); let bx = 1; let by = 0; if light(0,2) > b { b = light(0,2); bx = 0; by = 1; } if light(-2,0) > b { b = light(-2,0); bx = -1; by = 0; } if light(0,-2) > b { b = light(0,-2); bx = 0; by = -1; } step(bx,by); harvest(); }\n\
         if kin(1,0) == 1 && energy() > 450 { give(1,0, 60); }\n\
         if kin(0,1) == 1 && energy() > 450 { give(0,1, 60); }\n\
         if energy() > 650 { spawn(); }",
        8,
    ),
];

/// A ready-to-inject organism for the README and the observatory's Inject
/// panel: tracks oracle scent, solves tier I. Not part of genesis — drop it
/// in yourself and watch intelligent design compete with evolution.
pub const SCHOLAR: &str = "\
invest(700);\n\
let x = puzzle(0,0);\n\
if x >= 0 { answer(0, 0, (2 * x + 1) % 64); }\n\
if x < 0 { let s = scent(0,0); let sx = 0; let sy = 0; if scent(1,0) > s { s = scent(1,0); sx = 1; sy = 0; } if scent(-1,0) > s { s = scent(-1,0); sx = -1; sy = 0; } if scent(0,1) > s { s = scent(0,1); sx = 0; sy = 1; } if scent(0,-1) > s { s = scent(0,-1); sx = 0; sy = -1; } if sx == 0 && sy == 0 { sx = roll(3) - 1; sy = roll(3) - 1; } step(sx, sy); }\n\
harvest();\n\
if energy() > 1300 { spawn(); }";
