//! The world: a torus grid under a drifting sun, a ledger of ergs, a
//! population of minds, six oracles, a compost heap, and one hash that is
//! the whole history's receipt. The kernel (this module) is the sole writer;
//! minds act only through the capability table in `host.rs`.

use crate::events::{Counters, Event, Feed};
use crate::genome;
use crate::hash;
use crate::laws::*;
use crate::ledger::Ledger;
use crate::rng::Rng;
use std::collections::VecDeque;

pub struct Agent {
    pub id: usize,
    pub alive: bool,
    pub x: usize,
    pub y: usize,
    pub genome: String,
    pub genome_hash: u64,
    pub lineage: usize,
    pub parent: Option<usize>,
    pub generation: u32,
    pub born: u64,
    pub died: Option<u64>,
    pub cause: Option<&'static str>,
    pub mem: [i64; MEM_SLOTS as usize],
    pub voice: String,
    pub last_diag: Option<String>,
    pub mutation: String,
    pub kids: u32,
    pub solved: u32,
    pub bitten: bool,
    pub name: Option<String>,
}

pub struct Oracle {
    pub tier: usize,
    pub cell: usize,
    pub x_val: i64,
    pub expires: u64,
}

pub struct World {
    pub tick: u64,
    pub seed: u64,
    pub rng: Rng,
    pub ledger: Ledger,
    pub agents: Vec<Agent>,
    pub occ: Vec<Option<usize>>,
    pub scent: Vec<i64>,
    pub oracles: Vec<Oracle>,
    pub compost: VecDeque<String>,
    pub feed: Feed,
    pub counters: Counters,
    pub history: VecDeque<(u64, u32, u64, u64)>,
    pub world_hash: u64,
}

/// sin(2*pi*k/64) * 1000, k = 0..63. Integer trig, the family way.
const SIN64: [i64; 64] = [
    0, 98, 195, 290, 383, 471, 556, 634, 707, 773, 831, 882, 924, 957, 981, 995, 1000, 995, 981,
    957, 924, 882, 831, 773, 707, 634, 556, 471, 383, 290, 195, 98, 0, -98, -195, -290, -383,
    -471, -556, -634, -707, -773, -831, -882, -924, -957, -981, -995, -1000, -995, -981, -957,
    -924, -882, -831, -773, -707, -634, -556, -471, -383, -290, -195, -98,
];

fn sin64(k: u64) -> i64 {
    SIN64[(k % 64) as usize]
}
fn cos64(k: u64) -> i64 {
    SIN64[((k + 16) % 64) as usize]
}

impl World {
    pub fn new(seed: u64) -> Self {
        let n_oracles: usize = ORACLE_TIERS.iter().map(|t| t.0).sum();
        let mut w = World {
            tick: 0,
            seed,
            rng: Rng::new(seed),
            ledger: Ledger::new(CELLS, n_oracles),
            agents: Vec::new(),
            occ: vec![None; CELLS],
            scent: vec![0; CELLS],
            oracles: Vec::new(),
            compost: VecDeque::new(),
            feed: Feed::new(EVENTS_CAP),
            counters: Counters::default(),
            history: VecDeque::new(),
            world_hash: hash::FNV_OFFSET,
        };
        // Oracles: one per slot, tiers expanded in order.
        let mut slot = 0;
        for (tier, &(count, _escrow, _m)) in ORACLE_TIERS.iter().enumerate() {
            for _ in 0..count {
                let cell = w.free_oracle_cell();
                let x_val = w.roll_oracle_x(tier);
                w.oracles.push(Oracle { tier, cell, x_val, expires: ORACLE_TTL });
                w.ledger.mint_escrow(slot, ORACLE_TIERS[tier].1);
                slot += 1;
            }
        }
        // Primordial soup: the desert would kill the founders before the
        // sun fills it.
        for c in 0..CELLS {
            let amt = w.rng.range(SOUP_MIN, SOUP_MAX) as u64;
            w.ledger.mint_sun(c, amt, CELL_CAP);
        }
        w.seed_founders();
        w
    }

