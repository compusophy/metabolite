use metabolite::laws::*;
use metabolite::world::World;

fn main() {
    let mut w = World::new(5);
    let monk: String = metabolite::genesis::EMPIRICIST.lines()
        .filter(|l| !l.contains("spawn") && !l.contains("give")).collect::<Vec<_>>().join("\n");
    let teacher = w.inject("teacher", &monk).unwrap();
    w.ledger.mint_agent(teacher, 50_000, true);
    let ocell = w.oracles.iter().find(|o| o.tier == 2 && w.occ[o.cell].is_none()).unwrap().cell;
    let slot = w.oracles.iter().position(|o| o.cell == ocell).unwrap();
    let correct = oracle_f(2, w.oracles[slot].x_val);
    let old = World::cell_of(w.agents[teacher].x, w.agents[teacher].y);
    w.occ[old] = None; w.agents[teacher].x = ocell % GRID; w.agents[teacher].y = ocell / GRID; w.occ[ocell] = Some(teacher);
    let thief_cell = World::cell_of((ocell % GRID + 1) % GRID, ocell / GRID);
    let sniper: String = metabolite::genesis::PLAGIARIST.lines()
        .filter(|l| !l.contains("spawn")).collect::<Vec<_>>().join("\n");
    let thief = w.inject("thief", &sniper).unwrap();
    w.ledger.mint_agent(thief, 5_000, true);
    let told = World::cell_of(w.agents[thief].x, w.agents[thief].y);
    w.occ[told] = None; w.agents[thief].x = thief_cell % GRID; w.agents[thief].y = thief_cell / GRID; w.occ[thief_cell] = Some(thief);
    println!("correct={correct} escrow={}", w.ledger.escrow(slot));
    for t in 0..90 {
        w.tick();
        let te = &w.agents[teacher]; let th = &w.agents[thief];
        if t % 5 == 0 {
            println!("t{t} T(mem5={} mem1={} alive={}) TH(income={} alive={} at={},{}) escrow={} solves={:?} peeks={}",
                te.mem[5], te.mem[1], te.alive, th.income, th.alive, th.x, th.y, w.ledger.escrow(slot), w.counters.solves, w.counters.peeks);
        }
        if w.counters.solves[2] > 0 { println!("POT TAKEN at t{t}"); return; }
    }
}
