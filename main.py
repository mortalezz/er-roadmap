import os
import subprocess
import base64
import json
import sqlite3
import zlib
from typing import Dict, Any, List, Tuple
from fastapi import FastAPI, Request, Form, UploadFile, File
from fastapi.responses import HTMLResponse, JSONResponse
from fastapi.templating import Jinja2Templates

from seed_db import DB_PATH, init_db
from compiler import compile_run_plan

DIST_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "dist")
if not os.path.exists(DIST_DIR):
    build_script = os.path.join(os.path.dirname(os.path.abspath(__file__)), "build_dist.py")
    subprocess.run(["python3", build_script], check=True)

app = FastAPI(title="Elden Ring Current Run Roadmap")
templates = Jinja2Templates(directory="templates")

NO_CACHE_HEADERS = {
    "Cache-Control": "no-store, no-cache, must-revalidate, max-age=0",
    "Pragma": "no-cache",
    "Vary": "HX-Request, Cookie",
}

MAX_DECOMPRESSED_COOKIE_BYTES = 65536


def get_conn() -> sqlite3.Connection:
    if not DB_PATH.exists():
        init_db()
    return sqlite3.connect(DB_PATH)


def _encode_v2(lines: List[str]) -> str:
    if not lines:
        return ""
    raw = "\n".join(lines).encode("utf-8")
    compressed = zlib.compress(raw, level=9)
    b64 = base64.urlsafe_b64encode(compressed).decode("ascii").rstrip("=")
    return f"v2_{b64}"


def _decode_v2(token: str) -> List[str]:
    payload = token[3:]
    if not payload:
        return []
    padded = payload + "=" * (-len(payload) % 4)
    compressed = base64.urlsafe_b64decode(padded.encode("ascii"))
    dobj = zlib.decompressobj()
    raw = dobj.decompress(compressed, MAX_DECOMPRESSED_COOKIE_BYTES)
    if dobj.unconsumed_tail:
        raise ValueError("Cookie payload exceeds maximum decompressed size")
    text = raw.decode("utf-8")
    return [line for line in text.split("\n") if line]


def load_session(request: Request) -> Tuple[List[Dict[str, Any]], Dict[str, bool]]:
    """Loads and validates current run order and step progress from request cookies (supports v2 compressed & v1 JSON)."""
    raw_order = request.cookies.get("er_order", "")
    raw_steps = request.cookies.get("er_steps", "")

    order: List[Dict[str, Any]] = []
    seen_ids = set()
    if raw_order:
        try:
            if raw_order.startswith("v2_"):
                for entry in _decode_v2(raw_order):
                    if ":" in entry:
                        i_id, flag = entry.rsplit(":", 1)
                        collected = 1 if flag == "1" else 0
                    else:
                        i_id, collected = entry, 0
                    if i_id and i_id not in seen_ids:
                        seen_ids.add(i_id)
                        order.append({"item_id": i_id, "is_collected": collected})
            else:
                parsed = json.loads(raw_order)
                if isinstance(parsed, list):
                    for item in parsed:
                        if isinstance(item, dict):
                            i_id = item.get("item_id")
                            if isinstance(i_id, str) and i_id and i_id not in seen_ids:
                                seen_ids.add(i_id)
                                order.append({"item_id": i_id, "is_collected": 1 if item.get("is_collected") else 0})
        except Exception:
            order = []

    steps: Dict[str, bool] = {}
    if raw_steps:
        try:
            if raw_steps.startswith("v2_"):
                for s_key in _decode_v2(raw_steps):
                    steps[s_key] = True
            else:
                parsed_steps = json.loads(raw_steps)
                if isinstance(parsed_steps, dict):
                    for k, v in parsed_steps.items():
                        if isinstance(k, str) and k and bool(v):
                            steps[k] = True
        except Exception:
            steps = {}

    return order, steps


def save_session(response: HTMLResponse, order: List[Dict[str, Any]], steps: Dict[str, bool]):
    """Saves current run order and step progress into compact, unescaped v2 browser cookies."""
    order_lines = [
        f"{i['item_id']}:1" if i.get("is_collected") else str(i["item_id"])
        for i in order
        if isinstance(i, dict) and i.get("item_id")
    ]
    step_lines = [str(k) for k, v in steps.items() if k and v]
    response.set_cookie(
        key="er_order",
        value=_encode_v2(order_lines),
        httponly=True,
        samesite="lax",
        max_age=86400 * 30,
    )
    response.set_cookie(
        key="er_steps",
        value=_encode_v2(step_lines),
        httponly=True,
        samesite="lax",
        max_age=86400 * 30,
    )


