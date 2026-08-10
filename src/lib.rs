//! metabolite — a sealed ecology where fuel is money.
//!
//! Every organism's mind is a wit program (src/mind.rs): it provably halts
//! within its fuel, provably touches only the capability table in
//! `host.rs`, and its fuel tank is its own erg balance. The termination
//! proof and the solvency check are the same mechanism. Selection is
//! bankruptcy. Zero dependencies; the world is a pure function of its seed.

pub mod events;
pub mod genesis;
pub mod genome;
pub mod hash;
pub mod host;
pub mod json;
pub mod laws;
pub mod ledger;
pub mod mind;
pub mod rng;
pub mod server;
pub mod world;

use laws::*;

/// The physics card — the REFERENCE artifact. Everything an organism's
/// author (human, LLM, or mutation) needs, generated from the same table
/// and constants the world runs on, so the card cannot drift from the law.
pub fn physics_card() -> String {
    let mut caps = String::new();
    for cap in host::CAPS.iter() {
        let params = (0..cap.arity).map(|_| "i64").collect::<Vec<_>>().join(", ");
        caps.push_str(&format!("  {}({params}) -> i64 · {} fuel · {}\n", cap.name, cap.cost, cap.doc));
    }
    let mut tiers = String::new();
    let formulas = ["y = (2x + 1) mod 64", "y = (x*x + 7) mod 199", "y = (5x*x + 3x + 11) mod 509"];
    for (t, &(count, escrow, mod_)) in ORACLE_TIERS.iter().enumerate() {
        tiers.push_str(&format!(
            "  Oracle {} · {count} in the world · pays {escrow}e · x in [0,{mod_}) · {}\n",
            ["I", "II", "III"][t],
            formulas[t]
        ));
    }
    format!(
        "METABOLITE PHYSICS CARD (generated from the running law)\n\
         \n\
         Your organism is one wit program, run once per tick.\n\
         Language: 64-bit ints and bools. Statements: `let x = e;`  `x = e;`\n\
         `print e;`  `harvest();` (bare calls are statements)  `if e {{ }}\n\
         else if e {{ }} else {{ }}`  `repeat e {{ }}` (count evaluated once).\n\
         Operators: + - * / % comparisons == != && || ! parentheses.\n\
         Comments: // and /* */. No while, no functions, no recursion:\n\
         every wit program halts. Arithmetic is CHECKED — overflow or /0\n\
         CRASHES the tick and you forfeit the whole tank.\n\
         \n\
         FUEL IS MONEY. Each tick your mind runs with fuel = min(balance, {TANK_CAP}).\n\
         Every statement and expression node burns 1; capabilities burn their\n\
         listed cost; whatever you burn is gone from your balance. A crashed\n\
         run forfeits the whole tank. Existing costs {BASAL}/tick + 1 per {RENT_BYTES_PER_ERG}\n\
         genome bytes (big minds pay rent). Balance 0 at tick's end = death.\n\
         \n\
         CAPABILITIES (the complete effect surface — nothing else exists):\n\
         {caps}\
         \n\
         WORLD. {GRID}x{GRID} torus. A sun drifts in a figure-eight ({SUN_PERIOD}-tick\n\
         orbit) minting ~{SUN_INFLUX}e/tick of light into cells (cap {CELL_CAP}e/cell), plus\n\
         drizzle. Harvest moves cell ergs to you. Movement: one cell per step,\n\
         blocked by occupants. Senses clamp to radius {SENSE_RADIUS}; interactions\n\
         (bite/give/peek/answer) reach adjacent cells only (radius 1).\n\
         \n\
         TRANSFERS between organisms (bite, give, peek fees) burn a golden\n\
         tithe of 1.618%. Wash trading is thermodynamically lossy.\n\
         \n\
         REPRODUCTION. spawn() needs {SPAWN_MIN}e un-escrowed: burns {SPAWN_BURN}e, endows the\n\
         child {SPAWN_ENDOW}e, copies your genome through one random line-level\n\
         mutation (jitter a number, swap an operator, duplicate/delete/swap a\n\
         line, or splice a gene from the compost of the dead). A child that\n\
         fails to parse is a miscarriage — the burn is spent. Genome cap {GENOME_CAP}\n\
         bytes. Write one statement per line: the LINE is the gene.\n\
         \n\
         ORACLES pay escrowed bounties for computation. Each exudes {ORACLE_SCENT}\n\
         scent/tick into its cell (scent decays 1/8 per tick; emit() writes the\n\
         same field — mimicry is legal). Stand on or beside one; puzzle(dx,dy)\n\
         reads x; answer(dx,dy,y) pays out if y is exactly:\n\
         {tiers}\
         Unsolved oracles expire after {ORACLE_TTL} ticks (escrow burns) and move.\n\
         \n\
         Everything is deterministic from the seed. The world hash is the\n\
         receipt. Good luck.\n"
    )
}
