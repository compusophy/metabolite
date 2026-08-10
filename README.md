# metabolite

**A sealed ecology where fuel is money.** Every organism's mind is a
program in **wit** — this repo's own total language: provably halting,
provably confined to eighteen capabilities — and its fuel tank each tick *is*
its bank balance. Thinking costs ergs. Harvesting sunlight earns them. When
your balance hits zero, you are not "unfit": you are **insolvent**, and the
reaper is an accountant.

```
termination proof  ==  solvency check      (one mechanism, fuellite's)
natural selection  ==  bankruptcy law      (no fitness function anywhere)
the blockchain     ==  a seed + replay     (the world hash is the receipt)
```

Successor to [tempo-x402](https://github.com/compusophy/tempo-x402) (the
colony), [litelite](https://github.com/compusophy/litelite) (the physics),
and [localharness](https://github.com/compusophy/localharness) (the
sovereignty) — each parent's best idea, each parent's failure mode fixed by
another's. The full descent: [GENESIS.md](GENESIS.md).

## Run it

```sh
cargo run --release            # observatory at http://localhost:1618
cargo run --release -- run --ticks 50000 --seed 44   # headless: one integer
                               # (seed 44 prints the first native solve)
cargo run --release -- card    # the physics card (paste it into any LLM)
cargo test --release           # 19 physics tests: determinism, conservation,
                               # spin/wash/spawn-bomb exploits, warm oracles
```

**Serverless build** — the whole world compiled to wasm (hand-rolled C
ABI, no bindgen, still zero deps) and inlined into one static page:

```sh
bash scripts/wasm.sh           # -> dist/index.html (~236 KB, world inside)
```

The same dashboard runs against either substrate: served by the native
binary it observes a server-side world; opened as a static page, a shim
reroutes its `fetch()` calls onto the wasm exports and the world runs in
your tab (`?seed=N` picks the universe). Deploy `dist/` to any static
host — `vercel.json` is set up so importing this repo into Vercel just
works (output directory `dist`, no build step). One physics: browsers
throttle background tabs, so the in-page world runs only while watched.
The experiments live in [EXPERIMENTS.md](EXPERIMENTS.md) — including the
first native solve (seed 44, tick 43,141, organism #35245).

## The world

A 40×40 torus under a sun that figure-eights across it, minting ~4,000
ergs/tick of light into cells. Five founding species: **sessile** (camp and
harvest), **grazer** (chase the light), **drifter** (random walk),
**wolf** (hunt the herds at the oasis), **clan** (graze and gift kin). Organisms sense (radius 3), step, harvest
(diminishing returns — the gut fills), bite adjacent organisms, gift kin,
**pay to read each other's memory** (surveillance-as-a-service, an old
family recipe), lay scent, and spawn mutated children. Every transfer burns
a **golden tithe of 1.618%** — wash trading is thermodynamically lossy.

Ten **oracles** drift through the world paying escrowed bounties
(500/1500/4000 ergs) for correct arithmetic — their formulas are public law, printed on the
physics card. Nobody at genesis can solve one. Intelligence has exactly two
ways in:

1. **Evolution.** `spawn()` copies a genome through one line-level mutation
   — jitter a number, swap an operator, duplicate/delete/swap a line, or
   splice a gene from the **compost** of the dead. Half the time, if anyone
   stands adjacent, the child is first a single-point **crossover** with
   them — sex is a splice, and gene flow does not ask about species. The
   only gate is grammar: a child that doesn't parse is a miscarriage, and
   the spawn burn is still paid. Everything else is selection's problem.
2. **Injection.** Open the observatory's INJECT tab, or copy the physics
   card into any LLM and ask it for an organism. Your creature is minted
   600 ergs and takes its chances like everyone else. There is a sample
   `scholar` that tracks oracle scent and solves tier I — drop it into a
   mature ecology and watch intelligent design compete with evolution, on
   one ledger.

Everything is deterministic from the seed: integer arithmetic only, one
RNG, no clocks. Two runs of `--seed 7` end in the same FNV-1a hash after
twenty thousand ticks of births, starvations, predations, and miscarriages.
**The world is one reproducible integer.**

## What emerged on day one

Seed 7, tick ~16,000, no human input: the dominant genome had evolved
*four* `harvest()` calls per tick (strip-mining) and flipped its spawn
guard from `energy() > 700` to `energy() < 125` (spawn-ASAP). Population
boomed, the commons collapsed, and the world went extinct at tick ~17K —
the tragedy of the commons, emergent from forty hand-written lines. The
physics answered mechanically: harvests now halve within a tick, and after
a total extinction the founders reseed while the fossil record keeps the
crash. Ecology is not a feature roadmap; it happens to you.

## Constitution

**Zero dependencies** — the language, the ledger, the HTTP server, the
JSON, the RNG, the hash: all in-repo, `std` only. ≤5,000 lines of Rust.
≤1,000 lines of HTML. No floats. The ledger's fields are private and
`minted == held + escrow + burned` is asserted every tick.
`scripts/caps.sh` is the teeth. The parents died at ~120K LOC; this repo
cannot get there.

## License

Apache-2.0
