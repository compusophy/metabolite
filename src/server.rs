//! The observatory's window: a zero-dependency HTTP server over std::net.
//! One thread per connection; the world behind one mutex; every response
//! built by the pure api module. The wasm build replaces this whole file
//! with an in-page ABI (wasm.rs) over the same api.

use crate::api;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct Control {
    pub paused: AtomicBool,
    pub tps: AtomicU64,
}

const INDEX: &str = include_str!("../web/index.html");

pub fn serve(world: Arc<Mutex<crate::world::World>>, ctrl: Arc<Control>, port: u16) {
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

fn handle(mut stream: TcpStream, world: Arc<Mutex<crate::world::World>>, ctrl: Arc<Control>) {
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
        ("GET", "/state") => {
            let w = world.lock().unwrap();
            let payload = api::state_json(
                &w,
                ctrl.paused.load(Ordering::Relaxed),
                ctrl.tps.load(Ordering::Relaxed),
            );
            ("200 OK", "application/json", payload)
        }
        ("GET", "/agent") => match qget(query, "id").and_then(|v| v.parse::<usize>().ok()) {
            Some(id) => ("200 OK", "application/json", api::agent_json(&world.lock().unwrap(), id)),
            None => ("400 Bad Request", "application/json", "{\"err\":\"id?\"}".into()),
        },
        ("GET", "/physics") => ("200 OK", "application/json", api::physics_json()),
        ("POST", "/inject") => {
            ("200 OK", "application/json", api::inject_json(&mut world.lock().unwrap(), &body))
        }
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
