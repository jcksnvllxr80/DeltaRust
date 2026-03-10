# 🐉 THE DRAGON SEAL — Dungeon Design Document
*Visual ASCII Maps + Room-by-Room Notes*
*Scale: Small early → Large late | Format: ASCII map + notes*

---
---

# LEVEL I — 🌿 MOSSHAVEN CAVE
**Theme:** Mossy forest cavern. Damp stone, glowing fungi, dripping water, ancient root systems breaking through the walls.
**Difficulty:** Tutorial. Teaches: exploration, basic combat, pressure plates, locked doors, pushable objects, hidden walls.
**Total Rooms:** 10 *(7 main + 2 secret + 1 boss)*
**Puzzle Piece:** 🧩 Dragon's Tail Tip *(Piece 1/8)*
**Tool Earned:** 🪜 Ladder
**New Mechanics Introduced:** Pressure plates · Pushable boulders · Key-locked doors · Hidden walls

---

## ASCII MAP

```
                                        ┌───────────┐
                                        │    R09    │
                                        │   BOSS    │
                                        │  [GOLEM]  │
                                        └─────┬─────┘
                                              │ [BOSS DOOR]
                                        ┌─────┴─────┐
                             ┌──────────┤    R07    ├──────────┐
                             │          │  Pressure │          │
                             │    ◄═════╡  Puzzle   ╞═════►    │
                             │  [SECRET]│  Chamber  │[SECRET]  │
                             │          └─────┬─────┘          │
                        ┌────┴────┐           │           ┌────┴────┐
                        │  R08a   │      ┌────┴────┐      │  R08b   │
                        │ SECRET  │      │   R06   │      │ SECRET  │
                        │  NOOK   │      │ Flooded │      │  CACHE  │
                        └─────────┘      │  Grotto │      └─────────┘
                                         └────┬────┘
                                              │
                              ┌───────────────┼───────────────┐
                              │               │               │
                         ┌────┴────┐     ┌────┴────┐     ┌────┴────┐
                         │   R03   │     │   R04   │     │   R05   │
                         │ Mossy   │     │  Root   │     │ Fungal  │
                         │ Alcove  │     │ Warren  │     │  Hall   │
                         └────┬────┘     └────┬────┘     └─────────┘
                              │               │
                              └───────┬───────┘
                                      │
                                 ┌────┴────┐
                                 │   R02   │
                                 │ Combat  │
                                 │ Chamber │
                                 └────┬────┘
                                      │
                                 ┌────┴────┐
                                 │   R01   │
                                 │  ENTRY  │
                                 │ (Start) │
                                 └─────────┘
```

```
LEGEND
══════════════════════════════════════════
  │ ─ ┌ ┐ └ ┘ ┤ ├  = Tunnel / corridor
  ◄═══►         = Hidden / secret wall passage
  [BOSS DOOR]   = Requires Boss Key to open
  [LOCKED]      = Requires Small Key to open
  [★]           = Chest or item location
  [!]           = Pressure plate
  [?]           = Lore tablet / hint stone
  [B]           = Boulder (pushable object)
  ///           = Cracked / breakable wall (future tool)
  ~~~           = Water / flooded floor
```

---

## ROOM-BY-ROOM NOTES

---

### R01 — ENTRY CHAMBER
**Shape:** Wide horizontal rectangle. Broad and open — welcoming, unthreatening.
**Size:** Large
**Connections:** North → R02 (open tunnel)

**Contents:**
- 🗺️ **DUNGEON MAP** — carved into a mossy stone slab directly opposite the entrance. Impossible to miss. Shows R01–R07 and R09. Secret rooms R08a and R08b are NOT shown.
- [?] **Lore Tablet** (east wall): *"The Golem sleeps in the deep moss. It was placed here to guard what the dragon left behind."*
- Three glowing fungi clusters on the ceiling — primary light source, establishes aesthetic.
- Shallow decorative pool in the southwest corner with lily pads. Non-functional.
- A crumbled archway frames the north exit — shows this place was once grand.

**Enemies:** None.
**Secrets:** None.

