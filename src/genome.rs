//! The mutation engine. A genome is a wit program with a world-level
//! convention: one statement per line, so the LINE is the gene. Mutation
//! operates on lines and digits; the only gate is grammar (wit's parser)
//! plus the byte cap. A mutant that fails the gate is a miscarriage — the
//! spawn burn is still paid. Everything else is selection's problem.

use crate::laws;
use crate::rng::Rng;
use std::collections::VecDeque;

/// Codon templates for the insert op. `{s}` fills with a step offset in
/// [-1,1], `{d}` a nonzero step, `{r}` a sense offset, `{n}` a small number,
/// `{N}` a big number, `{m}` a memory slot. The oracle codon carries the
/// SHAPE of an answer, never the answer — parameters must evolve.
const CODONS: &[&str] = &[
    "harvest();",
    "step({d},{d});",
    "if light({r},{r}) > {n} { step({s},{s}); }",
    "if light(0,0) > {n} { harvest(); }",
    "if energy() > {N} { spawn(); }",
    "if occupied({s},{s}) == 1 { bite({s},{s}); }",
    "if kin({s},{s}) == 1 { give({s},{s}, {n}); }",
    "store({m}, load({m}) + 1);",
    "emit({n});",
    "if roll({n}) == 0 { step({d},{d}); }",
    "if scent({r},{r}) > {n} { step({s},{s}); }",
    "answer({s},{s}, (puzzle({s},{s}) * {n} + {n}) % {N});",
];

fn fill(template: &str, rng: &mut Rng) -> String {
    let mut out = String::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '{' {
            let k = chars.next().unwrap_or(' ');
            chars.next(); // consume '}'
            let v = match k {
                's' => rng.range(-1, 1),
                'd' => *[-1i64, 1].get(rng.below(2) as usize).unwrap_or(&1),
                'r' => rng.range(-laws::SENSE_RADIUS, laws::SENSE_RADIUS),
                'n' => rng.range(1, 64),
                'N' => rng.range(64, 900),
                'm' => rng.range(0, laws::MEM_SLOTS - 1),
                _ => 0,
            };
            out.push_str(&v.to_string());
        } else {
            out.push(c);
        }
    }
    out
}

/// Two-char operators first so `<` never matches inside `<=`.
const OP_SWAPS: &[(&str, &str)] = &[
    ("<=", ">="),
    (">=", "<="),
    ("==", "!="),
    ("!=", "=="),
    ("&&", "||"),
    ("||", "&&"),
    ("+", "-"),
    ("-", "+"),
    ("*", "%"),
    ("%", "*"),
    ("<", ">"),
    (">", "<"),
];

