//! The observatory's window: a zero-dependency HTTP server over std::net.
//! One thread per connection; the world behind one mutex. The client is a
//! single HTML file compiled into the binary.

use crate::json::{arr, esc, n, obj, s};
use crate::laws::*;
use crate::world::World;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct Control {
    pub paused: AtomicBool,
    pub tps: AtomicU64,
}

const INDEX: &str = include_str!("../web/index.html");

pub fn serve(world: Arc<Mutex<World>>, ctrl: Arc<Control>, port: u16) {
    let listener = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("cannot bind port {port}: {e}");
            std::process::exit(1);
        }
    };
    println!("observatory: http://localhost:{port}");
    for stream in listener.incoming().flatten() {
        let world = world.clone();
        let ctrl = ctrl.clone();
        std::thread::spawn(move || handle(stream, world, ctrl));
    }
}

fn handle(mut stream: TcpStream, world: Arc<Mutex<World>>, ctrl: Arc<Control>) {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 4096];
    // Read until end of headers, then honor Content-Length.
    let (head_end, mut have) = loop {
        match stream.read(&mut tmp) {
            Ok(0) => return,
            Ok(k) => {
                buf.extend_from_slice(&tmp[..k]);
                if let Some(p) = find_headers_end(&buf) {
                    break (p, buf.len());
                }
                if buf.len() > 1 << 20 {
                    return;
                }
            }
            Err(_) => return,
        }
    };
    let head = String::from_utf8_lossy(&buf[..head_end]).to_string();
    let clen = head
        .lines()
        .find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse::<usize>().unwrap_or(0)))
        .unwrap_or(0);
    let body_start = head_end + 4;
    while have < body_start + clen {
        match stream.read(&mut tmp) {
            Ok(0) => break,
            Ok(k) => {
                buf.extend_from_slice(&tmp[..k]);
                have = buf.len();
            }
            Err(_) => return,
        }
    }
    let body = String::from_utf8_lossy(&buf[body_start.min(buf.len())..]).to_string();
    let first = head.lines().next().unwrap_or("");
    let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");
    let (route, query) = match path.split_once('?') {
        Some((r, q)) => (r, q),
        None => (path, ""),
    };

    let (status, ctype, payload) = match (method, route) {
        ("GET", "/") => ("200 OK", "text/html; charset=utf-8", INDEX.to_string()),
        ("GET", "/state") => ("200 OK", "application/json", state_json(&world, &ctrl)),
        ("GET", "/agent") => match qget(query, "id").and_then(|v| v.parse::<usize>().ok()) {
            Some(id) => ("200 OK", "application/json", agent_json(&world, id)),
            None => ("400 Bad Request", "application/json", "{\"err\":\"id?\"}".into()),
        },
        ("GET", "/physics") => ("200 OK", "application/json", physics_json()),
        ("POST", "/inject") => ("200 OK", "application/json", inject(&world, &body)),
        ("POST", "/control") => {
            if let Some(p) = qget(query, "pause") {
                ctrl.paused.store(p == "1", Ordering::Relaxed);
            }
            if let Some(t) = qget(query, "tps").and_then(|v| v.parse::<u64>().ok()) {
                ctrl.tps.store(t.clamp(1, 1000), Ordering::Relaxed);
            }
            ("200 OK", "application/json", "{\"ok\":true}".into())
        }
        _ => ("404 Not Found", "text/plain", "no such path".into()),
    };
    let _ = write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        payload.len()
    );
    let _ = stream.write_all(payload.as_bytes());
}

fn find_headers_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn qget<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=')?;
        (k == key).then_some(v)
    })
}

