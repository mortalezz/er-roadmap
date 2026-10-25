use std::collections::{HashMap, HashSet};

use crate::generated_catalog::{CATALOG, GATES};
use crate::models::{
    CatalogItem, CompiledEpoch, CompiledStep, ConflictGroup, Gate, OrderEntry, OrderItemView,
    RunPlan, StepTargetBadge,
};

pub static LANDMARK_ZONE_MAP: &[(i32, &'static str, &'static [&'static str])] = &[
    (
        10,
        "Limgrave",
        &[
            "limgrave",
            "murkwater",
            "stormveil",
            "agheel",
            "church of elleh",
            "istvan",
            "waypoint ruins",
            "fort haight",
            "mistwood",
            "anticipation",
            "fringefolk",
            "dragon communion seal",
        ],
    ),
    (
        15,
        "Weeping Peninsula",
        &["weeping peninsula", "castle morne", "witchbane", "oridys"],
    ),
    (
        20,
        "Liurnia of the Lakes",
        &[
            "liurnia",
            "caria manor",
            "ranni's rise",
            "seluvis",
            "three sisters",
            "raya lucaria",
            "boilprawn",
            "moonlight altar",
            "albinaurics",
            "slumbering wolf",
            "belfries",
            "jarburg",
            "bellum",
            "smarag",
        ],
    ),
    (
        25,
        "Caelid & Dragonbarrow",
        &[
            "caelid",
            "swamp of aeonia",
            "church of the plague",
            "gowry",
            "redmane",
            "sellia",
            "radahn",
            "dragonbarrow",
            "bestial sanctum",
            "faroth",
            "ekzyke",
            "greyoll",
            "forsaken ruins",
            "st. trina",
            "dragonice",
            "dragonclaw",
            "dragonfire",
            "dragonmaw",
            "glintstone breath",
            "rotten breath",
        ],
    ),
    (
        30,
        "Nokron, Siofra & Underground",
        &[
            "nokron",
            "siofra",
            "ancestral woods",
            "fingerslayer",
            "night's sacred ground",
            "ainsel",
            "lake of rot",
            "deeproot",
        ],
    ),
    (
        40,
        "Altus Plateau & Capital Outskirts",
        &[
            "altus",
            "amber starlight",
            "shaded castle",
            "windmill",
            "dominula",
            "erdtree-gazing",
            "outer wall",
            "ruin-strewn",
            "capital outskirts",
            "outer moat",
            "rileigh",
            "auriza",
        ],
    ),
    (
        45,
        "Mt. Gelmir & Volcano Manor",
        &[
            "volcano manor",
            "mt. gelmir",
            "rykard",
            "fort laiedd",
            "hermit village",
            "eiglay",
            "azur",
        ],
    ),
    (
        50,
        "Leyndell & Shunning-Grounds",
        &[
            "leyndell",
            "royal capital",
            "erdtree sanctuary",
            "gransax",
            "fortified manor",
            "shunning-grounds",
            "forsaken",
            "sewer-gaol",
            "morgott",
        ],
    ),
    (
        60,
        "Mountaintops of the Giants",
        &[
            "mountaintops",
            "ancient snow valley",
            "castle sol",
            "fire giant",
            "stargazer",
            "lofty",
            "zamor",
            "borealis",
        ],
    ),
    (
        70,
        "Snowfield & Mohgwyn Palace",
        &[
            "snowfield",
            "mohgwyn",
            "mohg, lord of blood",
            "blood lord",
            "apostate derelict",
            "ordina",
            "yelough",
            "theodorix",
        ],
    ),
    (
        75,
        "Elphael & Haligtree",
        &["elphael", "haligtree", "drainage channel", "malenia"],
    ),
    (
        80,
        "Crumbling Farum Azula",
        &["farum azula", "maliketh", "placidusax", "dragon temple"],
    ),
    (
        100,
        "Gravesite Plain",
        &[
            "gravesite plain",
            "pillar path",
            "main gate cross",
            "prospect town",
            "lava intake",
            "benediction",
            "consolation",
            "scorched ruins",
            "belurat gaol",
            "ellac",
            "dragon's pit",
        ],
    ),
    (
        105,
        "Cerulean Coast & Charo's Grave",
        &[
            "cerulean coast",
            "charo",
            "rhia",
            "lamenter",
            "marigga",
            "southern nameless",
        ],
    ),
    (
        108,
        "Belurat, Tower Settlement",
        &["belurat", "dancing lion", "grandam"],
    ),
    (
        112,
        "Castle Ensis",
        &["castle ensis", "ensis", "rellana", "moonrithyll"],
    ),
    (
        114,
        "Stone Coffin Fissure",
        &[
            "stone coffin",
            "fissure",
            "putrescence",
            "putrescent",
            "deep purple",
            "st. trina",
        ],
    ),
    (
        120,
        "Scadu Altus",
        &[
            "scadu altus",
            "highroad cross",
            "bonny",
            "church of the crusade",
            "starfall past",
            "reprimand",
            "moorth",
            "fog rift",
            "rabbath",
            "eastern nameless",
        ],
    ),
    (
        125,
        "Cathedral of Manus Metyr",
        &["manus metyr", "ymir", "mother of fingers"],
    ),
    (
        130,
        "Shadow Keep & Hinterland",
        &[
            "shadow keep",
            "specimen storehouse",
            "church district",
            "messmer",
            "prayer room",
            "scadutree base",
            "gaius",
            "hinterland",
            "shaman village",
            "dheo",
        ],
    ),
    (
        140,
        "Jagged Peak",
        &[
            "jagged peak",
            "grand altar of dragon communion",
            "dragon communion",
            "florissax",
            "bayle",
            "igon",
        ],
    ),
    (
        150,
        "Ancient Ruins of Rauh",
        &[
            "rauh",
            "temple town",
            "romina",
            "sealing tree",
            "church of the bud",
            "taylew",
            "scorpion river",
            "devonia",
        ],
    ),
    (
        152,
        "Abyssal Woods",
        &["abyssal woods", "midra", "manse", "jori", "darklight"],
    ),
    (
        160,
        "Enir-Ilim",
        &[
            "enir-ilim",
            "promised consort",
            "god and a lord",
            "cleansing chamber",
        ],
    ),
];

#[inline]
pub fn gate_timeline_pos(gate_id: &str) -> i32 {
    match gate_id {
        "GATE_B_ALTUS" => 38,
        "GATE_B_FINGERSLAYER" => 42,
        "GATE_B_VOLCANO_MANOR" => 44,
        "GATE_B_RYKARD" => 48,
        "GATE_B_CASTLE_SOL" => 62,
        "GATE_B_MALIKETH" => 85,
        "GATE_S_RUNEBREAK" => 115,
        "GATE_S_MESSMER" => 135,
        "GATE_S_ROMINA" => 155,
        _ => 200,
    }
}

#[inline]
fn contains_ignore_ascii_case(haystack: &str, needle_lower: &str) -> bool {
    let h = haystack.as_bytes();
    let n = needle_lower.as_bytes();
    if n.is_empty() {
        return true;
    }
    if n.len() > h.len() {
        return false;
    }
    let first = n[0];
    let limit = h.len() - n.len();
    let mut i = 0;
    while i <= limit {
        if h[i].to_ascii_lowercase() == first {
            let mut matched = true;
            let mut j = 1;
            while j < n.len() {
                if h[i + j].to_ascii_lowercase() != n[j] {
                    matched = false;
                    break;
                }
                j += 1;
            }
            if matched {
                return true;
            }
        }
        i += 1;
    }
    false
}

pub fn infer_zone_info(step_text: &str, fallback_region: &str, campaign: &str) -> (i32, &'static str) {
    let is_dlc = campaign.contains("Shadow");

    for &(z_ord, canonical_name, keywords) in LANDMARK_ZONE_MAP {
        if (is_dlc && z_ord < 100) || (!is_dlc && z_ord >= 100) {
            continue;
        }
        if keywords
            .iter()
            .any(|&kw| contains_ignore_ascii_case(step_text, kw))
        {
            return (z_ord, canonical_name);
        }
    }

    for &(z_ord, canonical_name, keywords) in LANDMARK_ZONE_MAP {
        if (is_dlc && z_ord < 100) || (!is_dlc && z_ord >= 100) {
            continue;
        }
        if keywords
            .iter()
            .any(|&kw| contains_ignore_ascii_case(fallback_region, kw))
        {
            return (z_ord, canonical_name);
        }
    }

    if is_dlc {
        (100, "Gravesite Plain")
    } else {
        (40, "Lands Between")
    }
}

pub fn find_catalog_item(item_id: &str) -> Option<&'static CatalogItem> {
    CATALOG.iter().find(|it| it.item_id == item_id)
}

pub fn search_catalog(query: &str, limit: usize) -> Vec<&'static CatalogItem> {
    let q_lower = query.to_ascii_lowercase();
    let mut tok_buf = [""; 8];
    let mut tok_count = 0usize;
    for part in q_lower.split(|c: char| !c.is_ascii_alphanumeric()) {
        if !part.is_empty() && tok_count < 8 {
            tok_buf[tok_count] = part;
            tok_count += 1;
        }
    }
    if tok_count == 0 {
        return Vec::new();
    }
    let tokens = &tok_buf[..tok_count];

    let mut matches = Vec::with_capacity(limit);
    for item in CATALOG.iter() {
        let mut all_match = true;
        for &tok in tokens {
            if !item.name_lower.contains(tok) {
                all_match = false;
                break;
            }
        }
        if all_match {
            matches.push(item);
            if matches.len() >= limit {
                break;
            }
        }
    }
    matches
}

