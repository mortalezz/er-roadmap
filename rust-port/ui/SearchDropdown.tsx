// AOT-Compiled Hypermedia JSX Component -> Rust Zero-Copy HTML Writer
// Compiled at `cargo build` time by `build.rs` into `render_search_results(buf: &mut String, rows: &[&'static CatalogItem], ordered_ids: &HashSet<String>)`

export function SearchDropdown(rows: CatalogItem[], ordered_ids: Set<string>) {
  return (
    <template name="render_search_results" args="rows: &[&'static CatalogItem], ordered_ids: &std::collections::HashSet<String>">
      <div className="divide-y divide-zinc-800/90 rounded-xl border border-amber-500/30 bg-zinc-900/95 backdrop-blur-md shadow-2xl max-h-[60vh] overflow-y-auto">
        <div className="sticky top-0 z-10 flex items-center justify-between bg-zinc-950/95 backdrop-blur px-3 py-1.5 text-xs text-zinc-400 border-b border-zinc-800">
          <span>Matching Catalog Items ({fmt: rows.len()})</span>
          <button type="button" onclick="closeSearchDropdown(false)" className="text-zinc-400 hover:text-zinc-200 px-1 py-0.5">
            Close ✕
          </button>
        </div>

        {if rows.is_empty() {
          <div className="p-3 text-sm text-zinc-400">No matching item in catalog.</div>
        }}

        {for r in rows {
          <div className="flex items-center justify-between gap-2 px-3 py-2.5 hover:bg-zinc-800/70 min-w-0">
            <div className="min-w-0">
              <div className="text-sm font-medium text-zinc-100 break-words">
                {esc: r.name}
                {if r.is_missable {
                  <span className="ml-2 rounded bg-amber-950/80 px-1.5 py-0.5 text-xs font-medium text-amber-300 border border-amber-700/50">Missable</span>
                }}
                {if ordered_ids.contains(r.item_id) {
                  <span className="ml-2 rounded bg-emerald-950/80 px-1.5 py-0.5 text-xs text-emerald-300 border border-emerald-700/50">In Roadmap</span>
                }}
              </div>
              <div className="text-xs text-zinc-400 break-words">{esc: r.campaign} • {esc: r.category} • {esc: r.region}</div>
            </div>
            <div className="flex items-center gap-1.5 shrink-0">
              {if ordered_ids.contains(r.item_id) {
                <form hx-post="/order/remove" hx-target="#workspace" hx-swap="innerHTML" hx-on::after-request="closeSearchDropdown()" className="inline">
                  <input type="hidden" name="item_id" value="{esc: r.item_id}" />
                  <button type="submit" className="rounded border border-red-700/70 bg-red-950/60 hover:bg-red-900 px-2.5 py-1 text-xs font-medium text-red-200 transition">
                    Remove
                  </button>
                </form>
              }}
              {if !ordered_ids.contains(r.item_id) {
                <form hx-post="/order/add" hx-target="#workspace" hx-swap="innerHTML" hx-on::after-request="closeSearchDropdown()" className="inline">
                  <input type="hidden" name="item_id" value="{esc: r.item_id}" />
                  <button type="submit" className="rounded bg-amber-600/90 hover:bg-amber-500 px-2.5 py-1 text-xs font-semibold text-zinc-950 transition">
                    + Add to Run
                  </button>
                </form>
              }}
            </div>
          </div>
        }}
      </div>
    </template>
  );
}
