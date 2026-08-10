//! The physics answers for itself. Each test is pinned to a named claim or
//! a named exploit; a red test here means the LAWS are broken, not a
//! feature.

use metabolite::genesis::{SCHOLAR, SEED_POP};
use metabolite::genome;
use metabolite::laws::*;
use metabolite::world::World;

/// THE claim: the world is a pure function of its seed. Same seed, same
/// history, same integer — replay IS the audit.
#[test]
fn determinism_same_seed_same_hash() {
    let mut a = World::new(42);
    let mut b = World::new(42);
    for _ in 0..600 {
        a.tick();
        b.tick();
    }
    assert_eq!(a.world_hash, b.world_hash);
    assert_eq!(a.alive_count(), b.alive_count());

    let mut c = World::new(43);
    for _ in 0..600 {
        c.tick();
    }
    assert_ne!(a.world_hash, c.world_hash, "different seeds must diverge");
}

/// Conservation: minted == held + escrow + burned, at every tick (tick()
/// asserts it internally; this exercises 1000 ticks of full ecology —
/// births, deaths, bites, gifts, miscarriages — under the invariant).
#[test]
fn conservation_holds_through_a_thousand_ticks() {
    let mut w = World::new(1618);
    for _ in 0..1000 {
        w.tick();
    }
    assert!(w.ledger.conserved());
    assert!(w.counters.births > 0, "the ecology must actually live during this test");
}

/// Named exploit: the spin genome. An adversarial mind that tries to loop
/// forever cannot hang the world — fuel is a termination proof, and the
/// crash forfeits its whole tank (economic pain, not just a stop).
#[test]
fn a_spinning_mind_cannot_hang_the_world_and_pays_for_trying() {
    let mut w = World::new(7);
    // Pure spin, no income — so the forfeit is exactly measurable.
    let id = w
        .inject("spinner", "repeat 1000000000 { let x = 1; }")
        .expect("the spinner parses — it is legal, just doomed");
    let before = w.ledger.agent(id);
    w.tick(); // must return; a hang here is the test failing by timeout
    let a = &w.agents[id];
    assert!(a.last_diag.as_deref().unwrap_or("").contains("E-FUEL"), "died of fuel exhaustion");
    assert!(w.counters.crashed_runs >= 1);
    // The whole tank is gone (plus basal; bites could only lower it further).
    assert!(w.ledger.agent(id) <= before - TANK_CAP - BASAL);
}

/// Named exploit: wash trading. The golden tithe makes moving ergs in a
/// circle strictly lossy — reputation-by-volume can never be free.
#[test]
fn wash_trading_is_thermodynamically_lossy() {
    let mut w = World::new(1);
    let a = w.inject("washer-a", "let h = harvest();").unwrap();
    let b = w.inject("washer-b", "let h = harvest();").unwrap();
    let start = w.ledger.agent(a) + w.ledger.agent(b);
    for _ in 0..50 {
        w.ledger.transfer(a, b, 100);
        w.ledger.transfer(b, a, 100);
    }
    let end = w.ledger.agent(a) + w.ledger.agent(b);
    assert!(end < start, "every round trip must burn tithe");
    assert!(w.ledger.conserved());
}

/// Named exploit: the spawn bomb. A rich mind that spawns every tick is
/// bounded by space and by MAX_POP — population can never exceed the cap.
#[test]
fn spawn_bomb_is_bounded() {
    let mut w = World::new(9);
    let id = w.inject("bomber", "let s = spawn();").unwrap();
    w.ledger.mint_agent(id, 1_000_000, true); // obscenely rich
    for _ in 0..300 {
        w.tick();
        assert!(w.alive_count() <= MAX_POP);
    }
}

/// The founding genomes and the scholar must all pass the grammar gate —
/// a genesis that miscarries is a broken world.
#[test]
fn genesis_genomes_and_scholar_are_viable() {
    for (name, src, _) in SEED_POP {
        assert!(genome::viable(src).is_ok(), "genesis genome {name} must parse");
    }
    assert!(genome::viable(SCHOLAR).is_ok(), "the scholar must parse");
}

/// Garbage never gets born, and the byte cap is a hard wall.
#[test]
fn the_grammar_gate_rejects_garbage_and_giants() {
    assert!(genome::viable("this is not wit ⚡").is_err());
    let giant = "let h = harvest();\n".repeat(200); // > GENOME_CAP bytes
    assert!(giant.len() > GENOME_CAP);
    assert!(genome::viable(&giant).is_err());
}

/// Mutation output is either viable or a miscarriage — never a crash of the
/// WORLD. Hammer the mutator; every output must keep the world's promises.
#[test]
fn mutation_never_panics_and_miscarriage_rate_is_sane() {
    let mut rng = metabolite::rng::Rng::new(123);
    let compost = std::collections::VecDeque::new();
    let mut src = SEED_POP[1].1.to_string(); // the grazer
    let mut viable = 0u32;
    for _ in 0..2000 {
        let (child, _desc) = genome::mutate(&src, &mut rng, &compost);
        if genome::viable(&child).is_ok() {
            viable += 1;
            src = child; // walk the fitness landscape
        }
        if src.len() > GENOME_CAP {
            src = SEED_POP[1].1.to_string();
        }
    }
    // Most mutations of a viable genome should stay viable (line discipline).
    assert!(viable > 1200, "only {viable}/2000 mutants viable — the operators are too destructive");
}

