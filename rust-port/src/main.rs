mod compiler;
mod generated_catalog;
mod models;
mod offline_assets;
mod session;

pub use offline_assets::{INLINE_EMBLEM_SVG, OFFLINE_RUNTIME_JS, OFFLINE_STYLESHEET};

pub mod views {
    include!(concat!(env!("OUT_DIR"), "/generated_views.rs"));
}

use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

use compiler::{
    compile_run_plan, find_catalog_item, search_catalog, toggle_order_item, toggle_step_key,
};
use generated_catalog::CATALOG;
use models::OrderEntry;
use session::{decode_session_cookies, encode_session_cookies, export_json};

extern "C" {
    fn launch_native_macos_app();
}

static FAVICON_SVG: &[u8] = include_bytes!("../../static/favicon.svg");
static FAVICON_32: &[u8] = include_bytes!("../../static/favicon-32.png");
static APPLE_TOUCH_ICON: &[u8] = include_bytes!("../../static/apple-touch-icon.png");
static ICON_192: &[u8] = include_bytes!("../../static/icon-192.png");
static ICON_512: &[u8] = include_bytes!("../../static/icon-512.png");
static WEBMANIFEST: &[u8] = include_bytes!("../../static/site.webmanifest");

struct AppState {
    order: Vec<OrderEntry>,
    steps: HashMap<String, bool>,
}

static NATIVE_STATE: Mutex<Option<AppState>> = Mutex::new(None);

fn state_file_path() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let dir = PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("ER-Roadmap");
        let _ = fs::create_dir_all(&dir);
        dir.join("state.v2")
    } else {
        PathBuf::from("er-roadmap-state.v2")
    }
}

fn load_native_disk_state() -> AppState {
    let path = state_file_path();
    if let Ok(content) = fs::read_to_string(&path) {
        let mut lines = content.lines();
        let raw_order = lines.next().unwrap_or("");
        let raw_steps = lines.next().unwrap_or("");
        let (order, steps) = decode_session_cookies(raw_order, raw_steps);
        return AppState { order, steps };
    }
    AppState {
        order: Vec::new(),
        steps: HashMap::new(),
    }
}

fn save_native_disk_state(state: &AppState) {
    let (c_order, c_steps) = encode_session_cookies(&state.order, &state.steps);
    let path = state_file_path();
    let _ = fs::write(path, format!("{}\n{}\n", c_order, c_steps));
}

fn with_native_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut AppState) -> R,
{
    let mut guard = NATIVE_STATE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(load_native_disk_state());
    }
    let state = guard.as_mut().unwrap();
    f(state)
}

fn into_c_ptr(s: String) -> *mut c_char {
    CString::new(s)
        .unwrap_or_else(|_| CString::new("").unwrap())
        .into_raw()
}

#[no_mangle]
pub extern "C" fn rust_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_get_initial_html() -> *mut c_char {
    let html = with_native_state(|state| {
        let plan = compile_run_plan(&state.order, &state.steps);
        let mut buf = String::with_capacity(49152);
        views::render_index(&mut buf, &plan);
        buf
    });
    into_c_ptr(html)
}

#[no_mangle]
pub extern "C" fn rust_export_json() -> *mut c_char {
    let json = with_native_state(|state| export_json(&state.order, &state.steps));
    into_c_ptr(json)
}

