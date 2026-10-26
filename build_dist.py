#!/usr/bin/env python3
"""
build_dist.py
Builds the static frontend directory (dist/) for FastAPI Cloud deployment.
- Emits precompiled standalone CSS (zero runtime JIT compile delay)
- Bundles local htmx.min.js (zero unpkg.com network roundtrip)
- Copies all static assets and icons
- Generates dist/index.html containing the cached application shell + HTMX lazy loader
"""

import os
import shutil
import urllib.request

ROOT = os.path.dirname(os.path.abspath(__file__))
DIST = os.path.join(ROOT, "dist")
STATIC_SRC = os.path.join(ROOT, "static")

print("==> Cleaning and creating dist/...")
if os.path.exists(DIST):
    shutil.rmtree(DIST)
os.makedirs(os.path.join(DIST, "assets"), exist_ok=True)
os.makedirs(os.path.join(DIST, "static"), exist_ok=True)

# 1. Copy static images / assets
print("==> Copying static assets to dist/static/ and dist/ root...")
for fname in os.listdir(STATIC_SRC):
    src = os.path.join(STATIC_SRC, fname)
    if os.path.isfile(src):
        shutil.copy2(src, os.path.join(DIST, "static", fname))
        # Also copy standard root files so /favicon.ico and /apple-touch-icon.png resolve directly
        if fname in ("favicon-32.png", "favicon.svg", "apple-touch-icon.png", "site.webmanifest"):
            shutil.copy2(src, os.path.join(DIST, fname))
            if fname == "favicon-32.png":
                shutil.copy2(src, os.path.join(DIST, "favicon.ico"))

# 2. Bundle local htmx.min.js
htmx_path = os.path.join(DIST, "assets", "htmx.min.js")
cached_htmx = os.path.join(STATIC_SRC, "htmx.min.js")
if os.path.exists(cached_htmx):
    print("==> Copying cached htmx.min.js from static/...")
    shutil.copy2(cached_htmx, htmx_path)
else:
    print("==> Downloading and caching htmx.min.js...")
    htmx_url = "https://unpkg.com/htmx.org@1.9.12/dist/htmx.min.js"
    urllib.request.urlretrieve(htmx_url, htmx_path)

# 3. Extract and compile CSS
print("==> Compiling dist/assets/app.css...")
with open(os.path.join(ROOT, "rust-port", "src", "offline_assets.rs"), "r") as f:
    rust_src = f.read()

start_marker = 'pub static OFFLINE_STYLESHEET: &str = r##"'
end_marker = '"##;'
css_start = rust_src.find(start_marker) + len(start_marker)
css_end = rust_src.find(end_marker, css_start)
base_css = rust_src[css_start:css_end].strip()

# Supplemental CSS rules for perfect Tailwind fidelity and loading skeleton
supplemental_css = """
/* Extra Tailwind utility classes & Loading Skeleton */
.antialiased { -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; }
.font-sans { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; }
.list-none { list-style: none; }
.duration-200 { transition-duration: 200ms; }
.duration-500 { transition-duration: 500ms; }
.ease-out { transition-timing-function: cubic-bezier(0, 0, 0.2, 1); }
.focus\\:outline-none:focus { outline: 2px solid transparent; outline-offset: 2px; }
.focus\\:ring-1:focus { --tw-ring-offset-shadow: var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color); --tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color); box-shadow: var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000); }
.focus\\:ring-amber-500\\/40:focus { --tw-ring-color: rgba(245, 158, 11, 0.4); }
.placeholder-zinc-500::placeholder { color: #71717a; }
.shadow-\\[0_0_15px_rgba\\(245\\,158\\,11\\,0\\.18\\)\\] { box-shadow: 0 0 15px rgba(245, 158, 11, 0.18); }
.from-amber-500 { --tw-gradient-from: #f59e0b; --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, rgba(245, 158, 11, 0)); }
.via-amber-400 { --tw-gradient-stops: var(--tw-gradient-from), #fbbf24, var(--tw-gradient-to, rgba(251, 191, 36, 0)); }
.to-emerald-400 { --tw-gradient-to: #34d399; }
.bg-gradient-to-r { background-image: linear-gradient(to right, var(--tw-gradient-stops)); }
.divide-zinc-800\\/90 > :not([hidden]) ~ :not([hidden]) { border-color: rgba(39, 39, 42, 0.9); }

/* Animation Pulse for Loading Skeleton */
@keyframes pulse {
  50% { opacity: .45; }
}
.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
"""

