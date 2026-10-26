<div align="center">
  <img src="static/icon-192.png" alt="Elden Ring Current Run Roadmap Emblem" width="96" height="96" />
  <h1>Elden Ring Current Run Roadmap</h1>
  <p><strong>The smart, missable-proof route planner for Elden Ring &amp; Shadow of the Erdtree</strong></p>
  <p>
    <a href="https://er-ngplus-compiler.fastapicloud.dev"><strong>🌐 Launch Web App</strong></a>
    &nbsp;•&nbsp;
    <a href="https://github.com/mortalezz/er-roadmap/releases/latest"><strong>🍎 Download macOS App (1.5 MB, Offline)</strong></a>
    <br/>
    <img src="https://img.shields.io/badge/Catalog-1%2C241_Items-f59e0b?style=flat-square" alt="1,241 Items" />
    <img src="https://img.shields.io/badge/Curated_Chains-82_Quest_Rules-10b981?style=flat-square" alt="82 Rules" />
    <img src="https://img.shields.io/badge/Lockout_Protection-10_PONR_Gates-ef4444?style=flat-square" alt="10 Gates" />
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="MIT License" />
  </p>
</div>

---

## What is Elden Ring Roadmap?

If you're starting a new game, an NG+ run, or prepping a new build for PvP/co-op, you probably have a mental wishlist:
* *"I need the Magic Scorpion Charm, the Raging Wolf armor set, the Bolt of Gransax, and Dragonbolt of Florissax."*

The problem? **Elden Ring's questlines are full of invisible tripwires.** 
* Rest at an Altus Plateau site of grace too early? *Alexander, Corhyn, and Yura's quests jump forward.*
* Join Volcano Manor before doing Rya's errand in Liurnia? *Blackguard Boggart is locked out forever (no Boiled Crab for you).*
* Loot the Secret Medallion at Castle Sol before talking to Latenna? *She dies where she sits.*
* Defeat Rykard? *Every Volcano Manor assassination contract vanishes.*
* Cross north of Bonny Village in the DLC? *Miquella's Great Rune breaks, locking out Moore's and Thiollier's concoctions.*
* Defeat Maliketh? *Leyndell turns to ash, burying the Bolt of Gransax and Sanctified Whetblade.*

Traditional walkthroughs force you to read through hundreds of chronological steps for items you don't even want. 

**Elden Ring Roadmap inverts the entire process:**
You tell it **only what items you want for your build**, and it generates a **personalized, step-by-step geographical route** with explicit **Point-of-No-Return (PONR) stop barriers** so you never accidentally lock yourself out of a build-defining item again.

---

## How It Works

```
 1. Queue Your Wishlist        2. Smart Route Sequencing       3. Inline Stop Barriers
┌─────────────────────────┐   ┌─────────────────────────┐    ┌─────────────────────────┐
│ 🎯 Magic Scorpion Charm │   │ 📍 Liurnia of Lakes     │    │ [!] POINT OF NO RETURN  │
│ 🎯 Raging Wolf Set      │──►│ 📍 Altus Plateau        │──► │ Handing Fingerslayer    │
│ 🎯 Bolt of Gransax      │   │ 📍 Leyndell Capital     │    │ Blade to Ranni kills    │
│ 🎯 Florissax Dragonbolt │   │ 📍 Realm of Shadow      │    │ Seluvis. Complete charm │
└─────────────────────────┘   └─────────────────────────┘    │ errand BEFORE crossing! │
                                                             └─────────────────────────┘
```

### 1. Queue Only What You Care About
Search from **1,241 weapons, talismans, spells, armor sets, spirit ashes, cookbooks, and upgrade materials** across both the Base Game and *Shadow of the Erdtree*. Add only the loot you want for this run.

### 2. Zero-Backtracking Geographical Order
The planner unrolls NPC quest prerequisites and merges shared tasks into single checkpoints. It schedules every pickup along natural game flow:
> **Limgrave → Weeping Peninsula → Liurnia → Caelid → Underground → Altus → Mt. Gelmir → Leyndell → Mountaintops → Snowfield / Mohgwyn → Farum Azula → Realm of Shadow**

You sweep each region once, pick up what you need, and move on.

