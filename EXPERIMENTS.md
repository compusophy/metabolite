# EXPERIMENTS — the three questions, asked

GENESIS.md pre-registered three falsifiers. This file is the lab notebook:
every run is `metabolite run --ticks N --seed S [--scholar T]` — replayable
to the tick, hash and all. Two rounds: the original physics, then the same
protocol after the one mechanical change round 1 forced (senescence).

## Round 1 — original physics (no death by age)

**Protocol:** 5 evolution runs (seeds 11/22/33/44/55, 50K ticks); 3 scholar
runs (seeds 7/42/1618, 30K ticks, scholar injected at tick 5000).

**F1 — does the ecology learn?** Efficiency (income per 100 ergs burned)
by generation band, ~2,900–6,500 lives per seed, deepest run reached
generation 118:

| seed | band 1 | band 2 | band 3 | band 4 |
|---|---|---|---|---|
| 11 | 97 | 91 | 89 | 91 |
| 22 | 92 | 94 | 94 | 96 |
| 33 | 93 | 90 | 88 | 89 |
| 44 | 93 | 93 | 96 | 94 |
| 55 | 92 | 92 | 92 | 92 |

**Verdict: flat.** No efficiency ratchet — the reproducing population sits
at a mutation–selection equilibrium around ~92, not a learning curve.
(Answer to Q1 as posed: negative, honestly recorded.)

**The unplanned discovery.** On *every* seed, the wealth leaderboard was
swept by one phenotype nobody wrote: **the immortal miser** — drifters
whose spawn gene was deleted or disabled by mutation ("deleted line 5",
"1->0 on line 4"), zero children, ages 42,000–49,000 (they outlived the
run), hoards up to **2.68M ergs** while the flow economy churned beneath
them. Evolution found that the optimal *individual* strategy in this
economy is to stop reproducing and rent-seek. Their wealth was dead
capital: the only wealth tax in the physics was `bite`, and predation was
too rare to touch them.

**F3 — the scholar runs went darker.** In all three mature-world runs the
scholar lineage (3 ever lived each) died without a solve — and the worlds
themselves had *frozen*: population pinned at 11–16 with **zero births for
the last 9,000+ ticks**. The boom-bust cycles had killed every reproducing
lineage; only misers remained; and because population never touched zero,
regenesis never fired. **Economic heat death**: a dozen immortal rentiers,
no births, no deaths, forever.

**F2 — native oracle solve?** None observed in 400K cumulative ticks.

## The mechanical response

The narrative response would be "tune the misers away." The mechanical one
(GENESIS: *prompt discipline fails; mechanize*) is a law of nature:
**senescence** — `MAX_AGE = 6000` ticks, and the estate falls to the cell
where the owner stood. Hoards recycle as corpse bonanzas; frozen worlds
become mortal again; the miser strategy stays *legal* but now ends, and
its accumulated capital re-enters the commons at a point in space.

## Round 2 — with senescence

Same evolution protocol; scholars now injected as a cohort of 4 at tick
2000 (a colonization event is a cohort, not a castaway — and round 1
showed mature worlds are deserts for travelers).

**The worlds came back to life.** Every seed stayed dynamic through 50K
ticks: 24K–38K births (up to 12× round 1), predation up two orders of
magnitude on some seeds (1 → 109), top fortunes down from 2.68M to ~52K
ergs — wealth is mortal, estates recycle where the owner stood, and the
deepest run reached **generation 246** (round 1 max: 118). No frozen
worlds were observed.

**F1 with 10× the data: still flat.** Efficiency by generation band,
8,700–24,900 lives per seed: 88–98 in every band, no monotone trend on
any seed. This looks like a law of the current physics, not noise: the
margin over metabolism is set by the environment (sun density vs. burn
rates), and selection spends its wins on *architecture* (who survives,
who reproduces, who hoards) rather than on yield. Pinned as the
equilibrium hypothesis; a future physics with scarcer light or richer
skills would test it.

**F3 — the founder effect, confirmed and sharpened.** Cohorts of 4:
seed 7 → **1 solve**, seed 42 → **3 solves**, seed 1618 → 0; fully
deterministic and replayable. But no dynasty: 12–16 line-members ever
lived, deepest generation 1, none alive at 30K. The founders (endowed
1,500) hunt and solve; their children (endowed 200, running the same
expensive genome) die before their first paycheck. **Intelligence pays
for itself but not yet for its heirs** — the scholar niche exists and
earns, but its reproduction is underfunded by the fixed endowment law.

