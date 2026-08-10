//! The wasm ABI: hand-rolled C exports, no bindgen, no dependencies —
//! the whole world compiled into a page. JS instantiates the module,
//! drives ticks on a timer, and reads JSON through one out-buffer; the
//! dashboard's fetch() calls are shimmed onto these exports, so the same
//! index.html observes a server world or an in-page world identically.

use crate::api;
use crate::world::World;
use std::sync::Mutex;

static WORLD: Mutex<Option<World>> = Mutex::new(None);
static OUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static IN: Mutex<Vec<u8>> = Mutex::new(Vec::new());

fn put(s: String) -> u32 {
    let mut o = OUT.lock().unwrap();
    *o = s.into_bytes();
    o.len() as u32
}

#[no_mangle]
pub extern "C" fn mb_new(seed: u64) {
    *WORLD.lock().unwrap() = Some(World::new(seed));
}

#[no_mangle]
pub extern "C" fn mb_tick(n: u32) {
    if let Some(w) = WORLD.lock().unwrap().as_mut() {
        for _ in 0..n {
            w.tick();
        }
    }
}

#[no_mangle]
pub extern "C" fn mb_state(paused: u32, tps: u32) -> u32 {
    match WORLD.lock().unwrap().as_ref() {
        Some(w) => put(api::state_json(w, paused != 0, tps as u64)),
        None => put("{\"err\":\"no world\"}".into()),
    }
}

#[no_mangle]
pub extern "C" fn mb_agent(id: u32) -> u32 {
    match WORLD.lock().unwrap().as_ref() {
        Some(w) => put(api::agent_json(w, id as usize)),
        None => put("{\"err\":\"no world\"}".into()),
    }
}

#[no_mangle]
pub extern "C" fn mb_physics() -> u32 {
    put(api::physics_json())
}

/// Reserve the input buffer; JS writes UTF-8 bytes at the returned pointer.
#[no_mangle]
pub extern "C" fn mb_in_alloc(len: u32) -> *mut u8 {
    let mut i = IN.lock().unwrap();
    i.resize(len as usize, 0);
    i.as_mut_ptr()
}

/// Inject the organism described by the input buffer (name\ngenome).
#[no_mangle]
pub extern "C" fn mb_inject(len: u32) -> u32 {
    let body = {
        let i = IN.lock().unwrap();
        String::from_utf8_lossy(&i[..(len as usize).min(i.len())]).to_string()
    };
    match WORLD.lock().unwrap().as_mut() {
        Some(w) => put(api::inject_json(w, &body)),
        None => put("{\"err\":\"no world\"}".into()),
    }
}

#[no_mangle]
pub extern "C" fn mb_out_ptr() -> *const u8 {
    OUT.lock().unwrap().as_ptr()
}