---

### R02 — COMBAT CHAMBER
**Shape:** Square, slightly cramped. Designed to funnel the player into their first fight.
**Size:** Medium
**Connections:** South → R01 | Northwest → R03 (open) | Northeast → R05 (open) | North → R04 *(LOCKED — Small Key A required)*

**Contents:**
- ⚔️ **2× Mosscrawler** — slow, melee only. Telegraphed attacks. Die in 3 hits each. Tutorial enemies.
- [★] **Small Key A** — dropped by the second Mosscrawler on death. Unlocks north door to R04.
- Broken stone pillar in center of room — minor cover.
- [?] **Lore Tablet** (north wall, readable through the locked bars): *"Beyond this gate the stones have memory. They hold what is given to them."*
- Torch sconce on west wall — flickers, slightly ominous tone shift from R01.

**Enemies:** 2× Mosscrawler
**Secrets:** None.

---

### R03 — MOSSY ALCOVE
**Shape:** Tall vertical rectangle. Narrow entry opens into a wider back section.
**Size:** Medium-Small
**Connections:** Southeast → R02 (open) | North → R06 *(LOCKED — Small Key B required)*

**Contents:**
- [★] **🧭 COMPASS** — resting on a vine-wrapped pedestal. Glows softly. Tracks player position on the dungeon map.
- [★] **Small Key B** — hidden inside a hollowed-out root on the west wall. Not obvious — player must inspect the wall (teaches examination habit).
- [★] **Healing Herb ×2** on the floor near the north wall.
- Ceiling is low and root-covered. Roots hang like curtains — atmospheric.
- **Cracked wall** on the east side marked with faint scratch marks. Unbreakable now (no Hammer). Hint of future mechanic.

**Enemies:** None.
**Secrets:**
- 🔍 The hollowed root hiding Small Key B — rewards curious players who explore walls.

---

### R04 — ROOT WARREN
**Shape:** Irregular, organic. Feels like it was carved by the roots themselves rather than built. Winding with a few alcoves.
**Size:** Medium-Large
**Connections:** South → R02 (locked entry, key used) | North → R06 (open) | West alcove dead-end | East alcove dead-end

**Contents:**
- ⚔️ **3× Mosscrawler** — slightly more aggressive than R02 batch. One drops a **Gold Pouch (25 coins)**.
- [B] **2× Pushable Mossy Boulders** — introduced here as environmental objects but NOT yet puzzle-critical. Player can push them around freely. Seeds the mechanic before R07 demands it.
- [?] **Lore Tablet** (north alcove): *"The cave breathes. Listen and it will show you where it hides its oldest things."*
- West alcove: Dead end with **Healing Herb ×1** on the ground.
- East alcove: Dead end with a **cracked floor tile** — decorative only, cannot be broken yet.
- Thick root columns across the room act as natural obstacles.

**Enemies:** 3× Mosscrawler
**Secrets:** None — but the two dead-end alcoves make players think there might be something they're missing. Teaches suspicion.

---

### R05 — FUNGAL HALL
**Shape:** Long horizontal rectangle. A corridor-room — wide enough to fight in but clearly a passageway.
**Size:** Medium
**Connections:** West → R02 (open) | North → R06 (open)

**Contents:**
- ⚔️ **1× Thornback Sprout** — spiny, slightly faster than Mosscrawlers. Rolls into a ball and charges. Introduces a new enemy type.
- [★] **Gold Pouch (30 coins)** — on the ground midway through the room.
- Bioluminescent fungi line both walls in rows — beautiful, eerie lighting.
- A long crack runs along the ceiling the length of the room — water drips through it. Aesthetic.
- [?] **Lore Tablet** (east wall near north exit): *"The spore-things guard what they do not understand. They only know: protect."*

**Enemies:** 1× Thornback Sprout
**Secrets:** None.

---

### R06 — FLOODED GROTTO
**Shape:** Wide irregular oval. The ceiling vaults high. A central raised stone island sits above ankle-deep water covering the rest of the floor.
**Size:** Large
**Connections:** South → R03 (locked from R03 side, Key B used) | South → R04 (open) | South → R05 (open) | North → R07 (open)