**F2 — native solve:** still none in ~800K cumulative ticks across both
rounds. The 9-token tier-I expression remains undiscovered by mutation
and compost alone.

## Round 3 — gradients everywhere, cliffs nowhere

Both surviving falsifiers had failed for one structural reason: cliffs.
Native discovery (Q2) was a needle-in-a-haystack because oracles paid
all-or-nothing; dynasties (Q3) died because endowment was a constant no
gene could touch. Round 3 turned both cliffs into gradients:

- **Warm oracles**: a wrong answer with error e pays `escrow >> 2e` —
  each unit of error quarters the payout, the escrow drains as it is
  mined, and a dry oracle moves on. Parameter-jitter became hill-climbing.
- **`invest(amt)`**: parental endowment (clamped 100–1000) became a
  capability — a *gene*, inherited only through the genome that sets it.
- **Oracles bask in the sun**: they respawn near the light, where life
  actually is. A bounty nobody walks past selects for nothing.

**The bug that was the real wall.** First gradient runs: warmth = 0 on
all seeds. A carrier census revealed why: **zero answer-genes had ever
been born** — in ~250K cumulative births, ever, across every round. The
codon template filler expanded *every* `{`, including the literal block
braces of `if … { … }` codons, mangling every compound insertion into a
miscarriage. Every "evolved" genome in rounds 1–2 was built exclusively
from brace-free genes. This is the lineage's M2 lesson relearned in one
generation: *every string a canonical artifact interpolates is an
injection channel.* (Now pinned by a 300-fill parse test; miscarriage
load dropped ~75% on the fix, and predation jumped 20× as `if occupied
{ bite }` genes became insertable for the first time.)

**Then, at seed 44, tick 43,141 — the first native solve.** Organism
#35245, generation 11, lineage drifter, no scholar ancestry, no
injection, assembled by mutation and selection alone:

```
step(roll(3) - 1, roll(3) - 1);
harvest();
if occupied(0,-1) == 1 { bite(-2,-2); }
if light(0,0) > 15 { harvest(); }
if energy() > 599 { spawn(); }
if energy() > 1200 { spawn(); }
if light(0,0) > 15 { harvest(); }
if puzzle(0,0) >= 0 { answer(0, 0, (puzzle(0,0) * 56 + 59) % 218); }
harvest();
```

`(56x + 59) % 218` is wrong in general and right on a subset of
challenges — a lottery ticket, not a theory. It wandered, harvested,
occasionally bit a neighbor, and when it crossed an oracle holding an x
in its lucky set, it took the whole 500e. Replay `--seed 44` and it
happens again, to the tick. **Q2 is answered: yes — but only after the
physics offered a gradient, the oracles moved to where life is, and the
gene became cheap enough to carry idle.** Warmth flowed on 5/5 seeds
(34–928e); carriers arose 875–7,165 times per run.

**Q3 after invest():** cohorts solved on 3/3 seeds (10 solves, up from
4), and the scholar-solves test now finds the bounty already invested in
a 700e child by tick's end. But dynasties still fizzle at generation ~1:
the children carry the expensive genome without the founder's luck.
Parental investment is necessary but not sufficient; the rest is
evolution's job now that scholar genes compost into the pool.

## Round 4 — the intelligence gains: learning within a lifetime

The warmth gradient's deepest property had gone unused: **`answer()`
returns the payout, and payout is monotone in closeness** — a standing
organism can *feel how near it is*. Nothing in the physics prevented
in-lifetime learning; no organism had ever done it.

**The empiricist** (`genesis::EMPIRICIST`, in the observatory's INJECT
panel) is the first learning organism: it knows no formulas. It probes an
oracle, reads the warmth, remembers its best guess in memory slots,
shrinks its search window, and converges — guess/feel/refine, funded by
the warmth it mines while studying. In isolation it cracks **tier III**
(the quintic, unsolved by anything before) in ~53 ticks: search → first
warmth at error 4 → hill-climb → exact, collecting 5,174e en route.
Pinned by `the_empiricist_learns_tier_three_without_the_formula`.

Two lessons were bought on the way:

- **Research discipline**: the first funded empiricist, granted 50,000e,
  spent it all on children — `spawn()` fired every rich tick until the
  research fund was a nursery and the scientist starved mid-study at
  age 111. Its genome now reproduces only *between* studies
  (`if x < 0 && energy() > 1400`). Grant management is a gene.
