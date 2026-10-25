pub static INLINE_EMBLEM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" style="width:100%;height:100%;display:block"><defs><radialGradient id="bgGlow" cx="50%" cy="45%" r="60%"><stop offset="0%" stop-color="#272015"/><stop offset="65%" stop-color="#09090b"/><stop offset="100%" stop-color="#050507"/></radialGradient><linearGradient id="goldGrad" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" stop-color="#fde68a"/><stop offset="45%" stop-color="#f59e0b"/><stop offset="100%" stop-color="#b45309"/></linearGradient><linearGradient id="goldBright" x1="0%" y1="0%" x2="0%" y2="100%"><stop offset="0%" stop-color="#fef3c7"/><stop offset="50%" stop-color="#fbbf24"/><stop offset="100%" stop-color="#d97706"/></linearGradient></defs><rect width="512" height="512" rx="112" fill="url(#bgGlow)"/><rect x="14" y="14" width="484" height="484" rx="98" fill="none" stroke="url(#goldGrad)" stroke-width="6" opacity="0.35"/><path d="M 96 148 Q 256 248 416 148" fill="none" stroke="url(#goldBright)" stroke-width="20" stroke-linecap="round"/><path d="M 136 356 Q 256 420 376 356" fill="none" stroke="url(#goldGrad)" stroke-width="14" stroke-linecap="round" opacity="0.9"/><circle cx="256" cy="244" r="96" fill="none" stroke="url(#goldBright)" stroke-width="18"/><circle cx="204" cy="288" r="78" fill="none" stroke="url(#goldGrad)" stroke-width="14" opacity="0.92"/><circle cx="308" cy="288" r="78" fill="none" stroke="url(#goldGrad)" stroke-width="14" opacity="0.92"/><line x1="256" y1="68" x2="256" y2="444" stroke="url(#goldBright)" stroke-width="20" stroke-linecap="round"/><polygon points="256,192 272,240 320,256 272,272 256,320 240,272 192,256 240,240" fill="#fef3c7"/></svg>"##;

