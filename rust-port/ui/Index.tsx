// AOT-Compiled Hypermedia JSX Component -> Rust Zero-Copy HTML Writer
// Compiled at `cargo build` time by `build.rs` into `render_index(buf: &mut String, plan: &RunPlan)`
// 100% Standalone Offline Ready: Embeds complete Tailwind utility stylesheet + Native WKWebView / HTTP hypermedia bridge.

export function IndexPage(plan: RunPlan) {
  return (
    <template name="render_index" args="plan: &RunPlan">
      <!DOCTYPE html>
      <html lang="en" className="dark overflow-x-hidden">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover" />
        <title>Elden Ring Current Run Roadmap</title>
        <meta name="theme-color" content="#09090b" />
        <style>
          {raw: crate::OFFLINE_STYLESHEET}
        </style>
      </head>
      <body className="bg-zinc-950 bg-[radial-gradient(ellipse_80%_50%_at_50%_-10%,rgba(245,158,11,0.10),transparent)] text-zinc-100 min-h-screen w-full max-w-full overflow-x-hidden font-sans antialiased">
        <div className="max-w-7xl w-full mx-auto px-4 pt-6 pb-12 overflow-x-visible">
          <div className="flex flex-col md:flex-row md:items-center md:justify-between border-b border-zinc-800/90 pb-6 mb-7 gap-4">
            <div className="flex items-center gap-3.5">
              <img
                src="{raw: crate::OFFLINE_EMBLEM_DATA_URI}"
                alt="ER Roadmap Emblem"
                className="h-11 w-11 rounded-xl border border-amber-500/40 shadow-[0_0_15px_rgba(245,158,11,0.18)] shrink-0"
              />
              <div>
                <div className="flex items-center gap-2">
                  <h1 className="text-2xl font-bold tracking-tight text-amber-400">
                    Elden Ring Current Run Roadmap
                  </h1>
                  <span className="rounded bg-amber-500/15 border border-amber-500/40 px-2 py-0.5 text-[11px] font-mono font-semibold text-amber-300">
                    🦀 Native Offline App
                  </span>
                </div>
                <p className="text-sm text-zinc-400 mt-0.5">
                  Demand-driven geographical route &amp; Point-of-No-Return solver (Base Game + Shadow of the Erdtree)
                </p>
              </div>
            </div>

            <div className="flex flex-wrap items-center gap-2.5">
              <button
                type="button"
                onclick="window.__erExportJson()"
                className="rounded-lg border border-zinc-700 bg-zinc-900 hover:bg-zinc-800 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-amber-300 transition flex items-center gap-1.5 cursor-pointer"
              >
                💾 Export JSON
              </button>

              <button
                type="button"
                onclick="window.__erImportJson()"
                className="rounded-lg border border-zinc-700 bg-zinc-900 hover:bg-zinc-800 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-emerald-300 transition flex items-center gap-1.5 cursor-pointer"
              >
                📂 Import JSON
              </button>

              <form hx-post="/order/clear" hx-target="#workspace" hx-swap="innerHTML" className="inline">
                <button
                  type="submit"
                  className="rounded-lg border border-red-900/60 bg-red-950/40 hover:bg-red-900/60 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-red-300 transition cursor-pointer"
                >
                  Reset Slate
                </button>
              </form>
            </div>
          </div>

          <div id="search-container" className="relative z-40 rounded-xl border border-zinc-800 bg-zinc-900/70 backdrop-blur-sm p-4 mb-6 shadow-lg">
            <label for="order-search-input" className="block text-xs font-semibold uppercase tracking-wider text-amber-400 mb-1.5">
              Add Desirables to Current Run Roadmap (Compiles Geographical Route &amp; PONR Gates)
            </label>
            <input
              id="order-search-input"
              type="text"
              name="q"
              placeholder="Type any Base Game or DLC item (e.g. Magic Scorpion, Bolt of Gransax, Florissax, Swift Slash, Euporia)..."
              autocomplete="off"
              autocorrect="off"
              autocapitalize="off"
              spellcheck="false"
              hx-get="/search"
              hx-trigger="input delay:150ms, focus, refreshSearch from:body"
              hx-target="#order-search-results"
              className="w-full rounded-lg border border-zinc-700 bg-zinc-950 px-3.5 py-2.5 text-sm text-zinc-100 placeholder-zinc-500 focus:border-amber-500 focus:ring-1 focus:ring-amber-500/40 focus:outline-none transition"
            />
            <div id="order-search-results" className="absolute left-4 right-4 top-full -mt-1 z-50"></div>
          </div>

          <div id="workspace">
            {call: render_workspace(buf, plan)}
          </div>
        </div>

        <script>
          {raw: crate::OFFLINE_RUNTIME_JS}
        </script>
      </body>
      </html>
    </template>
  );
}
