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
pub const SUN_INFLUX: u64 = 4000;
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
pub const COST_STEP: u64 = 6;
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
pub const ENDOW_MAX: u64 = 1000;
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
pub const ORACLE_TTL: u64 = 600;
/// Tier definitions: (count, escrow, modulus). Formulas in `oracle_f`.
/// Escrows are sized so that SOLVING PAYS: a scholar's hunt costs ~1,000
/// ergs of thought; at 80e per solve intelligence was charity work.
pub const ORACLE_TIERS: [(usize, u64, i64); 3] = [(5, 500, 64), (3, 1500, 199), (2, 4000, 509)];

/// The oracle functions, tier-indexed. Public law: an organism (or its
/// author) that encodes f(x) correctly gets the escrow.
///   tier 0: y = (2x + 1) mod 64
///   tier 1: y = (x*x + 7) mod 199
///   tier 2: y = (5x*x + 3x + 11) mod 509
pub fn oracle_f(tier: usize, x: i64) -> i64 {
    match tier {
        0 => (2 * x + 1).rem_euclid(64),
        1 => (x * x + 7).rem_euclid(199),
        _ => (5 * x * x + 3 * x + 11).rem_euclid(509),
    }
}

/// Compost: dead genomes' lines, scavengeable by mutation (ring buffer).
pub const COMPOST_CAP: usize = 256;

/// Event feed ring size (observatory).
pub const EVENTS_CAP: usize = 200;
/// Population history ring (one sample per HISTORY_EVERY ticks).
pub const HISTORY_CAP: usize = 512;
pub const HISTORY_EVERY: u64 = 4;

/// Default HTTP port. Golden.
pub const PORT: u16 = 1618;