fn parse_imported_json(raw: &str) -> (Vec<OrderEntry>, HashMap<String, bool>) {
    let mut order = Vec::new();
    let mut seen = HashSet::new();
    let mut steps = HashMap::new();

    // Lightweight zero-dependency scanner for exported roadmap JSON
    if let Some(order_idx) = raw.find("\"current_run_order\"") {
        let after = &raw[order_idx..];
        let end_idx = after.find(']').unwrap_or(after.len());
        let block = &after[..end_idx];
        for obj in block.split('{').skip(1) {
            if let Some(id_pos) = obj.find("\"item_id\"") {
                let rest = &obj[id_pos + 9..];
                if let Some(q1) = rest.find('"') {
                    let rest2 = &rest[q1 + 1..];
                    if let Some(q2) = rest2.find('"') {
                        let item_id = &rest2[..q2];
                        let is_col = obj.contains("\"is_collected\":1")
                            || obj.contains("\"is_collected\": 1")
                            || obj.contains("\"is_collected\":true")
                            || obj.contains("\"is_collected\": true");
                        if find_catalog_item(item_id).is_some() && seen.insert(item_id.to_string())
                        {
                            order.push(OrderEntry {
                                item_id: item_id.to_string(),
                                is_collected: is_col,
                            });
                        }
                    }
                }
            }
        }
    }

    if let Some(steps_idx) = raw.find("\"step_progress\"") {
        let after = &raw[steps_idx..];
        let end_idx = after.find(']').unwrap_or(after.len());
        let block = &after[..end_idx];
        for obj in block.split('{').skip(1) {
            if let Some(sk_pos) = obj.find("\"step_key\"") {
                let rest = &obj[sk_pos + 10..];
                if let Some(q1) = rest.find('"') {
                    let rest2 = &rest[q1 + 1..];
                    if let Some(q2) = rest2.find('"') {
                        let s_key = &rest2[..q2];
                        let is_done = obj.contains("\"is_done\":1")
                            || obj.contains("\"is_done\": 1")
                            || obj.contains("\"is_done\":true")
                            || obj.contains("\"is_done\": true");
                        if !s_key.is_empty() && is_done {
                            steps.insert(s_key.to_string(), true);
                        }
                    }
                }
            }
        }
    }

    (order, steps)
}

#[no_mangle]
pub extern "C" fn rust_import_json(json_cstr: *const c_char) -> *mut c_char {
    let raw = if json_cstr.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(json_cstr).to_str().unwrap_or("") }
    };
    let html = with_native_state(|state| {
        let (new_order, new_steps) = parse_imported_json(raw);
        if !new_order.is_empty() || raw.contains("\"current_run_order\"") {
            state.order = new_order;
            state.steps = new_steps;
            save_native_disk_state(state);
        }
        let plan = compile_run_plan(&state.order, &state.steps);
        let mut buf = String::with_capacity(32768);
        views::render_workspace(&mut buf, &plan);
        buf
    });
    into_c_ptr(html)
}

#[no_mangle]
pub extern "C" fn rust_handle_ipc(
    action_cstr: *const c_char,
    payload_cstr: *const c_char,
) -> *mut c_char {
    let action = if action_cstr.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(action_cstr).to_str().unwrap_or("") }
    };
    let payload = if payload_cstr.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(payload_cstr).to_str().unwrap_or("") }
    };

    let html = with_native_state(|state| match action {
        "/search" => {
            let q = payload.trim();
            if q.is_empty() {
                return String::new();
            }
            let rows = search_catalog(q, 8);
            let ordered_ids: HashSet<String> =
                state.order.iter().map(|i| i.item_id.clone()).collect();
            let mut buf = String::with_capacity(8192);
            views::render_search_results(&mut buf, &rows, &ordered_ids);
            buf
        }
        "/order/add" => {
            let form = parse_kv_pairs(payload, '&');
            if let Some(item_id) = form.get("item_id") {
                if !state.order.iter().any(|i| &i.item_id == item_id) {
                    state.order.push(OrderEntry {
                        item_id: item_id.clone(),
                        is_collected: false,
                    });
                    save_native_disk_state(state);
                }
            }
            let plan = compile_run_plan(&state.order, &state.steps);
            let mut buf = String::with_capacity(32768);
            views::render_workspace(&mut buf, &plan);
            buf
        }
        "/order/remove" => {
            let form = parse_kv_pairs(payload, '&');
            if let Some(item_id) = form.get("item_id") {
                state.order.retain(|i| &i.item_id != item_id);
                save_native_disk_state(state);
            }
            let plan = compile_run_plan(&state.order, &state.steps);
            let mut buf = String::with_capacity(32768);
            views::render_workspace(&mut buf, &plan);
            buf
        }
        "/order/clear" => {
            state.order.clear();
            state.steps.clear();
            save_native_disk_state(state);
            let plan = compile_run_plan(&state.order, &state.steps);
            let mut buf = String::with_capacity(32768);
            views::render_workspace(&mut buf, &plan);
            buf
        }
        "/order/toggle" => {
            let form = parse_kv_pairs(payload, '&');
            if let Some(item_id) = form.get("item_id") {
                toggle_order_item(&mut state.order, &mut state.steps, item_id);
                save_native_disk_state(state);
            }
            let plan = compile_run_plan(&state.order, &state.steps);
            let mut buf = String::with_capacity(32768);
            views::render_workspace(&mut buf, &plan);
            buf
        }
        "/step/toggle" => {
            let form = parse_kv_pairs(payload, '&');
            if let Some(step_key) = form.get("step_key") {
                toggle_step_key(&mut state.order, &mut state.steps, step_key);
                save_native_disk_state(state);
            }
            let plan = compile_run_plan(&state.order, &state.steps);
            let mut buf = String::with_capacity(32768);
            views::render_workspace(&mut buf, &plan);
            buf
        }
        _ => String::new(),
    });

    into_c_ptr(html)
}

