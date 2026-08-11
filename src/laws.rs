//! The physical constants of the world. Every tunable lives here, named,
//! and nowhere else — this table is served verbatim at `/physics` so the
//! observatory and the source can never disagree about the laws.

/// Grid side length. The world is a torus: coordinates wrap.
pub const GRID: usize = 40;
/// Cells in the world.
pub const CELLS: usize = GRID * GRID;

/// Fuel tank per tick: an agent thinks with `min(balance, TANK_CAP)` ergs.
/// The tank is escrowed for the duration of the run; a crash forfeits it all.
pub const TANK_CAP: u64 = 300;
/// Byte cap on a mind's `print` output per tick (its "voice").
pub const VOICE_BYTES: usize = 256;
/// Basal metabolic rate: 1 erg/tick plus rent on genome size.
pub const BASAL: u64 = 1;
/// One erg of rent per this many genome bytes per tick. Big minds cost more
/// to keep alive — the constitutional LOC cap, enforced as physics.
pub const RENT_BYTES_PER_ERG: u64 = 256;
/// Hard cap on genome size in bytes. Spawns beyond it are miscarriages.
pub const GENOME_CAP: usize = 2048;

/// Total sunlight minted into cells per tick (the only steady mint).
/// Squeezed from 4000 in round 5: when farming pays this well, cognition
/// can never compete — intelligence needs a wage share of GDP.
pub const SUN_INFLUX: u64 = 2800;
/// Radius of the drifting sun spot, in cells.
pub const SUN_RADIUS: i64 = 7;
/// Ticks for the sun to complete one orbit of the grid.
pub const SUN_PERIOD: u64 = 1024;
/// Random cells receiving DRIZZLE_ERG each tick (life far from the sun).
pub const DRIZZLE_CELLS: u64 = 120;
pub const DRIZZLE_ERG: u64 = 4;
/// Primordial soup: every cell starts with this range of ergs (else the
/// founding population starves in the desert before the sun fills it).
pub const SOUP_MIN: i64 = 20;
pub const SOUP_MAX: i64 = 90;
/// Founding and injected organisms' endowments.
pub const GENESIS_ENDOW: u64 = 600;
/// Injected organisms arrive well-provisioned — the observer is a patron,
/// and study (empirical oracle-probing) is capital-intensive.
pub const INJECT_ENDOW: u64 = 2500;
/// Max ergs a cell can hold; sunlight past the cap is never minted.
pub const CELL_CAP: u64 = 600;

/// Fuel costs of capabilities (burned from the one tank, per call).
pub const COST_SENSE: u64 = 1; // light / occupied / kin / scent
pub const COST_SELF: u64 = 0; // energy / age
pub const COST_STEP: u64 = 4;
pub const COST_HARVEST: u64 = 3;
pub const COST_BITE: u64 = 12;
pub const COST_GIVE: u64 = 2;
pub const COST_PEEK: u64 = 2;
pub const COST_EMIT: u64 = 2;
pub const COST_SPAWN: u64 = 10;
pub const COST_MEM: u64 = 1; // load / store
pub const COST_ROLL: u64 = 1;
pub const COST_PUZZLE: u64 = 1;
pub const COST_ANSWER: u64 = 6;

/// Max ergs a single harvest() pulls from the cell.
pub const HARVEST_MAX: u64 = 60;
/// Max ergs a single bite() steals from an adjacent victim.
pub const BITE_MAX: u64 = 50;
/// Scent an oracle exudes into its cell each tick (organisms can track it —
/// and fake it: emit() writes the same field. Mimicry is legal). Scent
/// diffuses to neighbors, so this sets how far the gradient reaches.
pub const ORACLE_SCENT: i64 = 300;
/// Fee paid to the *target* of a peek (reading a mind is a paid service).
pub const PEEK_FEE: u64 = 4;
/// Memory slots per agent.
pub const MEM_SLOTS: i64 = 8;
/// Scent field decay: each tick scent = scent * SCENT_KEEP / SCENT_DIV.
/// 15/16 (not 7/8): integer diffusion truncates the gradient's tail hard,
/// and oracles must be findable from most of the torus.
pub const SCENT_KEEP: u64 = 15;
pub const SCENT_DIV: u64 = 16;
/// Scent cap per cell.
pub const SCENT_CAP: i64 = 1_000_000;

/// Ergs endowed to the child at birth, by default. `invest(amt)` lets a
/// parent set this within [ENDOW_MIN, ENDOW_MAX] — parental investment is
/// an evolvable gene, inherited only through the genome that sets it.
pub const SPAWN_ENDOW: u64 = 200;
pub const ENDOW_MIN: u64 = 100;
/// High enough to fully fund a researcher: the round-4 dynasties died
/// because the investment ceiling sat below the cost of an education.
pub const ENDOW_MAX: u64 = 2500;
/// Ergs burned by the act of reproduction (meiosis is not free).
pub const SPAWN_BURN: u64 = 30;
/// A spawn needs endowment + burn + this reserve, un-escrowed.
pub const SPAWN_RESERVE: u64 = 270;

