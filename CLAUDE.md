# CLAUDE.md — metabolite

Read this first; it is the whole map. `GENESIS.md` is the origin story and
the lineage's distilled findings — this repo is SELF-CONTAINED; never read
the parents (tempo-x402, litelite, localharness) into context. Their code
is dead; only their knowledge is inherited here.

## What this is

A sealed ecology where **fuel is money**. Every organism's mind is a
program in **wit** (`src/mind.rs`, this repo's own total language) —
provably halting, effect-bounded by one capability table — and its fuel
tank each tick IS its erg balance. The termination proof and the solvency
check are the same mechanism. Selection is bankruptcy. No LLM at runtime;
the world is a pure function of its seed, and the world hash is the
receipt.

Run it: `cargo run --release` → observatory at `http://localhost:1618`.
Headless: `cargo run --release -- run --ticks 20000 --seed 7`.
The physics card: `cargo run --release -- card` (or GET `/physics`).

## THE CONSTITUTION (teeth: `scripts/caps.sh`, run it before pushing)

1. **Zero dependencies. std only.** The language, ledger, HTTP server,
   JSON, RNG, and hash are all in-repo. If a feature needs a crate, the
   feature is out of scope.
2. **Caps**: Rust (src+tests) ≤5,000 LOC; `web/index.html` ≤1,000 lines;
   this file ≤8,000 chars. At a cap: split, shrink, or kill — never raise.
3. **No floats.** Integer determinism is the replay guarantee.
4. **The ledger is the only money path.** Private fields; mint/burn/transfer
   verbs only; `conserved()` asserts every tick. Never move ergs around it.
5. **The physics never faults.** Capability handlers return sentinels
   (-1/0), never errors — `mind::Host::call` returns a bare i64 on purpose.
6. **Every law lives in `laws.rs`** and is served by the physics card;
   constants nowhere else. Card and law cannot drift (one source).
7. **Tests pin named exploits** (spin genome, wash trade, spawn bomb).
   A red test means the LAWS broke.
8. Narrative lives in GENESIS.md and README.md; code names stay boring.

## Map

```
src/mind.rs      wit: lexer, depth-guarded parser, fueled tree-walk eval,
                 coded spanned diagnostics (E-FUEL, E-TYPE, ...), Host trait
src/laws.rs      every constant + oracle formulas (the physics, one place)
src/ledger.rs    ALL ergs live here; conservation invariant; golden tithe
src/world.rs     grid/torus, sun (integer trig), tick loop, spawn/death,
                 compost, regenesis-after-extinction, world hash (FNV)
src/host.rs      the capability table (18 caps = the COMPLETE effect
                 surface) + TickHost + run_agent (tank escrow/settle)
src/genome.rs    line-level mutation ops + codons + the grammar gate
src/genesis.rs   founding genomes (sessile/grazer/drifter/wolf) + SCHOLAR
src/rng.rs       splitmix64-seeded xorshift64* (the only randomness)
src/hash.rs      FNV-1a-64 (world hash = the receipt)
src/events.rs    feed ring + counters   src/json.rs  hand-rolled JSON
src/server.rs    zero-dep HTTP: / /state /agent /physics /inject /control
src/lib.rs       module roots + physics_card()   src/main.rs  CLI
web/index.html   the observatory (single file, compiled in via include_str!)
tests/physics.rs determinism, conservation, named exploits, scholar-solves,
                 and wit's own semantics (fuel exactness, checked math)
```

## Gotchas (each cost a real debugging session — keep them true)

- **wit allows bare expression statements** (`harvest();`) precisely
  because its predecessor language didn't and forty founders starved in
  perfect conservation before the parse failure was noticed. If you touch
  the parser, keep `Stmt::Expr`.
- **The LINE is the gene.** Genomes are written one statement per line;
  mutation operates on lines/digits. Compound statements stay on one line.
- **A crashed run forfeits the whole tank** (settle with used=tank) — this
  is load-bearing selection pressure, not an accident.
- **Adjacent seeds must diverge**: Rng::new uses a splitmix64 scramble
  because `seed ^ C | 1` collapsed 42/43 (pinned by the determinism test).
- **Harvest has diminishing returns within a tick** (cap >> n): evolution
  found strip-mining in seed 7 and crashed the commons; the halving is the
  mechanical fix. Extinction still happens; `seed_founders()` reseeds.
- **Parser folds charge the depth guard** (`binary()` enters per fold):
  long `1+1+…` chains would otherwise build an AST spine that overflows
  the evaluator's stack. Keep the charge if you touch precedence climbing.
- Newborns don't run in their birth tick (loop bound `n` is captured before
  the loop) — determinism depends on this.
- `energy()` reads UN-escrowed balance (the tank is out on loan during the
  run); spawn/give/peek draw from the same un-escrowed pool.

## Build / verify

```sh
cargo test --release          # 15 tests: physics + wit semantics, ~2s
bash scripts/caps.sh          # the constitution
cargo run --release -- run --ticks 20000 --seed 7   # the world as one integer
```

Two runs, same seed → same final hash. If not, something broke determinism:
suspect any new use of iteration order, time, or floats.