pub static OFFLINE_STYLESHEET: &str = r##"
*, ::before, ::after { box-sizing: border-box; border-width: 0; border-style: solid; border-color: #27272a; }
html, body, * { scrollbar-width: none; -ms-overflow-style: none; }
html::-webkit-scrollbar, body::-webkit-scrollbar, *::-webkit-scrollbar { display: none; width: 0; height: 0; }
html { line-height: 1.5; -webkit-text-size-adjust: 100%; font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", Roboto, Helvetica, Arial, sans-serif; background-color: #09090b; color: #f4f4f5; }
body { margin: 0; line-height: inherit; background-color: #09090b; background-image: radial-gradient(ellipse 80% 50% at 50% -10%, rgba(245, 158, 11, 0.10), transparent); color: #f4f4f5; min-height: 100vh; width: 100%; max-width: 100%; overflow-x: hidden; -webkit-font-smoothing: antialiased; }
h1, h2, p { margin: 0; }
code, .font-mono { font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; }
button, input { font-family: inherit; font-size: 100%; line-height: inherit; color: inherit; margin: 0; padding: 0; background-color: transparent; }
button { cursor: pointer; }
summary { list-style: none; }
summary::-webkit-details-marker { display: none; }

.max-w-7xl { max-width: 80rem; }
.max-w-full { max-width: 100%; }
.w-full { width: 100%; }
.w-11 { width: 2.75rem; }
.w-4 { width: 1rem; }
.w-px { width: 1px; }
.h-11 { height: 2.75rem; }
.h-8 { height: 2rem; }
.h-4 { height: 1rem; }
.h-1 { height: 0.25rem; }
.h-full { height: 100%; }
.min-h-screen { min-height: 100vh; }
.min-w-0 { min-width: 0px; }
.max-h-\[60vh\] { max-height: 60vh; }
.mx-auto { margin-left: auto; margin-right: auto; }
.mb-1 { margin-bottom: 0.25rem; }
.mb-1\.5 { margin-bottom: 0.375rem; }
.mb-2 { margin-bottom: 0.5rem; }
.mb-3 { margin-bottom: 0.75rem; }
.mb-4 { margin-bottom: 1rem; }
.mb-6 { margin-bottom: 1.5rem; }
.mb-7 { margin-bottom: 1.75rem; }
.mt-0\.5 { margin-top: 0.125rem; }
.mt-1\.5 { margin-top: 0.375rem; }
.-mt-1 { margin-top: -0.25rem; }
.ml-1 { margin-left: 0.25rem; }
.ml-2 { margin-left: 0.5rem; }

.p-3 { padding: 0.75rem; }
.p-4 { padding: 1rem; }
.p-5 { padding: 1.25rem; }
.p-8 { padding: 2rem; }
.px-1 { padding-left: 0.25rem; padding-right: 0.25rem; }
.px-1\.5 { padding-left: 0.375rem; padding-right: 0.375rem; }
.px-2 { padding-left: 0.5rem; padding-right: 0.5rem; }
.px-2\.5 { padding-left: 0.625rem; padding-right: 0.625rem; }
.px-3 { padding-left: 0.75rem; padding-right: 0.75rem; }
.px-3\.5 { padding-left: 0.875rem; padding-right: 0.875rem; }
.px-4 { padding-left: 1rem; padding-right: 1rem; }
.px-5 { padding-left: 1.25rem; padding-right: 1.25rem; }
.py-0\.5 { padding-top: 0.125rem; padding-bottom: 0.125rem; }
.py-1 { padding-top: 0.25rem; padding-bottom: 0.25rem; }
.py-1\.5 { padding-top: 0.375rem; padding-bottom: 0.375rem; }
.py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }
.py-2\.5 { padding-top: 0.625rem; padding-bottom: 0.625rem; }
.py-3 { padding-top: 0.75rem; padding-bottom: 0.75rem; }
.py-3\.5 { padding-top: 0.875rem; padding-bottom: 0.875rem; }
.py-4 { padding-top: 1rem; padding-bottom: 1rem; }
.pt-1 { padding-top: 0.25rem; }
.pt-6 { padding-top: 1.5rem; }
.pb-4 { padding-bottom: 1rem; }
.pb-6 { padding-bottom: 1.5rem; }
.pb-12 { padding-bottom: 3rem; }

.block { display: block; }
.inline { display: inline; }
.inline-block { display: inline-block; }
.hidden { display: none; }
.flex { display: flex; }
.flex-1 { flex: 1 1 0%; }
.flex-col { flex-direction: column; }
.flex-wrap { flex-wrap: wrap; }
.items-center { align-items: center; }
.items-start { align-items: flex-start; }
.justify-between { justify-content: space-between; }
.shrink-0 { flex-shrink: 0; }
.gap-1\.5 { gap: 0.375rem; }
.gap-2 { gap: 0.5rem; }
.gap-2\.5 { gap: 0.625rem; }
.gap-3 { gap: 0.75rem; }
.gap-3\.5 { gap: 0.875rem; }
.gap-4 { gap: 1rem; }
.gap-5 { gap: 1.25rem; }
.gap-6 { gap: 1.5rem; }

.grid { display: grid; }
.grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
.space-y-2 > :not([hidden]) ~ :not([hidden]) { margin-top: 0.5rem; }
.space-y-3 > :not([hidden]) ~ :not([hidden]) { margin-top: 0.75rem; }
.space-y-6 > :not([hidden]) ~ :not([hidden]) { margin-top: 1.5rem; }
.divide-y > :not([hidden]) ~ :not([hidden]) { border-top-width: 1px; border-color: rgba(39, 39, 42, 0.9); }

.relative { position: relative; }
.absolute { position: absolute; }
.sticky { position: sticky; }
.top-0 { top: 0px; }
.top-full { top: 100%; }
.bottom-0 { bottom: 0px; }
.left-0 { left: 0px; }
.right-0 { right: 0px; }
.left-4 { left: 1rem; }
.right-4 { right: 1rem; }
.z-10 { z-index: 10; }
.z-40 { z-index: 40; }
.z-50 { z-index: 50; }

.overflow-hidden { overflow: hidden; }
.overflow-x-hidden { overflow-x: hidden; }
.overflow-x-visible { overflow-x: visible; }
.overflow-y-auto { overflow-y: auto; }

.rounded { border-radius: 0.25rem; }
.rounded-lg { border-radius: 0.5rem; }
.rounded-xl { border-radius: 0.75rem; }
.border { border-width: 1px; }
.border-2 { border-width: 2px; }
.border-b { border-bottom-width: 1px; }
.border-t { border-top-width: 1px; }
.border-dashed { border-style: dashed; }

.border-zinc-600 { border-color: #52525b; }
.border-zinc-700 { border-color: #3f3f46; }
.border-zinc-800 { border-color: #27272a; }
.border-zinc-800\/50 { border-color: rgba(39, 39, 42, 0.5); }
.border-zinc-800\/60 { border-color: rgba(39, 39, 42, 0.6); }
.border-zinc-800\/90 { border-color: rgba(39, 39, 42, 0.9); }
.border-amber-500\/30 { border-color: rgba(245, 158, 11, 0.3); }
.border-amber-500\/40 { border-color: rgba(245, 158, 11, 0.4); }
.border-amber-700\/50 { border-color: rgba(180, 83, 9, 0.5); }
.border-emerald-700\/50 { border-color: rgba(4, 120, 87, 0.5); }
.border-emerald-800\/40 { border-color: rgba(6, 95, 70, 0.4); }
.border-emerald-800\/50 { border-color: rgba(6, 95, 70, 0.5); }
.border-red-500\/80 { border-color: rgba(239, 68, 68, 0.8); }
.border-red-600\/70 { border-color: rgba(220, 38, 38, 0.7); }
.border-red-700\/50 { border-color: rgba(185, 28, 28, 0.5); }
.border-red-700\/70 { border-color: rgba(185, 28, 28, 0.7); }
.border-red-800\/60 { border-color: rgba(153, 27, 27, 0.6); }
.border-red-800\/70 { border-color: rgba(153, 27, 27, 0.7); }
.border-red-900\/60 { border-color: rgba(127, 29, 29, 0.6); }

.bg-zinc-800 { background-color: #27272a; }
.bg-zinc-800\/80 { background-color: rgba(39, 39, 42, 0.8); }
.bg-zinc-900 { background-color: #18181b; }
.bg-zinc-900\/30 { background-color: rgba(24, 24, 27, 0.3); }
.bg-zinc-900\/50 { background-color: rgba(24, 24, 27, 0.5); }
.bg-zinc-900\/70 { background-color: rgba(24, 24, 27, 0.7); }
.bg-zinc-900\/80 { background-color: rgba(24, 24, 27, 0.8); }
.bg-zinc-900\/95 { background-color: rgba(24, 24, 27, 0.95); }
.bg-zinc-950 { background-color: #09090b; }
.bg-zinc-950\/40 { background-color: rgba(9, 9, 11, 0.4); }
.bg-zinc-950\/80 { background-color: rgba(9, 9, 11, 0.8); }
.bg-zinc-950\/95 { background-color: rgba(9, 9, 11, 0.95); }
.bg-amber-500\/15 { background-color: rgba(245, 158, 11, 0.15); }
.bg-amber-600\/90 { background-color: rgba(217, 119, 6, 0.9); }
.bg-amber-950\/80 { background-color: rgba(69, 26, 3, 0.8); }
.bg-amber-950\/90 { background-color: rgba(69, 26, 3, 0.9); }
.bg-emerald-950 { background-color: #022c22; }
.bg-emerald-950\/10 { background-color: rgba(2, 44, 34, 0.1); }
.bg-emerald-950\/30 { background-color: rgba(2, 44, 34, 0.3); }
.bg-emerald-950\/80 { background-color: rgba(2, 44, 34, 0.8); }
.bg-red-900\/40 { background-color: rgba(127, 29, 29, 0.4); }
.bg-red-950 { background-color: #450a0a; }
.bg-red-950\/40 { background-color: rgba(69, 10, 10, 0.4); }
.bg-red-950\/50 { background-color: rgba(69, 10, 10, 0.5); }
.bg-red-950\/60 { background-color: rgba(69, 10, 10, 0.6); }
.bg-gradient-to-r { background-image: linear-gradient(to right, #f59e0b, #fbbf24, #34d399); }

.text-2xl { font-size: 1.5rem; line-height: 2rem; }
.text-lg { font-size: 1.125rem; line-height: 1.75rem; }
.text-base { font-size: 1rem; line-height: 1.5rem; }
.text-sm { font-size: 0.875rem; line-height: 1.25rem; }
.text-xs { font-size: 0.75rem; line-height: 1rem; }
.text-\[11px\] { font-size: 11px; line-height: 1.2; }
.font-bold { font-weight: 700; }
.font-semibold { font-weight: 600; }
.font-medium { font-weight: 500; }
.font-normal { font-weight: 400; }
.uppercase { text-transform: uppercase; }
.tracking-tight { letter-spacing: -0.025em; }
.tracking-wider { letter-spacing: 0.05em; }
.text-center { text-align: center; }
.line-through { text-decoration-line: line-through; }
.break-words { overflow-wrap: break-word; word-break: break-word; }

.text-zinc-100 { color: #f4f4f5; }
.text-zinc-200 { color: #e4e4e7; }
.text-zinc-300 { color: #d4d4d8; }
.text-zinc-400 { color: #a1a1aa; }
.text-zinc-500 { color: #71717a; }
.text-zinc-950 { color: #09090b; }
.text-amber-300 { color: #fcd34d; }
.text-amber-400 { color: #fbbf24; }
.text-emerald-300 { color: #6ee7b7; }
.text-emerald-400 { color: #34d399; }
.text-red-200 { color: #fecaca; }
.text-red-300 { color: #fca5a5; }
input::placeholder { color: #71717a; }

.opacity-60 { opacity: 0.6; }
.opacity-65 { opacity: 0.65; }
.opacity-90 { opacity: 0.9; }
.shadow-md { box-shadow: 0 4px 6px -1px rgba(0,0,0,0.4); }
.shadow-lg { box-shadow: 0 10px 15px -3px rgba(0,0,0,0.5); }
.shadow-2xl { box-shadow: 0 25px 50px -12px rgba(0,0,0,0.75); }
.shadow-\[0_0_15px_rgba\(245\,158\,11\,0\.18\)\] { box-shadow: 0 0 15px rgba(245, 158, 11, 0.18); }
.backdrop-blur, .backdrop-blur-sm, .backdrop-blur-md { -webkit-backdrop-filter: blur(10px); backdrop-filter: blur(10px); }

.cursor-pointer { cursor: pointer; }
.select-none { -webkit-user-select: none; user-select: none; }
.accent-amber-500 { accent-color: #f59e0b; }
.accent-emerald-500 { accent-color: #10b981; }
.transition, .transition-all, .transition-transform { transition: all 150ms cubic-bezier(0.4, 0, 0.2, 1); }
details[open] > summary .group-open\:rotate-90 { transform: rotate(90deg); display: inline-block; }

.hover\:bg-amber-500:hover { background-color: #f59e0b; }
.hover\:bg-red-800\/60:hover { background-color: rgba(153, 27, 27, 0.6); }
.hover\:bg-red-900:hover { background-color: #7f1d1d; }
.hover\:bg-red-900\/60:hover { background-color: rgba(127, 29, 29, 0.6); }
.hover\:bg-red-900\/70:hover { background-color: rgba(127, 29, 29, 0.7); }
.hover\:bg-red-950:hover { background-color: #450a0a; }
.hover\:bg-zinc-800:hover { background-color: #27272a; }
.hover\:bg-zinc-800\/70:hover { background-color: rgba(39, 39, 42, 0.7); }
.hover\:bg-zinc-900\/50:hover { background-color: rgba(24, 24, 27, 0.5); }
.hover\:border-red-700:hover { border-color: #b91c1c; }
.hover\:border-zinc-700:hover { border-color: #3f3f46; }
.hover\:text-red-300:hover { color: #fca5a5; }
.hover\:text-zinc-200:hover { color: #e4e4e7; }
.focus\:border-amber-500:focus { border-color: #f59e0b; outline: none; box-shadow: 0 0 0 2px rgba(245, 158, 11, 0.35); }
.active\:scale-\[0\.98\]:active { transform: scale(0.98); }

@media (min-width: 640px) {
  .sm\:gap-6 { gap: 1.5rem; }
}
@media (min-width: 768px) {
  .md\:flex-row { flex-direction: row; }
  .md\:items-center { align-items: center; }
  .md\:justify-between { justify-content: space-between; }
}
@media (min-width: 1024px) {
  .lg\:grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }
  .lg\:col-span-4 { grid-column: span 4 / span 4; }
  .lg\:col-span-8 { grid-column: span 8 / span 8; }
}
"##;

pub static OFFLINE_RUNTIME_JS: &str = r##"
function closeSearchDropdown(clearInput) {
  if (clearInput === undefined) clearInput = true;
  const r = document.getElementById('order-search-results');
  const i = document.getElementById('order-search-input');
  if (r) r.innerHTML = '';
  if (i) {
    if (clearInput) i.value = '';
    i.blur();
  }
}

window.__erUpdateWorkspace = function(html) {
  const ws = document.getElementById('workspace');
  if (ws) ws.innerHTML = html;
  const input = document.getElementById('order-search-input');
  const results = document.getElementById('order-search-results');
  if (input && results && results.innerHTML.trim() !== '' && input.value.trim() !== '') {
    window.__erTriggerSearch(input.value);
  }
};

window.__erUpdateSearch = function(html) {
  const r = document.getElementById('order-search-results');
  if (r) r.innerHTML = html;
};

function hasNativeBridge() {
  return !!(window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.erBridge);
}

window.__erTriggerSearch = function(q) {
  if (!q || !q.trim()) {
    window.__erUpdateSearch('');
    return;
  }
  if (hasNativeBridge()) {
    window.webkit.messageHandlers.erBridge.postMessage({ action: '/search', payload: q });
  } else {
    fetch('/search?q=' + encodeURIComponent(q))
      .then(function(res) { return res.text(); })
      .then(function(html) { window.__erUpdateSearch(html); });
  }
};

window.__erExportJson = function() {
  if (hasNativeBridge()) {
    window.webkit.messageHandlers.erBridge.postMessage({ action: '/export', payload: '' });
  } else {
    window.location.href = '/export';
  }
};

window.__erImportJson = function() {
  if (hasNativeBridge()) {
    window.webkit.messageHandlers.erBridge.postMessage({ action: '/import', payload: '' });
  }
};

document.addEventListener('submit', function(e) {
  const form = e.target.closest('form[hx-post]');
  if (!form) return;
  e.preventDefault();
  const action = form.getAttribute('hx-post');
  const fd = new FormData(form);
  const params = new URLSearchParams();
  fd.forEach(function(v, k) { params.append(k, v); });
  const afterReq = form.getAttribute('hx-on::after-request');

  if (hasNativeBridge()) {
    window.webkit.messageHandlers.erBridge.postMessage({
      action: action,
      payload: params.toString()
    });
    if (afterReq && afterReq.indexOf('closeSearchDropdown') !== -1) {
      closeSearchDropdown(true);
    }
  } else {
    fetch(action, {
      method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      body: params.toString()
    })
      .then(function(res) { return res.text(); })
      .then(function(html) {
        window.__erUpdateWorkspace(html);
        if (afterReq && afterReq.indexOf('closeSearchDropdown') !== -1) {
          closeSearchDropdown(true);
        }
      });
  }
});

(function() {
  let debounceTimer = null;
  const input = document.getElementById('order-search-input');
  if (input) {
    input.addEventListener('input', function() {
      clearTimeout(debounceTimer);
      const val = input.value;
      debounceTimer = setTimeout(function() {
        window.__erTriggerSearch(val);
      }, 90);
    });
    input.addEventListener('focus', function() {
      if (input.value.trim() !== '') {
        window.__erTriggerSearch(input.value);
      }
    });
  }
})();

document.addEventListener('pointerdown', function(e) {
  const c = document.getElementById('search-container');
  const r = document.getElementById('order-search-results');
  if (c && r && r.innerHTML.trim() !== '' && !c.contains(e.target)) {
    r.innerHTML = '';
  }
});

document.addEventListener('keydown', function(e) {
  if (e.key === 'Escape') closeSearchDropdown(false);
});
"##;