### 3. Hard Stop Barriers (Points of No Return)
Whenever a quest is at risk of being locked out by a world event, the planner places an inline **Stop Barrier**:
* **🔴 Red Alert:** *"DO NOT TRIGGER YET — Resting at Altus Plateau / Touching Ranni's Fingerslayer Blade will lock out this step."*
* **🟢 Green Clear:** Once all prerequisite steps before that gate are checked off, the barrier turns green: *"SAFE TO CROSS."*

### 4. Build Conflict Detection
Certain rewards are strictly mutually exclusive in a single playthrough (e.g. *Millicent's Prosthesis* vs. *Rotten Winged Sword Insignia*, or *Nepheli Puppet* vs. *Dung Eater Puppet*). The planner immediately highlights collisions with a one-click button to resolve your choice.

### 5. Laconic, Spoiler-Free Checkpoints
No bloated lore paragraphs. Every checkpoint gives you only what you need to act:
> `[Altus Plateau • Altus Highway Junction] Rest at Grace to trigger Corhyn movement. (GATE_B_ALTUS)`

---

## Play Your Way: Web, Mobile, or Native Desktop

Whether you play next to a dual monitor, a laptop, or your phone propped against your TV, the roadmap fits your setup:

### 🌐 Live Web App (Fast & Mobile-Friendly)
Open [**er-ngplus-compiler.fastapicloud.dev**](https://er-ngplus-compiler.fastapicloud.dev) in any browser.
* **Instant Load:** Cached at Cloudflare's Edge CDN worldwide for sub-20ms first paint.
* **Installable PWA:** Open in Safari on iPhone or Chrome on Android and tap **"Add to Home Screen"** for a full-screen, app-like experience with no browser address bar.
* **No Accounts / No Passwords:** Your checklist is saved automatically in your browser.

### 🍎 Native macOS Desktop App (`1.5 MB`, 100% Offline)
Playing on a Mac or keeping a MacBook on your gaming desk? Grab the native desktop app:
* **Download:** Grab [`ER-Roadmap-macOS-arm64.zip`](https://github.com/mortalezz/er-roadmap/releases/latest) from Releases, unzip, and launch **`ER Roadmap.app`**.
* **Zero Internet Required:** Completely self-contained single binary with embedded styling and catalog. Runs smoothly even on airplane mode or spotty Wi-Fi.
* **Microsecond Speed:** Powered by a native Rust solver and Cocoa WebKit runtime (~0.02 ms response time).

### 💾 Seamless Multi-Device Sync (Tiny 1 KB File)
Switching between your desktop and your phone mid-playthrough?
1. Click **💾 Export JSON** on your computer (it downloads a tiny `~1 KB` file).
2. AirDrop, text, or email the file to your phone.
3. Open the Web App on your phone, click **📂 Import JSON**, and pick the file.
4. Your entire wishlist, checked steps, and progress are instantly restored.

---

## Major Lockouts Protected

| Critical Trigger | What Locks Out If You Cross Too Early |
| :--- | :--- |
| **Resting at Altus Plateau** | *Raptor of the Mists* ash of war, *Rogier's Rapier +8*, early Corhyn/Alexander dialogue |
| **Handing Fingerslayer Blade to Ranni** | *Magic Scorpion Charm*, *Dung Eater Puppet*, *Nepheli Loux Puppet*, *Spellblade Set* |
| **Joining Volcano Manor Early** | Blackguard Boggart vendor (*Boiled Prawn / Boiled Crab*), *Hammer Talisman* |
| **Defeating Rykard, Lord of Blasphemy** | *Raging Wolf Set*, *Bull-Goat Set*, *Magma Whip Candlestick*, *Taker's Cameo*, *Hoslow's Set*, *Crepus's Vial* |
| **Looting Haligtree Secret Medallion (Left)** | Latenna's *Somber Ancient Dragon Smithing Stone* (if she wasn't recruited in Liurnia first) |
| **Defeating Maliketh (Ashen Capital)** | *Bolt of Gransax*, *Sanctified Whetblade*, *Gravel Stone Seal*, *Coded Sword*, *Lionel's Set* |
| **Approaching Shadow Keep (DLC Great Rune Breaks)** | Moore's *Black Syrup*, *Thiollier's Concoction*, *Dragonbolt of Florissax*, *Florissax Spirit Ash* |
| **Opening Messmer's Chamber (DLC)** | *Lacerating Crossed-Tree*, *Ash of War: Swift Slash*, *Retaliatory Crossed-Tree*, *Hornsent Grandam* |
| **Burning the Sealing Tree (DLC)** | *Falx* invasion, *St. Trina's Smile*, *Thiollier's Needle*, *Dryleaf Arts*, *Verdigris Greatshield* |

---

## Mutual Exclusivity Quick Reference

These items cannot be obtained in the same NG cycle; queuing both will alert you:

* **Millicent's Quest:** *Millicent's Prosthesis* (betray) **vs.** *Rotten Winged Sword Insignia & Miquella's Needle* (assist).
* **Seluvis's Potion:** *Dung Eater Puppet* **vs.** *Nepheli Loux Puppet* **vs.** *2× Ancient Dragon Smithing Stones* (Nepheli/Gostoc).
* **Sorceress Sellen:** *Glintstone Kris & Witch's Crown* (side with Sellen) **vs.** *Ancient Dragon Smithing Stone* (side with Jerren).
* **Dragon Communion Priestess:** *Dragonbolt of Florissax & Spirit Ash* (administer sleep draught) **vs.** *Flowerstone Gavel & Priestess Heart* (no draught).
* **Leda's Invasions:** *Lacerating Crossed-Tree* (side with Leda) **vs.** *Ash of War: Swift Slash* (side with Hornsent).
* **Hornsent / Ansbach:** *Retaliatory Crossed-Tree* (side with Leda) **vs.** *Obsidian Lamina & Furious Blade* (keep Ansbach alive).
* **Iris of Grace vs. Occultation:** *Sword of Night* **vs.** *Jolán Spirit Ash* (Jolán), and *Queelign's Greatsword* **vs.** *Queelign Spirit Ash*.

---

<details>
<summary><strong>🛠️ Under the Hood: Architecture & Developer Notes</strong></summary>

### The Dependency-Inverted DAG Solver
Traditional walkthroughs walk forward trying to track everything at once. `er-ngplus-compiler` treats your queued items as leaf nodes in a **Directed Acyclic Graph (DAG)** and solves **backward from the loot to its earliest prerequisites**:
1. **Tree Shaking & Diamond Deduplication:** Pulls only prerequisites needed for your targets and merges shared ancestor steps (like `volcano_contracts_1_2` or `moore_thiollier_concoction`) into single checkpoints with combined target badges.
2. **Backward PONR Ceiling Propagation:** Iterates backward through multi-step chains to clamp every prerequisite *before* the earliest downstream lockout gate ($pos(s_i) = \min(Z(s_i), C_i - 1)$).
3. **Geographical Ordinal Weighting:** Sorts all surviving checkpoints along canonical progression numbers (`zone_order` 10–160).
4. **Collision Detection:** Flags multiple items sharing an `exclusive_group`.

### Tech Stack
* **Cloud App:** FastAPI (`main.py`) + HTMX + Cloudflare Edge CDN caching via `app.frontend("/", directory="dist")`.
* **Database:** SQLite 3 catalog + FTS5 full-text search (`ngplus.db`, 1.1 MB, zero write locks, read-only mode).
* **Stateless Cookies:** `v2_` zlib level 9 + URL-safe Base64 codec compresses 68 simultaneous missables into ~1 KB, dodging the browser 4 KB limit. Single-tab mutations serialized via `hx-sync="this:queue"`.
* **Native Desktop Port (`rust-port/`):** Pure-Rust AOT `.tsx` compiler that compiles React-style templates into zero-allocation string writers, bundled with an embedded Cocoa `WKWebView` IPC bridge (`20.7 µs` solve time, `1.5 MB` binary).

### Local Development
```bash
# Clone the repository
git clone https://github.com/mortalezz/er-roadmap.git
cd er-roadmap

# Install Python dependencies and build frontend
uv sync
python build_dist.py
uv run fastapi dev main.py

# Or build the native macOS desktop app
./rust-port/build_macos_app.sh
open "rust-port/ER Roadmap.app"
```
</details>

---

## License

Released under the [MIT License](LICENSE). 
Elden Ring and Shadow of the Erdtree are trademarks of FromSoftware and Bandai Namco. This project is an independent fan-made tool.