    /// Seed the founding population. Runs at genesis, and again after any
    /// total extinction — the world always recovers; the fossil record
    /// (events, counters) keeps the crash.
    fn seed_founders(&mut self) {
        let mut seeded = 0;
        for (kind, &(name, src, count)) in crate::genesis::SEED_POP.iter().enumerate() {
            assert!(genome::viable(src).is_ok(), "genesis genome {name} must parse");
            for _ in 0..count {
                if let Some(cell) = self.random_free_cell() {
                    let id =
                        self.new_agent(src.to_string(), kind, None, 0, cell, format!("genesis:{name}"));
                    self.ledger.mint_agent(id, GENESIS_ENDOW, false);
                    seeded += 1;
                }
            }
        }
        if self.tick > 0 {
            self.counters.extinctions += 1;
            let era = self.counters.extinctions + 1;
            self.feed.push(crate::events::Event::Genesis { tick: self.tick, count: seeded, era });
        }
    }

    // ---- geometry ----

    pub fn wrap(v: i64) -> usize {
        v.rem_euclid(GRID as i64) as usize
    }

    pub fn cell_of(x: usize, y: usize) -> usize {
        y * GRID + x
    }

    /// The cell at (dx,dy) relative to agent `me`, offsets clamped to radius.
    pub fn cell_at(&self, me: usize, dx: i64, dy: i64, radius: i64) -> usize {
        let a = &self.agents[me];
        let dx = dx.clamp(-radius, radius);
        let dy = dy.clamp(-radius, radius);
        Self::cell_of(Self::wrap(a.x as i64 + dx), Self::wrap(a.y as i64 + dy))
    }

    pub fn occupant(&self, cell: usize) -> Option<usize> {
        self.occ[cell]
    }

    fn adjacent(&self, me: usize, dx: i64, dy: i64) -> Option<usize> {
        if dx == 0 && dy == 0 {
            return None;
        }
        let cell = self.cell_at(me, dx, dy, 1);
        self.occ[cell].filter(|&o| o != me)
    }

    fn random_free_cell(&mut self) -> Option<usize> {
        for _ in 0..128 {
            let c = self.rng.below(CELLS as u64) as usize;
            if self.occ[c].is_none() {
                return Some(c);
            }
        }
        (0..CELLS).find(|&c| self.occ[c].is_none())
    }

    fn free_oracle_cell(&mut self) -> usize {
        for _ in 0..64 {
            let c = self.rng.below(CELLS as u64) as usize;
            if !self.oracles.iter().any(|o| o.cell == c) {
                return c;
            }
        }
        self.rng.below(CELLS as u64) as usize
    }

    fn roll_oracle_x(&mut self, tier: usize) -> i64 {
        self.rng.below(ORACLE_TIERS[tier].2 as u64) as i64
    }

    // ---- agents ----

    #[allow(clippy::too_many_arguments)]
    fn new_agent(
        &mut self,
        genome: String,
        lineage: usize,
        parent: Option<usize>,
        generation: u32,
        cell: usize,
        mutation: String,
    ) -> usize {
        let id = self.agents.len();
        let (x, y) = (cell % GRID, cell / GRID);
        self.agents.push(Agent {
            id,
            alive: true,
            x,
            y,
            genome_hash: hash::hash_str(&genome),
            genome,
            lineage,
            parent,
            generation,
            born: self.tick,
            died: None,
            cause: None,
            mem: [0; MEM_SLOTS as usize],
            voice: String::new(),
            last_diag: None,
            mutation,
            kids: 0,
            solved: 0,
            bitten: false,
            name: None,
        });
        self.occ[cell] = Some(id);
        id
    }

    pub fn alive_count(&self) -> usize {
        self.agents.iter().filter(|a| a.alive).count()
    }

