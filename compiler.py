import json
import sqlite3
from collections import defaultdict
from typing import Dict, Any, List, Tuple

LANDMARK_ZONE_MAP: List[Tuple[int, str, List[str]]] = [
    (10, "Limgrave", ["limgrave", "murkwater", "stormveil", "agheel", "church of elleh", "istvan", "waypoint ruins", "fort haight", "mistwood", "anticipation", "fringefolk", "dragon communion seal"]),
    (15, "Weeping Peninsula", ["weeping peninsula", "castle morne", "witchbane", "oridys"]),
    (20, "Liurnia of the Lakes", ["liurnia", "caria manor", "ranni's rise", "seluvis", "three sisters", "raya lucaria", "boilprawn", "moonlight altar", "albinaurics", "slumbering wolf", "belfries", "jarburg", "bellum", "smarag"]),
    (25, "Caelid & Dragonbarrow", ["caelid", "swamp of aeonia", "church of the plague", "gowry", "redmane", "sellia", "radahn", "dragonbarrow", "bestial sanctum", "faroth", "ekzyke", "greyoll", "forsaken ruins", "st. trina", "dragonice", "dragonclaw", "dragonfire", "dragonmaw", "glintstone breath", "rotten breath"]),
    (30, "Nokron, Siofra & Underground", ["nokron", "siofra", "ancestral woods", "fingerslayer", "night's sacred ground", "ainsel", "lake of rot", "deeproot"]),
    (40, "Altus Plateau & Capital Outskirts", ["altus", "amber starlight", "shaded castle", "windmill", "dominula", "erdtree-gazing", "outer wall", "ruin-strewn", "capital outskirts", "outer moat", "rileigh", "auriza"]),
    (45, "Mt. Gelmir & Volcano Manor", ["volcano manor", "mt. gelmir", "rykard", "fort laiedd", "hermit village", "eiglay", "azur"]),
    (50, "Leyndell & Shunning-Grounds", ["leyndell", "royal capital", "erdtree sanctuary", "gransax", "fortified manor", "shunning-grounds", "forsaken", "sewer-gaol", "morgott"]),
    (60, "Mountaintops of the Giants", ["mountaintops", "ancient snow valley", "castle sol", "fire giant", "stargazer", "lofty", "zamor", "borealis"]),
    (70, "Snowfield & Mohgwyn Palace", ["snowfield", "mohgwyn", "mohg, lord of blood", "blood lord", "apostate derelict", "ordina", "yelough", "theodorix"]),
    (75, "Elphael & Haligtree", ["elphael", "haligtree", "drainage channel", "malenia"]),
    (80, "Crumbling Farum Azula", ["farum azula", "maliketh", "placidusax", "dragon temple"]),
    (100, "Gravesite Plain", ["gravesite plain", "pillar path", "main gate cross", "prospect town", "lava intake", "benediction", "consolation", "scorched ruins", "belurat gaol", "ellac", "dragon's pit"]),
    (105, "Cerulean Coast & Charo's Grave", ["cerulean coast", "charo", "rhia", "lamenter", "marigga", "southern nameless"]),
    (108, "Belurat, Tower Settlement", ["belurat", "dancing lion", "grandam"]),
    (112, "Castle Ensis", ["castle ensis", "ensis", "rellana", "moonrithyll"]),
    (114, "Stone Coffin Fissure", ["stone coffin", "fissure", "putrescence", "putrescent", "deep purple", "st. trina"]),
    (120, "Scadu Altus", ["scadu altus", "highroad cross", "bonny", "church of the crusade", "starfall past", "reprimand", "moorth", "fog rift", "rabbath", "eastern nameless"]),
    (125, "Cathedral of Manus Metyr", ["manus metyr", "ymir", "mother of fingers"]),
    (130, "Shadow Keep & Hinterland", ["shadow keep", "specimen storehouse", "church district", "messmer", "prayer room", "scadutree base", "gaius", "hinterland", "shaman village", "dheo"]),
    (140, "Jagged Peak", ["jagged peak", "grand altar of dragon communion", "dragon communion", "florissax", "bayle", "igon"]),
    (150, "Ancient Ruins of Rauh", ["rauh", "temple town", "romina", "sealing tree", "church of the bud", "taylew", "scorpion river", "devonia"]),
    (152, "Abyssal Woods", ["abyssal woods", "midra", "manse", "jori", "darklight"]),
    (160, "Enir-Ilim", ["enir-ilim", "promised consort", "god and a lord", "cleansing chamber"]),
]