with open(os.path.join(DIST, "assets", "app.css"), "w") as f:
    f.write(base_css + "\n" + supplemental_css)

# 4. Generate dist/index.html
print("==> Generating dist/index.html static shell...")
index_html = """<!DOCTYPE html>
<html lang="en" class="dark overflow-x-hidden">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover" />
  <title>Elden Ring Current Run Roadmap</title>
  <meta name="theme-color" content="#09090b" />
  <meta name="apple-mobile-web-app-capable" content="yes" />
  <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
  <meta name="apple-mobile-web-app-title" content="ER Roadmap" />
  <link rel="icon" type="image/svg+xml" href="/static/favicon.svg" />
  <link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32.png" />
  <link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png" />
  <link rel="manifest" href="/static/site.webmanifest" />
  <link rel="stylesheet" href="/assets/app.css" />
  <script src="/assets/htmx.min.js"></script>
</head>
<body class="bg-zinc-950 text-zinc-100 min-h-screen w-full max-w-full overflow-x-hidden font-sans antialiased">
  <div class="max-w-7xl w-full mx-auto px-4 pt-6 pb-12 overflow-x-visible">
    <!-- Header -->
    <div class="flex flex-col md:flex-row md:items-center md:justify-between border-b border-zinc-800/90 pb-6 mb-7 gap-4">
      <div class="flex items-center gap-3.5">
        <img
          src="/static/icon-192.png"
          alt="ER Roadmap Emblem"
          class="h-11 w-11 rounded-xl border border-amber-500/40 shadow-[0_0_15px_rgba(245,158,11,0.18)] shrink-0"
        />
        <div>
          <h1 class="text-2xl font-bold tracking-tight text-amber-400">
            Elden Ring Current Run Roadmap
          </h1>
          <p class="text-sm text-zinc-400 mt-0.5">
            Demand-driven geographical route &amp; Point-of-No-Return solver (Base Game + Shadow of the Erdtree)
          </p>
        </div>
      </div>

      <!-- Export / Import / Reset Controls -->
      <div class="flex flex-wrap items-center gap-2.5">
        <a
          href="/export"
          class="rounded-lg border border-zinc-700 bg-zinc-900 hover:bg-zinc-800 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-amber-300 transition flex items-center gap-1.5"
        >
          💾 Export JSON
        </a>

        <form
          hx-post="/import"
          hx-encoding="multipart/form-data"
          hx-target="#workspace"
          hx-swap="innerHTML"
          class="inline"
        >
          <label class="cursor-pointer rounded-lg border border-zinc-700 bg-zinc-900 hover:bg-zinc-800 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-emerald-300 transition flex items-center gap-1.5">
            📂 Import JSON
            <input
              type="file"
              name="file"
              accept=".json,application/json"
              onchange="this.form.requestSubmit(); this.value='';"
              class="hidden"
            />
          </label>
        </form>

        <form hx-post="/order/clear" hx-target="#workspace" hx-swap="innerHTML" class="inline">
          <button
            type="submit"
            class="rounded-lg border border-red-900/60 bg-red-950/40 hover:bg-red-900/60 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-red-300 transition"
          >
            Reset Slate
          </button>
        </form>
      </div>
    </div>

    <!-- Single Typeahead Search Bar (Relative container with floating overlay dropdown) -->
    <div id="search-container" class="relative z-40 rounded-xl border border-zinc-800 bg-zinc-900/70 backdrop-blur-sm p-4 mb-6 shadow-lg">
      <label for="order-search-input" class="block text-xs font-semibold uppercase tracking-wider text-amber-400 mb-1.5">
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
        hx-trigger="input delay:150ms, focus"
        hx-target="#order-search-results"
        class="w-full rounded-lg border border-zinc-700 bg-zinc-950 px-3.5 py-2.5 text-sm text-zinc-100 placeholder-zinc-500 focus:border-amber-500 focus:ring-1 focus:ring-amber-500/40 focus:outline-none transition"
      />
      <div id="order-search-results" class="absolute left-4 right-4 top-full -mt-1 z-50"></div>
    </div>

    <!-- Live Roadmap Workspace (Lazy loaded from cookie-backed /workspace endpoint) -->
    <div id="workspace" hx-get="/workspace" hx-trigger="load" hx-swap="innerHTML">
      <!-- High-fidelity dark loading skeleton for instant first paint -->
      <div class="animate-pulse space-y-6">
        <!-- Progress Bar Skeleton -->
        <div class="rounded-xl border border-zinc-800 bg-zinc-900/80 px-5 py-4 shadow-md">
          <div class="flex items-center justify-between gap-4 mb-3">
            <div class="flex items-center gap-5">
              <div>
                <div class="h-3 w-24 bg-zinc-800 rounded mb-1.5"></div>
                <div class="h-5 w-32 bg-zinc-800/80 rounded"></div>
              </div>
              <div class="h-8 w-px bg-zinc-800"></div>
              <div>
                <div class="h-3 w-28 bg-zinc-800 rounded mb-1.5"></div>
                <div class="h-5 w-36 bg-zinc-800/80 rounded"></div>
              </div>
              <div class="h-8 w-px bg-zinc-800"></div>
              <div>
                <div class="h-3 w-20 bg-zinc-800 rounded mb-1.5"></div>
                <div class="h-5 w-12 bg-zinc-800/80 rounded"></div>
              </div>
            </div>
          </div>
          <div class="h-1.5 w-full bg-zinc-800/60 rounded-full overflow-hidden">
            <div class="h-full bg-amber-500/40 w-1/4 rounded-full"></div>
          </div>
        </div>

        <!-- 2 Column Layout Skeleton -->
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
          <!-- Left Column -->
          <div class="lg:col-span-4 rounded-xl border border-zinc-800 bg-zinc-900/80 p-5 shadow-md">
            <div class="h-4 w-40 bg-zinc-800 rounded mb-4"></div>
            <div class="space-y-3">
              <div class="h-14 bg-zinc-950/80 border border-zinc-800/80 rounded-lg"></div>
              <div class="h-14 bg-zinc-950/80 border border-zinc-800/80 rounded-lg"></div>
            </div>
          </div>
          <!-- Right Column -->
          <div class="lg:col-span-8 rounded-xl border border-zinc-800 bg-zinc-900/80 p-5 shadow-md">
            <div class="h-4 w-64 bg-zinc-800 rounded mb-4"></div>
            <div class="space-y-4">
              <div class="h-28 bg-zinc-950/80 border border-zinc-800/80 rounded-lg"></div>
              <div class="h-28 bg-zinc-950/80 border border-zinc-800/80 rounded-lg"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <script>
    function closeSearchDropdown(clearInput) {
      if (clearInput === undefined) clearInput = true;
      const results = document.getElementById("order-search-results");
      const input = document.getElementById("order-search-input");
      if (results) results.innerHTML = "";
      if (input) {
        if (clearInput) input.value = "";
        input.blur();
      }
    }

    // Auto-close search dropdown when clicking/tapping outside #search-container
    document.addEventListener("pointerdown", function (e) {
      const container = document.getElementById("search-container");
      const results = document.getElementById("order-search-results");
      if (container && results && results.innerHTML.trim() !== "" && !container.contains(e.target)) {
        results.innerHTML = "";
      }
    });

    // Auto-close search dropdown on Escape key
    document.addEventListener("keydown", function (e) {
      if (e.key === "Escape") {
        closeSearchDropdown(false);
      }
    });
  </script>
</body>
</html>
"""

with open(os.path.join(DIST, "index.html"), "w") as f:
    f.write(index_html)

print("==> Frontend dist/ build completed successfully!")