- **Study is capital-intensive**: random probing over the full answer
  space costs more than a founder's endowment. The genome now searches
  the small range first (tier-I answers live under 64) and escalates
  after 24 failures; the physics widened warmth to `escrow >> error`
  (halving, not quartering), making near-miss income fund the study.

**Field results** (cohorts at tick 2000, 50K ticks): the empiricist line
itself solved on 2/3 seeds within its founders' lifetimes, and mined up
to 9,608e of warmth. Dynasties still end at generation ~1 — learning is
heritable (it's in the genome) but the capital to apply it is not yet.

**The control result that matters most**: seeds with NO injection now
show a native intelligence economy — seed 11 produced **2 exact native
solves and 2,915e of mined warmth**, unaided, and warmth flows on every
control seed. The halving turned near-answer probing into a viable
native profession: temple beggars who occasionally become discoverers.
Intelligence is no longer an event in this world; it is an industry.

## Round 5 — the inheritance problem

Round 4 ended with dynasties dying at generation 1: learning was heritable
(genome) but nothing else was. Six sub-rounds of hand-tuning attacked it:

1. **Full-grant inheritance** (ENDOW_MAX 1000→2500, invest(2500), spawn
   bar 3600): a demographic transition — founders could no longer afford
   children at all ("4 ever lived, deepest gen 0"). The fully-funded-
   education strategy priced reproduction out of existence.
2. **Repriced education** (tier escrows 900/2000/5000 — tier I had been
   priced for formula-knowers, not students): founders solved on 3/3
   seeds and still never spawned; post-study balance (~2,300) sat just
   under the bar.
3. **The commute diagnosis**: a traced founder died at t22, burning
   132/tick against 19/tick income, never reaching an oracle. The
   bottleneck was never capital — a grant buys ~20 ticks and commutes
   cost 30–80. Five of six educated children died in transit.
4. **Position inheritance** (spawn at the study site, oracles basking
   radius 8): generation 1 again, no further.
5. **Ambush learning** (follow scent, else follow light, else STAND
   STILL — camping in food is free) + **fast oracle turnover** (TTL
   600→200, so oracles move faster than lifetimes and come to you):
   generation 2, once. Still extinction.
6. **The concession that solved it**: stop hand-tuning what evolution
   explores 40,000 births at a time. The empiricist entered the
   germline as a sixth founding species — reseeded at every genesis,
   thresholds open to mutation, genes open to crossover — and the
   pre-registered scarcity experiment ran: SUN_INFLUX 4000→2800, oracle
   industry grown to 16 (8/5/3). When farming pays less and thinking
   pays more, the wage share of intelligence rises.

**The result (100K ticks, no injection, ever):** native solves in EVERY
era of the world's history — seed 7: epochs 0–5K, 15–20K, 45–50K (tier
II), **75–80K (tier III — the quintic, solved natively for the first
time, ~300 generations after genesis)**, and 95–100K. Seed 1618: five
separate solving epochs. Warmth mined: 14K–21K ergs per run — a
permanent industry.

**And the shape of the answer**: every founding lineage still dissolves
— the census reads all-drifter — but 2,411–7,203 answer-gene carriers
arise per run, continuously, assembled from compost splices and
crossover. **Intelligence does not persist as a dynasty. It persists as
circulating genes** — a profession individuals take up and pass on, not
a bloodline. The lineage's own founding law ("code dies; knowledge
survives") reproduced itself as an empirical result inside its own
descendant: the family name is lost; the knowledge is immortal.

## Round 6 — the knowledge economy: memes, theft, and bistable markets

Round 5 proved intelligence circulates through genes. But a third
transmission channel had never carried a single erg: `peek()` — paid
memory-reading. A converged empiricist holds the oracle's answer in its
memory slot 5, in the open, purchasable for 4e — and `answer()` works
from adjacent cells.

**The plagiarist** (`genesis::PLAGIARIST`) is the first organism to buy
knowledge: it finds an oracle occupied by a studying teacher, pays
tuition to read the teacher's working memory, and submits the teacher's
own best guess from the next seat over. It does not study. Its education
is someone else's.

**The staged classroom** (one teacher, one thief, tier III) produced the
round's sharpest result — the heist works *too* well. As the teacher
converges, the thief's snipes pay more (each submits the teacher's
ever-improving guess): income 85 → 2,507e while the escrow bled 5,000 →
21. **Nobody ever took the pot.** The prize was strip-mined to nothing
through the open window of the teacher's mind — the tragedy of the
knowledge commons, in the first classroom this world ever had. Pinned by
`the_plagiarist_profits_from_a_teachers_study`.