    /// Observer injection: a hand- or LLM-written organism enters the world.
    /// Returns Err with a rendered diagnostic if the genome fails the gate.
    pub fn inject(&mut self, name: &str, src: &str) -> Result<usize, String> {
        if src.len() > GENOME_CAP {
            return Err(format!("genome is {} bytes; the cap is {}", src.len(), GENOME_CAP));
        }
        if let Err(d) = crate::mind::parse(src) {
            return Err(d.render(src));
        }
        if self.alive_count() >= MAX_POP {
            return Err("the world is full".to_string());
        }
        let cell = self.random_free_cell().ok_or("no free cell")?;
        let id = self.new_agent(src.to_string(), 0, None, 0, cell, format!("injected:{name}"));
        self.agents[id].lineage = id; // injected founders are their own lineage
        self.agents[id].name = Some(name.to_string());
        self.ledger.mint_agent(id, INJECT_ENDOW, true);
        self.feed.push(Event::Inject { tick: self.tick, id, name: name.to_string() });
        Ok(id)
    }

    // ---- capabilities (called from host.rs) ----

    pub fn act_step(&mut self, me: usize, dx: i64, dy: i64) -> i64 {
        let (dx, dy) = (dx.clamp(-1, 1), dy.clamp(-1, 1));
        if dx == 0 && dy == 0 {
            return 0;
        }
        let cell = self.cell_at(me, dx, dy, 1);
        if self.occ[cell].is_some() {
            return 0;
        }
        let old = Self::cell_of(self.agents[me].x, self.agents[me].y);
        self.occ[old] = None;
        self.occ[cell] = Some(me);
        self.agents[me].x = cell % GRID;
        self.agents[me].y = cell / GRID;
        1
    }

    pub fn act_harvest(&mut self, me: usize, cap: u64) -> i64 {
        let cell = Self::cell_of(self.agents[me].x, self.agents[me].y);
        self.ledger.cell_to_agent(cell, me, cap) as i64
    }

    pub fn act_bite(&mut self, me: usize, dx: i64, dy: i64) -> i64 {
        let Some(prey) = self.adjacent(me, dx, dy) else { return -1 };
        let (took, _) = self.ledger.transfer(prey, me, BITE_MAX);
        self.agents[prey].bitten = true;
        self.counters.bites += 1;
        if took > 0 {
            self.feed.push(Event::Bite { tick: self.tick, pred: me, prey, amt: took });
        }
        took as i64
    }

    pub fn act_give(&mut self, me: usize, dx: i64, dy: i64, amt: i64) -> i64 {
        let Some(to) = self.adjacent(me, dx, dy) else { return -1 };
        let (got, _) = self.ledger.transfer(me, to, amt.max(0) as u64);
        self.counters.gifts += 1;
        if got > 0 {
            self.feed.push(Event::Gift { tick: self.tick, from: me, to, amt: got });
        }
        got as i64
    }

    pub fn act_peek(&mut self, me: usize, dx: i64, dy: i64, slot: i64) -> i64 {
        let Some(to) = self.adjacent(me, dx, dy) else { return -1 };
        if self.ledger.agent(me) < PEEK_FEE {
            return -1;
        }
        self.ledger.transfer(me, to, PEEK_FEE);
        self.counters.peeks += 1;
        self.feed.push(Event::Peek { tick: self.tick, from: me, to });
        self.agents[to].mem[slot.rem_euclid(MEM_SLOTS) as usize]
    }

    pub fn act_emit(&mut self, me: usize, v: i64) -> i64 {
        let cell = Self::cell_of(self.agents[me].x, self.agents[me].y);
        self.scent[cell] = (self.scent[cell].saturating_add(v)).clamp(-SCENT_CAP, SCENT_CAP);
        self.scent[cell]
    }