pub fn toggle_order_item(
    order: &mut [OrderEntry],
    steps: &mut HashMap<String, bool>,
    item_id: &str,
) {
    let mut new_state = false;
    for entry in order.iter_mut() {
        if entry.item_id == item_id {
            new_state = !entry.is_collected;
            entry.is_collected = new_state;
            break;
        }
    }

    if let Some(cat) = find_catalog_item(item_id) {
        if cat.steps.is_empty() {
            steps.insert(format!("pickup_{}", item_id), new_state);
        } else {
            for s in cat.steps {
                steps.insert(s.step_key.to_string(), new_state);
            }
        }
    }
}

pub fn toggle_step_key(
    order: &mut [OrderEntry],
    steps: &mut HashMap<String, bool>,
    step_key: &str,
) {
    let current_done = *steps.get(step_key).unwrap_or(&false);
    let new_done = !current_done;
    steps.insert(step_key.to_string(), new_done);

    for entry in order.iter_mut() {
        if let Some(cat) = find_catalog_item(&entry.item_id) {
            if cat.steps.is_empty() {
                let pk = format!("pickup_{}", entry.item_id);
                if pk == step_key {
                    entry.is_collected = new_done;
                }
            } else {
                if let Some(matching) = cat.steps.iter().find(|s| s.step_key == step_key) {
                    let target_seq = matching.seq;
                    if new_done {
                        for s in cat.steps {
                            if s.seq < target_seq {
                                steps.insert(s.step_key.to_string(), true);
                            }
                        }
                    }
                    let all_done = cat
                        .steps
                        .iter()
                        .all(|s| *steps.get(s.step_key).unwrap_or(&false));
                    entry.is_collected = all_done;
                }
            }
        }
    }
}

