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
    // The empiricist enters the germline: reseeded at every genesis, its
    // thresholds tuned by mutation, its genes mixed by crossover. Rounds
    // 4-5 hand-built the strategy; selection owns the constants now.
    ("empiricist", EMPIRICIST, 6),
    // And its parasite. Producers and scroungers, coevolving: thieves
    // read minds through peek(_, _, 5); a single digit mutation lets a
    // studier hide its working memory in another slot. Deception is one
    // jitter away, and the arms race has eight rooms to hide in.
    ("plagiarist", PLAGIARIST, 5),
];

/// A ready-to-inject organism for the README and the observatory's Inject
/// panel: tracks oracle scent, solves tier I. Not part of genesis — drop it
/// in yourself and watch intelligent design compete with evolution.
/// The empiricist: the first LEARNING organism. It knows no formulas — it
/// probes an oracle and reads the warmth its answer() returns (payout is
/// monotone in closeness), remembers its best guess in memory, shrinks
/// its search window, and converges. In-lifetime learning from economic
/// feedback alone; it can crack any tier it can afford to study.
/// mem: 1=best warmth, 2=step, 3=last x seen, 4=next guess, 5=best guess.
pub const EMPIRICIST: &str = "\
let x = puzzle(0,0);\n\
if x < 0 { let s = scent(0,0); let sx = 0; let sy = 0; if scent(1,0) > s { s = scent(1,0); sx = 1; sy = 0; } if scent(-1,0) > s { s = scent(-1,0); sx = -1; sy = 0; } if scent(0,1) > s { s = scent(0,1); sx = 0; sy = 1; } if scent(0,-1) > s { s = scent(0,-1); sx = 0; sy = -1; } if sx == 0 && sy == 0 && light(2,0) > light(0,0) + 20 { sx = 1; } if sx == 0 && sy == 0 && light(-2,0) > light(0,0) + 20 { sx = -1; } if sx == 0 && sy == 0 && light(0,2) > light(0,0) + 20 { sy = 1; } if sx == 0 && sy == 0 && light(0,-2) > light(0,0) + 20 { sy = -1; } if sx != 0 || sy != 0 { step(sx, sy); } store(3, -1); }\n\
if x >= 0 && load(3) != x { store(3, x); store(1, 0); store(2, 8); store(6, 0); store(4, roll(64)); store(5, 0); }\n\
if x >= 0 { let w = answer(0, 0, load(4)); store(6, load(6) + 1); if w > load(1) { store(1, w); store(5, load(4)); } if load(1) < 1 && load(6) <= 24 { store(4, roll(64)); } if load(1) < 1 && load(6) > 24 { store(4, roll(512)); } if load(1) >= 1 { let st = load(2); if st < 1 { st = 1; } store(4, load(5) + roll(2 * st + 1) - st); store(2, st * 3 / 4); } }\n\
harvest();\n\
invest(1600);\n\
if energy() > 2100 { spawn(); }";

/// The plagiarist: the first organism to buy knowledge. It does not
/// study — it finds an oracle occupied by a studying teacher, pays the
/// peek fee to read the teacher's working memory (slot 5, the best guess
/// so far), and submits that guess from the adjacent cell. When the
/// teacher's study converges, the plagiarist wins the race to the pot:
/// the teacher probes AROUND its best guess; the thief submits the best
/// guess ITSELF. Tuition: 4e. Education: someone else's.
pub const PLAGIARIST: &str = "\
if puzzle(0,0) >= 0 { step(roll(3) - 1, roll(3) - 1); }\n\
let t = 0; let tx = 0; let ty = 0;\n\
if puzzle(1,0) >= 0 && occupied(1,0) == 1 { t = 1; tx = 1; }\n\
if t == 0 && puzzle(-1,0) >= 0 && occupied(-1,0) == 1 { t = 1; tx = -1; }\n\
if t == 0 && puzzle(0,1) >= 0 && occupied(0,1) == 1 { t = 1; ty = 1; }\n\
if t == 0 && puzzle(0,-1) >= 0 && occupied(0,-1) == 1 { t = 1; ty = -1; }\n\
if t == 1 { let g = peek(tx, ty, 5); if g > 0 { answer(tx, ty, g); } }\n\
if t == 0 { let s = scent(0,0); let sx = 0; let sy = 0; if scent(1,0) > s { s = scent(1,0); sx = 1; sy = 0; } if scent(-1,0) > s { s = scent(-1,0); sx = -1; sy = 0; } if scent(0,1) > s { s = scent(0,1); sx = 0; sy = 1; } if scent(0,-1) > s { s = scent(0,-1); sx = 0; sy = -1; } if sx == 0 && sy == 0 && light(2,2) > light(0,0) + 20 { sx = 1; sy = 1; } if sx != 0 || sy != 0 { step(sx, sy); } }\n\
harvest();\n\
invest(400);\n\
if energy() > 900 { spawn(); }";

pub const SCHOLAR: &str = "\
invest(700);\n\
let x = puzzle(0,0);\n\
if x >= 0 { answer(0, 0, (2 * x + 1) % 64); }\n\
if x < 0 { let s = scent(0,0); let sx = 0; let sy = 0; if scent(1,0) > s { s = scent(1,0); sx = 1; sy = 0; } if scent(-1,0) > s { s = scent(-1,0); sx = -1; sy = 0; } if scent(0,1) > s { s = scent(0,1); sx = 0; sy = 1; } if scent(0,-1) > s { s = scent(0,-1); sx = 0; sy = -1; } if sx == 0 && sy == 0 { sx = roll(3) - 1; sy = roll(3) - 1; } step(sx, sy); }\n\
harvest();\n\
if energy() > 1300 { spawn(); }";
