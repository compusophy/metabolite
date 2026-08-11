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
            let scholar_at = flag(&args, "--scholar").and_then(|v| v.parse().ok());
            let empiricist_at = flag(&args, "--empiricist").and_then(|v| v.parse().ok());
            let eternal = args.iter().any(|a| a == "--eternal");
            headless(seed, ticks, scholar_at, empiricist_at, eternal);
        }
        "card" => print!("{}", metabolite::physics_card()),
        // The training-corpus exporter: every life as one JSONL record —
        // genome in, economic outcome out. The dataset a model that
        // writes organisms trains against (fitness is ground truth; no
        // judge anywhere).
        "corpus" => {
            let ticks: u64 = flag(&args, "--ticks").and_then(|v| v.parse().ok()).unwrap_or(50000);
            let mut w = World::new(seed);
            for _ in 0..ticks {
                w.tick();
            }
            use metabolite::json::{n, obj, s};
            for a in &w.agents {
                let lifespan = a.died.unwrap_or(w.tick).saturating_sub(a.born);
                println!(
                    "{}",
                    obj(vec![
                        n("seed", seed),
                        n("id", a.id),
                        s("genome", &a.genome),
                        s(
                            "kind",
                            metabolite::genesis::SEED_POP
                                .get(a.lineage)
                                .map(|k| k.0)
                                .unwrap_or("injected")
                        ),
                        n("generation", a.generation),
                        n("born", a.born),
                        n("lifespan", lifespan),
                        n("alive_at_end", a.alive),
                        n("income", a.income),
                        n("spent", a.spent),
                        n("solved", a.solved),
                        n("kids", a.kids),
                    ])
                );
            }
        }
        _ => {
            let port = flag(&args, "--port").and_then(|v| v.parse().ok()).unwrap_or(laws::PORT);
            observatory(seed, port);
        }
    }
}

