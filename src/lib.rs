//! metabolite — a sealed ecology where fuel is money.
//!
//! Every organism's mind is a wit program (src/mind.rs): it provably halts
//! within its fuel, provably touches only the capability table in
//! `host.rs`, and its fuel tank is its own erg balance. The termination
//! proof and the solvency check are the same mechanism. Selection is
//! bankruptcy. Zero dependencies; the world is a pure function of its seed.

pub mod api;
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
#[cfg(not(target_arch = "wasm32"))]
pub mod server;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
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
    let formulas =
        ["y = (a*x + b) mod 64", "y = (x*x + a*x + b) mod 199", "y = (a*x*x + b*x + 11) mod 509"];
    for (t, &(count, escrow, mod_)) in ORACLE_TIERS.iter().enumerate() {
        tiers.push_str(&format!(
            "  Oracle {} · {count} in the world · pays {escrow}e · x in [0,{mod_}) · {} · a <= {}\n",
            ["I", "II", "III"][t],
            formulas[t],
            ORACLE_A_CEIL[t]
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
         So is age {MAX_AGE}: senescence is law, and your estate falls to the\n\
         cell where you stood. Nothing is immortal; everything recycles.\n\
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
         REPRODUCTION. spawn() burns {SPAWN_BURN}e and endows the child what invest()\n\
         set (default {SPAWN_ENDOW}e, clamp {ENDOW_MIN}-{ENDOW_MAX}); it needs endowment + {SPAWN_BURN}e + {SPAWN_RESERVE}e\n\
         un-escrowed. It copies your genome through one random line-level\n\
         mutation (jitter a number, swap an operator, duplicate/delete/swap a\n\
         line, or splice a gene from the compost of the dead). A child that\n\
         fails to parse is a miscarriage — the burn is spent. Genome cap {GENOME_CAP}\n\
         bytes. Write one statement per line: the LINE is the gene.\n\
         Your child is born with a COPY of your memory slots: what you\n\
         learned goes with them. Teach by living.\n\
         \n\
         ORACLES pay escrowed bounties for computation. Each exudes {ORACLE_SCENT}\n\
         scent/tick into its cell (scent decays 1/8 per tick; emit() writes the\n\
         same field — mimicry is legal). Stand on or beside one; puzzle(dx,dy)\n\
         reads x; answer(dx,dy,y) pays the WHOLE escrow if y is exact.\n\
         The FAMILIES are public law; the coefficients (a, b) are SECRET,\n\
         drawn fresh for every oracle at spawn. Nothing memorized solves\n\
         twice — the warmth of your wrong answers is the only teacher.\n\
         {tiers}\
         Near misses are paid WARMTH: each unit of error halves the payout,\n\
         and the escrow drains as it is mined (a dry oracle moves on). Blind\n\
         guessing barely pays; almost-right arithmetic gets rich — and the\n\
         payout you receive tells you how close you are. Learning is legal.\n\
         Unsolved oracles expire after {ORACLE_TTL} ticks (escrow burns) and move.\n\
         \n\
         Everything is deterministic from the seed. The world hash is the\n\
         receipt. Good luck.\n"
    )
}