def render_workspace(request: Request, order: List[Dict[str, Any]], steps: Dict[str, bool]) -> HTMLResponse:
    with get_conn() as conn:
        state = compile_run_plan(conn, order, steps)
    resp = templates.TemplateResponse(
        request=request,
        name="partials/workspace.html",
        context=state,
        headers={**NO_CACHE_HEADERS, "HX-Trigger": "refreshSearch"},
    )
    save_session(resp, order, steps)
    return resp


@app.get("/workspace", response_class=HTMLResponse)
async def workspace_endpoint(request: Request):
    order, steps = load_session(request)
    return render_workspace(request, order, steps)


@app.get("/export")
async def export_roadmap(request: Request):
    order, steps = load_session(request)
    payload = {
        "app": "elden-ring-current-run-roadmap",
        "version": 1,
        "current_run_order": order,
        "step_progress": [{"step_key": k, "is_done": 1 if v else 0} for k, v in steps.items() if v],
    }
    return JSONResponse(
        content=payload,
        headers={
            **NO_CACHE_HEADERS,
            "Content-Disposition": 'attachment; filename="elden-ring-roadmap.json"',
        },
    )


@app.post("/import", response_class=HTMLResponse)
async def import_roadmap(request: Request, file: UploadFile = File(...)):
    order: List[Dict[str, Any]] = []
    steps: Dict[str, bool] = {}
    try:
        raw = await file.read()
        data = json.loads(raw.decode("utf-8"))
        if isinstance(data, dict):
            raw_order = data.get("current_run_order", [])
            if isinstance(raw_order, list):
                seen_ids = set()
                for item in raw_order:
                    if isinstance(item, dict):
                        i_id = item.get("item_id")
                        if isinstance(i_id, str) and i_id and i_id not in seen_ids:
                            seen_ids.add(i_id)
                            order.append({"item_id": i_id, "is_collected": 1 if item.get("is_collected") else 0})
            raw_steps = data.get("step_progress", [])
            if isinstance(raw_steps, list):
                for step in raw_steps:
                    if isinstance(step, dict):
                        s_key = step.get("step_key")
                        if isinstance(s_key, str) and s_key and bool(step.get("is_done")):
                            steps[s_key] = True
    except Exception:
        order, steps = load_session(request)

    return render_workspace(request, order, steps)


@app.get("/search", response_class=HTMLResponse)
async def search_items(request: Request, q: str = ""):
    q_clean = q.strip()
    if not q_clean:
        return HTMLResponse("", headers=NO_CACHE_HEADERS)

    order, _ = load_session(request)
    ordered_ids = {i["item_id"] for i in order}

    tokens = [f'name:"{tok}"*' for tok in q_clean.replace('"', "").split() if tok]
    fts_query = " AND ".join(tokens)

    with get_conn() as conn:
        conn.row_factory = sqlite3.Row
        cur = conn.cursor()
        rows = cur.execute(
            """
            SELECT c.item_id, c.name, c.category, c.campaign, c.region, c.is_missable
            FROM items_fts f
            JOIN items_catalog c ON f.item_id = c.item_id
            WHERE items_fts MATCH ?
            ORDER BY c.is_missable DESC, (c.steps_json != '[]') DESC, c.name ASC
            LIMIT 8
            """,
            (fts_query,),
        ).fetchall()

    html_parts = [
        '<div class="divide-y divide-zinc-800/90 rounded-xl border border-amber-500/30 bg-zinc-900/95 backdrop-blur-md shadow-2xl max-h-[60vh] overflow-y-auto">'
    ]
    html_parts.append(
        f"""
        <div class="sticky top-0 z-10 flex items-center justify-between bg-zinc-950/95 backdrop-blur px-3 py-1.5 text-xs text-zinc-400 border-b border-zinc-800">
          <span>Matching Catalog Items ({len(rows)})</span>
          <button type="button" onclick="closeSearchDropdown(false)" class="text-zinc-400 hover:text-zinc-200 px-1 py-0.5">
            Close ✕
          </button>
        </div>
        """
    )
    if not rows:
        html_parts.append('<div class="p-3 text-sm text-zinc-400">No matching item in catalog.</div>')
    for r in rows:
        in_order = r["item_id"] in ordered_ids
        missable_badge = (
            '<span class="ml-2 rounded bg-amber-950/80 px-1.5 py-0.5 text-xs font-medium text-amber-300 border border-amber-700/50">Missable</span>'
            if r["is_missable"]
            else ""
        )
        order_badge = (
            '<span class="ml-2 rounded bg-emerald-950/80 px-1.5 py-0.5 text-xs text-emerald-300 border border-emerald-700/50">In Roadmap</span>'
            if in_order
            else ""
        )

        action_form = (
            f"""
            <form hx-post="/order/remove" hx-target="#workspace" hx-swap="innerHTML" hx-on::after-request="closeSearchDropdown()" class="inline">
              <input type="hidden" name="item_id" value="{r["item_id"]}" />
              <button type="submit" class="rounded border border-red-700/70 bg-red-950/60 hover:bg-red-900 px-2.5 py-1 text-xs font-medium text-red-200 transition">
                Remove
              </button>
            </form>
            """
            if in_order
            else f"""
            <form hx-post="/order/add" hx-target="#workspace" hx-swap="innerHTML" hx-on::after-request="closeSearchDropdown()" class="inline">
              <input type="hidden" name="item_id" value="{r["item_id"]}" />
              <button type="submit" class="rounded bg-amber-600/90 hover:bg-amber-500 px-2.5 py-1 text-xs font-semibold text-zinc-950 transition">
                + Add to Run
              </button>
            </form>
            """
        )

        html_parts.append(
            f"""
            <div class="flex items-center justify-between gap-2 px-3 py-2.5 hover:bg-zinc-800/70 min-w-0">
              <div class="min-w-0">
                <div class="text-sm font-medium text-zinc-100 break-words">
                  {r["name"]} {missable_badge} {order_badge}
                </div>
                <div class="text-xs text-zinc-400 break-words">{r["campaign"]} • {r["category"]} • {r["region"]}</div>
              </div>
              <div class="flex items-center gap-1.5 shrink-0">
                {action_form}
              </div>
            </div>
            """
        )
    html_parts.append("</div>")
    return HTMLResponse("".join(html_parts), headers=NO_CACHE_HEADERS)