GATE_TIMELINE_POS: Dict[str, int] = {
    "GATE_B_ALTUS": 38,
    "GATE_B_FINGERSLAYER": 42,
    "GATE_B_VOLCANO_MANOR": 44,
    "GATE_B_RYKARD": 48,
    "GATE_B_CASTLE_SOL": 62,
    "GATE_B_MALIKETH": 85,
    "GATE_S_RUNEBREAK": 115,
    "GATE_S_MESSMER": 135,
    "GATE_S_ROMINA": 155,
    "GATE_NONE": 200,
}


def infer_zone_info(step_text: str, fallback_region: str, campaign: str) -> Tuple[int, str]:
    is_dlc = "Shadow" in campaign
    candidate_zones = [
        z for z in LANDMARK_ZONE_MAP if (z[0] >= 100 if is_dlc else z[0] < 100)
    ]

    text_lower = step_text.lower()
    for z_ord, canonical_name, keywords in candidate_zones:
        if any(kw in text_lower for kw in keywords):
            return z_ord, canonical_name

    fallback_lower = fallback_region.lower()
    for z_ord, canonical_name, keywords in candidate_zones:
        if any(kw in fallback_lower for kw in keywords):
            return z_ord, canonical_name

    return (100, "Gravesite Plain") if is_dlc else (40, "Lands Between")