/// The scholar actually earns: standing on a tier-I oracle whose scent has
/// had time to pool, it solves it.
#[test]
fn the_scholar_solves_oracle_one() {
    let mut w = World::new(5);
    let id = w.inject("scholar", SCHOLAR).unwrap();
    // Let scent form (a scent-cold oracle makes the scholar wander off it).
    for _ in 0..6 {
        w.tick();
    }
    // Teleport the scholar onto an unoccupied tier-I oracle (test-only
    // surgery: move via the occupancy map like the physics would).
    let oracle_cell = w
        .oracles
        .iter()
        .find(|o| o.tier == 0 && w.occ[o.cell].is_none())
        .expect("a free tier-I oracle")
        .cell;
    let old = metabolite::world::World::cell_of(w.agents[id].x, w.agents[id].y);
    w.occ[old] = None;
    w.agents[id].x = oracle_cell % GRID;
    w.agents[id].y = oracle_cell / GRID;
    w.occ[oracle_cell] = Some(id);
    let before = w.ledger.agent(id);
    w.tick();
    assert_eq!(w.counters.solves[0], 1, "the scholar must solve tier I on contact");
    assert!(w.ledger.agent(id) > before, "and be richer for it");
}

/// Oracle law sanity: the published formulas are the ones that pay.
#[test]
fn oracle_formulas_are_the_published_law() {
    assert_eq!(oracle_f(0, 10), 21);
    assert_eq!(oracle_f(0, 63), 127 % 64);
    assert_eq!(oracle_f(1, 14), (14 * 14 + 7) % 199);
    assert_eq!(oracle_f(2, 20), (5 * 400 + 60 + 11) % 509);
}

// ---- wit, the language itself ----

mod wit {
    use metabolite::mind::{run, Cap, Host, Limits};

    struct Counter(i64);
    static CAPS: &[Cap] = &[Cap { name: "next", arity: 0, cost: 3, doc: "monotonic counter" }];
    impl Host for Counter {
        fn caps(&self) -> &'static [Cap] {
            CAPS
        }
        fn call(&mut self, _idx: usize, _args: &[i64]) -> i64 {
            self.0 += 1;
            self.0
        }
    }

    fn out(src: &str) -> String {
        run(src, Limits { fuel: 100_000, output_bytes: 4096 }, &mut Counter(0)).unwrap().output
    }
    fn code(src: &str) -> &'static str {
        run(src, Limits { fuel: 100_000, output_bytes: 4096 }, &mut Counter(0)).unwrap_err().code
    }

    #[test]
    fn semantics() {
        assert_eq!(out("print 1 + 2 * 3;"), "7\n");
        assert_eq!(out("print (1 + 2) * 3;"), "9\n");
        assert_eq!(out("print -7 / 2;"), "-3\n"); // trunc toward zero
        assert_eq!(out("print 1_000_000;"), "1000000\n");
        assert_eq!(out("let x = 1; if true { x = 2; let x = 9; } print x;"), "2\n");
        assert_eq!(out("let s = 0; repeat 5 { s = s + 1; } print s;"), "5\n");
        // Count evaluated once — mutating it can't extend the loop.
        assert_eq!(out("let n = 3; repeat n { n = n + 10; } print n;"), "33\n");
        // else-if chains and short-circuit.
        assert_eq!(out("let n = 2; if n == 1 { print 10; } else if n == 2 { print 20; } else { print 30; }"), "20\n");
        assert_eq!(out("print false && 1 / 0 == 0;"), "false\n");
        assert_eq!(out("print true || 1 / 0 == 0;"), "true\n");
        // Bare expression statements (the ergonomic fix the parents lacked).
        assert_eq!(out("next(); print next();"), "2\n");
    }

    #[test]
    fn checked_arithmetic_and_types_crash_cleanly() {
        assert_eq!(code("print 1 / 0;"), "E-DIV0");
        assert_eq!(code("print 9223372036854775807 + 1;"), "E-OVERFLOW");
        assert_eq!(code("print 1 + true;"), "E-TYPE");
        assert_eq!(code("if 1 { }"), "E-TYPE");
        assert_eq!(code("repeat -1 { }"), "E-NEGREP");
        assert_eq!(code("print nope;"), "E-UNDEF");
        assert_eq!(code("nope();"), "E-CAP");
        assert_eq!(code("next(1);"), "E-ARGS");
    }

    #[test]
    fn fuel_is_a_termination_proof_for_any_composition() {
        // Nested repeats share the ONE tank — the fractal invariant.
        let e = run(
            "repeat 100000 { repeat 100000 { } }",
            Limits { fuel: 10_000, output_bytes: 0 },
            &mut Counter(0),
        )
        .unwrap_err();
        assert_eq!(e.code, "E-FUEL");
        // Capability costs burn from the same tank.
        let e = run("repeat 1000 { next(); }", Limits { fuel: 50, output_bytes: 0 }, &mut Counter(0))
            .unwrap_err();
        assert_eq!(e.code, "E-FUEL");
    }

    #[test]
    fn deep_nesting_is_a_diagnostic_never_a_stack_abort() {
        let deep = format!("print {}1{};", "(".repeat(500), ")".repeat(500));
        assert_eq!(code(&deep), "E-DEPTH");
        // Long operator chains charge the guard too (the AST-spine lesson).
        let chain = format!("print {}0;", "1+".repeat(500));
        assert_eq!(code(&chain), "E-DEPTH");
    }

    #[test]
    fn output_is_byte_capped() {
        let o = run(
            "repeat 1000 { print 123456; }",
            Limits { fuel: 100_000, output_bytes: 14 },
            &mut Counter(0),
        )
        .unwrap();
        assert_eq!(o.output, "123456\n123456\n");
    }
}