**At civilization scale** (plagiarists in the germline, 100K ticks), the
same physics produced OPPOSITE regimes on different seeds — the
knowledge economy is bistable:

- Seed 7: the tragedy dominates. One early solve, then intellectual
  silence; scroungers suppress study.
- Seed 1618: thieves TURBOCHARGED the market — the best intelligence
  curve ever recorded: [6, 3, 0] with a late-run golden age (3 tier-I +
  2 tier-II solves in the single epoch 65-70K), record warmth of
  26,095e, solving sustained to the end of history. More minds attempt
  answers, tuition circulates, and crossover hybridizes thief and
  studier genes into producer-scrounger mosaics.

Open, and one digit-jitter away: **deception**. Thieves read slot 5 by
convention; a studier that stores its working memory in another slot is
invisible to them, and `store(5, ...)` mutating to `store(2, ...)` is a
single-token change. The arms race has eight rooms to hide in; whether
evolution finds cryptic cognition is the next thing the world gets to
answer.

## Round 7 — the hourly surveys, iteration 1: the lottery-ticket economy

First fully autonomous loop iteration: four prospectors swept eight fresh
seeds at 150K ticks while a forensics auditor dumped and analyzed every
answer-gene carrier (5,578 records) from two 100K-tick reference worlds.

**The survey: native intelligence is universal now.** Solves on 8/8
never-before-run seeds — 50 tier-I, 20 tier-II, 3 tier-III across the
sweep, including a purely evolved tier-III solve (seed 577, grazer
lineage, gen 6: `(52x + 52) % 507`). De-novo answer genes arose
independently on at least six seeds (three separate times on seed 2027;
a two-ticket genome on 1013; a gen-257 solver on 1381). Specimen of the
sweep: seed 3163's gen-153 drifter solved tier II with
`answer(0, 0, (puzzle(0,1) * 2 + 6) % 489)` — it reads the NEIGHBOR
cell's puzzle (usually -1), making its answer a constant. A wrong theory,
profitably held. Also observed: the **phoenix cycle** (crash → regenesis
→ fresh gen-0 empiricists resume solving — the germline is intelligence's
disaster recovery), a 441,974e fortune built in one 6,000-tick lifetime,
and vestigial mutation scars (`light(-16,0)`, clamped and unexpressed —
junk DNA).

**The audit: no deception — and the reason rewrites round 6.** Across
5,578 carriers and 200K ticks: 96-97% are **lottery tickets** — bare
hard-coded `(a*x+b)%m` genes riding farming torsos (418-629 distinct
formula families per world; top family cloned 367 times). Only 69
carriers hold the intact learning loop, 66 of them gen-0 reseeds; the
learning haplotype has left 3 descendants ever (one of which, #52524,
became the first descendant learner to solve — its inheritance
byte-faithful). Every learning line in both worlds is **byte-identical
to genesis** (the full 18-digit slot signature conserved, 69/69); every
answer feed reads slot 4 (69/69); every peek reads slot 5 (59/59).

Round 6 called deception "one digit-jitter away." **Wrong — the audit
found the valley**: because the LINE is the gene, the writer
`store(5, load(4))` and the hill-climb reader sit on the same line — a
lone digit flip yields a learner that stores its discovery in one room
and searches a stale room, strictly worse than honest AND worse than
bare. Hidden-but-intact cognition requires two coordinated mutations,
and selection cannot cross that valley through broken intermediates.
**The genome representation itself forbids cryptic drift.** Deeper
still: crossover-transferred thief genes are POISON — chimeras inherit
the snipe gene without its `let t` declaration line and crash E-UNDEF
every tick, forfeiting their tanks (0 hybrid solves ever).
Representation determines evolvability; the line-as-gene convention
that makes mutation safe also makes multi-line strategies nearly
non-heritable. That is the next wall, and it is a wall in the genetics,
not the economics.

## Round 8 — loop iteration 2: cassette genetics, and why instinct beats learning

Iteration 1 located the wall in the genetics: multi-line strategies were
non-heritable (splices tore declarations from the genes that needed
them). Iteration 2 built **cassette genetics**: dying organisms compost
2-3-line consecutive blocks that splice as one unit; gene duplication
can copy whole blocks; and the census now reports heritability directly.
A pinned test proves a cassette never arrives torn (2,000 trials).

**Transmission: cured.** Across six 100K-tick validation seeds, snipe
genes now arrive **140 intact vs 4 torn** — under the old genetics,
essentially every transfer was poison. Learning-loop descendants: 11
(deepest gen 2), up from 3 in twice the tick-volume.