**Contents:**
- ~~~ **Flooded floor** — walking through it is slow (wade mechanic introduced). The raised central island is dry.
- ⚔️ **2× Bog Lurker** — new enemy. Submerged in water, pop up to grab player's legs (slows movement). Best fought by luring onto the island and striking there.
- [★] **Lantern** on the central island — equippable light source. Makes dark rooms easier to navigate. Not mandatory.
- [★] **Gold Pouch (20 coins)** — on a ledge on the east wall, reachable by wading.
- Massive glowing moss formation on the north wall marks the transition into the dungeon's second half.
- Waterfall sound from a crack in the ceiling. Water level is stable — not a rising water hazard here.

**Enemies:** 2× Bog Lurker
**Secrets:** None directly — but the moss formation on the north wall conceals the exit until the player circles around it.

---

### R07 — PRESSURE PLATE PUZZLE CHAMBER
**Shape:** Large square with thick stone walls. The central showpiece room of the dungeon.
**Size:** Very Large
**Connections:** South → R06 (open) | North → R09 *(BOSS DOOR — requires Boss Key)* | West hidden wall → R08a *(SECRET)* | East hidden wall → R08b *(SECRET)*

**Contents:**
- [!] **4× Pressure Plates** — arranged in a square pattern on the floor, one in each quadrant.
- [B] **4× Mossy Boulders** — one in each corner. Each must be pushed onto its corresponding plate.
- Plates are offset diagonally from boulders — not trivially obvious which goes where. Player must plan.
- When all 4 plates are held simultaneously: stone panel on north wall slides open → **Boss Key** revealed.
- If any boulder rolls off its plate, the panel reseals and Boss Key retracts.
- [?] **Lore Tablet** (south wall near entry): *"Four stones. Four resting places. Only when each finds its home will the way forward open."*
- ⚔️ **1× Thornback Sprout** patrols the room — collides with boulders, knocking them off plates. Must be killed first or kited carefully.
- **[BOSS DOOR]** north wall — sealed stone mechanism. Boss Key slot visible beside it.

**Enemies:** 1× Thornback Sprout
**Secrets:**
- 🔍 **West hidden wall → R08a:** Slightly darker, drier moss patch on the west wall. Pushing it opens a narrow tunnel. Pure observation — no item or mechanic required.
- 🔍 **East hidden wall → R08b:** A vine curtain on the east wall hangs unnaturally still (no draft behind it). Pushing through reveals a tunnel. Again, observation only.

---

### R08a — SECRET NOOK *(West)*
**Shape:** Small irregular alcove. Rough unfinished walls. Feels genuinely hidden.
**Size:** Very Small
**Connections:** East → R07 (hidden wall)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **📜 Lore Scroll — "The Dragonbinder's First Entry"** — first page of the Dragon Codex. Reads: *"I have found the first fragment. The dragon did not simply die — it was sealed. Eight stones. Eight pieces of its form. Find them all and the seal breaks... or holds. That choice is mine alone."* Foreshadows the game's moral ending.
- [★] **Rare Healing Herb** — restores full HP instantly.
- Faded mural on the back wall: silhouette of a dragon broken into 8 glowing fragments. Direct visual foreshadowing of the full puzzle.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R08b — SECRET CACHE *(East)*
**Shape:** Slightly larger than R08a. Feels deliberately stashed — like someone hid things here intentionally.
**Size:** Small
**Connections:** West → R07 (hidden wall)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **Gold Pouch (75 coins)** — largest single gold reward in the dungeon.
- [★] **Healing Herb ×3** — small stockpile.
- [★] **Worn Adventurer's Journal** — optional readable item. Last entry: *"I can hear the Golem from here. I'm leaving my supplies. Maybe the next one will get further than I did."* Atmospheric world-building.
- Scratched tally marks on the wall — 7 sets of 4 and one stray line. Someone was counting something for a very long time.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R09 — BOSS CHAMBER
**Shape:** Large octagon suggested by chamfered stone corners. Ceiling vaults dramatically upward.
**Size:** Very Large — biggest room in the dungeon.
**Connections:** South → R07 *(BOSS DOOR — Boss Key required to open)*