def compile_run_plan(conn: sqlite3.Connection, session_order: List[Dict[str, Any]], step_done_map: Dict[str, bool]) -> Dict[str, Any]:
    conn.row_factory = sqlite3.Row
    cur = conn.cursor()

    if not session_order:
        return {
            "order_items": [],
            "collected_count": 0,
            "total_steps": 0,
            "done_steps": 0,
            "conflicts": [],
            "epochs": [],
        }

    item_ids = [i["item_id"] for i in session_order]
    placeholders = ",".join("?" for _ in item_ids)
    catalog_rows = cur.execute(
        f"""
        SELECT item_id, name, category, campaign, region, is_missable, exclusive_group, exclusive_reason, steps_json
        FROM items_catalog
        WHERE item_id IN ({placeholders})
        """,
        item_ids,
    ).fetchall()
    catalog_map = {r["item_id"]: dict(r) for r in catalog_rows}

    order_items = []
    for item in session_order:
        i_id = item["item_id"]
        if i_id in catalog_map:
            meta = catalog_map[i_id]
            order_items.append(
                {
                    "item_id": i_id,
                    "is_collected": int(item.get("is_collected", 0)),
                    "name": meta["name"],
                    "category": meta["category"],
                    "campaign": meta["campaign"],
                    "region": meta["region"],
                    "is_missable": meta["is_missable"],
                    "exclusive_group": meta["exclusive_group"],
                    "exclusive_reason": meta["exclusive_reason"],
                    "steps_json": meta["steps_json"],
                }
            )

    gates_rows = cur.execute("SELECT * FROM gates ORDER BY sort_order ASC").fetchall()
    gates_by_id = {g["gate_id"]: dict(g) for g in gates_rows}

    exclusive_buckets: Dict[str, List[Dict[str, Any]]] = defaultdict(list)
    for item in order_items:
        if item["exclusive_group"]:
            exclusive_buckets[item["exclusive_group"]].append(item)

    conflicts = []
    for group_id, colliding_items in exclusive_buckets.items():
        if len(colliding_items) > 1:
            conflicts.append(
                {
                    "group_id": group_id,
                    "reason": colliding_items[0]["exclusive_reason"],
                    "items": colliding_items,
                }
            )

    merged_steps: Dict[str, Dict[str, Any]] = {}
    has_dlc_targets = any(i["campaign"] == "Shadow of the Erdtree" for i in order_items)

    if has_dlc_targets:
        dlc_key = "auto_dlc_access_spine"
        merged_steps[dlc_key] = {
            "step_key": dlc_key,
            "seq": 0,
            "zone_order": 70,
            "region_label": "Caelid & Mohgwyn Palace",
            "effective_ponr": 115,
            "gate_id": "GATE_NONE",
            "is_gate_critical": False,
            "text": "Defeat Starscourge Radahn (Redmane Castle) & Mohg, Lord of Blood (via Varre's Questline or Snowfield) to enter the Realm of Shadow.",
            "is_done": bool(step_done_map.get(dlc_key, False)),
            "targets": [
                {
                    "item_id": "dlc_entry",
                    "name": "Realm of Shadow Access",
                    "is_collected": False,
                    "is_missable": False,
                }
            ],
        }

    for item in order_items:
        raw_steps = json.loads(item["steps_json"] or "[]")
        if not raw_steps:
            z_ord, region_label = infer_zone_info(f"{item['name']} {item['region']}", item["region"], item["campaign"])
            raw_steps = [
                {
                    "step_key": f"pickup_{item['item_id']}",
                    "gate_id": "GATE_NONE",
                    "seq": 50,
                    "zone_order": z_ord,
                    "region_label": region_label,
                    "text": f"Collect {item['name']} ({item['category']}) — {item['region']}.",
                }
            ]

        running_ponr_ceiling = 200
        computed_chain = []
        for s in reversed(raw_steps):
            g_id = s.get("gate_id", "GATE_NONE")
            gate_pos = GATE_TIMELINE_POS.get(g_id, 200)
            running_ponr_ceiling = min(running_ponr_ceiling, gate_pos)

            z_ord, region_label = infer_zone_info(s["text"], item["region"], item["campaign"])
            if "zone_order" in s:
                z_ord = s["zone_order"]
            if "region_label" in s:
                region_label = s["region_label"]

            scheduled_pos = min(z_ord, running_ponr_ceiling - 1)
            computed_chain.append((s, scheduled_pos, region_label, running_ponr_ceiling, g_id))

        computed_chain.reverse()

        for s, scheduled_pos, region_label, ponr_ceil, g_id in computed_chain:
            s_key = s["step_key"]
            if s_key not in merged_steps:
                merged_steps[s_key] = {
                    "step_key": s_key,
                    "seq": s["seq"],
                    "zone_order": scheduled_pos,
                    "region_label": region_label,
                    "effective_ponr": ponr_ceil,
                    "gate_id": g_id,
                    "is_gate_critical": ponr_ceil < 200,
                    "text": s["text"],
                    "is_done": bool(step_done_map.get(s_key, False)),
                    "targets": [],
                }
            else:
                merged_steps[s_key]["zone_order"] = min(
                    merged_steps[s_key]["zone_order"], scheduled_pos
                )
                merged_steps[s_key]["effective_ponr"] = min(
                    merged_steps[s_key]["effective_ponr"], ponr_ceil
                )
                if ponr_ceil < 200:
                    merged_steps[s_key]["is_gate_critical"] = True
                if g_id != "GATE_NONE" and merged_steps[s_key]["gate_id"] == "GATE_NONE":
                    merged_steps[s_key]["gate_id"] = g_id

            existing_target_ids = {t["item_id"] for t in merged_steps[s_key]["targets"]}
            if item["item_id"] not in existing_target_ids:
                merged_steps[s_key]["targets"].append(
                    {
                        "item_id": item["item_id"],
                        "name": item["name"],
                        "is_collected": bool(item["is_collected"]),
                        "is_missable": bool(item["is_missable"]),
                    }
                )

    all_steps = list(merged_steps.values())
    active_gate_ids = {s["gate_id"] for s in all_steps if s["gate_id"] != "GATE_NONE"}
    sorted_checkpoint_gates = sorted(
        (gates_by_id[gid] for gid in active_gate_ids if gid in gates_by_id),
        key=lambda g: GATE_TIMELINE_POS.get(g["gate_id"], 200),
    )

    compiled_epochs = []
    assigned_keys = set()

    for gate in sorted_checkpoint_gates:
        g_id = gate["gate_id"]
        g_pos = GATE_TIMELINE_POS.get(g_id, 200)

        gate_steps = [
            s
            for s in all_steps
            if s["step_key"] not in assigned_keys
            and (
                s["gate_id"] == g_id
                or (s["zone_order"] <= g_pos and s["effective_ponr"] <= g_pos)
            )
        ]

        if not gate_steps:
            gate_steps = [
                s
                for s in all_steps
                if s["step_key"] not in assigned_keys and s["zone_order"] <= g_pos
            ]

        if gate_steps:
            for s in gate_steps:
                assigned_keys.add(s["step_key"])

            gate_steps.sort(key=lambda x: (x["zone_order"], x["seq"]))
            pending_count = sum(1 for s in gate_steps if not s["is_done"])

            compiled_epochs.append(
                {
                    "gate": gate,
                    "steps": gate_steps,
                    "pending_count": pending_count,
                    "all_clear": pending_count == 0,
                }
            )

    remaining_steps = [s for s in all_steps if s["step_key"] not in assigned_keys]
    if remaining_steps and "GATE_NONE" in gates_by_id:
        remaining_steps.sort(key=lambda x: (x["zone_order"], x["seq"]))
        pending_count = sum(1 for s in remaining_steps if not s["is_done"])
        compiled_epochs.append(
            {
                "gate": gates_by_id["GATE_NONE"],
                "steps": remaining_steps,
                "pending_count": pending_count,
                "all_clear": pending_count == 0,
            }
        )

    total_steps = len(all_steps)
    done_steps = sum(1 for s in all_steps if s["is_done"])
    collected_items = sum(1 for i in order_items if i["is_collected"])

    return {
        "order_items": order_items,
        "collected_count": collected_items,
        "total_steps": total_steps,
        "done_steps": done_steps,
        "conflicts": conflicts,
        "epochs": compiled_epochs,
    }