**Adoption: refused — and the refusal is the discovery.** Zero
descendant learners solved anything, on any seed. Every learning-loop
solve in ~600K cumulative ticks was a gen-0 genesis reseed. Meanwhile
evolved HARD-CODED solvers flourished at depth: gen-326, gen-215 (a
formula with an evolved negative term, `(31x - 37) % 170`), gen-122,
gen-92 (paired `%447`/`%223` formula lines from one comparison flip),
gen-23 (took 5,000e from oracle III with a grown codon). Evolution,
offered heritable learning, keeps choosing lottery tickets.

It is right to. The oracle formulas are **eternal constants of this
world** — the same three functions, forever, on every seed. When the
environment never changes within a lifetime, evolutionary theory says
instinct beats plasticity: write the knowledge into the genome, not the
brain. The formula IS the memory, and DNA is the only RAM that
inherits. The learner pays ~60/tick for knowledge that evaporates when
its oracle moves; the instinct-carrier pays ~25/tick for knowledge that
its children get for free. **In a fixed world, the genome is a better
memory than memory.**

Which hands iteration 3 its mission: make the world's problems
ephemeral. Give each oracle instance its own secret parameters, and no
genome-inscribed formula can ever solve twice — within-lifetime
learning becomes the only strategy that pays, and the Baldwin
conditions for intelligence finally hold.

## Round 9 — loop iteration 3: ephemeral oracles, and the Baldwin prize

Iteration 2 ended with a mission: make the world's problems ephemeral so
learning can beat instinct. Done — **every oracle instance now draws
secret coefficients (a, b) at spawn**. The families stay public law
(tier I: `(ax+b) mod 64`; II: `(x²+ax+b) mod 199`; III:
`(ax²+bx+11) mod 509`); the constants die with each oracle. Nothing
memorized solves twice. The empiricist was pre-adapted (it never knew
the formulas — its tier-III exam passed unchanged); the scholar is
retired to a staged-relic test, an exhibit of the fixed-world era.

**The Baldwin test (six 100K-tick seeds):**

- **Learning now outsolves instinct: 17 learner solves vs 13 instinct
  solves** — the first reversal ever. Under eternal formulas, learners
  took ~25% of solves; on two seeds (271, 577) instinct **collapsed to
  zero** and every solve was a warmth-feedback learner, including a
  tier-II secret hill-climbed for 1,985e.
- **The Baldwin prize**: seed 271, tick 34,394 — #17259, lineage 5,
  generation 1, an inherited intact learning loop, solved. The first
  gen>0 learner solve in the world's history. Loop descendants now
  reach generation 3.
- **Re-convergence observed**: #42274 solved twice in 11 ticks —
  learning that generalizes across instances, the thing no lottery
  ticket can do.
- Total solve volume halved (25 vs ~50 per six seeds) — as predicted:
  removing instance-luck removed the lottery revenue. What remains is
  earned. Instinct persists on some seeds as deep-generation flukes
  (gen 203, 222 carriers whose mutated constants happened to match one
  instance once) — vestiges of a business model the physics revoked.

Three iterations of the autonomous loop tell one story: locate the wall
(iteration 1: multi-line strategies non-heritable), remove it (iteration
2: cassette genetics — and discover transmission wasn't the binding
constraint), then change what the world rewards (iteration 3: ephemeral
problems) — and evolution finally chooses minds over instincts, and
passes them on.

## Round 10 — loop iteration 4: the first standing mind, and the silence of stable worlds

Endurance runs (200K ticks — twice the deepest horizon ever) on the
three flagship seeds, asking whether ephemeral physics lets a standing
learner class form.

**A first, by the thinnest margin**: seed 1618 ended with **1
answer-gene carrier alive** — the first living carrier at any final
census in this world's history (3,685 ever on that seed). The same run
produced a gen-260 drifter cracking tier III for 5,000e at tick 159K and
a gen-359 solve at tick 190K.

**And a warning**: on the other two seeds, intelligence went silent —
zero solves in the final 60–87K ticks. The mechanism is now clear: the
empiricist lineage goes extinct in mature ecologies, fresh learners
arrive only through regenesis, and regenesis fires only at total
extinction. **Stable worlds starve themselves of minds.** The phoenix
cycle that saved intelligence in young worlds is a bottleneck in old
ones — catastrophe is currently this world's only teacher-training
program. Learning-loop descent still stalls shallow (deepest gen 3;
the Baldwin solve remains the only inherited-learner solve).

