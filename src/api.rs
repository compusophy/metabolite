//! The observatory's JSON API, pure: every function here takes a World and
//! returns a String, touching no sockets. The native server (server.rs)
//! and the wasm ABI (wasm.rs) are both thin shells over this module.

use crate::json::{arr, esc, n, obj, s};
use crate::laws::*;
use crate::world::World;

pub fn state_json(w: &World, paused: bool, tps: u64) -> String {
    let agents = w
        .agents
        .iter()
        .filter(|a| a.alive)
        .map(|a| {
            let mut f = vec![
                n("id", a.id),
                n("x", a.x),
                n("y", a.y),
                n("e", w.ledger.agent(a.id)),
                n("lin", a.lineage),
                n("gen", a.generation),
                n("age", w.tick - a.born),
            ];
            if let Some(name) = &a.name {
                f.push(s("name", name));
            }
            obj(f)
        })
        .collect::<Vec<_>>()
        .join(",");
    let oracles = w
        .oracles
        .iter()
        .map(|o| {
            obj(vec![
                n("x", o.cell % GRID),
                n("y", o.cell / GRID),
                n("tier", o.tier),
                n("ttl", o.expires.saturating_sub(w.tick)),
            ])
        })
        .collect::<Vec<_>>()
        .join(",");
    let events =
        w.feed.ring.iter().rev().take(60).map(|e| format!("\"{}\"", esc(&e.line()))).collect::<Vec<_>>().join(",");
    let history = w
        .history
        .iter()
        .map(|(t, p, ae, ce)| format!("[{t},{p},{ae},{ce}]"))
        .collect::<Vec<_>>()
        .join(",");
    let (sx, sy) = w.sun_pos();
    obj(vec![
        n("tick", w.tick),
        s("hash", &format!("{:016x}", w.world_hash)),
        n("seed", w.seed),
        n("era", w.counters.extinctions + 1),
        n("paused", paused),
        n("tps", tps),
        n("grid", GRID),
        format!("\"sun\":[{sx},{sy}]"),
        n("pop", w.alive_count()),
        n("born", w.counters.births),
        n("minted", w.ledger.minted()),
        n("burned", w.ledger.burned()),
        n("agentErg", w.ledger.total_agent_erg()),
        n("cellErg", w.ledger.total_cell_erg()),
        n("compost", w.compost.len()),
        format!(
            "\"burns\":{}",
            obj(vec![
                n("fuel", w.ledger.burns.fuel),
                n("basal", w.ledger.burns.basal),
                n("tithe", w.ledger.burns.tithe),
                n("spawn", w.ledger.burns.spawn),
                n("expired", w.ledger.burns.expired),
            ])
        ),
        format!(
            "\"counters\":{}",
            obj(vec![
                n("births", w.counters.births),
                n("miscarriages", w.counters.miscarriages),
                n("starved", w.counters.starved),
                n("predated", w.counters.predated),
                n("aged", w.counters.aged),
                n("crashes", w.counters.crashed_runs),
                n("bites", w.counters.bites),
                n("gifts", w.counters.gifts),
                n("peeks", w.counters.peeks),
                n("extinctions", w.counters.extinctions),
                n("warmth", w.counters.warmth),
                format!("\"solves\":[{},{},{}]", w.counters.solves[0], w.counters.solves[1], w.counters.solves[2]),
            ])
        ),
        format!("\"cells\":{}", arr((0..CELLS).map(|c| w.ledger.cell(c)))),
        format!("\"scent\":{}", arr(w.scent.iter())),
        format!("\"agents\":[{agents}]"),
        format!("\"oracles\":[{oracles}]"),
        format!("\"events\":[{events}]"),
        format!("\"history\":[{history}]"),
    ])
}

pub fn agent_json(w: &World, id: usize) -> String {
    let Some(a) = w.agents.get(id) else {
        return "{\"err\":\"no such agent\"}".into();
    };
    // Ancestry: walk parents upward (id, mutation, born) — the diff chain.
    let mut chain = Vec::new();
    let mut cur = a.parent;
    while let Some(p) = cur {
        let pa = &w.agents[p];
        chain.push(obj(vec![n("id", pa.id), s("mutation", &pa.mutation), n("born", pa.born), n("alive", pa.alive)]));
        cur = pa.parent;
        if chain.len() >= 16 {
            break;
        }
    }
    let mut f = vec![
        n("id", a.id),
        n("alive", a.alive),
        s("genome", &a.genome),
        s("mutation", &a.mutation),
        n("lineage", a.lineage),
        n("generation", a.generation),
        n("born", a.born),
        n("kids", a.kids),
        n("solved", a.solved),
        n("e", w.ledger.agent(a.id)),
        n("income", a.income),
        n("spent", a.spent),
        format!("\"mem\":{}", arr(a.mem.iter())),
        s("voice", &a.voice),
        format!("\"ancestors\":[{}]", chain.join(",")),
    ];
    if let Some(p) = a.parent {
        f.push(n("parent", p));
        f.push(s("parentGenome", &w.agents[p].genome));
    }
    if let Some(name) = &a.name {
        f.push(s("name", name));
    }
    if let Some(d) = a.died {
        f.push(n("died", d));
        f.push(s("cause", a.cause.unwrap_or("?")));
    }
    if let Some(diag) = &a.last_diag {
        f.push(s("diag", diag));
    }
    obj(f)
}

pub fn physics_json() -> String {
    obj(vec![
        s("card", &crate::physics_card()),
        s("manifest", &crate::host::manifest()),
        s("scholar", crate::genesis::SCHOLAR),
    ])
}

/// Body format: first line is the organism's name, the rest is the genome.
pub fn inject_json(w: &mut World, body: &str) -> String {
    let (name, genome) = match body.split_once('\n') {
        Some((nm, g)) => (nm.trim(), g),
        None => ("anonymous", body),
    };
    let name = if name.is_empty() { "anonymous" } else { name };
    match w.inject(name, genome) {
        Ok(id) => obj(vec![n("ok", true), n("id", id)]),
        Err(e) => obj(vec![n("ok", false), s("err", &e)]),
    }
}
