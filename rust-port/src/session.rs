use std::collections::{HashMap, HashSet};
use std::os::raw::{c_int, c_ulong};

use crate::models::OrderEntry;

pub const MAX_DECOMPRESSED_COOKIE_BYTES: usize = 65536;

#[link(name = "z")]
extern "C" {
    fn compress2(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
        level: c_int,
    ) -> c_int;

    fn uncompress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;
}

fn zlib_compress(input: &[u8]) -> Vec<u8> {
    let mut dest_len: c_ulong = (input.len() as c_ulong) + (input.len() as c_ulong / 10) + 32;
    let mut out = vec![0u8; dest_len as usize];
    let rc = unsafe {
        compress2(
            out.as_mut_ptr(),
            &mut dest_len,
            input.as_ptr(),
            input.len() as c_ulong,
            9,
        )
    };
    if rc == 0 {
        out.truncate(dest_len as usize);
        out
    } else {
        Vec::new()
    }
}

fn zlib_decompress_bounded(input: &[u8]) -> Option<Vec<u8>> {
    let mut dest_len: c_ulong = MAX_DECOMPRESSED_COOKIE_BYTES as c_ulong;
    let mut out = vec![0u8; MAX_DECOMPRESSED_COOKIE_BYTES];
    let rc = unsafe {
        uncompress(
            out.as_mut_ptr(),
            &mut dest_len,
            input.as_ptr(),
            input.len() as c_ulong,
        )
    };
    if rc == 0 {
        out.truncate(dest_len as usize);
        Some(out)
    } else {
        None
    }
}