The iteration's ledger: the learner class CAN stand (observed once);
what it lacks is a renewal channel that doesn't require the end of the
world. That is the next frontier the loop inherits.

## Round 11 — loop iteration 5: amber, and the resurrection of minds

Iteration 4's frontier: a renewal channel for intelligence that doesn't
require the end of the world. The mechanism: **amber** — a 64-entry
protected stratum of the compost preserving every oracle-touching gene
and cassette indefinitely (only newer amber displaces older amber), with
a quarter of all mutation splices drawing from it. The compost forgets;
the amber does not. Pinned by test: a doomed learner's genes survive
4,000 ticks of monoculture churn.

**The A/B against iteration 4's baselines (three flagship seeds, 200K):**

- **Eleven amber-splice solves** — resurrections: organisms whose last
  mutation reads "spliced a gene from the amber," solving. On seed 1618
  the world's FIRST solve arrived by resurrection (tick 29,351, a
  gen-41 grazer). On seed 7 the world's first tier-II solve came
  directly from an amber splice. On seed 271, two resurrections landed
  in the final 15K ticks of a world whose baseline had been silent.
- **Late-epoch silence broken on 2 of 3 seeds**: seed 7 went 0 → 10
  solves after tick 150K; seed 271 went 0 → 6 (including its first
  tier-III, at tick 154,812). Seed 1618 regressed there (2 → 0) while
  front-loading — amber is a strong but not universal cure, honestly
  logged.
- **Every intelligence metric moved**: total solves 39/10/4 by tier
  across the trio (baseline ~27 total); warmth ~228K vs ~57K; carriers
  alive at final census 4 (baseline 1 — and that 1 was the first ever);
  learning-loop descendants 117-166 per seed (baseline 1-8), deepest
  descent **generation 58** (baseline 3). Amber saturated 64/64 on all
  seeds.

The arc the loop has now built, layer by layer: energy became money
(genesis), money selected genes (rounds 1-5), genes learned to learn
(rounds 8-9), and now the learned dead are a library the living splice
from. The amber is this world's first cultural institution — and its
worlds no longer need apocalypse to remember how to think.

## Round 12 — loop iteration 6: the bequest, a null result proven by hash

The mechanism: **memory inheritance** — children are born with a copy of
the spawning parent's memory slots (`child.mem = parent.mem`, pinned by
test), so a converged study could in principle continue across
generations. The physics card: "Teach by living."

