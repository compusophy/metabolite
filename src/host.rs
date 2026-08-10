//! The physics ABI. The capability table below is the COMPLETE effect
//! surface of a mind: a wit program provably cannot touch anything not
//! named here, and every call burns its declared cost from the one tank,
//! which is the agent's own balance.
//!
//! House rule: the physics never faults. Every impossible act returns a
//! sentinel (-1 or 0); there is no error path out of a capability.

use crate::laws::*;
use crate::mind::{Cap, Host, Limits};
use crate::world::World;

pub static CAPS: &[Cap] = &[
    Cap { name: "light", arity: 2, cost: COST_SENSE, doc: "Ergs in the cell at offset (dx,dy), clamped to radius 3." },
    Cap { name: "occupied", arity: 2, cost: COST_SENSE, doc: "1 if an agent stands at (dx,dy), else 0." },
    Cap { name: "kin", arity: 2, cost: COST_SENSE, doc: "1 if the agent at (dx,dy) shares your lineage, else 0." },
    Cap { name: "scent", arity: 2, cost: COST_SENSE, doc: "Scent value in the cell at (dx,dy). Decays every tick." },
    Cap { name: "energy", arity: 0, cost: COST_SELF, doc: "Your un-escrowed balance (this tick's tank not included)." },
    Cap { name: "age", arity: 0, cost: COST_SELF, doc: "Ticks since your birth." },
    Cap { name: "step", arity: 2, cost: COST_STEP, doc: "Move one cell (dx,dy each clamped to +/-1). 1 if moved, 0 if blocked." },
    Cap { name: "harvest", arity: 0, cost: COST_HARVEST, doc: "Take up to 60 ergs from your cell; each further harvest this tick yields half as much. Returns the amount." },
    Cap { name: "bite", arity: 2, cost: COST_BITE, doc: "Steal up to 50 ergs from the adjacent agent at (dx,dy). Tithed. Returns your take, -1 if nobody there." },
    Cap { name: "give", arity: 3, cost: COST_GIVE, doc: "Give amt ergs to the adjacent agent at (dx,dy). Tithed. Returns what they received, -1 if nobody there." },
    Cap { name: "peek", arity: 3, cost: COST_PEEK, doc: "Pay 4 ergs to the adjacent agent at (dx,dy) and read its memory slot. -1 if nobody there or you can't pay." },
    Cap { name: "emit", arity: 1, cost: COST_EMIT, doc: "Add v to the scent in your cell. Returns the new scent." },
    Cap { name: "spawn", arity: 0, cost: COST_SPAWN, doc: "Reproduce into a free adjacent cell: burns 30 ergs and endows the child your invest() amount (default 200); needs endowment + 300 un-escrowed. Child = your genome (sometimes crossed with a neighbor), mutated, if it parses. 1 born, 0 failed." },
    Cap { name: "load", arity: 1, cost: COST_MEM, doc: "Read your memory slot (0-7). Slots persist across ticks." },
    Cap { name: "store", arity: 2, cost: COST_MEM, doc: "Write v to your memory slot (0-7). Returns v." },
    Cap { name: "roll", arity: 1, cost: COST_ROLL, doc: "A number in [0, n) from the world's deterministic dice." },
    Cap { name: "puzzle", arity: 2, cost: COST_PUZZLE, doc: "The oracle's challenge value x at cell (dx,dy), or -1 if no oracle there." },
    Cap { name: "answer", arity: 3, cost: COST_ANSWER, doc: "Submit y to the oracle at (dx,dy). Exact: the whole escrow (oracle moves on). Near: warmth — each unit of error halves the payout, mining the escrow down. No oracle: -1." },
    Cap { name: "invest", arity: 1, cost: 1, doc: "Set what your future children are endowed at birth (clamped 100-1000, default 200). Parental investment is a gene: only your genome carries it forward." },
];

// Indices into CAPS — keep in lockstep with the table above.
const I_LIGHT: usize = 0;
const I_OCCUPIED: usize = 1;
const I_KIN: usize = 2;
const I_SCENT: usize = 3;
const I_ENERGY: usize = 4;
const I_AGE: usize = 5;
const I_STEP: usize = 6;
const I_HARVEST: usize = 7;
const I_BITE: usize = 8;
const I_GIVE: usize = 9;
const I_PEEK: usize = 10;
const I_EMIT: usize = 11;
const I_SPAWN: usize = 12;
const I_LOAD: usize = 13;
const I_STORE: usize = 14;
const I_ROLL: usize = 15;
const I_PUZZLE: usize = 16;
const I_ANSWER: usize = 17;
const I_INVEST: usize = 18;