/// One mutation of `src`. Returns (child_source, description). The caller
/// applies the grammar gate; this function only edits text.
pub fn mutate(src: &str, rng: &mut Rng, compost: &VecDeque<String>) -> (String, String) {
    let mut lines: Vec<String> = src.lines().map(|l| l.to_string()).collect();
    if lines.is_empty() {
        lines.push("harvest();".to_string());
    }
    let roll = rng.below(100);
    let desc;
    if roll < 40 {
        // Jitter a number.
        let spans = digit_spans(&lines);
        if let Some(&(li, start, end)) = pick(&spans, rng) {
            let old: i64 = lines[li][start..end].parse().unwrap_or(0);
            let new = jitter(old, rng);
            lines[li].replace_range(start..end, &new.to_string());
            desc = format!("{old}->{new} on line {}", li + 1);
        } else {
            desc = "no-op".to_string();
        }
    } else if roll < 52 {
        // Swap an operator.
        let mut hits: Vec<(usize, usize, usize)> = Vec::new(); // line, pos, swap idx
        for (li, line) in lines.iter().enumerate() {
            for (oi, (from, _)) in OP_SWAPS.iter().enumerate() {
                let mut at = 0;
                while let Some(p) = line[at..].find(from) {
                    let pos = at + p;
                    let two = OP_SWAPS[..6].iter().any(|(f, _)| line[pos..].starts_with(f));
                    if from.len() == 2 || !two {
                        hits.push((li, pos, oi));
                    }
                    at = pos + from.len();
                }
            }
        }
        if let Some(&(li, pos, oi)) = pick(&hits, rng) {
            let (from, to) = OP_SWAPS[oi];
            lines[li].replace_range(pos..pos + from.len(), to);
            desc = format!("{from}->{to} on line {}", li + 1);
        } else {
            desc = "no-op".to_string();
        }
    } else if roll < 62 {
        let li = rng.below(lines.len() as u64) as usize;
        let l = lines[li].clone();
        lines.insert(li, l);
        desc = format!("duplicated line {}", li + 1);
    } else if roll < 74 {
        if lines.len() > 1 {
            let li = rng.below(lines.len() as u64) as usize;
            lines.remove(li);
            desc = format!("deleted line {}", li + 1);
        } else {
            desc = "no-op".to_string();
        }
    } else if roll < 84 {
        if lines.len() > 1 {
            let a = rng.below(lines.len() as u64) as usize;
            let b = rng.below(lines.len() as u64) as usize;
            lines.swap(a, b);
            desc = format!("swapped lines {} and {}", a + 1, b + 1);
        } else {
            desc = "no-op".to_string();
        }
    } else {
        // Insert: half fresh codon, half scavenged from the compost.
        let line = if !compost.is_empty() && rng.below(2) == 0 {
            let i = rng.below(compost.len() as u64) as usize;
            desc = "spliced a composted gene".to_string();
            compost[i].clone()
        } else {
            let i = rng.below(CODONS.len() as u64) as usize;
            desc = "grew a new codon".to_string();
            fill(CODONS[i], rng)
        };
        let li = rng.below(lines.len() as u64 + 1) as usize;
        lines.insert(li, line);
    }
    (lines.join("\n"), desc)
}

fn jitter(v: i64, rng: &mut Rng) -> i64 {
    let j = match rng.below(6) {
        0 => v + 1,
        1 => v - 1,
        2 => v * 2,
        3 => v / 2,
        4 => v + rng.range(1, 16),
        _ => v - rng.range(1, 16),
    };
    j.max(0) // literals are non-negative in wit (unary minus is an operator)
}

fn digit_spans(lines: &[String]) -> Vec<(usize, usize, usize)> {
    let mut spans = Vec::new();
    for (li, line) in lines.iter().enumerate() {
        let b = line.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if b[i].is_ascii_digit() {
                let start = i;
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
                spans.push((li, start, i));
            } else {
                i += 1;
            }
        }
    }
    spans
}

fn pick<'a, T>(v: &'a [T], rng: &mut Rng) -> Option<&'a T> {
    if v.is_empty() {
        None
    } else {
        v.get(rng.below(v.len() as u64) as usize)
    }
}

/// Single-point line-level crossover: a prefix of one parent's genes, a
/// suffix of the other's. Sex is a splice; the grammar gate still rules.
pub fn crossover(a: &str, b: &str, rng: &mut Rng) -> String {
    let al: Vec<&str> = a.lines().collect();
    let bl: Vec<&str> = b.lines().collect();
    if al.is_empty() || bl.is_empty() {
        return a.to_string();
    }
    let i = 1 + rng.below(al.len() as u64) as usize;
    let j = rng.below(bl.len() as u64 + 1) as usize;
    let mut child: Vec<&str> = Vec::new();
    child.extend(&al[..i.min(al.len())]);
    child.extend(&bl[j.min(bl.len())..]);
    child.truncate(64);
    child.join("\n")
}

/// The grammar gate: byte cap, then wit's parser. This is the only
/// admission check a newborn faces; economics does the rest.
pub fn viable(src: &str) -> Result<(), String> {
    if src.len() > laws::GENOME_CAP {
        return Err(format!("E-SIZE: genome {} bytes > cap {}", src.len(), laws::GENOME_CAP));
    }
    match crate::mind::parse(src) {
        Ok(_) => Ok(()),
        Err(d) => Err(d.code.to_string()),
    }
}