**Contents:**
- 🔒 **BOSS DOOR** south side — Boss Key required.
- **BOSS: Thornback Golem** — dormant stone figure atop a raised circular rune seal in the center. Activates when player steps onto the seal.
- [★] **🧩 Dragon's Tail Tip (Piece 1/8)** — sealed inside stone altar chest on the north wall. Opens automatically on boss defeat.
- [★] **🪜 LADDER** — propped against the north wall beside the altar. Collectible after defeat.
- Glowing rune ring on the floor marks the arena boundary — crossing back out resets the boss to full HP. Discourages fleeing.
- **2× Stone Pillar** east and west — usable cover early. Golem smashes one at 75% HP, the other at 40% HP. Escalates tension naturally.
- Cracked ceiling — chunks fall periodically as environmental hazard. Small damage, telegraphed by dust particles falling first.

**Boss — Thornback Golem:**
- **Phase 1 (100%–50% HP):** Slow stomp (2 sec telegraph, ground shockwave 2 tiles from foot). Wide horizontal swipe (dodge by rolling). Weakness: glowing back crack exposed for 2 sec after each stomp.
- **Phase 2 (50%–0% HP):** Adds thorn shard throw (3-projectile cone, dodge sideways). Stomp rate increases. Begins spinning occasionally — forces player to arena edge. Back crack window shrinks to 1.5 sec.
- **Defeat:** Golem shatters. Rune ring fades. Altar chest and Ladder unlock. Music resolves.

**Secrets:** None — boss rooms are always clean and readable.

---

## DUNGEON FLOW SUMMARY

```
CRITICAL PATH:
R01 (get map) → R02 (fight, get Key A) → R04 (fight, explore) → R06 (fight)
→ R07 (pressure puzzle, get Boss Key) → R09 (boss, get Piece 1 + Ladder)

NOTE: To reach R06 from R03 side, Key B is needed.
Key B is in R03. R03 is accessed from R02 (northwest, open).
Player must explore R03 to unlock the R03→R06 northern path.
Alternatively: R04→R06 and R05→R06 are both open — player can bypass R03
but will miss the Compass and Key B (Key B only matters for R03→R06 shortcut).

OPTIONAL PATHS:
R02 → R03 (Compass + Key B + healing)
R02 → R05 (Fungal Hall shortcut to R06, skips R03 and R04)
R07 → R08a (west secret wall — Lore Scroll + healing)
R07 → R08b (east secret wall — gold cache + journal)

BLOCKED UNTIL FUTURE TOOLS:
R03 cracked east wall → requires Hammer (Level 3)
R04 cracked floor tile → requires Hammer (Level 3)
```

---

## FULL ITEM & TOOL CHECKLIST

| Item | Location | Required? |
|------|----------|-----------|
| 🗺️ Dungeon Map | R01 — stone slab on entry | Auto-collected |
| 🧭 Compass | R03 — vine pedestal | Optional |
| 🗝️ Small Key A | R02 — enemy drop | Yes (opens R04 north door) |
| 🗝️ Small Key B | R03 — hidden in hollow root | Optional (opens R03→R06 path) |
| 🔑 Boss Key | R07 — revealed after puzzle | Yes (opens R09) |
| 🏮 Lantern | R06 — central island | Optional |
| 🧩 Piece 1/8 | R09 — altar chest after boss | Main Objective |
| 🪜 Ladder | R09 — north wall after boss | Yes (needed for Level 2) |
| 📜 Lore Scroll | R08a — secret nook | Optional (collectible) |
| 📓 Adventurer's Journal | R08b — secret cache | Optional (lore) |
| 💰 Gold (total ~150) | R04, R05, R06, R08b | Optional |
| 🌿 Healing Herbs | R03, R04, R06, R08a, R08b | Optional |

---
---

*[Level II — Ruins of Ashenfall will be added below upon completion]*
