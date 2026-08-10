//! A JSON writer the size of the need. No serde in a world this small.

pub fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// `"key":"escaped"` — string field.
pub fn s(key: &str, v: &str) -> String {
    format!("\"{key}\":\"{}\"", esc(v))
}

/// `"key":123` — numeric field (anything Display that is JSON-legal).
pub fn n<T: std::fmt::Display>(key: &str, v: T) -> String {
    format!("\"{key}\":{v}")
}

pub fn obj(fields: Vec<String>) -> String {
    format!("{{{}}}", fields.join(","))
}

pub fn arr<T: std::fmt::Display>(vals: impl Iterator<Item = T>) -> String {
    format!("[{}]", vals.map(|v| v.to_string()).collect::<Vec<_>>().join(","))
}