/// Warm oracles: a wrong answer with error e still pays escrow >> e —
/// each unit of error halves the payout, so guessing becomes
/// hill-climbing toward arithmetic. Near tier-I oracles random probing is
/// mildly positive-sum: temple beggars are the larval stage of
/// empiricists, and their probing drains the escrow honestly.
/// The escrow DRAINS as it is mined; a drained or solved oracle respawns.
pub const WARMTH_SHIFT_PER_ERROR: u32 = 1;

/// The golden tithe: every transfer between agents burns amount * 1618 /
/// 100_000 (min 1). Wash trading is thermodynamically lossy.
pub const TITHE_NUM: u64 = 1618;
pub const TITHE_DEN: u64 = 100_000;

/// Sensing radius for light/occupied/kin/scent (offsets clamp to this).
pub const SENSE_RADIUS: i64 = 3;

/// Population cap (space is the real cap; this bounds compute).
pub const MAX_POP: usize = 400;

/// Senescence. Without it, evolution finds the immortal-miser strategy
/// (delete the spawn gene, hoard millions, never die) and whole worlds
/// freeze into a dozen rentiers with zero births — observed on every
/// experiment seed. Death recycles: the estate falls where they stood.
pub const MAX_AGE: u64 = 6000;

/// Oracles: cells that pay an escrowed bounty for a correct computation.
/// The formulas are public law (see the physics card); discovering an
/// expression that computes one is evolution's IQ test — or yours.
/// Short: oracles must TURN OVER faster than organisms live, or they are
/// static geography and ambush-learning (camp, harvest, wait for scent)
/// can never work. At 200, a fresh oracle lands near a district camper
/// every few dozen ticks.
pub const ORACLE_TTL: u64 = 200;
/// Tier definitions: (count, escrow, modulus). Families in `oracle_value`.
/// Escrows are sized so that STUDY pays: an empirical education (travel +
/// probing to convergence) costs ~800-1,500 ergs; tier I priced below
/// that made dynasties unaffordable (round 5's demographic transition).
pub const ORACLE_TIERS: [(usize, u64, i64); 3] = [(8, 900, 64), (5, 2000, 199), (3, 5000, 509)];

/// Oracle FAMILIES are public law; every oracle INSTANCE draws secret
/// coefficients (a, b) at spawn. Iteration 2 proved that with eternal
/// formulas, evolution inscribes the answers into DNA and learning never
/// pays — so no formula is eternal anymore. Nothing inscribed solves
/// twice; warmth feedback is the only teacher that generalizes.
///   tier 0: y = (a*x + b)          mod 64,  a in 1..=8,  b in 0..64
///   tier 1: y = (x*x + a*x + b)    mod 199, a in 1..=14, b in 0..199
///   tier 2: y = (a*x*x + b*x + 11) mod 509, a in 1..=6,  b in 0..509
pub fn oracle_value(tier: usize, a: i64, b: i64, x: i64) -> i64 {
    match tier {
        0 => (a * x + b).rem_euclid(64),
        1 => (x * x + a * x + b).rem_euclid(199),
        _ => (a * x * x + b * x + 11).rem_euclid(509),
    }
}
/// Per-tier ceiling for the secret `a` (drawn 1..=ceiling).
pub const ORACLE_A_CEIL: [u64; 3] = [8, 14, 6];

/// Compost: dead genomes' lines, scavengeable by mutation (ring buffer).
pub const COMPOST_CAP: usize = 256;
/// Amber: the compost's protected stratum. Entries that touch the oracles
/// (contain an answer gene) are also preserved here, and only newer amber
/// displaces older amber. In a mature monoculture the compost flushes a
/// dead learner's genes within a few hundred ticks — the amber is why the
/// death of the last mind is no longer the death of the idea.
pub const AMBER_CAP: usize = 64;
/// Stratified amber: half the ring is reserved for COGNITIVE entries —
/// genes that both answer and remember (contain `answer(` and `store(`).
/// Without the reserve, the amber saturates with short lottery-ticket
/// formula lines (every dying ticket-carrier contributes one) and the
/// rare learning loops get flushed; a formula can never evict a mind.
pub const AMBER_MIND_RESERVE: usize = 32;

/// Event feed ring size (observatory).
pub const EVENTS_CAP: usize = 200;
/// Population history ring (one sample per HISTORY_EVERY ticks).
pub const HISTORY_CAP: usize = 512;
pub const HISTORY_EVERY: u64 = 4;

/// Default HTTP port. Golden.
pub const PORT: u16 = 1618;