**The result: provably inert.** The A/B runs on seeds 7 and 1618
produced world hashes **identical to the iteration-5 baselines**
(`d5f0b6cc3480939a`, `f9752a81332fed38`) — 200,000 ticks of history,
bit-for-bit unchanged. Determinism upgraded a null result into a proof:
across two full world-histories, not one organism ever behaved
differently because of its inheritance. (This is the determinism
dividend at its sharpest — most simulators could never distinguish "no
effect" from "small effect.")

**Why the bequest cannot work — and why that's the finding.** The
empiricist's reset guard (`if load(3) != x { reset }`) discards
inherited study state at any unfamiliar oracle — and it is RIGHT to:
under ephemeral physics (round 9), a parent's converged best-guess is
knowledge about coefficients that die with their oracle. The only
scenario where inherited memory pays is seat-succession — a child taking
its parent's exact seat mid-study before the oracle turns over — which
occurred zero times in 400K observed ticks. Rounds 9 and 12 are in
structural tension, now stated as law: **making problems ephemeral makes
learning valuable and answers unbequeathable, by construction. In an
ephemeral world you cannot inherit answers — only the ability to find
them.** Which is precisely what the amber already transmits (genes for
the search loop), and why round 11's resurrections worked where round
12's bequests could not.

The bequest stays in the physics (one line, provably harmless, and it
becomes load-bearing the moment any durable knowledge exists). The
iteration's yield is the theorem, honestly priced: one hour, one null,
one law.

## Round 13 — loop iteration 7: stratified amber, and the paper

The mechanism: the amber now runs a quota — half its 64 slots reserved
for COGNITIVE entries (genes that both answer and remember); a lottery
ticket can never evict a mind (pinned by a flood test). The hypothesis:
resurrections would start carrying learning loops instead of bare
formulas.

**A/B (flagship seeds, 200K; seed 271's agent failed reporting):**

- **The deep numbers exploded**: learning-loop descent went from
  generation 58 to **generations 467 and 476**, with ~3,200-3,400 loop
  carriers ever and thousands of descendants per seed — the learning
  haplotype is now a permanent fixture of the gene pool, not a reseed
  artifact.
- **Seed 1618's regression is cured**: [13,2,4] with 6 late-epoch solves
  (was [8,4,1] with 0); four tier-III solves in one run — a record.
  Seed 7 went learner-majority (8 of 14 solves) and produced the
  world's first **serial solver**: #122887, four solves in 48 ticks.
- **The named prize stays unclaimed, for a newly-understood reason**:
  no resurrected learner — and amber-splice solves became RARER (1 and
  0 vs 3 and 3). Stratification halved the formula slots that produced
  ticket-resurrections, while spliced cognitive genes need their
  carrier to reach an oracle and survive study — a taller order than a
  ticket's lucky hit. The library now reliably holds minds; getting
  them read profitably remains open.

Also this hour, in the family tradition — the paper is the product:
**paper/METABOLITE.md** (~2,900 words), the twelve-round arc from
fuel=money to the ephemerality law, negatives first-class, every figure
sourced from this notebook, reproduction commands with expected hashes.

## Round 14 — loop iteration 8: reads-as-zero, and the resurrection of learners

Round 13 found the amber's mind-reserve full of runtime poison: spliced
study lines referencing an `x` whose `let` the cassette tore off — and
in wit an unbound read was fatal (whole tank forfeited, every tick).
The fix was a language decision: **unbound names now read as 0**
(assignment still requires `let`). A torn gene is dead, not fatal —
degraded, cheap, polishable. Composability at the language level.

**The A/B (three flagship seeds, 200K ticks) claimed the prize six
times over:**

- **Six resurrected-learner solves** — amber-splice organisms carrying
  the full learning machinery, a class with zero members in all prior
  history. Among them: #149472 (gen 1) with the intact empiricist loop
  on a light-seeking torso; #169325 (gen 7) — an amber-spliced learner
  that **cracked oracle III**; and #120693 (gen 7, seed 271) — the
  first true **hybrid**: the intact learning loop grafted onto a
  SESSILE forager torso. A different species, carrying a mind.
- **Deep learner solves: six this round** (gens 1, 7, 7, 48, 79, 109)
  vs exactly one in all previous history combined.
- Seed 7: 23 solves [14,6,3] vs 14 baseline, warmth 147,846 (~2x), 9
  carriers alive at census (record). Several working loops carry
  mutation scars (store(19,...), answer(18,0,...), thresholds jittered)
  that only function because unbound reads are 0 — selection is already
  polishing degraded fragments, exactly as designed.
- The honest trade: pure-line depth fell (deepest loop-gen 118-339 vs
  467-476), and seed 1618 mixed ([7,5,0]). Composability converts
  single-lineage accumulation into CROSS-TORSO SPREAD — the loop now
  migrates between species instead of piling up in one. Minds became
  organs: modular, portable, grafted across the tree of life by a
  library and a splice.

Fourteen rounds ago this world could not add. It now resurrects dead
mathematicians into farmers' bodies, and they solve quadratics for
money.

## Round 15 — loop iteration 9: the Atlas

Eight never-run seeds (5001-5008), 100K ticks each, under the final
physics — what an *ordinary* universe now produces:

| seed | solves I/II/III | learner:instinct | resurrected learners | deep learners | minds at census |
|---|---|---|---|---|---|
| 5001 | 5/1/1 | 3:4 | 0 | 1 | 0 |
| 5002 | 8/4/1 | 9:4 | **6** | 7 | 3 |
| 5003 | 7/4/2 | 7:6 | **6** | 6 | **32** |
| 5004 | 7/1/0 | 5:3 | 4 | 5 | 0 |
| 5005 | 5/5/0 | 3:7 | 0 | 0 | 0 |
| 5006 | 3/1/1 | 2:3 | 1 | 1 | 1 |
| 5007 | 4/0/1 | 1:4 | 1 | 1 | 0 |
| 5008 | 2/1/0 | 2:1 | 1 | 1 | 1 |

Reading the table: solves on 8/8 seeds (41/17/6 by tier — six tier-III
cracks in fresh universes); **19 resurrected-learner solves** across the
sweep — two iterations ago that class had zero members ever, now it is
routine, including one at generation 203; **22 deep learner solves** vs
the single one that existed in all history before round 14; and seed
5003 ended with **32 standing minds** — the first true scholar class,
3.5x the previous record. Learner and instinct economies sit at parity
(32:32) as the norm, with honest variance: 5004 front-loaded then went
mind-extinct; 5005 stayed instinct-heavy; 5002 survived a crash to
population ONE and came back learning-dominant.

Also this hour: paper/METABOLITE.md revised through round 14 (4,054
words) — abstract, results, the new composability law (law 10:
composability converts lineage depth into cross-species spread), and
the depth-vs-spread trade in Limitations.

## Round 16 — loop iteration 10: deep time (partial, and good)

The first half-million-tick horizon. One flagship delivered (seed 7,
truncated at ~452K by the harness — the two other runs died with their
supervising agents; noted for ops: background simulations must live in
the orchestrator's session, not a subagent's).

**Seed 7's five-era verdict: intelligence sustains.** Solves per
100K-tick era: 10, 13, 9, 6, and 5 in the final half-era — a per-tick
rate in era 5 comparable to the founding eras. Totals at 450K:
[28, 10, 4] with warmth 216,340; 27 of 43 dumped solves were learner
genomes; 13 deep-learner solves; learning-loop descent to gen 351.

**The late-history novelty is the designed one**: all five
resurrected-learner solves occurred AFTER tick 300K — including a
gen-85 amber-spliced learner solving at tick 451,863 with the full
adaptive-search loop (widening exploration, step-size annealing). In
deep time, resurrection is not a curiosity; it is the renewal channel
that keeps old worlds thinking, precisely what the amber was built for
and never before observed at this depth.

Also this hour: CLAUDE.md refreshed to the current world (stratified
amber, reads-as-zero, 25 tests) — the surface cap holds at 5,560/8,000.

## Round 17 — loop iteration 11: the referee round (pre-registration)

The paper review (three adversarial referees) identified two fatal
apparatus gaps: numbers extracted by in-loop agents, and no controlled,
powered test of the headline claim. This round fixes both.

**New instruments**: `--eternal` mode (per-tier oracle coefficients drawn
once at genesis, shared by every instance — the condition instinct can
inscribe; ephemeral remains the law), and `scripts/analyze.py` — a
deterministic parser and exact sign test. Numbers now come from the
script, never from agents.

**PRE-REGISTERED (before any run): H1** — under ephemeral oracles the
learner share of solves exceeds the same seed's share under eternal
oracles. Design: seeds 9001-9016, both conditions, 100K ticks, same
binary. Classification (fixed in the script): learner-solve = SOLVE
block using memory (`load(`/`store(`); instinct = no memory use. Test:
exact one-sided paired sign test on learner-share, ties dropped.
Secondary (descriptive): aggregate learner/instinct counts and warmth
per condition. RESULTS BELOW WERE WRITTEN AFTER THIS PARAGRAPH.

RESULTS-PENDING-ROUND-17

## Standing answers

- **Q1 (does evolution beat drift?)** Efficiency: no. Strategy: yes — the
  miser phenotype was a genuine invention, twice removed from any founding
  genome, discovered independently on 5/5 seeds. The ecology optimizes
  *survival architecture*, not thermodynamic efficiency, because the
  binding constraint is death, not yield.
- **Q2 (native oracle solve?)** **YES** — seed 44, tick 43,141, organism
  #35245 (round 3). It required three physics changes (warmth gradients,
  sun-basking oracles, a cheap guarded gene) and one bug fix (the codon
  injection channel). The discovery was not made *harder* by economics —
  it was made *possible* by it: every enabling change was a change to
  what pays.
- **Q3 (injected intelligence: sweep or integrate?)** Neither — it
  *visits*. Founders solve (2/3 seeds, deterministically) and the live
  observatory saw a double-solver leave double the median offspring; but
  the line always fizzles at generation 1 because children inherit the
  genome without the capital. Intelligence has a founder-effect niche:
  it must arrive early or rich, and heredity of wealth is the missing
  organ.

## Next experiments (each is one small law away)

1. **Parental investment as an evolvable trait**: `spawn(endowment)` with
   the amount as a genome parameter — lets scholar lines evolve to fund
   their children, and lets misers evolve dowries. Directly attacks the
  Q3 bottleneck.
2. **Scarcity ladder**: halve SUN_INFLUX mid-run and watch whether the
   efficiency equilibrium (F1) shifts or the population simply shrinks —
   distinguishes "efficiency is environmental" from "efficiency is
   unevolvable."
3. **Oracle apprenticeship**: a tier-0 oracle whose formula is `y = x`
   (one token from the codon shape) — measures how far mutation actually
   is from the discovery threshold (F2).
