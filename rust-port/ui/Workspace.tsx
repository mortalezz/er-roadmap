// AOT-Compiled Hypermedia JSX Component -> Rust Zero-Copy HTML Writer
// Compiled at `cargo build` time by `build.rs` into `render_workspace(buf: &mut String, plan: &RunPlan)`

export function WorkspacePartial(plan: RunPlan) {
  return (
    <template name="render_workspace" args="plan: &RunPlan">
      <div className="relative overflow-hidden rounded-xl border border-zinc-800 bg-zinc-900/80 px-5 py-4 mb-6 shadow-md">
        <div className="flex flex-wrap items-center justify-between gap-4">
          <div className="flex flex-wrap items-center gap-5 sm:gap-6">
            <div>
              <span className="text-xs uppercase tracking-wider text-zinc-400 block">Targets Queued</span>
              <span className="text-lg font-bold text-amber-400">
                {fmt: plan.collected_count} / {fmt: plan.order_items.len()} Collected
              </span>
            </div>
            <div className="h-8 w-px bg-zinc-800"></div>
            <div>
              <span className="text-xs uppercase tracking-wider text-zinc-400 block">Route Checkpoints</span>
              <span className="text-lg font-semibold text-emerald-400">
                {fmt: plan.done_steps} / {fmt: plan.total_steps} Steps Done
                {if plan.total_steps > 0 {
                  <span className="ml-1 text-xs font-normal text-zinc-400">({fmt: plan.pct()}%)</span>
                }}
              </span>
            </div>
            <div className="h-8 w-px bg-zinc-800"></div>
            <div>
              <span className="text-xs uppercase tracking-wider text-zinc-400 block">Route Legs</span>
              <span className="text-lg font-semibold text-zinc-100">{fmt: plan.epochs.len()}</span>
            </div>
          </div>

          <div className="text-xs text-zinc-400">
            Use <strong>💾 Export JSON</strong> above to save your checked progress anytime.
          </div>
        </div>

        {if plan.total_steps > 0 {
          <div className="absolute bottom-0 left-0 right-0 h-1 bg-zinc-800/80">
            <div
              className="h-full bg-gradient-to-r from-amber-500 via-amber-400 to-emerald-400 transition-all duration-500 ease-out"
              style="width: {fmt: plan.pct()}%;"
            ></div>
          </div>
        }}
      </div>

      {if !plan.conflicts.is_empty() {
        <div className="mb-6 rounded-xl border-2 border-red-500/80 bg-red-950/40 p-4">
          <div className="flex items-center gap-2 text-red-300 font-bold text-sm uppercase tracking-wider mb-2">
            <span>⚠️ Same-Run Mutual Exclusivity Collision Detected ({fmt: plan.conflicts.len()})</span>
          </div>
          <div className="space-y-3">
            {for c in &plan.conflicts {
              <div className="rounded-lg bg-zinc-950/80 p-3 border border-red-800/60">
                <p className="text-sm text-zinc-200 mb-2">{esc: c.reason}</p>
                <div className="flex flex-wrap gap-2">
                  {for item in &c.items {
                    <form hx-post="/order/remove" hx-target="#workspace" hx-swap="innerHTML" className="inline">
                      <input type="hidden" name="item_id" value="{esc: item.item_id}" />
                      <button
                        type="submit"
                        className="rounded border border-red-600/70 bg-red-900/40 hover:bg-red-800/60 px-2.5 py-1 text-xs text-red-200 transition"
                      >
                        Drop &quot;{esc: item.name}&quot; from Current Run ✕
                      </button>
                    </form>
                  }}
                </div>
              </div>
            }}
          </div>
        </div>
      }}

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 w-full max-w-full">
        <div className="lg:col-span-4 space-y-6 min-w-0">
          <div className="rounded-xl border border-zinc-800 bg-zinc-900/50 p-4">
            <div className="flex items-center justify-between mb-3">
              <h2 className="text-sm font-bold uppercase tracking-wider text-amber-400">
                Current Run Targets ({fmt: plan.order_items.len()})
              </h2>
              {if !plan.order_items.is_empty() {
                <form hx-post="/order/clear" hx-target="#workspace" hx-swap="innerHTML">
                  <button
                    type="submit"
                    className="rounded border border-zinc-700 bg-zinc-800 hover:bg-red-950 hover:border-red-700 hover:text-red-300 px-2 py-0.5 text-xs text-zinc-400 transition"
                  >
                    Clear All
                  </button>
                </form>
              }}
            </div>

            {if plan.order_items.is_empty() {
              <p className="text-sm text-zinc-500">
                Clean slate — no items queued yet. Search above or import a saved <code>.json</code> roadmap.
              </p>
            }}
            {if !plan.order_items.is_empty() {
              <div className="space-y-2">
                {for item in &plan.order_items {
                  <div className="flex items-center justify-between gap-2 rounded-lg border {raw: if item.is_collected { "border-zinc-800/50 bg-zinc-950/40 opacity-65" } else { "border-zinc-800 bg-zinc-950/80 hover:border-zinc-700" }} px-3 py-2.5 min-w-0 transition">
                    <form hx-post="/order/toggle" hx-target="#workspace" hx-swap="innerHTML" className="flex-1 min-w-0">
                      <input type="hidden" name="item_id" value="{esc: item.item_id}" />
                      <label className="flex items-center gap-3 min-w-0 cursor-pointer select-none">
                        <input
                          type="checkbox"
                          {raw: if item.is_collected { "checked" } else { "" }}
                          onchange="this.form.requestSubmit()"
                          className="h-4 w-4 rounded border-zinc-600 bg-zinc-900 accent-amber-500 cursor-pointer shrink-0"
                        />
                        <div className="min-w-0">
                          <div className="text-sm font-medium break-words {raw: if item.is_collected { "line-through text-zinc-500" } else { "text-zinc-100" }}">
                            {esc: item.name}
                          </div>
                          <div className="text-xs text-zinc-400 break-words">
                            {esc: item.campaign} • {esc: item.category}
                          </div>
                        </div>
                      </label>
                    </form>
                    <form hx-post="/order/remove" hx-target="#workspace" hx-swap="innerHTML" className="shrink-0">
                      <input type="hidden" name="item_id" value="{esc: item.item_id}" />
                      <button
                        type="submit"
                        className="rounded border border-red-800/60 bg-red-950/40 hover:bg-red-900/70 px-2 py-1 text-xs text-red-300 transition"
                      >
                        Remove
                      </button>
                    </form>
                  </div>
                }}
              </div>
            }}
          </div>
        </div>

        <div className="lg:col-span-8 min-w-0">
          <div className="rounded-xl border border-zinc-800 bg-zinc-900/50 p-5">
            <h2 className="text-base font-bold uppercase tracking-wider text-amber-400 mb-4">
              Compiled Geographical Roadmap &amp; PONR Work Order
            </h2>

            {if plan.epochs.is_empty() {
              <div className="rounded-lg border border-dashed border-zinc-800 p-8 text-center text-sm text-zinc-400">
                Add items to your <strong>Current Run Roadmap</strong> above, or click <strong>📂 Import JSON</strong> to restore a saved run. The solver unrolls multi-region quest chains, sorts checkpoints by geographical progression weight along natural game flow, detects same-run mutual exclusivity collisions, and places Point-of-No-Return stop barriers inline.
              </div>
            }}
            {if !plan.epochs.is_empty() {
              <div className="space-y-6">
                {for (idx, ep) in plan.epochs.iter().enumerate() {
                  <details {raw: if !ep.all_clear { "open" } else { "" }} className="group rounded-xl border {raw: if ep.all_clear { "border-emerald-800/50 bg-emerald-950/10" } else { "border-zinc-700 bg-zinc-950/80" }} overflow-hidden transition">
                    <summary className="flex flex-wrap items-center justify-between gap-2 px-4 py-3.5 cursor-pointer select-none list-none hover:bg-zinc-900/50 transition">
                      <div className="flex items-center gap-2">
                        <span className="text-xs text-zinc-400 transition-transform duration-200 group-open:rotate-90">▶</span>
                        <span className="text-xs font-bold uppercase tracking-wider {raw: if ep.all_clear { "text-emerald-400" } else { "text-zinc-300" }}">
                          Route Leg {fmt: idx + 1} • {esc: ep.gate.campaign}
                        </span>
                      </div>
                      {if ep.gate.gate_id != "GATE_NONE" {
                        {if ep.all_clear {
                          <span className="rounded bg-emerald-950 px-2 py-0.5 text-xs font-semibold text-emerald-300 border border-emerald-700/50">
                            ✓ PONR Cleared — Safe to Cross {esc: ep.gate.title}
                          </span>
                        }}
                        {if !ep.all_clear {
                          <span className="rounded bg-red-950 px-2 py-0.5 text-xs font-semibold text-red-300 border border-red-700/50">
                            {fmt: ep.pending_count} Critical Step(s) Before Gate
                          </span>
                        }}
                      }}
                      {if ep.gate.gate_id == "GATE_NONE" {
                        {if ep.all_clear {
                          <span className="rounded bg-emerald-950 px-2 py-0.5 text-xs font-semibold text-emerald-300 border border-emerald-700/50">
                            ✓ All Sweep Steps Complete
                          </span>
                        }}
                        {if !ep.all_clear {
                          <span className="rounded bg-zinc-800 px-2 py-0.5 text-xs font-semibold text-zinc-300">
                            {fmt: ep.pending_count} Step(s) Remaining
                          </span>
                        }}
                      }}
                    </summary>

                    <div className="px-4 pb-4 pt-1 space-y-3 border-t border-zinc-800/60">
                      {for step in &ep.steps {
                        <form hx-post="/step/toggle" hx-target="#workspace" hx-swap="innerHTML" className="block">
                          <input type="hidden" name="step_key" value="{esc: step.step_key}" />
                          <label className="flex items-start gap-3 rounded-lg border {raw: if step.is_done { "border-zinc-800/50 bg-zinc-900/30 opacity-60" } else { "border-zinc-800/90 bg-zinc-900/70 hover:border-zinc-700" }} p-3 min-w-0 cursor-pointer select-none transition">
                            <input
                              type="checkbox"
                              {raw: if step.is_done { "checked" } else { "" }}
                              onchange="this.form.requestSubmit()"
                              className="mt-0.5 h-4 w-4 rounded border-zinc-600 bg-zinc-950 accent-emerald-500 cursor-pointer shrink-0"
                            />
                            <div className="flex-1 min-w-0">
                              <div className="flex flex-wrap items-center gap-2 mb-1">
                                <span className="inline-block rounded bg-zinc-800 px-2 py-0.5 text-xs font-semibold text-emerald-300 border border-zinc-700 break-words">
                                  📍 {esc: step.region_label}
                                </span>
                                {if step.is_gate_critical {
                                  <span className="inline-block rounded bg-amber-950/90 px-2 py-0.5 text-xs font-medium text-amber-300 border border-amber-700/50">
                                    ⚡ PONR-Sensitive Prerequisite
                                  </span>
                                }}
                                {if !step.is_gate_critical {
                                  <span className="inline-block rounded bg-zinc-900 px-2 py-0.5 text-xs text-zinc-400 border border-zinc-800">
                                    On-The-Way Regional Sweep
                                  </span>
                                }}
                              </div>
                              <p className="text-sm break-words {raw: if step.is_done { "line-through text-zinc-500" } else { "text-zinc-100" }}">
                                {esc: step.text}
                              </p>
                              <div className="mt-1.5 flex flex-wrap gap-1.5">
                                {for t in &step.targets {
                                  <span className="rounded bg-zinc-950 px-2 py-0.5 text-xs text-amber-300 border border-zinc-800 break-words">
                                    Target: {esc: t.name}
                                  </span>
                                }}
                              </div>
                            </div>
                          </label>
                        </form>
                      }}
                    </div>

                    {if ep.gate.gate_id != "GATE_NONE" {
                      <div className="border-t {raw: if ep.all_clear { "border-emerald-800/40 bg-emerald-950/30 text-emerald-300" } else { "border-red-800/70 bg-red-950/50 text-red-200" }} px-4 py-3">
                        <div className="text-xs font-bold uppercase tracking-wider">
                          {if ep.all_clear {
                            <span>🟢 SAFE TO CROSS POINT OF NO RETURN: {esc: ep.gate.title}</span>
                          }}
                          {if !ep.all_clear {
                            <span>🛑 POINT OF NO RETURN — DO NOT TRIGGER YET: {esc: ep.gate.title}</span>
                          }}
                        </div>
                        <div className="text-xs opacity-90 mt-0.5">
                          <strong>Trigger:</strong> {esc: ep.gate.trigger_desc} — {esc: ep.gate.warning_desc}
                        </div>
                      </div>
                    }}
                  </details>
                }}
              </div>
            }}
          </div>
        </div>
      </div>
    </template>
  );
}
