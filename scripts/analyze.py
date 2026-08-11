#!/usr/bin/env python3
"""Deterministic analysis of metabolite headless logs — the versioned
pipeline the referees asked for. Numbers come from this script, never
from in-loop judgment.

Usage: python scripts/analyze.py <dir-of-logs>
Log filename convention: <condition>-<seed>.log (e.g. ephemeral-9001.log).

Pre-registered classification (round 17):
  learner-solve  := a SOLVE block whose genome uses memory (load( or store()
  instinct-solve := a SOLVE block with no memory use
Pre-registered test: for seeds run under BOTH conditions, compare learner
share L/(L+I) pairwise; exact one-sided sign test (ties dropped) of
H1: share(ephemeral) > share(eternal).
"""
import math
import re
import sys
from pathlib import Path


def parse_log(path: Path):
    text = path.read_text(encoding="utf-8", errors="replace")
    learner = instinct = 0
    for m in re.finditer(r"SOLVE at tick \d+.*?\(last mutation:", text, re.S):
        block = m.group(0)
        if "load(" in block or "store(" in block:
            learner += 1
        else:
            instinct += 1
    final = re.search(
        r"tick\s+(\d+) .*?solves \[(\d+), (\d+), (\d+)\] · warmth\s+(\d+)",
        text.splitlines()[-40:] and "\n".join(text.splitlines()[-60:]) or text,
    )
    solves = tuple(int(final.group(i)) for i in (2, 3, 4)) if final else (0, 0, 0)
    warmth = int(final.group(5)) if final else 0
    return dict(learner=learner, instinct=instinct, solves=solves, warmth=warmth)


def sign_test_one_sided(pos: int, neg: int) -> float:
    """P(X >= pos) for X ~ Binomial(pos+neg, 0.5)."""
    n = pos + neg
    if n == 0:
        return 1.0
    return sum(math.comb(n, k) for k in range(pos, n + 1)) / 2**n


def main(d: str) -> None:
    logs = sorted(Path(d).glob("*.log"))
    data: dict[tuple[str, int], dict] = {}
    for p in logs:
        m = re.match(r"(ephemeral|eternal)-(\d+)\.log", p.name)
        if m:
            data[(m.group(1), int(m.group(2)))] = parse_log(p)
    seeds = sorted({s for (_, s) in data})
    print(f"| seed | eph L:I | eph share | ete L:I | ete share | Δ |")
    print("|---|---|---|---|---|---|")
    pos = neg = 0
    for s in seeds:
        e = data.get(("ephemeral", s))
        t = data.get(("eternal", s))
        if not e or not t:
            continue

        def share(r):
            tot = r["learner"] + r["instinct"]
            return r["learner"] / tot if tot else None

        se, st = share(e), share(t)
        delta = "—"
        if se is not None and st is not None:
            if se > st:
                pos += 1
                delta = "+"
            elif se < st:
                neg += 1
                delta = "-"
            else:
                delta = "0"
        fmt = lambda x: "n/a" if x is None else f"{x:.2f}"
        print(
            f"| {s} | {e['learner']}:{e['instinct']} | {fmt(se)} "
            f"| {t['learner']}:{t['instinct']} | {fmt(st)} | {delta} |"
        )
    for cond in ("ephemeral", "eternal"):
        rows = [v for (c, _), v in data.items() if c == cond]
        L = sum(r["learner"] for r in rows)
        I = sum(r["instinct"] for r in rows)
        W = sum(r["warmth"] for r in rows)
        print(f"{cond}: {len(rows)} seeds · learner {L} · instinct {I} · warmth {W}")
    p = sign_test_one_sided(pos, neg)
    print(f"sign test (H1: ephemeral learner-share > eternal): +{pos}/-{neg}, one-sided p = {p:.4f}")


if __name__ == "__main__":
    main(sys.argv[1])
