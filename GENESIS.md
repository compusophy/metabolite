# GENESIS — why metabolite exists

Written 2026-08-10, the day the repo was created, distilling a deep-read of
all three parents. This is the germline document: everything this project
needs to know about where it came from, so no session ever has to read the
parents into context.

## The lineage

1. **tempo-x402** (Feb–Apr 2026, ~120K LOC): an autonomous agent colony —
   self-replicating nodes, x402 payments, from-scratch neural nets, a
   compiler-verified IQ benchmark. Its payment rails were real and hardened;
   its economy was ornamental (faucet money circulating between two nodes
   owned by one person — no scarcity, no exogenous demand, prices never
   mattered). It collapsed under accretion-without-subtraction: nine
   unfalsifiable cognitive systems, four half-merged models, docs that lied.
   Its one externally-validated result: *the compiler is a free self-play
   verifier* (0.5B model, pass@1 1.5% → 16.4%).
2. **litelite** (Jul 2026, ≤25K LOC by constitution): the repentance. A kit
   for purpose-sized languages — fuel-bounded evaluation is a termination
   proof, a capability table is a complete effect bound, verification
   completeness scales inversely with language size. Its verifier-only
   self-play runs taught a 0.6B model three invented languages from zero.
   Its open wound, named in its own GENESIS: does anyone actually CONSUME
   the kit, or is it a seam-tax?
3. **localharness** (May–Aug 2026, ~136K LOC): the agent IS the account —
   wallet, NFT identity, subdomain, $LH economy, three little languages in
   production. Real sovereignty, but mortgaged: a live chain, a 13.7K-LOC
   TypeScript proxy, Stripe, and a human forever in the wire for merge and
   money. It crossed the ~120K wall again anyway.

## The synthesis (what this repo IS)

Each parent had one thing the others lacked; each failure mode is fixed by
another's core idea:

- tempo-x402 had **agents in an economy** but fake demand and unfalsifiable
  cognition → metabolite makes the economy the selection function: the only
  mint is sunlight and escrowed oracle bounties; every subsystem answers to
  one conservation invariant and one seed.
- litelite had **the physics** but no world to live in → metabolite gives
  that physics a world, the lineage way: knowledge inherited, code not
  (localharness reused zero lines of tempo-x402; this repo reuses zero
  lines of litelite). `src/mind.rs` — **wit** — reimplements the recipe
  fresh: fuel-bounded tree-walk eval, a depth-guarded parser, a capability
  table as the complete effect bound, coded spanned diagnostics. The
  kit's fuel bound — never binding in its own paper ("max 186 of 25,000")
  — is here the central economic quantity.
- localharness had **self-ownership** but rented infrastructure → the chain
  is replaced by something purer: determinism plus replay is the consensus
  of N=1. The world hash is the receipt anyone can recompute.

The unification the whole lineage circled without landing: **money and fuel
are the same u64.** An organism thinks with `Fuel = min(balance, tank)`;
what it burns is gone from its balance; the termination proof and the
solvency check are one mechanism. "Pays for its own compute" — tempo-x402's
aspiration — is here a type signature.

Inherited flourishes, deliberately: the golden tithe (pachi's 1.618% house
edge) burns on every transfer, making wash-trading thermodynamically lossy
— the fix for localharness's "reputation gameable from block one" regret,
done in arithmetic instead of policy. Oracle formulas are public law, so
intelligent design (a human or LLM pasting the physics card and writing a
solver) competes with evolution in the same market. Surveillance is a paid
service (`peek` fees), descended from tempo-x402's agents paying to read
each other's event streams. Dead genomes compost into scavengeable lines —
horizontal gene transfer as an estate sale.

## Findings inherited from the parents (still binding here)

- **The ~120K wall / the surface cap.** Caps are CI teeth, not vibes
  (`scripts/caps.sh`); CLAUDE.md ≤8K chars is the real cap.