const B64_URL_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn b64url_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity((data.len() * 4 + 2) / 3);
    let mut i = 0;
    while i + 3 <= data.len() {
        let n = ((data[i] as u32) << 16) | ((data[i + 1] as u32) << 8) | (data[i + 2] as u32);
        out.push(B64_URL_ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(B64_URL_ALPHABET[((n >> 12) & 63) as usize] as char);
        out.push(B64_URL_ALPHABET[((n >> 6) & 63) as usize] as char);
        out.push(B64_URL_ALPHABET[(n & 63) as usize] as char);
        i += 3;
    }
    let rem = data.len() - i;
    if rem == 1 {
        let n = (data[i] as u32) << 16;
        out.push(B64_URL_ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(B64_URL_ALPHABET[((n >> 12) & 63) as usize] as char);
    } else if rem == 2 {
        let n = ((data[i] as u32) << 16) | ((data[i + 1] as u32) << 8);
        out.push(B64_URL_ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(B64_URL_ALPHABET[((n >> 12) & 63) as usize] as char);
        out.push(B64_URL_ALPHABET[((n >> 6) & 63) as usize] as char);
    }
    out
}

fn b64url_val(b: u8) -> Option<u32> {
    match b {
        b'A'..=b'Z' => Some((b - b'A') as u32),
        b'a'..=b'z' => Some((b - b'a' + 26) as u32),
        b'0'..=b'9' => Some((b - b'0' + 52) as u32),
        b'-' | b'+' => Some(62),
        b'_' | b'/' => Some(63),
        _ => None,
    }
}

fn b64url_decode(s: &str) -> Option<Vec<u8>> {
    let clean = s.trim_end_matches('=');
    let bytes = clean.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut i = 0;
    while i + 4 <= bytes.len() {
        let a = b64url_val(bytes[i])?;
        let b = b64url_val(bytes[i + 1])?;
        let c = b64url_val(bytes[i + 2])?;
        let d = b64url_val(bytes[i + 3])?;
        let n = (a << 18) | (b << 12) | (c << 6) | d;
        out.push(((n >> 16) & 0xFF) as u8);
        out.push(((n >> 8) & 0xFF) as u8);
        out.push((n & 0xFF) as u8);
        i += 4;
    }
    let rem = bytes.len() - i;
    if rem == 1 {
        return None;
    } else if rem == 2 {
        let a = b64url_val(bytes[i])?;
        let b = b64url_val(bytes[i + 1])?;
        let n = (a << 18) | (b << 12);
        out.push(((n >> 16) & 0xFF) as u8);
    } else if rem == 3 {
        let a = b64url_val(bytes[i])?;
        let b = b64url_val(bytes[i + 1])?;
        let c = b64url_val(bytes[i + 2])?;
        let n = (a << 18) | (b << 12) | (c << 6);
        out.push(((n >> 16) & 0xFF) as u8);
        out.push(((n >> 8) & 0xFF) as u8);
    }
    Some(out)
}

pub fn encode_v2(lines: &[String]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let raw = lines.join("\n");
    let compressed = zlib_compress(raw.as_bytes());
    if compressed.is_empty() {
        return String::new();
    }
    format!("v2_{}", b64url_encode(&compressed))
}

pub fn decode_v2(token: &str) -> Option<Vec<String>> {
    if !token.starts_with("v2_") {
        return None;
    }
    let payload = &token[3..];
    if payload.is_empty() {
        return Some(Vec::new());
    }
    let compressed = b64url_decode(payload)?;
    let raw = zlib_decompress_bounded(&compressed)?;
    let text = String::from_utf8(raw).ok()?;
    Some(
        text.split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect(),
    )
}

pub fn encode_session_cookies(
    order: &[OrderEntry],
    steps: &HashMap<String, bool>,
) -> (String, String) {
    let order_lines: Vec<String> = order
        .iter()
        .filter(|i| !i.item_id.is_empty())
        .map(|i| {
            if i.is_collected {
                format!("{}:1", i.item_id)
            } else {
                i.item_id.clone()
            }
        })
        .collect();
    let mut step_lines: Vec<String> = steps
        .iter()
        .filter(|(k, v)| !k.is_empty() && **v)
        .map(|(k, _)| k.clone())
        .collect();
    step_lines.sort();
    (encode_v2(&order_lines), encode_v2(&step_lines))
}

pub fn decode_session_cookies(
    raw_order: &str,
    raw_steps: &str,
) -> (Vec<OrderEntry>, HashMap<String, bool>) {
    let mut order = Vec::new();
    let mut seen = HashSet::new();
    if let Some(lines) = decode_v2(raw_order) {
        for entry in lines {
            let (i_id, collected) = if let Some(idx) = entry.rfind(':') {
                (&entry[..idx], &entry[idx + 1..] == "1")
            } else {
                (entry.as_str(), false)
            };
            if !i_id.is_empty() && seen.insert(i_id.to_string()) {
                order.push(OrderEntry {
                    item_id: i_id.to_string(),
                    is_collected: collected,
                });
            }
        }
    }

    let mut steps = HashMap::new();
    if let Some(lines) = decode_v2(raw_steps) {
        for k in lines {
            if !k.is_empty() {
                steps.insert(k, true);
            }
        }
    }

    (order, steps)
}

pub fn export_json(order: &[OrderEntry], steps: &HashMap<String, bool>) -> String {
    let mut buf = String::from("{\"app\":\"elden-ring-current-run-roadmap\",\"version\":1,\"current_run_order\":[");
    for (idx, item) in order.iter().enumerate() {
        if idx > 0 {
            buf.push(',');
        }
        buf.push_str(&format!(
            "{{\"item_id\":\"{}\",\"is_collected\":{}}}",
            item.item_id,
            if item.is_collected { 1 } else { 0 }
        ));
    }
    buf.push_str("],\"step_progress\":[");
    let mut first = true;
    for (k, v) in steps {
        if *v {
            if !first {
                buf.push(',');
            }
            first = false;
            buf.push_str(&format!("{{\"step_key\":\"{}\",\"is_done\":1}}", k));
        }
    }
    buf.push_str("]}");
    buf
}
