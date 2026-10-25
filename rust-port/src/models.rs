#[derive(Clone, Copy, Debug)]
pub struct Gate {
    pub gate_id: &'static str,
    pub campaign: &'static str,
    pub sort_order: i32,
    pub title: &'static str,
    pub trigger_desc: &'static str,
    pub warning_desc: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct CatalogStep {
    pub step_key: &'static str,
    pub gate_id: &'static str,
    pub seq: i32,
    pub zone_order: Option<i32>,
    pub region_label: Option<&'static str>,
    pub text: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct CatalogItem {
    pub item_id: &'static str,
    pub name: &'static str,
    pub name_lower: &'static str,
    pub category: &'static str,
    pub campaign: &'static str,
    pub region: &'static str,
    pub is_missable: bool,
    pub exclusive_group: &'static str,
    pub exclusive_reason: &'static str,
    pub steps: &'static [CatalogStep],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderEntry {
    pub item_id: String,
    pub is_collected: bool,
}

#[derive(Clone, Debug)]
pub struct OrderItemView {
    pub item_id: &'static str,
    pub is_collected: bool,
    pub name: &'static str,
    pub category: &'static str,
    pub campaign: &'static str,
    pub region: &'static str,
    pub is_missable: bool,
    pub exclusive_group: &'static str,
    pub exclusive_reason: &'static str,
    pub steps: &'static [CatalogStep],
}

#[derive(Clone, Debug)]
pub struct ConflictGroup {
    pub group_id: &'static str,
    pub reason: &'static str,
    pub items: Vec<OrderItemView>,
}

#[derive(Clone, Debug)]
pub struct StepTargetBadge {
    pub item_id: &'static str,
    pub name: &'static str,
    pub is_collected: bool,
    pub is_missable: bool,
}

#[derive(Clone, Debug)]
pub struct CompiledStep {
    pub step_key: String,
    pub seq: i32,
    pub zone_order: i32,
    pub region_label: &'static str,
    pub effective_ponr: i32,
    pub gate_id: &'static str,
    pub is_gate_critical: bool,
    pub text: String,
    pub is_done: bool,
    pub targets: Vec<StepTargetBadge>,
}

#[derive(Clone, Debug)]
pub struct CompiledEpoch {
    pub gate: Gate,
    pub steps: Vec<CompiledStep>,
    pub pending_count: usize,
    pub all_clear: bool,
}

#[derive(Clone, Debug)]
pub struct RunPlan {
    pub order_items: Vec<OrderItemView>,
    pub collected_count: usize,
    pub total_steps: usize,
    pub done_steps: usize,
    pub conflicts: Vec<ConflictGroup>,
    pub epochs: Vec<CompiledEpoch>,
}

impl RunPlan {
    pub fn pct(&self) -> usize {
        if self.total_steps == 0 {
            0
        } else {
            ((self.done_steps as f64 / self.total_steps as f64) * 100.0).round() as usize
        }
    }
}