    pub fn act_spawn(&mut self, me: usize) -> i64 {
        if self.alive_count() >= MAX_POP || self.ledger.agent(me) < SPAWN_MIN {
            return 0;
        }
        // A free adjacent cell, fixed scan order (deterministic).
        const DIRS: [(i64, i64); 8] =
            [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
        let Some(cell) = DIRS.iter().map(|&(dx, dy)| self.cell_at(me, dx, dy, 1)).find(|&c| self.occ[c].is_none())
        else {
            return 0;
        };
        self.ledger.burn_spawn(me, SPAWN_BURN);
        let (child_src, desc) =
            genome::mutate(&self.agents[me].genome.clone(), &mut self.rng, &self.compost);
        if let Err(code) = genome::viable(&child_src) {
            self.counters.miscarriages += 1;
            self.feed.push(Event::Miscarriage { tick: self.tick, parent: me, code });
            return 0;
        }
        let lineage = self.agents[me].lineage;
        let generation = self.agents[me].generation + 1;
        let child = self.new_agent(child_src, lineage, Some(me), generation, cell, desc.clone());
        self.ledger.endow(me, child, SPAWN_ENDOW);
        self.agents[me].kids += 1;
        self.counters.births += 1;
        self.feed.push(Event::Birth { tick: self.tick, child, parent: me, desc });
        1
    }

    pub fn act_puzzle(&mut self, me: usize, dx: i64, dy: i64) -> i64 {
        let cell = self.cell_at(me, dx, dy, 1);
        self.oracles.iter().find(|o| o.cell == cell).map(|o| o.x_val).unwrap_or(-1)
    }

    pub fn act_answer(&mut self, me: usize, dx: i64, dy: i64, y: i64) -> i64 {
        let cell = self.cell_at(me, dx, dy, 1);
        let Some(slot) = self.oracles.iter().position(|o| o.cell == cell) else { return -1 };
        let o = &self.oracles[slot];
        if y != oracle_f(o.tier, o.x_val) {
            return 0;
        }
        let tier = o.tier;
        let payout = self.ledger.escrow_to_agent(slot, me);
        self.counters.solves[tier] += 1;
        self.agents[me].solved += 1;
        self.feed.push(Event::Solve { tick: self.tick, id: me, tier, amt: payout });
        self.respawn_oracle(slot);
        payout as i64
    }

    fn respawn_oracle(&mut self, slot: usize) {
        let tier = self.oracles[slot].tier;
        let cell = self.free_oracle_cell();
        let x_val = self.roll_oracle_x(tier);
        self.oracles[slot] = Oracle { tier, cell, x_val, expires: self.tick + ORACLE_TTL };
        self.ledger.mint_escrow(slot, ORACLE_TIERS[tier].1);
    }

    // ---- the tick ----

    pub fn sun_pos(&self) -> (i64, i64) {
        let k = self.tick * 64 / SUN_PERIOD;
        let c = GRID as i64 / 2;
        let orbit = GRID as i64 / 2 - 6;
        // Lissajous 1:2 — the sun figure-eights across the torus.
        (c + orbit * cos64(k) / 1000, c + orbit * sin64(k * 2) / 1000)
    }

    fn shine(&mut self) {
        let (cx, cy) = self.sun_pos();
        let r2 = SUN_RADIUS * SUN_RADIUS;
        let mut weights: Vec<(usize, i64)> = Vec::new();
        let mut wsum: i64 = 0;
        for y in 0..GRID as i64 {
            for x in 0..GRID as i64 {
                let dx = (x - cx).abs().min(GRID as i64 - (x - cx).abs());
                let dy = (y - cy).abs().min(GRID as i64 - (y - cy).abs());
                let d2 = dx * dx + dy * dy;
                if d2 < r2 {
                    let w = r2 - d2;
                    weights.push((Self::cell_of(x as usize, y as usize), w));
                    wsum += w;
                }
            }
        }
        if wsum > 0 {
            for (cell, w) in weights {
                let amt = (SUN_INFLUX as i64 * w / wsum) as u64;
                self.ledger.mint_sun(cell, amt, CELL_CAP);
            }
        }
        for _ in 0..DRIZZLE_CELLS {
            let c = self.rng.below(CELLS as u64) as usize;
            self.ledger.mint_sun(c, DRIZZLE_ERG, CELL_CAP);
        }
    }

    pub fn tick(&mut self) {
        self.shine();

        // Oracles expire: escrow burns, a new challenge appears elsewhere.
        for slot in 0..self.oracles.len() {
            if self.tick >= self.oracles[slot].expires {
                self.ledger.burn_escrow(slot);
                self.respawn_oracle(slot);
            }
        }

        // Minds run in id order. Newborns (id >= n) wait for the next tick.
        let n = self.agents.len();
        for me in 0..n {
            if !self.agents[me].alive {
                continue;
            }
            self.agents[me].bitten = false;
            let rent = BASAL + self.agents[me].genome.len() as u64 / RENT_BYTES_PER_ERG;
            self.ledger.burn_basal(me, rent);
            crate::host::run_agent(self, me);
        }

        // The reaping: insolvency is death.
        for me in 0..self.agents.len() {
            if self.agents[me].alive && self.ledger.agent(me) == 0 {
                self.die(me);
            }
        }

        // After a total extinction, life returns.
        if self.alive_count() == 0 {
            self.seed_founders();
        }

        // Oracles exude scent; scent diffuses to neighbors, then decays.
        // Without diffusion there is no gradient and scent-following is
        // blind — the first injected scholar starved at age 9 proving it.
        for i in 0..self.oracles.len() {
            let c = self.oracles[i].cell;
            self.scent[c] = (self.scent[c] + ORACLE_SCENT).min(SCENT_CAP);
        }
        // Two mixing passes per tick (one pass attenuates ~5x per ring —
        // gradients died within 4 cells; two passes reach ~10), then decay.
        for pass in 0..2 {
            let old = std::mem::replace(&mut self.scent, vec![0; CELLS]);
            for y in 0..GRID as i64 {
                for x in 0..GRID as i64 {
                    let at = |dx: i64, dy: i64| {
                        old[Self::cell_of(Self::wrap(x + dx), Self::wrap(y + dy))]
                    };
                    let mut s = (at(0, 0) * 4 + at(1, 0) + at(-1, 0) + at(0, 1) + at(0, -1)) / 8;
                    if pass == 1 {
                        s = s * SCENT_KEEP as i64 / SCENT_DIV as i64;
                    }
                    self.scent[Self::cell_of(x as usize, y as usize)] = s.clamp(-SCENT_CAP, SCENT_CAP);
                }
            }
        }

        self.tick += 1;
        self.fold_hash();
        if self.tick % HISTORY_EVERY == 0 {
            if self.history.len() == HISTORY_CAP {
                self.history.pop_front();
            }
            self.history.push_back((
                self.tick,
                self.alive_count() as u32,
                self.ledger.total_agent_erg(),
                self.ledger.total_cell_erg(),
            ));
        }
        assert!(self.ledger.conserved(), "conservation violated at tick {}", self.tick);
    }

    fn die(&mut self, me: usize) {
        let cell = Self::cell_of(self.agents[me].x, self.agents[me].y);
        self.ledger.agent_to_cell(me, cell); // detritus (usually 0 — they died broke)
        self.occ[cell] = None;
        let cause: &'static str = if self.agents[me].bitten { "predation" } else { "starvation" };
        if self.agents[me].bitten {
            self.counters.predated += 1;
        } else {
            self.counters.starved += 1;
        }
        let age = self.tick - self.agents[me].born;
        // The genome returns to the compost, line by line, for later splicing.
        for line in self.agents[me].genome.clone().lines() {
            let t = line.trim();
            if !t.is_empty() {
                if self.compost.len() == COMPOST_CAP {
                    self.compost.pop_front();
                }
                self.compost.push_back(t.to_string());
            }
        }
        let a = &mut self.agents[me];
        a.alive = false;
        a.died = Some(self.tick);
        a.cause = Some(cause);
        a.voice = String::new();
        a.last_diag = None;
        self.feed.push(Event::Death { tick: self.tick, id: me, age, cause });
    }

    fn fold_hash(&mut self) {
        let mut h = self.world_hash;
        h = hash::fold_u64(h, self.tick);
        for a in self.agents.iter().filter(|a| a.alive) {
            h = hash::fold_u64(h, a.id as u64);
            h = hash::fold_u64(h, (a.x + a.y * GRID) as u64);
            h = hash::fold_u64(h, self.ledger.agent(a.id));
            h = hash::fold_u64(h, a.genome_hash);
        }
        for c in 0..CELLS {
            h = hash::fold_u64(h, self.ledger.cell(c));
        }
        self.world_hash = h;
    }
}
