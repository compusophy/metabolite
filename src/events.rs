//! The event feed: what the observatory scrolls, and what the counters
//! accumulate. Events are display artifacts; the ledger is the truth.

use std::collections::VecDeque;

pub enum Event {
    Birth { tick: u64, child: usize, parent: usize, desc: String },
    Miscarriage { tick: u64, parent: usize, code: String },
    Death { tick: u64, id: usize, age: u64, cause: &'static str },
    Bite { tick: u64, pred: usize, prey: usize, amt: u64 },
    Gift { tick: u64, from: usize, to: usize, amt: u64 },
    Peek { tick: u64, from: usize, to: usize },
    Solve { tick: u64, id: usize, tier: usize, amt: u64 },
    Inject { tick: u64, id: usize, name: String },
    Genesis { tick: u64, count: usize, era: u64 },
}

impl Event {
    pub fn line(&self) -> String {
        match self {
            Event::Birth { tick, child, parent, desc } => {
                format!("{tick} · #{parent} spawned #{child} ({desc})")
            }
            Event::Miscarriage { tick, parent, code } => {
                format!("{tick} · #{parent} miscarried ({code})")
            }
            Event::Death { tick, id, age, cause } => {
                format!("{tick} · #{id} died of {cause} at age {age}")
            }
            Event::Bite { tick, pred, prey, amt } => {
                format!("{tick} · #{pred} bit #{prey} for {amt}e")
            }
            Event::Gift { tick, from, to, amt } => {
                format!("{tick} · #{from} gave #{to} {amt}e")
            }
            Event::Peek { tick, from, to } => {
                format!("{tick} · #{from} paid to read #{to}'s mind")
            }
            Event::Solve { tick, id, tier, amt } => {
                format!("{tick} · #{id} solved oracle {} for {amt}e", ["I", "II", "III"][*tier])
            }
            Event::Inject { tick, id, name } => {
                format!("{tick} · observer injected #{id} \"{name}\"")
            }
            Event::Genesis { tick, count, era } => {
                format!("{tick} · EXTINCTION — genesis {era}: {count} founders seeded anew")
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Counters {
    pub births: u64,
    pub miscarriages: u64,
    pub starved: u64,
    pub predated: u64,
    pub aged: u64,
    pub crashed_runs: u64,
    pub bites: u64,
    pub gifts: u64,
    pub peeks: u64,
    pub solves: [u64; 3],
    pub extinctions: u64,
}

pub struct Feed {
    pub ring: VecDeque<Event>,
    cap: usize,
}

impl Feed {
    pub fn new(cap: usize) -> Self {
        Feed { ring: VecDeque::new(), cap }
    }
    pub fn push(&mut self, e: Event) {
        if self.ring.len() == self.cap {
            self.ring.pop_front();
        }
        self.ring.push_back(e);
    }
}
