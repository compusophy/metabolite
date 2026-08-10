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

## Standing answers

- **Q1 (does evolution beat drift?)** Efficiency: no. Strategy: yes — the
  miser phenotype was a genuine invention, twice removed from any founding
  genome, discovered independently on 5/5 seeds. The ecology optimizes
  *survival architecture*, not thermodynamic efficiency, because the
  binding constraint is death, not yield.
- **Q2 (native oracle solve?)** Not yet observed. The tier-I expression is
  ~9 tokens of exact arithmetic; the codon library provides the shape but
  the parameters must jitter into place. Open.
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