pub fn compile_run_plan(
    session_order: &[OrderEntry],
    step_done_map: &HashMap<String, bool>,
) -> RunPlan {
    if session_order.is_empty() {
        return RunPlan {
            order_items: Vec::new(),
            collected_count: 0,
            total_steps: 0,
            done_steps: 0,
            conflicts: Vec::new(),
            epochs: Vec::new(),
        };
    }

    let mut order_items = Vec::with_capacity(session_order.len());
    for entry in session_order {
        if let Some(meta) = find_catalog_item(&entry.item_id) {
            order_items.push(OrderItemView {
                item_id: meta.item_id,
                is_collected: entry.is_collected,
                name: meta.name,
                category: meta.category,
                campaign: meta.campaign,
                region: meta.region,
                is_missable: meta.is_missable,
                exclusive_group: meta.exclusive_group,
                exclusive_reason: meta.exclusive_reason,
                steps: meta.steps,
            });
        }
    }

    let mut exclusive_order: Vec<&'static str> = Vec::new();
    let mut exclusive_buckets: HashMap<&'static str, Vec<OrderItemView>> = HashMap::new();
    for item in &order_items {
        if !item.exclusive_group.is_empty() {
            if !exclusive_buckets.contains_key(item.exclusive_group) {
                exclusive_order.push(item.exclusive_group);
            }
            exclusive_buckets
                .entry(item.exclusive_group)
                .or_insert_with(Vec::new)
                .push(item.clone());
        }
    }

    let mut conflicts = Vec::new();
    for g_id in exclusive_order {
        if let Some(colliding) = exclusive_buckets.remove(g_id) {
            if colliding.len() > 1 {
                conflicts.push(ConflictGroup {
                    group_id: g_id,
                    reason: colliding[0].exclusive_reason,
                    items: colliding,
                });
            }
        }
    }

    let mut step_order_keys: Vec<String> = Vec::new();
    let mut merged_steps: HashMap<String, CompiledStep> = HashMap::new();

    let has_dlc_targets = order_items
        .iter()
        .any(|i| i.campaign == "Shadow of the Erdtree");

    if has_dlc_targets {
        let dlc_key = String::from("auto_dlc_access_spine");
        let is_done = *step_done_map.get(&dlc_key).unwrap_or(&false);
        step_order_keys.push(dlc_key.clone());
        merged_steps.insert(
            dlc_key.clone(),
            CompiledStep {
                step_key: dlc_key,
                seq: 0,
                zone_order: 70,
                region_label: "Caelid & Mohgwyn Palace",
                effective_ponr: 115,
                gate_id: "GATE_NONE",
                is_gate_critical: false,
                text: String::from("Defeat Starscourge Radahn (Redmane Castle) & Mohg, Lord of Blood (via Varre's Questline or Snowfield) to enter the Realm of Shadow."),
                is_done,
                targets: vec![StepTargetBadge {
                    item_id: "dlc_entry",
                    name: "Realm of Shadow Access",
                    is_collected: false,
                    is_missable: false,
                }],
            },
        );
    }

    for item in &order_items {
        if item.steps.is_empty() {
            let (z_ord, region_label) = infer_zone_info(
                &format!("{} {}", item.name, item.region),
                item.region,
                item.campaign,
            );
            let s_key = format!("pickup_{}", item.item_id);
            let scheduled_pos = std::cmp::min(z_ord, 199);
            let is_done = *step_done_map.get(&s_key).unwrap_or(&false);
            if !merged_steps.contains_key(&s_key) {
                step_order_keys.push(s_key.clone());
                merged_steps.insert(
                    s_key.clone(),
                    CompiledStep {
                        step_key: s_key,
                        seq: 50,
                        zone_order: scheduled_pos,
                        region_label,
                        effective_ponr: 200,
                        gate_id: "GATE_NONE",
                        is_gate_critical: false,
                        text: format!(
                            "Collect {} ({}) — {}.",
                            item.name, item.category, item.region
                        ),
                        is_done,
                        targets: vec![StepTargetBadge {
                            item_id: item.item_id,
                            name: item.name,
                            is_collected: item.is_collected,
                            is_missable: item.is_missable,
                        }],
                    },
                );
            }
            continue;
        }

        let mut running_ponr_ceiling = 200i32;
        let mut computed_chain = Vec::with_capacity(item.steps.len());

        for s in item.steps.iter().rev() {
            let g_id = s.gate_id;
            let gate_pos = gate_timeline_pos(g_id);
            if gate_pos < running_ponr_ceiling {
                running_ponr_ceiling = gate_pos;
            }

            let (z_ord, region_label) = match (s.zone_order, s.region_label) {
                (Some(z), Some(r)) => (z, r),
                (z_opt, r_opt) => {
                    let (inf_z, inf_r) = infer_zone_info(s.text, item.region, item.campaign);
                    (z_opt.unwrap_or(inf_z), r_opt.unwrap_or(inf_r))
                }
            };

            let scheduled_pos = std::cmp::min(z_ord, running_ponr_ceiling - 1);
            computed_chain.push((s, scheduled_pos, region_label, running_ponr_ceiling, g_id));
        }

        computed_chain.reverse();

        for (s, scheduled_pos, region_label, ponr_ceil, g_id) in computed_chain {
            let s_key = s.step_key.to_string();
            if let Some(existing) = merged_steps.get_mut(&s_key) {
                if scheduled_pos < existing.zone_order {
                    existing.zone_order = scheduled_pos;
                }
                if ponr_ceil < existing.effective_ponr {
                    existing.effective_ponr = ponr_ceil;
                }
                if ponr_ceil < 200 {
                    existing.is_gate_critical = true;
                }
                if g_id != "GATE_NONE" && existing.gate_id == "GATE_NONE" {
                    existing.gate_id = g_id;
                }
                if !existing.targets.iter().any(|t| t.item_id == item.item_id) {
                    existing.targets.push(StepTargetBadge {
                        item_id: item.item_id,
                        name: item.name,
                        is_collected: item.is_collected,
                        is_missable: item.is_missable,
                    });
                }
            } else {
                step_order_keys.push(s_key.clone());
                let is_done = *step_done_map.get(&s_key).unwrap_or(&false);
                merged_steps.insert(
                    s_key.clone(),
                    CompiledStep {
                        step_key: s_key,
                        seq: s.seq,
                        zone_order: scheduled_pos,
                        region_label,
                        effective_ponr: ponr_ceil,
                        gate_id: g_id,
                        is_gate_critical: ponr_ceil < 200,
                        text: s.text.to_string(),
                        is_done,
                        targets: vec![StepTargetBadge {
                            item_id: item.item_id,
                            name: item.name,
                            is_collected: item.is_collected,
                            is_missable: item.is_missable,
                        }],
                    },
                );
            }
        }
    }

    let all_steps: Vec<CompiledStep> = step_order_keys
        .into_iter()
        .filter_map(|k| merged_steps.remove(&k))
        .collect();

    let active_gate_ids: HashSet<&'static str> = all_steps
        .iter()
        .filter(|s| s.gate_id != "GATE_NONE")
        .map(|s| s.gate_id)
        .collect();

    let mut sorted_checkpoint_gates: Vec<Gate> = GATES
        .iter()
        .copied()
        .filter(|g| active_gate_ids.contains(g.gate_id))
        .collect();
    sorted_checkpoint_gates.sort_by_key(|g| gate_timeline_pos(g.gate_id));

    let mut compiled_epochs = Vec::new();
    let mut assigned_keys: HashSet<String> = HashSet::new();

    for gate in sorted_checkpoint_gates {
        let g_id = gate.gate_id;
        let g_pos = gate_timeline_pos(g_id);

        let mut gate_steps: Vec<CompiledStep> = all_steps
            .iter()
            .filter(|s| {
                !assigned_keys.contains(&s.step_key)
                    && (s.gate_id == g_id
                        || (s.zone_order <= g_pos && s.effective_ponr <= g_pos))
            })
            .cloned()
            .collect();

        if gate_steps.is_empty() {
            gate_steps = all_steps
                .iter()
                .filter(|s| !assigned_keys.contains(&s.step_key) && s.zone_order <= g_pos)
                .cloned()
                .collect();
        }

        if !gate_steps.is_empty() {
            for s in &gate_steps {
                assigned_keys.insert(s.step_key.clone());
            }
            gate_steps.sort_by_key(|x| (x.zone_order, x.seq));
            let pending_count = gate_steps.iter().filter(|s| !s.is_done).count();
            compiled_epochs.push(CompiledEpoch {
                gate,
                steps: gate_steps,
                pending_count,
                all_clear: pending_count == 0,
            });
        }
    }

    let mut remaining_steps: Vec<CompiledStep> = all_steps
        .iter()
        .filter(|s| !assigned_keys.contains(&s.step_key))
        .cloned()
        .collect();

    if !remaining_steps.is_empty() {
        if let Some(&gate_none) = GATES.iter().find(|g| g.gate_id == "GATE_NONE") {
            remaining_steps.sort_by_key(|x| (x.zone_order, x.seq));
            let pending_count = remaining_steps.iter().filter(|s| !s.is_done).count();
            compiled_epochs.push(CompiledEpoch {
                gate: gate_none,
                steps: remaining_steps,
                pending_count,
                all_clear: pending_count == 0,
            });
        }
    }

    let total_steps = all_steps.len();
    let done_steps = all_steps.iter().filter(|s| s.is_done).count();
    let collected_count = order_items.iter().filter(|i| i.is_collected).count();

    RunPlan {
        order_items,
        collected_count,
        total_steps,
        done_steps,
        conflicts,
        epochs: compiled_epochs,
    }
}