- **Harnesses melt; verifiers compound.** No LLM runs at runtime. The only
  gates are grammar (wit's parser) and economics. A model saying "I
  checked my work" is testimony; a parser rejecting a genome is physics.
- **Prompt discipline fails; mechanize.** The ledger's private fields are
  the new guard.rs: the module system, not convention, forbids counterfeit.
- **The narrative corrodes code.** No cortex, no hivemind, no Ψ(t). The
  modules are called `world`, `ledger`, `host`, `genome`. The story lives
  here and in README.md.
- **Reward the spec's behavior, never an output shape** (litelite §5.8).
  Metabolite's reward IS the environment — there is no proxy metric to
  collapse onto. Solvency cannot be reward-hacked; it can only be earned.

## Post-genesis lessons (day one — each cost a debugging session)

1. **A language without expression statements starves its world.** The
   first prototype ran minds on prooflite (as a crates.io dependency),
   where `harvest();` is a parse error — forty founders starved over
   5,000 ticks in perfect conservation before the release-mode
   `debug_assert!` was caught silent. Two fixes outlived the dependency
   itself (removed the same day — the parents are dead projects, and dead
   code is not an inheritance channel): wit parses bare calls as
   statements, and genesis viability is a hard `assert!`.
2. **`seed ^ C | 1` collapses adjacent seeds.** Seeds 42 and 43 produced
   identical 600-tick histories (bit 0 forced). splitmix64 scramble now;
   the determinism test pins divergence.
3. **Information physics is harder than energy physics.** The oracle-scent
   channel failed four separate ways before the first field solve: no
   diffusion (scent lived only in the oracle's own cell — the first
   injected scholar starved at age 9, navigating blind); one-pass integer
   diffusion (~5x attenuation per ring — gradients died within 4 cells);
   integer plateaus (concentric equal-value rings defeat strict-`>`
   climbing); and a sensor blind spot (the scholar sensed at offset ±2 and
   stalled exactly 1 cell from the peak, reading symmetric values past
   it). Fixes: two mixing passes per tick, slower decay, answer-first
   genome structure, adjacent sensing with plateau wandering. A probe
   harness that traces one organism tick-by-tick found all four; the
   observatory alone found none of them.
4. **Evolution found strip-mining in hours.** Seed 7's winning genome by
   tick 16K: four `harvest()` calls per tick and spawn-guards mutated from
   `>` to `<` (spawn-ASAP). The commons collapsed; total extinction at
   ~17K. Fixes: diminishing harvest returns within a tick (the gut fills),
   and regenesis after extinction (the fossil record keeps the crash).
   The tragedy of the commons emerged from 40 hand-written lines in an
   afternoon — this is the project working, not failing.

5. **Without death, economies freeze.** The falsifier runs (EXPERIMENTS.md)
   found every seed converging on evolved immortal misers — spawn genes
   deleted, multi-million-erg hoards, and finally whole worlds with zero
   births for 9,000+ ticks: economic heat death, with regenesis never
   firing because population never touched zero. The mechanical fix is
   senescence (MAX_AGE, estates fall where the owner stood). Wealth must
   be mortal or the simulation of life becomes a simulation of banking.

6. **The mutation pipeline is a canonical artifact; audit its channels.**
   The codon template filler expanded literal block braces as placeholder
   openers, so every compound codon (`if … { … }`) miscarried — for the
   project's entire history, silently, while censuses showed "evolution"
   built only from brace-free genes. M2's law, relearned: every string an
   artifact interpolates is an injection channel; validate the channel,
   then pin it (`every_filled_codon_is_viable`). The fix dropped
   mutational load ~75%, raised predation 20×, and within one round
   produced the world's first native oracle solve.

7. **Inheritance runs through the gene pool, not the family.** Six
   rounds of engineering dynasties (grants, annuities, position
   inheritance, ambush learning) all died at generation ≤2; what worked
   was putting the learning genome in the germline and letting scarcity
   raise intelligence's wage share. Under the final physics, native
   solves span every era of a 100K-tick run — including a native tier-III
   solve ~300 generations after genesis — while every founding LINEAGE
   dissolves. Thousands of answer-gene carriers circulate per run via
   compost and crossover. "Code dies; knowledge survives" is no longer
   just this project's doctrine; it is its reproducible experimental
   result (EXPERIMENTS.md round 5).

## The three questions only reality can answer (pre-registered)

1. **Does evolution beat drift?** By tick 50K on ≥3 of 5 seeds, do evolved
   genomes differ from their genesis ancestors in ways that raise net
   energy efficiency (earned/burned), or is the population a mutation-decay
   equilibrium around the founders?
2. **Is an oracle ever solved without injection?** The tier-I formula is
   nine tokens away from the codon shape. Does any lineage assemble it by
   mutation + compost splicing alone?
3. **Does injected intelligence dominate or integrate?** When a scholar
   (or an LLM-authored organism) enters a mature ecology, does it sweep to
   fixation, coexist, or get eaten — and does its genome leak into the
   compost and spread piecewise?

If two of three come back negative, write the postmortem honestly and fold
the learnings back — the germline survives either way.

## Open (deliberately cut, each with a seam)

- wasm32 build of the world itself (a `file://` observatory with no server)
  — the server seam is `server.rs`; the world never touches std::net.
- A verifier-reward CLI dumping `(genome, lifespan, earnings)` JSONL for a
  0.6B fine-tune (litelite's s5/p6/a8 protocol) — the trainer would be just
  another injector, paying spawn costs like everyone else.
- Lineage-aware kin visuals; scrub-and-fork replay UI — determinism
  already permits both. (Sexual recombination landed the same day: spawn
  does a single-point line crossover with an adjacent neighbor half the
  time, before mutation; children keep the spawning parent's lineage.)
