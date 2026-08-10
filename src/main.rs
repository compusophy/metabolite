use metabolite::laws;
use metabolite::server::{serve, Control};
use metabolite::world::World;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(|s| s.as_str()).unwrap_or("serve");
    let seed = flag(&args, "--seed").and_then(|v| v.parse().ok()).unwrap_or(1618);
    match mode {
        "run" => {
            let ticks: u64 = flag(&args, "--ticks").and_then(|v| v.parse().ok()).unwrap_or(5000);
            headless(seed, ticks);
        }
        "card" => print!("{}", metabolite::physics_card()),
        _ => {
            let port = flag(&args, "--port").and_then(|v| v.parse().ok()).unwrap_or(laws::PORT);
            observatory(seed, port);
        }
    }
}

/// Headless: the world as one reproducible integer.
fn headless(seed: u64, ticks: u64) {
    let mut w = World::new(seed);
    println!("metabolite · seed {seed} · {ticks} ticks");
    let report_every = (ticks / 10).max(1);
    for t in 0..ticks {
        w.tick();
        if (t + 1) % report_every == 0 {
            println!(
                "tick {:>6} · pop {:>3} · born {:>5} · starved {:>5} · predated {:>4} · miscarried {:>4} · solves {:?} · hash {:016x}",
                w.tick,
                w.alive_count(),
                w.counters.births,
                w.counters.starved,
                w.counters.predated,
                w.counters.miscarriages,
                w.counters.solves,
                w.world_hash
            );
        }
    }
    // Census: who won, and what do their minds look like now?
    let mut alive: Vec<_> = w.agents.iter().filter(|a| a.alive).collect();
    alive.sort_by_key(|a| std::cmp::Reverse(w.ledger.agent(a.id)));
    let avg_gen: u64 = if alive.is_empty() {
        0
    } else {
        alive.iter().map(|a| a.generation as u64).sum::<u64>() / alive.len() as u64
    };
    println!("\navg generation of the living: {avg_gen}");
    for a in alive.iter().take(3) {
        println!(
            "\n#{} · lineage {} ({}) · gen {} · {}e · age {} · kids {} · last mutation: {}",
            a.id,
            a.lineage,
            metabolite::genesis::SEED_POP.get(a.lineage).map(|s| s.0).unwrap_or("injected"),
            a.generation,
            w.ledger.agent(a.id),
            w.tick - a.born,
            a.kids,
            a.mutation
        );
        for line in a.genome.lines() {
            println!("    {line}");
        }
    }
    println!("\nworld hash: {:016x}", w.world_hash);
    println!(
        "minted {} = held {} + escrow {} + burned {} · conservation {}",
        w.ledger.minted(),
        w.ledger.total_agent_erg() + w.ledger.total_cell_erg(),
        w.ledger.total_escrow(),
        w.ledger.burned(),
        if w.ledger.conserved() { "holds" } else { "VIOLATED" }
    );
}

/// Live: sim thread + observatory server.
fn observatory(seed: u64, port: u16) {
    let world = Arc::new(Mutex::new(World::new(seed)));
    let ctrl = Arc::new(Control { paused: AtomicBool::new(false), tps: AtomicU64::new(8) });
    let (w2, c2) = (world.clone(), ctrl.clone());
    std::thread::spawn(move || loop {
        let tps = c2.tps.load(Ordering::Relaxed).max(1);
        if !c2.paused.load(Ordering::Relaxed) {
            w2.lock().unwrap().tick();
        }
        std::thread::sleep(std::time::Duration::from_millis(1000 / tps));
    });
    println!("metabolite · seed {seed} · the economy is the metabolism");
    serve(world, ctrl, port);
}