/// The versioned ABI manifest: one line per capability, plus an FNV hash —
/// change the physics and the number moves, on purpose.
pub fn manifest() -> String {
    let mut m = String::from("wit-manifest/1\n");
    for (i, c) in CAPS.iter().enumerate() {
        m.push_str(&format!("{i} {}/{} cost={}\n", c.name, c.arity, c.cost));
    }
    format!("{m}hash {:016x}\n", crate::hash::hash_str(&m))
}

struct TickHost<'a> {
    world: &'a mut World,
    me: usize,
    harvests: u32,
}

impl<'a> Host for TickHost<'a> {
    fn caps(&self) -> &'static [Cap] {
        CAPS
    }

    fn call(&mut self, idx: usize, a: &[i64]) -> i64 {
        let w = &mut *self.world;
        let me = self.me;
        match idx {
            I_LIGHT => w.ledger.cell(w.cell_at(me, a[0], a[1], SENSE_RADIUS)) as i64,
            I_OCCUPIED => w.occupant(w.cell_at(me, a[0], a[1], SENSE_RADIUS)).is_some() as i64,
            I_KIN => {
                let c = w.cell_at(me, a[0], a[1], SENSE_RADIUS);
                match w.occupant(c) {
                    Some(o) => (w.agents[o].lineage == w.agents[me].lineage) as i64,
                    None => 0,
                }
            }
            I_SCENT => w.scent[w.cell_at(me, a[0], a[1], SENSE_RADIUS)],
            I_ENERGY => w.ledger.agent(me) as i64,
            I_AGE => (w.tick - w.agents[me].born) as i64,
            I_STEP => w.act_step(me, a[0], a[1]),
            I_HARVEST => {
                // Diminishing returns within one tick: the gut fills. The
                // strip-mining strategy evolution found in seed 7 dies here.
                let cap = HARVEST_MAX >> self.harvests.min(6);
                self.harvests += 1;
                self.world.act_harvest(self.me, cap)
            }
            I_BITE => w.act_bite(me, a[0], a[1]),
            I_GIVE => w.act_give(me, a[0], a[1], a[2]),
            I_PEEK => w.act_peek(me, a[0], a[1], a[2]),
            I_EMIT => w.act_emit(me, a[0]),
            I_SPAWN => w.act_spawn(me),
            I_LOAD => w.agents[me].mem[a[0].rem_euclid(MEM_SLOTS) as usize],
            I_STORE => {
                w.agents[me].mem[a[0].rem_euclid(MEM_SLOTS) as usize] = a[1];
                a[1]
            }
            I_ROLL => w.rng.below(a[0].max(0) as u64) as i64,
            I_PUZZLE => w.act_puzzle(me, a[0], a[1]),
            I_ANSWER => w.act_answer(me, a[0], a[1], a[2]),
            I_INVEST => w.act_invest(me, a[0]),
            _ => -1, // unreachable: the evaluator resolves against CAPS
        }
    }
}

/// One tick of one mind. Escrows the tank, runs the genome under wit,
/// settles the tank. A crashed run forfeits the whole tank (a seizure is
/// expensive); the rendered diagnostic becomes the agent's last words.
pub fn run_agent(world: &mut World, me: usize) {
    let tank = world.ledger.hold_tank(me, TANK_CAP);
    if tank == 0 {
        return;
    }
    let src = world.agents[me].genome.clone();
    let limits = Limits { fuel: tank, output_bytes: VOICE_BYTES };
    let result = {
        let mut host = TickHost { world, me, harvests: 0 };
        crate::mind::run(&src, limits, &mut host)
    };
    match result {
        Ok(out) => {
            world.ledger.settle_tank(me, tank, out.fuel_used);
            world.agents[me].spent += out.fuel_used;
            world.agents[me].voice = out.output;
            world.agents[me].last_diag = None;
        }
        Err(diag) => {
            world.ledger.settle_tank(me, tank, tank);
            world.agents[me].spent += tank;
            world.counters.crashed_runs += 1;
            world.agents[me].last_diag = Some(diag.render(&src));
        }
    }
}
