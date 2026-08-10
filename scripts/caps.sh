#!/usr/bin/env bash
# The constitution's teeth. Run by CI and by hand. At a cap: split, shrink,
# or kill — never raise the cap. The great-grandparents died at ~120K LOC;
# this repo cannot get there.
set -euo pipefail
cd "$(dirname "$0")/.."

fail=0

rust_loc=$(cat src/*.rs tests/*.rs | wc -l)
echo "rust LOC (src+tests): $rust_loc / 5000"
if [ "$rust_loc" -gt 5000 ]; then echo "CAP EXCEEDED: rust"; fail=1; fi

html_loc=$(wc -l < web/index.html)
echo "web/index.html lines: $html_loc / 1000"
if [ "$html_loc" -gt 1000 ]; then echo "CAP EXCEEDED: html"; fail=1; fi

claude_chars=$(wc -c < CLAUDE.md)
echo "CLAUDE.md chars: $claude_chars / 8000"
if [ "$claude_chars" -gt 8000 ]; then echo "CAP EXCEEDED: CLAUDE.md (the surface cap is the real cap)"; fail=1; fi

# Zero dependencies. std only. The parents' code is dead; only their
# knowledge is inherited.
deps=$(sed -n '/^\[dependencies\]/,/^\[/p' Cargo.toml | grep -c '^[a-z]' || true)
echo "dependencies: $deps / 0"
if [ "$deps" -ne 0 ]; then echo "CAP EXCEEDED: dependencies (zero, forever)"; fail=1; fi

# No floats. Integer determinism is the replay guarantee.
if grep -nE '\bf(32|64)\b' src/*.rs tests/*.rs; then
  echo "CAP EXCEEDED: floats found (determinism forbids them)"; fail=1
fi

if [ "$fail" -ne 0 ]; then echo "THE CONSTITUTION IS VIOLATED"; exit 1; fi
echo "the constitution holds"
