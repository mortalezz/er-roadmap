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

# Supplemental CSS rules for perfect Tailwind fidelity, modals, and loading skeleton
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

/* Modal Overlay & Positioning primitives */
.fixed { position: fixed; }
.inset-0 { inset: 0px; top: 0; right: 0; bottom: 0; left: 0; }
.z-50 { z-index: 50; }
.hidden { display: none; }
.flex { display: flex; }
.items-start { align-items: flex-start; }
.items-center { align-items: center; }
.justify-center { justify-content: center; }
.justify-between { justify-content: space-between; }
.justify-end { justify-content: flex-end; }
.bg-black\\/80 { background-color: rgba(0, 0, 0, 0.8); }
.backdrop-blur-sm { -webkit-backdrop-filter: blur(8px); backdrop-filter: blur(8px); }
.max-w-lg { max-width: 32rem; }
.rounded-2xl { border-radius: 1rem; }
.bg-amber-950\\/20 { background-color: rgba(69, 26, 3, 0.2); }
.hover\\:bg-amber-900\\/30:hover { background-color: rgba(120, 53, 15, 0.3); }
.border-amber-500\\/40 { border-color: rgba(245, 158, 11, 0.4); }
.border-amber-500\\/30 { border-color: rgba(245, 158, 11, 0.3); }
.bg-amber-500\\/10 { background-color: rgba(245, 158, 11, 0.1); }
.overflow-y-auto { overflow-y: auto; }
.overflow-hidden { overflow: hidden; }
.select-all { user-select: all; -webkit-user-select: all; }
.cursor-pointer { cursor: pointer; }
.w-5 { width: 1.25rem; }
.h-5 { height: 1.25rem; }
.w-10 { width: 2.5rem; }
.h-10 { height: 2.5rem; }
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
<body hx-sync="this:queue" class="bg-zinc-950 text-zinc-100 min-h-screen w-full max-w-full overflow-x-hidden font-sans antialiased">
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
        <button
          type="button"
          onclick="openExportModal()"
          class="rounded-lg border border-zinc-700 bg-zinc-900 hover:bg-zinc-800 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-amber-300 transition flex items-center gap-1.5 cursor-pointer"
        >
          💾 Export JSON
        </button>

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
            class="rounded-lg border border-red-900/60 bg-red-950/40 hover:bg-red-900/60 active:scale-[0.98] px-3.5 py-2 text-xs font-semibold text-red-300 transition cursor-pointer"
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

  <!-- Export JSON Modal Overlay -->
  <div
    id="export-modal"
    class="fixed inset-0 z-50 hidden items-center justify-center bg-black/80 backdrop-blur-sm p-4 overflow-y-auto"
    onclick="handleExportModalBackdrop(event)"
  >
    <div
      class="relative w-full max-w-lg rounded-2xl border border-zinc-800 bg-zinc-950 p-6 shadow-2xl text-zinc-100"
      onclick="event.stopPropagation()"
    >
      <!-- Modal Header -->
      <div class="flex items-start justify-between gap-4 pb-4 border-b border-zinc-800">
        <div class="flex items-center gap-3">
          <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-amber-500/10 border border-amber-500/30 text-amber-400 text-lg">
            💾
          </div>
          <div>
            <h3 class="text-lg font-bold text-amber-400">Export Roadmap</h3>
            <p class="text-xs text-zinc-400">Save, share, or backup your run order and progress</p>
          </div>
        </div>
        <button
          type="button"
          onclick="closeExportModal()"
          class="rounded-lg p-1.5 text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800 transition cursor-pointer"
          aria-label="Close modal"
        >
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Modal Body / Export Options -->
      <div class="py-5 space-y-3">
        <!-- Option 1: Mobile Share / Save to Files (Native Share Sheet) -->
        <button
          id="export-share-btn"
          type="button"
          onclick="exportViaShare()"
          class="w-full flex items-center justify-between rounded-xl border border-amber-500/40 bg-amber-950/20 hover:bg-amber-900/30 active:scale-[0.99] p-3.5 text-left transition group cursor-pointer"
        >
          <div class="flex items-center gap-3">
            <span class="text-2xl">📱</span>
            <div>
              <div class="text-sm font-semibold text-amber-300 group-hover:text-amber-200">
                Share / Save to Files
              </div>
              <div class="text-xs text-zinc-400">
                Save to iPhone Files, AirDrop, or send to apps
              </div>
            </div>
          </div>
          <svg class="w-5 h-5 text-amber-400/70 group-hover:text-amber-300 transition shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>

        <!-- Option 2: Download File -->
        <button
          type="button"
          onclick="exportViaDownload()"
          class="w-full flex items-center justify-between rounded-xl border border-zinc-800 bg-zinc-900/70 hover:bg-zinc-800 active:scale-[0.99] p-3.5 text-left transition group cursor-pointer"
        >
          <div class="flex items-center gap-3">
            <span class="text-2xl">💾</span>
            <div>
              <div class="text-sm font-semibold text-zinc-200 group-hover:text-white">
                Download .json File
              </div>
              <div class="text-xs text-zinc-400">
                Direct browser download (<code class="text-zinc-300">elden-ring-roadmap.json</code>)
              </div>
            </div>
          </div>
          <svg class="w-5 h-5 text-zinc-500 group-hover:text-zinc-300 transition shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
        </button>

        <!-- Option 3: Copy JSON to Clipboard -->
        <button
          id="export-copy-btn"
          type="button"
          onclick="exportViaClipboard()"
          class="w-full flex items-center justify-between rounded-xl border border-zinc-800 bg-zinc-900/70 hover:bg-zinc-800 active:scale-[0.99] p-3.5 text-left transition group cursor-pointer"
        >
          <div class="flex items-center gap-3">
            <span id="export-copy-icon" class="text-2xl">📋</span>
            <div>
              <div id="export-copy-label" class="text-sm font-semibold text-zinc-200 group-hover:text-white">
                Copy JSON to Clipboard
              </div>
              <div id="export-copy-sub" class="text-xs text-zinc-400">
                Instant clipboard copy for Notes, Bear, or text backup
              </div>
            </div>
          </div>
          <svg class="w-5 h-5 text-zinc-500 group-hover:text-zinc-300 transition shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
          </svg>
        </button>

        <!-- Collapsible JSON Preview -->
        <details class="pt-2 text-xs text-zinc-400">
          <summary class="cursor-pointer font-medium text-zinc-400 hover:text-amber-400 transition select-none">
            Preview Raw JSON Data
          </summary>
          <div class="mt-2.5">
            <textarea
              id="export-preview-textarea"
              readonly
              rows="6"
              class="w-full rounded-lg border border-zinc-800 bg-zinc-950 p-2.5 font-mono text-[11px] text-zinc-300 focus:outline-none select-all"
              placeholder="Loading roadmap data..."
            ></textarea>
          </div>
        </details>
      </div>

      <!-- Modal Footer -->
      <div class="flex items-center justify-end gap-2.5 pt-4 border-t border-zinc-800">
        <button
          type="button"
          onclick="closeExportModal()"
          class="rounded-lg border border-zinc-700 bg-zinc-900 hover:bg-zinc-800 active:scale-[0.98] px-4 py-2 text-xs font-semibold text-zinc-300 hover:text-white transition cursor-pointer"
        >
          Cancel
        </button>
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

    // Auto-close search dropdown & export modal on Escape key
    document.addEventListener("keydown", function (e) {
      if (e.key === "Escape") {
        closeSearchDropdown(false);
        closeExportModal();
      }
    });

    // --- Export JSON Modal & Handlers ---
    let cachedExportJson = null;

    async function getExportPayload() {
      const resp = await fetch("/export", { headers: { "Accept": "application/json" } });
      if (!resp.ok) throw new Error("Failed to export roadmap session");
      const data = await resp.json();
      return JSON.stringify(data, null, 2);
    }

    async function openExportModal() {
      const modal = document.getElementById("export-modal");
      const preview = document.getElementById("export-preview-textarea");
      const shareBtn = document.getElementById("export-share-btn");

      if (!modal) return;
      modal.classList.remove("hidden");
      modal.classList.add("flex");
      document.body.classList.add("overflow-hidden");

      const canShare = typeof navigator !== "undefined" && typeof navigator.share === "function";
      if (shareBtn) {
        shareBtn.style.display = canShare ? "flex" : "none";
      }

      try {
        if (preview) preview.value = "Fetching roadmap JSON...";
        cachedExportJson = await getExportPayload();
        if (preview) preview.value = cachedExportJson;
      } catch (err) {
        if (preview) preview.value = "Error fetching export data: " + err.message;
      }
    }

    function closeExportModal() {
      const modal = document.getElementById("export-modal");
      if (!modal) return;
      modal.classList.add("hidden");
      modal.classList.remove("flex");
      document.body.classList.remove("overflow-hidden");
    }

    function handleExportModalBackdrop(event) {
      if (event.target.id === "export-modal") {
        closeExportModal();
      }
    }

    async function exportViaShare() {
      try {
        const jsonStr = cachedExportJson || (await getExportPayload());
        let fileToShare = null;
        try {
          fileToShare = new File([jsonStr], "elden-ring-roadmap.json", { type: "application/json" });
        } catch (_) {
          fileToShare = null;
        }

        if (navigator.canShare && fileToShare && navigator.canShare({ files: [fileToShare] })) {
          await navigator.share({
            files: [fileToShare],
            title: "Elden Ring Roadmap",
          });
          closeExportModal();
        } else if (navigator.share) {
          await navigator.share({
            title: "Elden Ring Roadmap",
            text: jsonStr,
          });
          closeExportModal();
        } else {
          exportViaDownload();
        }
      } catch (err) {
        // If user dismissed/cancelled the share sheet (AbortError), stay on page peacefully
        if (err.name !== "AbortError") {
          console.warn("Share failed:", err);
          exportViaDownload();
        }
      }
    }

    async function exportViaDownload() {
      try {
        const jsonStr = cachedExportJson || (await getExportPayload());
        const blob = new Blob([jsonStr], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "elden-ring-roadmap.json";
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        setTimeout(() => URL.revokeObjectURL(url), 1000);
        closeExportModal();
      } catch (err) {
        alert("Download failed: " + err.message);
      }
    }

    async function exportViaClipboard() {
      try {
        const jsonStr = cachedExportJson || (await getExportPayload());
        if (navigator.clipboard && navigator.clipboard.writeText) {
          await navigator.clipboard.writeText(jsonStr);
        } else {
          const preview = document.getElementById("export-preview-textarea");
          if (preview) {
            preview.select();
            document.execCommand("copy");
          }
        }

        const label = document.getElementById("export-copy-label");
        const sub = document.getElementById("export-copy-sub");
        const icon = document.getElementById("export-copy-icon");

        if (label && sub && icon) {
          const origLabel = label.innerText;
          const origSub = sub.innerText;
          const origIcon = icon.innerText;

          label.innerText = "Copied to Clipboard!";
          label.className = "text-sm font-semibold text-emerald-400";
          sub.innerText = "JSON copied — paste anywhere";
          icon.innerText = "✅";

          setTimeout(() => {
            label.innerText = origLabel;
            label.className = "text-sm font-semibold text-zinc-200 group-hover:text-white";
            sub.innerText = origSub;
            icon.innerText = origIcon;
          }, 2500);
        }
      } catch (err) {
        alert("Could not copy to clipboard: " + err.message);
      }
    }
  </script>
</body>
</html>
"""

with open(os.path.join(DIST, "index.html"), "w") as f:
    f.write(index_html)

print("==> Frontend dist/ build completed successfully!")