fn url_decode(s: &str) -> String {
    let mut bytes = Vec::with_capacity(s.len());
    let raw = s.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'+' {
            bytes.push(b' ');
            i += 1;
        } else if raw[i] == b'%' && i + 2 < raw.len() {
            if let Ok(hex_str) = std::str::from_utf8(&raw[i + 1..i + 3]) {
                if let Ok(val) = u8::from_str_radix(hex_str, 16) {
                    bytes.push(val);
                    i += 3;
                    continue;
                }
            }
            bytes.push(raw[i]);
            i += 1;
        } else {
            bytes.push(raw[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn parse_kv_pairs(s: &str, sep: char) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for part in s.split(sep) {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(eq_idx) = trimmed.find('=') {
            let k = url_decode(&trimmed[..eq_idx]);
            let v = url_decode(trimmed[eq_idx + 1..].trim_matches('"'));
            map.insert(k, v);
        }
    }
    map
}

fn send_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    extra_headers: &[(&str, String)],
    body: &[u8],
) {
    let mut header = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store, no-cache, must-revalidate, max-age=0\r\nPragma: no-cache\r\nVary: HX-Request, Cookie\r\nConnection: close\r\n",
        status,
        content_type,
        body.len()
    );
    for (k, v) in extra_headers {
        header.push_str(&format!("{}: {}\r\n", k, v));
    }
    header.push_str("\r\n");
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
}

fn cookie_headers(
    order: &[OrderEntry],
    steps: &HashMap<String, bool>,
) -> Vec<(&'static str, String)> {
    let (c_order, c_steps) = encode_session_cookies(order, steps);
    let max_age = 86400 * 30;
    vec![
        (
            "Set-Cookie",
            format!(
                "er_order={}; HttpOnly; Max-Age={}; Path=/; SameSite=Lax",
                c_order, max_age
            ),
        ),
        (
            "Set-Cookie",
            format!(
                "er_steps={}; HttpOnly; Max-Age={}; Path=/; SameSite=Lax",
                c_steps, max_age
            ),
        ),
    ]
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 65536];
    let n = match stream.read(&mut buf) {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    let req_str = String::from_utf8_lossy(&buf[..n]);
    let header_end = match req_str.find("\r\n\r\n") {
        Some(idx) => idx,
        None => return,
    };
    let head = &req_str[..header_end];
    let body = &req_str[header_end + 4..];

    let mut lines = head.lines();
    let first_line = match lines.next() {
        Some(l) => l,
        None => return,
    };
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let full_path = parts.next().unwrap_or("/");

    let (path, query_str) = match full_path.find('?') {
        Some(idx) => (&full_path[..idx], &full_path[idx + 1..]),
        None => (full_path, ""),
    };

    let mut cookies = HashMap::new();
    for line in lines {
        if let Some(colon) = line.find(':') {
            let k = line[..colon].trim().to_ascii_lowercase();
            let v = line[colon + 1..].trim();
            if k == "cookie" {
                for (ck, cv) in parse_kv_pairs(v, ';') {
                    cookies.insert(ck, cv);
                }
            }
        }
    }

    let raw_order = cookies.get("er_order").map(|s| s.as_str()).unwrap_or("");
    let raw_steps = cookies.get("er_steps").map(|s| s.as_str()).unwrap_or("");
    let (mut order, mut steps) = decode_session_cookies(raw_order, raw_steps);

    match (method, path) {
        ("GET", "/favicon.ico") | ("GET", "/static/favicon-32.png") => {
            send_response(&mut stream, "200 OK", "image/png", &[], FAVICON_32);
        }
        ("GET", "/static/favicon.svg") => {
            send_response(&mut stream, "200 OK", "image/svg+xml", &[], FAVICON_SVG);
        }
        ("GET", "/apple-touch-icon.png")
        | ("GET", "/apple-touch-icon-precomposed.png")
        | ("GET", "/static/apple-touch-icon.png") => {
            send_response(&mut stream, "200 OK", "image/png", &[], APPLE_TOUCH_ICON);
        }
        ("GET", "/static/icon-192.png") => {
            send_response(&mut stream, "200 OK", "image/png", &[], ICON_192);
        }
        ("GET", "/static/icon-512.png") => {
            send_response(&mut stream, "200 OK", "image/png", &[], ICON_512);
        }
        ("GET", "/static/site.webmanifest") => {
            send_response(
                &mut stream,
                "200 OK",
                "application/manifest+json",
                &[],
                WEBMANIFEST,
            );
        }
        ("GET", "/") => {
            let plan = compile_run_plan(&order, &steps);
            let mut html = String::with_capacity(49152);
            views::render_index(&mut html, &plan);
            let headers = cookie_headers(&order, &steps);
            send_response(
                &mut stream,
                "200 OK",
                "text/html; charset=utf-8",
                &headers,
                html.as_bytes(),
            );
        }
        ("GET", "/search") => {
            let q_map = parse_kv_pairs(query_str, '&');
            let q = q_map.get("q").map(|s| s.as_str()).unwrap_or("");
            if q.trim().is_empty() {
                send_response(&mut stream, "200 OK", "text/html; charset=utf-8", &[], b"");
                return;
            }
            let rows = search_catalog(q, 8);
            let ordered_ids: HashSet<String> = order.iter().map(|i| i.item_id.clone()).collect();
            let mut html = String::with_capacity(8192);
            views::render_search_results(&mut html, &rows, &ordered_ids);
            send_response(
                &mut stream,
                "200 OK",
                "text/html; charset=utf-8",
                &[],
                html.as_bytes(),
            );
        }
        ("GET", "/export") => {
            let payload = export_json(&order, &steps);
            let headers = vec![(
                "Content-Disposition",
                String::from("attachment; filename=\"elden-ring-roadmap.json\""),
            )];
            send_response(
                &mut stream,
                "200 OK",
                "application/json",
                &headers,
                payload.as_bytes(),
            );
        }
        ("POST", "/order/add") => {
            let form = parse_kv_pairs(body, '&');
            if let Some(item_id) = form.get("item_id") {
                if !order.iter().any(|i| &i.item_id == item_id) {
                    order.push(OrderEntry {
                        item_id: item_id.clone(),
                        is_collected: false,
                    });
                }
            }
            render_and_send_workspace(&mut stream, &order, &steps);
        }
        ("POST", "/order/remove") => {
            let form = parse_kv_pairs(body, '&');
            if let Some(item_id) = form.get("item_id") {
                order.retain(|i| &i.item_id != item_id);
            }
            render_and_send_workspace(&mut stream, &order, &steps);
        }
        ("POST", "/order/clear") => {
            order.clear();
            steps.clear();
            render_and_send_workspace(&mut stream, &order, &steps);
        }
        ("POST", "/order/toggle") => {
            let form = parse_kv_pairs(body, '&');
            if let Some(item_id) = form.get("item_id") {
                toggle_order_item(&mut order, &mut steps, item_id);
            }
            render_and_send_workspace(&mut stream, &order, &steps);
        }
        ("POST", "/step/toggle") => {
            let form = parse_kv_pairs(body, '&');
            if let Some(step_key) = form.get("step_key") {
                toggle_step_key(&mut order, &mut steps, step_key);
            }
            render_and_send_workspace(&mut stream, &order, &steps);
        }
        _ => {
            send_response(&mut stream, "404 Not Found", "text/plain", &[], b"Not Found");
        }
    }
}

fn render_and_send_workspace(
    stream: &mut TcpStream,
    order: &[OrderEntry],
    steps: &HashMap<String, bool>,
) {
    let plan = compile_run_plan(order, steps);
    let mut html = String::with_capacity(24576);
    views::render_workspace(&mut html, &plan);
    let mut headers = cookie_headers(order, steps);
    headers.push(("HX-Trigger", String::from("refreshSearch")));
    send_response(
        stream,
        "200 OK",
        "text/html; charset=utf-8",
        &headers,
        html.as_bytes(),
    );
}

fn run_benchmarks() {
    println!("=========================================================================");
    println!("🦀 Elden Ring Roadmap — Single-Binary Rust + AOT JSX Compiler Benchmark");
    println!("=========================================================================");
    println!(
        "Embedded in .rodata: {} Catalog Items | {} Missable Rules | 10 PONR Gates",
        CATALOG.len(),
        CATALOG.iter().filter(|i| i.is_missable).count()
    );

    let missable_items: Vec<&models::CatalogItem> =
        CATALOG.iter().filter(|i| i.is_missable).collect();

    let iters = 10_000;
    let t0 = Instant::now();
    let mut search_hits = 0usize;
    for _ in 0..iters {
        let res = search_catalog("magic scorpion", 8);
        search_hits += res.len();
    }
    let search_ns = t0.elapsed().as_nanos() as f64 / iters as f64;

    let order_9: Vec<OrderEntry> = missable_items[..9]
        .iter()
        .map(|i| OrderEntry {
            item_id: i.item_id.to_string(),
            is_collected: false,
        })
        .collect();
    let steps_empty: HashMap<String, bool> = HashMap::new();

    let t1 = Instant::now();
    let mut html_bytes_9 = 0usize;
    let mut buf = String::with_capacity(32768);
    for _ in 0..iters {
        buf.clear();
        let plan = compile_run_plan(&order_9, &steps_empty);
        views::render_workspace(&mut buf, &plan);
        html_bytes_9 = buf.len();
    }
    let dag_render_9_ns = t1.elapsed().as_nanos() as f64 / iters as f64;

    let order_68: Vec<OrderEntry> = missable_items
        .iter()
        .map(|i| OrderEntry {
            item_id: i.item_id.to_string(),
            is_collected: false,
        })
        .collect();

    let n_68 = 2_000;
    let t2 = Instant::now();
    let mut html_bytes_68 = 0usize;
    for _ in 0..n_68 {
        buf.clear();
        let plan = compile_run_plan(&order_68, &steps_empty);
        views::render_workspace(&mut buf, &plan);
        html_bytes_68 = buf.len();
    }
    let dag_render_68_ns = t2.elapsed().as_nanos() as f64 / n_68 as f64;

    let t3 = Instant::now();
    let mut cookie_len = 0usize;
    for _ in 0..n_68 {
        let (c_o, c_s) = encode_session_cookies(&order_9, &steps_empty);
        cookie_len = c_o.len();
        let (dec_o, _) = decode_session_cookies(&c_o, &c_s);
        assert_eq!(dec_o.len(), 9);
    }
    let cookie_rt_ns = t3.elapsed().as_nanos() as f64 / n_68 as f64;

    println!("-------------------------------------------------------------------------");
    println!(
        "1. In-RAM Prefix Search (`GET /search?q=magic+scorpion`) : {:>7.2} µs ({:.0} ops/sec, {} matches)",
        search_ns / 1000.0,
        1_000_000_000.0 / search_ns,
        search_hits / iters
    );
    println!(
        "2. 9-Item DAG Solve + AOT TSX Render ({} B HTML)     : {:>7.2} µs ({:.0} req/sec)",
        html_bytes_9,
        dag_render_9_ns / 1000.0,
        1_000_000_000.0 / dag_render_9_ns
    );
    println!(
        "3. All 68 Missables DAG + AOT TSX Render ({} B HTML) : {:>7.2} µs ({:.0} req/sec)",
        html_bytes_68,
        dag_render_68_ns / 1000.0,
        1_000_000_000.0 / dag_render_68_ns
    );
    println!(
        "4. Cookie v2_ zlib+base64url Roundtrip ({} B token)     : {:>7.2} µs ({:.0} ops/sec)",
        cookie_len,
        cookie_rt_ns / 1000.0,
        1_000_000_000.0 / cookie_rt_ns
    );
    println!("=========================================================================");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--bench") {
        run_benchmarks();
        return;
    }

    if args.iter().any(|a| a == "--server") {
        let addr = env::var("BIND_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
        let listener = TcpListener::bind(&addr).expect("Failed to bind address");
        println!(
            "🦀 Elden Ring Roadmap (HTTP Server Mode) listening on http://{}",
            addr
        );
        for stream in listener.incoming() {
            if let Ok(s) = stream {
                thread::spawn(move || {
                    handle_client(s);
                });
            }
        }
        return;
    }

    // Default: Launch Standalone Offline Native macOS Desktop App (Cocoa + WKWebView, zero network)
    unsafe {
        launch_native_macos_app();
    }
}
