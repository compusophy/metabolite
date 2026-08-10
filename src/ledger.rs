//! The ledger. Every erg in the world lives here — in an agent, in a cell,
//! in an oracle's escrow, or momentarily held in an active agent's fuel tank
//! — or it has been burned. Fields are private; the verbs below are the ONLY
//! way ergs move. Conservation is checked every tick:
//!
//!   minted == Σ agent_erg + Σ cell_erg + Σ escrow + held + burned
//!
//! This is guard.rs done right: not a prompt, not a convention — the module
//! system forbids touching money any other way.

/// Where burned ergs went (the observatory's trophic accounting).
#[derive(Default, Clone)]
pub struct Burns {
    pub fuel: u64,    // thinking (fuel_used + forfeited tanks)
    pub basal: u64,   // existing (basal + genome rent)
    pub tithe: u64,   // the golden tithe on transfers
    pub spawn: u64,   // reproduction burn (incl. miscarriages)
    pub expired: u64, // oracle escrows nobody solved
}

/// Where minted ergs came from.
#[derive(Default, Clone)]
pub struct Mints {
    pub sun: u64,     // sunlight into cells
    pub genesis: u64, // founding endowments
    pub inject: u64,  // observer-injected organisms
    pub oracle: u64,  // escrowed bounties
}

pub struct Ledger {
    agent_erg: Vec<u64>,
    cell_erg: Vec<u64>,
    escrow: Vec<u64>,
    held: u64,
    minted: u64,
    burned: u64,
    pub mints: Mints,
    pub burns: Burns,
}

impl Ledger {
    pub fn new(cells: usize, escrows: usize) -> Self {
        Ledger {
            agent_erg: Vec::new(),
            cell_erg: vec![0; cells],
            escrow: vec![0; escrows],
            held: 0,
            minted: 0,
            burned: 0,
            mints: Mints::default(),
            burns: Burns::default(),
        }
    }

    // ---- read ----

    pub fn agent(&self, id: usize) -> u64 {
        self.agent_erg.get(id).copied().unwrap_or(0)
    }
    pub fn cell(&self, c: usize) -> u64 {
        self.cell_erg[c]
    }
    pub fn escrow(&self, slot: usize) -> u64 {
        self.escrow[slot]
    }
    pub fn minted(&self) -> u64 {
        self.minted
    }
    pub fn burned(&self) -> u64 {
        self.burned
    }
    pub fn total_agent_erg(&self) -> u64 {
        self.agent_erg.iter().sum()
    }
    pub fn total_cell_erg(&self) -> u64 {
        self.cell_erg.iter().sum()
    }
    pub fn total_escrow(&self) -> u64 {
        self.escrow.iter().sum()
    }

    /// The conservation invariant. Called every tick; a violation is a bug in
    /// the physics, and the world halts rather than run on counterfeit ergs.
    pub fn conserved(&self) -> bool {
        let held_total = self.total_agent_erg()
            + self.total_cell_erg()
            + self.escrow.iter().sum::<u64>()
            + self.held;
        self.minted == held_total + self.burned
    }

    // ---- mint (the only ways ergs enter) ----

    fn mint(&mut self, amt: u64) {
        self.minted += amt;
    }

    /// Sunlight into a cell, respecting the cap; returns what was minted.
    pub fn mint_sun(&mut self, c: usize, amt: u64, cap: u64) -> u64 {
        let room = cap.saturating_sub(self.cell_erg[c]);
        let amt = amt.min(room);
        self.mint(amt);
        self.mints.sun += amt;
        self.cell_erg[c] += amt;
        amt
    }

    /// A founding or injected agent's endowment.
    pub fn mint_agent(&mut self, id: usize, amt: u64, inject: bool) {
        self.ensure(id);
        self.mint(amt);
        if inject {
            self.mints.inject += amt;
        } else {
            self.mints.genesis += amt;
        }
        self.agent_erg[id] += amt;
    }