fn state_json(world: &Arc<Mutex<World>>, ctrl: &Arc<Control>) -> String {
    let w = world.lock().unwrap();
    let agents = w
        .agents
        .iter()
        .filter(|a| a.alive)
        .map(|a| {
            let mut f = vec![
                n("id", a.id),
                n("x", a.x),
                n("y", a.y),
                n("e", w.ledger.agent(a.id)),
                n("lin", a.lineage),
                n("gen", a.generation),
                n("age", w.tick - a.born),
            ];
            if let Some(name) = &a.name {
                f.push(s("name", name));
            }
            obj(f)
        })
        .collect::<Vec<_>>()
        .join(",");
    let oracles = w
        .oracles
        .iter()
        .map(|o| {
            obj(vec![
                n("x", o.cell % GRID),
                n("y", o.cell / GRID),
                n("tier", o.tier),
                n("ttl", o.expires.saturating_sub(w.tick)),
            ])
        })
        .collect::<Vec<_>>()
        .join(",");
    let events =
        w.feed.ring.iter().rev().take(60).map(|e| format!("\"{}\"", esc(&e.line()))).collect::<Vec<_>>().join(",");
    let history = w
        .history
        .iter()
        .map(|(t, p, ae, ce)| format!("[{t},{p},{ae},{ce}]"))
        .collect::<Vec<_>>()
        .join(",");
    let (sx, sy) = w.sun_pos();
    obj(vec![
        n("tick", w.tick),
        s("hash", &format!("{:016x}", w.world_hash)),
        n("seed", w.seed),
        n("era", w.counters.extinctions + 1),
        n("paused", ctrl.paused.load(Ordering::Relaxed)),
        n("tps", ctrl.tps.load(Ordering::Relaxed)),
        n("grid", GRID),
        format!("\"sun\":[{sx},{sy}]"),
        n("pop", w.alive_count()),
        n("born", w.counters.births),
        n("minted", w.ledger.minted()),
        n("burned", w.ledger.burned()),
        n("agentErg", w.ledger.total_agent_erg()),
        n("cellErg", w.ledger.total_cell_erg()),
        n("compost", w.compost.len()),
        format!(
            "\"burns\":{}",
            obj(vec![
                n("fuel", w.ledger.burns.fuel),
                n("basal", w.ledger.burns.basal),
                n("tithe", w.ledger.burns.tithe),
                n("spawn", w.ledger.burns.spawn),
                n("expired", w.ledger.burns.expired),
            ])
        ),
        format!(
            "\"counters\":{}",
            obj(vec![
                n("births", w.counters.births),
                n("miscarriages", w.counters.miscarriages),
                n("starved", w.counters.starved),
                n("predated", w.counters.predated),
                n("aged", w.counters.aged),
                n("crashes", w.counters.crashed_runs),
                n("bites", w.counters.bites),
                n("gifts", w.counters.gifts),
                n("peeks", w.counters.peeks),
                n("extinctions", w.counters.extinctions),
                format!("\"solves\":[{},{},{}]", w.counters.solves[0], w.counters.solves[1], w.counters.solves[2]),
            ])
        ),
        format!("\"cells\":{}", arr((0..CELLS).map(|c| w.ledger.cell(c)))),
        format!("\"scent\":{}", arr(w.scent.iter())),
        format!("\"agents\":[{agents}]"),
        format!("\"oracles\":[{oracles}]"),
        format!("\"events\":[{events}]"),
        format!("\"history\":[{history}]"),
    ])
}

fn agent_json(world: &Arc<Mutex<World>>, id: usize) -> String {
    let w = world.lock().unwrap();
    let Some(a) = w.agents.get(id) else {
        return "{\"err\":\"no such agent\"}".into();
    };
    // Ancestry: walk parents upward (id, mutation, born) — the diff chain.
    let mut chain = Vec::new();
    let mut cur = a.parent;
    while let Some(p) = cur {
        let pa = &w.agents[p];
        chain.push(obj(vec![n("id", pa.id), s("mutation", &pa.mutation), n("born", pa.born), n("alive", pa.alive)]));
        cur = pa.parent;
        if chain.len() >= 16 {
            break;
        }
    }
    let mut f = vec![
        n("id", a.id),
        n("alive", a.alive),
        s("genome", &a.genome),
        s("mutation", &a.mutation),
        n("lineage", a.lineage),
        n("generation", a.generation),
        n("born", a.born),
        n("kids", a.kids),
        n("solved", a.solved),
        n("e", w.ledger.agent(a.id)),
        n("income", a.income),
        n("spent", a.spent),
        format!("\"mem\":{}", arr(a.mem.iter())),
        s("voice", &a.voice),
        format!("\"ancestors\":[{}]", chain.join(",")),
    ];
    if let Some(p) = a.parent {
        f.push(n("parent", p));
        f.push(s("parentGenome", &w.agents[p].genome));
    }
    if let Some(name) = &a.name {
        f.push(s("name", name));
    }
    if let Some(d) = a.died {
        f.push(n("died", d));
        f.push(s("cause", a.cause.unwrap_or("?")));
    }
    if let Some(diag) = &a.last_diag {
        f.push(s("diag", diag));
    }
    obj(f)
}

fn physics_json() -> String {
    obj(vec![
        s("card", &crate::physics_card()),
        s("manifest", &crate::host::manifest()),
        s("scholar", crate::genesis::SCHOLAR),
    ])
}

fn inject(world: &Arc<Mutex<World>>, body: &str) -> String {
    // Body format: first line is the organism's name, the rest is the genome.
    let (name, genome) = match body.split_once('\n') {
        Some((n, g)) => (n.trim(), g),
        None => ("anonymous", body),
    };
    let name = if name.is_empty() { "anonymous" } else { name };
    let mut w = world.lock().unwrap();
    match w.inject(name, genome) {
        Ok(id) => obj(vec![n("ok", true), n("id", id)]),
        Err(e) => obj(vec![n("ok", false), s("err", &e)]),
    }
}