@app.post("/order/add", response_class=HTMLResponse)
async def order_add(request: Request, item_id: str = Form(...)):
    order, steps = load_session(request)
    if not any(i["item_id"] == item_id for i in order):
        order.append({"item_id": item_id, "is_collected": 0})
    return render_workspace(request, order, steps)


@app.post("/order/remove", response_class=HTMLResponse)
async def order_remove(request: Request, item_id: str = Form(...)):
    order, steps = load_session(request)
    order = [i for i in order if i["item_id"] != item_id]
    return render_workspace(request, order, steps)


@app.post("/order/clear", response_class=HTMLResponse)
async def order_clear(request: Request):
    return render_workspace(request, [], {})


@app.post("/order/toggle", response_class=HTMLResponse)
async def order_toggle(request: Request, item_id: str = Form(...)):
    order, steps = load_session(request)
    new_state = 0
    for item in order:
        if item["item_id"] == item_id:
            new_state = 0 if item.get("is_collected") == 1 else 1
            item["is_collected"] = new_state
            break

    with get_conn() as conn:
        cur = conn.cursor()
        row = cur.execute("SELECT steps_json FROM items_catalog WHERE item_id = ?", (item_id,)).fetchone()
        if row:
            raw_steps = json.loads(row[0] or "[]")
            step_keys = [s["step_key"] for s in raw_steps] if raw_steps else [f"pickup_{item_id}"]
            for s_key in step_keys:
                steps[s_key] = bool(new_state)

    return render_workspace(request, order, steps)


@app.post("/step/toggle", response_class=HTMLResponse)
async def step_toggle(request: Request, step_key: str = Form(...)):
    order, steps = load_session(request)
    current_done = bool(steps.get(step_key, False))
    new_done = not current_done
    steps[step_key] = new_done

    with get_conn() as conn:
        cur = conn.cursor()
        catalog_map = {}
        for item in order:
            i_id = item["item_id"]
            row = cur.execute("SELECT steps_json FROM items_catalog WHERE item_id = ?", (i_id,)).fetchone()
            if row:
                st = json.loads(row[0] or "[]")
                if not st:
                    st = [{"step_key": f"pickup_{i_id}", "seq": 1}]
                catalog_map[i_id] = st

        for item in order:
            i_id = item["item_id"]
            st = catalog_map.get(i_id, [])
            matching = [s for s in st if s["step_key"] == step_key]
            if not matching:
                continue

            target_seq = matching[0]["seq"]
            if new_done:
                for s in st:
                    if s["seq"] < target_seq:
                        steps[s["step_key"]] = True

            all_keys = [s["step_key"] for s in st]
            done_count = sum(1 for k in all_keys if steps.get(k))
            item["is_collected"] = 1 if done_count == len(all_keys) else 0

    return render_workspace(request, order, steps)


# Low-priority static frontend routes with Cloudflare Edge CDN caching
app.frontend("/", directory=DIST_DIR)