/// Headless: the world as one reproducible integer.
fn headless(seed: u64, ticks: u64, scholar_at: Option<u64>, empiricist_at: Option<u64>, eternal: bool) {
    let mut w = World::new_mode(seed, eternal);
    println!(
        "metabolite · seed {seed} · {ticks} ticks · oracles {}",
        if eternal { "ETERNAL" } else { "ephemeral" }
    );
    let report_every = (ticks / 10).max(1);
    let mut scholar_ids: Vec<usize> = Vec::new();
    let mut solves_seen = 0u64;
    for t in 0..ticks {
        if Some(t) == scholar_at {
            // A colonization event is a cohort, not a castaway.
            for i in 0..4 {
                if let Ok(id) = w.inject(&format!("scholar-{i}"), metabolite::genesis::SCHOLAR) {
                    scholar_ids.push(id);
                }
            }
            println!("tick {t}: scholar cohort injected as {scholar_ids:?}");
        }
        if Some(t) == empiricist_at {
            for i in 0..4 {
                if let Ok(id) = w.inject(&format!("empiricist-{i}"), metabolite::genesis::EMPIRICIST) {
                    scholar_ids.push(id);
                }
            }
            println!("tick {t}: empiricist cohort injected");
        }
        w.tick();
        // A solve is history: print the mind that did it, verbatim.
        let total_solves: u64 = w.counters.solves.iter().sum();
        if total_solves > solves_seen {
            solves_seen = total_solves;
            for ev in w.feed.ring.iter().rev() {
                if let metabolite::events::Event::Solve { id, tier, amt, .. } = ev {
                    let a = &w.agents[*id];
                    println!(
                        "\nSOLVE at tick {}: #{id} (lineage {}, gen {}) took {amt}e from oracle {} — its mind:",
                        w.tick, a.lineage, a.generation, ["I", "II", "III"][*tier]
                    );
                    for line in a.genome.lines() {
                        println!("    {line}");
                    }
                    println!("    (last mutation: {})\n", a.mutation);
                    break;
                }
            }
        }
        if (t + 1) % report_every == 0 {
            println!(
                "tick {:>6} · pop {:>3} · born {:>5} · starved {:>5} · predated {:>4} · miscarried {:>4} · solves {:?} · warmth {:>6} · hash {:016x}",
                w.tick,
                w.alive_count(),
                w.counters.births,
                w.counters.starved,
                w.counters.predated,
                w.counters.miscarriages,
                w.counters.solves,
                w.counters.warmth,
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
    let carriers = alive.iter().filter(|a| a.genome.contains("answer(")).count();
    let carriers_ever = w.agents.iter().filter(|a| a.genome.contains("answer(")).count();
    println!("\navg generation of the living: {avg_gen}");
    println!("answer-gene carriers: {carriers} alive / {carriers_ever} ever");
    // Multi-line heritability metrics (iteration-2 question): does the
    // canonical learning loop propagate beyond gen 0, and do snipe genes
    // travel with their declaration line or arrive torn (E-UNDEF poison)?
    let loops_ever = w.agents.iter().filter(|a| a.genome.contains("store(5, load(4))")).count();
    let loop_descendants =
        w.agents.iter().filter(|a| a.genome.contains("store(5, load(4))") && a.generation > 0);
    let (mut desc_n, mut desc_maxgen) = (0u32, 0u32);
    for a in loop_descendants {
        desc_n += 1;
        desc_maxgen = desc_maxgen.max(a.generation);
    }
    let snipe_intact = w
        .agents
        .iter()
        .filter(|a| a.genome.contains("peek(") && a.genome.contains("let t = 0"))
        .count();
    let snipe_torn =
        w.agents.iter().filter(|a| a.genome.contains("peek(") && !a.genome.contains("let t = 0")).count();
    println!(
        "learning loops: {loops_ever} ever · {desc_n} descendants (deepest gen {desc_maxgen}) · snipe genes: {snipe_intact} intact / {snipe_torn} torn · amber {}/{}",
        w.amber.len(),
        metabolite::laws::AMBER_CAP
    );
    let census: Vec<String> = metabolite::genesis::SEED_POP
        .iter()
        .enumerate()
        .map(|(k, (name, _, _))| {
            format!("{name} {}", alive.iter().filter(|a| a.lineage == k).count())
        })
        .collect();
    println!("living by founding lineage: {}", census.join(" · "));
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
    // The intelligence curve: exact solves per 5K-tick epoch, by tier.
    if !w.counters.solve_log.is_empty() {
        println!("\nintelligence curve (solves per 5K-tick epoch, I/II/III):");
        let epochs = (ticks / 5000 + 1) as usize;
        let mut hist = vec![[0u32; 3]; epochs];
        for &(t, tier) in &w.counters.solve_log {
            hist[(t / 5000) as usize][tier as usize] += 1;
        }
        for (i, h) in hist.iter().enumerate() {
            if h.iter().any(|&c| c > 0) {
                println!("  {:>6}..{:<6} · {:>3} / {:>3} / {:>3}", i * 5000, (i + 1) * 5000, h[0], h[1], h[2]);
            }
        }
    }

    // Falsifier 1: does the ecology learn? Efficiency by generation band.
    let done: Vec<_> = w.agents.iter().filter(|a| a.spent >= 500).collect();
    let maxgen = done.iter().map(|a| a.generation).max().unwrap_or(0) as u64;
    if maxgen >= 4 {
        println!("\nefficiency by generation band (income per 100 ergs burned; {} lives):", done.len());
        for band in 0..4u64 {
            let lo = maxgen * band / 4;
            let hi = if band == 3 { maxgen + 1 } else { maxgen * (band + 1) / 4 };
            let members: Vec<_> =
                done.iter().filter(|a| (a.generation as u64) >= lo && (a.generation as u64) < hi).collect();
            if members.is_empty() {
                continue;
            }
            let n = members.len() as u64;
            let eff = members.iter().map(|a| a.income * 100 / a.spent).sum::<u64>() / n;
            let life = members.iter().map(|a| a.died.unwrap_or(w.tick) - a.born).sum::<u64>() / n;
            println!("  gen {lo:>4}..{:<4} · lives {n:>6} · efficiency {eff:>4} · mean lifespan {life:>5}", hi - 1);
        }
    }
    // Falsifier 3: the scholars' fate, if a cohort was injected.
    if !scholar_ids.is_empty() {
        let of_line = |a: &&metabolite::world::Agent| scholar_ids.contains(&a.lineage);
        let descendants = w.agents.iter().filter(of_line).count();
        let living = w.agents.iter().filter(of_line).filter(|a| a.alive).count();
        let solves_by_line: u32 = w.agents.iter().filter(of_line).map(|a| a.solved).sum();
        let max_gen = w.agents.iter().filter(of_line).map(|a| a.generation).max().unwrap_or(0);
        println!(
            "\nscholar cohort {scholar_ids:?}: {descendants} ever lived (deepest gen {max_gen}), {living} alive at end, {solves_by_line} oracle solves by the line"
        );
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
