import json
import re
import sqlite3
from pathlib import Path

DB_PATH = Path("ngplus.db")
RULES_PATH = Path("quest_rules.json")


def slugify(text: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_")


def init_db():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()

    cur.executescript(
        """
        DROP TABLE IF EXISTS permanent_vault;
        DROP TABLE IF EXISTS meta_state;
        DROP TABLE IF EXISTS current_run_order;
        DROP TABLE IF EXISTS step_progress;

        CREATE TABLE IF NOT EXISTS gates (
            gate_id TEXT PRIMARY KEY,
            sort_order INTEGER NOT NULL,
            campaign TEXT NOT NULL,
            title TEXT NOT NULL,
            trigger_desc TEXT NOT NULL,
            warning_desc TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS items_catalog (
            item_id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            campaign TEXT NOT NULL,
            region TEXT NOT NULL,
            is_missable INTEGER DEFAULT 0,
            exclusive_group TEXT,
            exclusive_reason TEXT,
            steps_json TEXT DEFAULT '[]'
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS items_fts USING fts5(
            item_id UNINDEXED,
            name,
            category,
            campaign,
            region,
            tokenize='porter unicode61'
        );
        """
    )

    if RULES_PATH.exists():
        data = json.loads(RULES_PATH.read_text(encoding="utf-8"))
        for g in data.get("gates", []):
            cur.execute(
                """
                INSERT OR REPLACE INTO gates (gate_id, sort_order, campaign, title, trigger_desc, warning_desc)
                VALUES (?, ?, ?, ?, ?, ?)
                """,
                (
                    g["gate_id"],
                    g["order"],
                    g["campaign"],
                    g["title"],
                    g["trigger"],
                    g["warning"],
                ),
            )

        for item in data.get("items", []):
            cur.execute(
                """
                INSERT OR REPLACE INTO items_catalog
                (item_id, name, category, campaign, region, is_missable, exclusive_group, exclusive_reason, steps_json)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    item["item_id"],
                    item["name"],
                    item["category"],
                    item["campaign"],
                    item["region"],
                    1 if item.get("is_missable") else 0,
                    item.get("exclusive_group"),
                    item.get("exclusive_reason"),
                    json.dumps(item.get("steps", [])),
                ),
            )

        cur.execute("DELETE FROM items_fts")
        cur.execute(
            """
            INSERT INTO items_fts (item_id, name, category, campaign, region)
            SELECT item_id, name, category, campaign, region FROM items_catalog
            """
        )

    conn.commit()
    conn.close()


if __name__ == "__main__":
    init_db()
    print("✅ Catalog initialized. SQLite is now a read-only search index; run state is stored in client cookies.")