    /// An oracle bounty, escrowed.
    pub fn mint_escrow(&mut self, slot: usize, amt: u64) {
        self.mint(amt);
        self.mints.oracle += amt;
        self.escrow[slot] += amt;
    }

    // ---- burn ----

    fn burn_from_agent(&mut self, id: usize, amt: u64) -> u64 {
        let amt = amt.min(self.agent(id));
        self.agent_erg[id] -= amt;
        self.burned += amt;
        amt
    }

    /// Basal metabolism + genome rent. Returns what was actually burned.
    pub fn burn_basal(&mut self, id: usize, amt: u64) -> u64 {
        let b = self.burn_from_agent(id, amt);
        self.burns.basal += b;
        b
    }

    /// Reproduction burn (spawn cost, miscarriage load).
    pub fn burn_spawn(&mut self, id: usize, amt: u64) -> u64 {
        let b = self.burn_from_agent(id, amt);
        self.burns.spawn += b;
        b
    }

    /// An unsolved oracle's escrow returns to entropy.
    pub fn burn_escrow(&mut self, slot: usize) {
        let amt = self.escrow[slot];
        self.escrow[slot] = 0;
        self.burned += amt;
        self.burns.expired += amt;
    }

    // ---- the fuel tank (hold / settle) ----

    /// Escrow an agent's tank for the duration of its run.
    pub fn hold_tank(&mut self, id: usize, cap: u64) -> u64 {
        let tank = self.agent(id).min(cap);
        self.agent_erg[id] -= tank;
        self.held += tank;
        tank
    }

    /// Settle the tank after a run: burn what was used, refund the rest.
    /// A crashed run passes `used = tank` — the whole allotment is forfeit.
    pub fn settle_tank(&mut self, id: usize, tank: u64, used: u64) {
        let used = used.min(tank);
        self.held -= tank;
        self.burned += used;
        self.burns.fuel += used;
        self.agent_erg[id] += tank - used;
    }

    // ---- movement ----

    /// Harvest: cell → agent. Returns what moved.
    pub fn cell_to_agent(&mut self, c: usize, id: usize, amt: u64) -> u64 {
        let amt = amt.min(self.cell_erg[c]);
        self.cell_erg[c] -= amt;
        self.ensure(id);
        self.agent_erg[id] += amt;
        amt
    }

    /// Death detritus: agent → cell (uncapped; the dead don't respect caps).
    pub fn agent_to_cell(&mut self, id: usize, c: usize) -> u64 {
        let amt = self.agent(id);
        self.agent_erg[id] = 0;
        self.cell_erg[c] += amt;
        amt
    }

    /// Transfer between agents with the golden tithe burned. Returns
    /// (delivered, tithe). `amt` is clamped to the payer's balance.
    pub fn transfer(&mut self, from: usize, to: usize, amt: u64) -> (u64, u64) {
        let amt = amt.min(self.agent(from));
        if amt == 0 {
            return (0, 0);
        }
        let tithe = (amt * crate::laws::TITHE_NUM / crate::laws::TITHE_DEN).max(1).min(amt);
        self.agent_erg[from] -= amt;
        self.ensure(to);
        self.agent_erg[to] += amt - tithe;
        self.burned += tithe;
        self.burns.tithe += tithe;
        (amt - tithe, tithe)
    }

    /// Endowment: parent → newborn child, no tithe (blood is thicker).
    pub fn endow(&mut self, parent: usize, child: usize, amt: u64) -> u64 {
        let amt = amt.min(self.agent(parent));
        self.agent_erg[parent] -= amt;
        self.ensure(child);
        self.agent_erg[child] += amt;
        amt
    }

    /// A solved oracle's escrow → the solver.
    pub fn escrow_to_agent(&mut self, slot: usize, id: usize) -> u64 {
        let amt = self.escrow[slot];
        self.escrow[slot] = 0;
        self.ensure(id);
        self.agent_erg[id] += amt;
        amt
    }

    fn ensure(&mut self, id: usize) {
        if id >= self.agent_erg.len() {
            self.agent_erg.resize(id + 1, 0);
        }
    }
}
