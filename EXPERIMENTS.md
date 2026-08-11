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
