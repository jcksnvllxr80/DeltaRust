# 🐉 THE DRAGON SEAL — Dungeon Design Document
*Visual ASCII Maps + Room-by-Room Notes*
*Scale: Small early → Large late | Format: ASCII map + notes*

---
---

# LEVEL VI — 🌑 THE FRACTURED SANCTUM
**Theme:** A reality-warping sanctum carved into a dimensional rift — a place where the laws of space have been partially dissolved by proximity to the dragon's sealing event. Rooms connect in ways that defy their physical layout. Mirrors reflect things that aren't there. Some corridors loop. Some doors open into the wrong room. The Void Compass (found in the overworld — dropped by a wandering Shade enemy in the wastes) is required to navigate the sanctum without becoming permanently lost, and is the key tool for identifying the real Duplexis during the boss fight. The dungeon uses all five prior tools in combination — this is the first dungeon where the player is expected to chain multiple tools together fluidly.
**Difficulty:** High. Teaches: Void Compass navigation (points toward correct path in looping corridors), mirror trap identification, reality-check mechanic (some rooms have false-duplicate layouts — Compass identifies the real exit), multi-tool chaining, Portal Tool preview (the sanctum foreshadows portal mechanics via environmental rifts the player cannot yet control).
**Total Rooms:** 18 *(11 main + 2 secret + 3 mirror-trap rooms + 1 loop corridor + 1 boss)*
**Puzzle Piece:** 🧩 Dragon's Right Wing *(Piece 6/8)*
**Tool Earned:** 🌀 Portal / Warp Tool
**World Item Required:** 🧭 Void Compass *(dropped by wandering Shade enemy in the overworld wastes — required to navigate and to identify the real boss)*
**New Mechanics Introduced:** Void Compass navigation · Mirror trap rooms · Looping corridor · Reality-check puzzle (false duplicate exits) · Environmental rifts (passive, non-controllable portals) · Multi-tool chaining · Shadow Clone enemies

---

## ASCII MAP

```
NOTE: This map shows the PHYSICAL layout of the sanctum.
The Void Compass reveals the TRUE navigable connections.
Some connections shown are mirror traps — they lead to duplicate
rooms that loop back. The Compass distinguishes real from false.

                                        ┌───────────┐
                                        │    R18    │
                                        │   BOSS    │
                                        │[DUPLEXIS] │
                                        └─────┬─────┘
                                              │ [BOSS DOOR]
                                        ┌─────┴─────┐
                              ◄══════════    R14     ══════════►
                           [SECRET]     │  The Void  │   [SECRET]
                                        │  Chamber   │
                                        └─────┬─────┘
                                              │
                               ┌──────────────┼──────────────┐
                               │              │              │
                          ┌────┴────┐    ┌────┴────┐    ┌────┴────┐
                          │   R11   │    │   R12   │    │   R13   │
                          │ Mirror  │    │ Compass │    │ Rift    │
                          │  Hall   │    │  Room   │    │ Bridge  │
                          └────┬────┘    └────┬────┘    └─────────┘
                               │              │
                         [MIRROR         [MIRROR
                          TRAP]           TRAP]
                          ┌────┴────┐    ┌────┴────┐
                          │  R11m   │    │  R12m   │
                          │ MIRROR  │    │ MIRROR  │
                          │ TRAP    │    │ TRAP    │
                          └─────────┘    └─────────┘
                                              │
                               ┌──────────────┼──────────────┐
                               │              │              │
                          ┌────┴────┐    ┌────┴────┐    ┌────┴────┐
                          │   R07   │    │   R08   │    │   R09   │
                          │ Shadow  │    │ Central │    │ Rift    │
                          │  Hall   │    │  Nexus  │    │  Room   │
                          └─────────┘    └────┬────┘    └─────────┘
                                              │
                                        ┌─────┴─────┐
                                        │   LOOP    │
                                        │  R10      │
                                        │(corridor) │
                                        └─────┬─────┘
                                              │
                               ┌──────────────┼──────────────┐
                               │              │              │
                          ┌────┴────┐    ┌────┴────┐    ┌────┴────┐
                          │   R03   │    │   R04   │    │   R05   │
                          │ Cracked │    │ Entry   │    │ False   │
                          │  Hall   │    │ Atrium  │    │  Hall   │
                          └─────────┘    └────┬────┘    └─────────┘
                                              │
                                        ┌─────┴─────┐
                                        │    R02    │
                                        │  Threshold│
                                        │  Chamber  │
                                        └─────┬─────┘
                                              │
                                        ┌─────┴─────┐
                                        │    R01    │
                                        │   ENTRY   │
                                        │  (Start)  │
                                        └───────────┘


SECRET ROOMS (not on dungeon map):
  R15a ◄═══ west hidden rift of R14
  R15b ═══► east hidden rift of R14

MIRROR TRAP ROOMS (not on dungeon map — appear identical to real rooms):
  R11m — false duplicate of R11, loops back to R07 south entrance
  R12m — false duplicate of R12, loops back to R08 south entrance

LOOP CORRIDOR:
  R10 — a corridor that loops back to itself if entered from the wrong end
        Void Compass required to identify the correct traversal direction
```

```
LEGEND
══════════════════════════════════════════
  │ ─ ┌ ┐ └ ┘ ┤ ├  = Tunnel / corridor
  ◄═══►         = Hidden / secret rift passage
  [BOSS DOOR]   = Requires Boss Key to open
  [LOCKED]      = Requires Small Key to open
  [★]           = Chest or item location
  [!]           = Pressure plate
  [?]           = Lore tablet / hint stone
  [B]           = Heavy object (liftable with Strong Arm Glove)
  [🪜]          = Ladder climb point (up or down)
  [🔨]          = Hammer-smashable wall or panel
  [💪]          = Strong Arm Glove required interaction
  [⛓️]          = Chain-pull point (Glove required)
  [🧭]          = Void Compass check point (Compass glows / directs here)
  [🪞]          = Mirror surface (trap or puzzle element)
  [≋]           = Environmental rift (passive, non-controllable portal)
  ~~~           = Shallow water
  ░░░           = Unstable floor (flickers between solid and void)
  ▓▓▓           = Dark zone / void-shadow region
  ↑ ↓           = Upper / lower level transition
  [LOOP]        = Corridor that loops if traversed incorrectly
```

---

## ROOM-BY-ROOM NOTES

---

### R01 — ENTRY CHAMBER
**Shape:** Wide horizontal rectangle. The entrance to the sanctum is not a door — it is a tear in the air. A vertical rift, 3 tiles tall, shimmers at the south wall. Stepping through it deposits the player in this room. The room itself looks almost normal — dressed stone, iron sconces — except that the ceiling is the floor of an identical room visible above, also with sconces lit, also with a version of the player's shadow moving in sync. The doubled ceiling is disorienting by design.
**Size:** Large
**Connections:** North → R02 (open)

**Contents:**
- 🗺️ **DUNGEON MAP** — carved into the east wall in precise geometric lines. Unusual for a dungeon map: it shows the rooms but the connecting lines between some of them are dotted rather than solid. Dotted lines indicate connections that may not be stable. The map is accurate but requires the Void Compass to interpret fully. Shows R01–R14 and R18. Mirror traps, loop corridor, and secret rooms R15a/R15b are NOT shown.
- [?] **Lore Tablet** (west wall): *"The Fractured Sanctum exists in the space between what is and what was. The dragon's sealing cracked more than stone. It cracked the rules. Navigate carefully. Trust the compass. Do not trust the mirrors."*
- [🧭] **Void Compass activation point** — a carved symbol on the north wall floor, glowing faintly. When the player stands on it with the Void Compass equipped, the Compass needle locks onto true north for the first time, establishing its baseline calibration for the dungeon. Without this calibration the Compass still works but is slightly less precise. This is the tutorial moment for the Compass mechanic.
- The doubled ceiling-room casts faint shadows downward — the shadows move independently of the player after a 2-second delay. Not enemies. Not interactive. Just wrong enough to be deeply unsettling.
- A cracked mirror on the east wall — frame shattered, glass in pieces on the floor. Even the broken shards reflect things that are not in the room. Do not look directly.

**Enemies:** None.
**Secrets:** None.

---

### R02 — THRESHOLD CHAMBER
**Shape:** Square. A transitional room — plainer than R01, which makes it more suspicious. Four identical doors, one on each wall. Only two lead anywhere real. The other two are mirror traps that loop back to this room.
**Size:** Medium
**Connections:** South → R01 (real — open) | North → R04 (real — open) | East → R04 *(MIRROR TRAP — loops back to R02 from east)* | West → R04 *(MIRROR TRAP — loops back to R02 from west)*

**Contents:**
- [🧭] **Void Compass check** — when equipped in this room, the Compass needle points north toward R04. The east and west doors cause the needle to spin erratically (indicating false passages). First practical Compass navigation use — teaches the mechanic clearly and safely (the penalty for entering a mirror trap here is mild: returned to R02 with a brief disorientation effect, no damage).
- ⚔️ **2× Shadow Clone** — new enemy. Appear identical to the player — same size, same shape, mirrored movements with a 1-second delay. Distinguishable only by the faint void-shimmer at their edges (visible in normal light, invisible in dark zones). Die in 3 hits but dodge in patterns that mirror the player's own recent movements. Fighting them requires breaking pattern — doing something unexpected.
- [?] **Lore Tablet** (north wall, beside real exit): *"Four doors. Two real. The compass knows. The mirrors lie but they lie consistently — learn the lie and you learn the truth."*
- The east and west doors are visually identical to the north door — same frame, same stonework. No visible distinguishing feature. The Compass is the only reliable tell. Teaches: equip the Compass before choosing a direction.
- Floor is polished to a mirror finish — the player's reflection walks below them throughout the room. The Shadow Clones cast no reflection.

**Enemies:** 2× Shadow Clone
**Secrets:** None.

---

### R03 — CRACKED HALL
**Shape:** Long vertical rectangle. The walls have deep cracks running floor to ceiling — not structural damage but dimensional fractures. Through the cracks, glimpses of other places are visible: a forest, a forge, an underwater scene, a sky. The dragon's sealing fractured space here and the wounds have never healed.
**Size:** Medium-Large
**Connections:** East → R04 (open) | North → R07 *(LOCKED — Small Key A required)*

**Contents:**
- ⚔️ **3× Shadow Clone** — these clones are slightly more aggressive, having had longer to observe the player's patterns from the dimensional rifts.
- [★] **Small Key A** — wedged into one of the dimensional cracks in the west wall. Visible as a glint. Requires inspecting the crack to retrieve — standard interact. The crack is at shoulder height and 2 tiles wide — clearly deliberate placement.
- [★] **Healing Herb ×2** — on the floor near the south wall, below a crack that shows a forest. The herbs fell through from the other side. Fresh, despite the dimensional crossing.
- [?] **Lore Tablet** (east wall, between two cracks): *"The cracks opened during the sealing. We tried to seal them with mortar. The mortar fell through. We tried stone. The stone fell through. We stopped trying. The cracks are part of the sanctum now."*
- [≋] **Environmental rifts** (3 of the largest cracks) — passive portals. Walking into them transports the player to a brief void-space (black, featureless, 3 seconds) before depositing them back at the same crack from the other side. Disorienting but not harmful. Teaches the concept of rifts before the Portal Tool makes them controllable.
- [🔨] **Cracked north wall section** — Hammer widens a dimensional fracture into a passable gap. Behind it: a small nook with **Gold Pouch (30 coins)** and a view through a rift into a room that looks exactly like R14 (the Void Chamber) — a foreshadowing glimpse.

**Enemies:** 3× Shadow Clone
**Secrets:**
- 🔍 Cracked north wall — Hammer widens rift-fracture. Gold and R14 foreshadowing.

---

### R04 — ENTRY ATRIUM
**Shape:** Large square. The central hub of the dungeon's lower half. High ceiling, four exits. A large circular void-compass symbol is inlaid on the floor in dark stone — a permanent navigation aid for anyone who knew the sanctum's original purpose.
**Size:** Large
**Connections:** South → R02 (open) | North → R08 (open) | East → R05 (open) | West → R03 (open)

**Contents:**
- ⚔️ **2× Shadow Clone** + **1× Void Wraith** — new enemy. A Void Wraith is a larger, faster Shadow Clone variant with a rift-step ability — it can teleport 3 tiles in any direction once every 5 seconds (a short-range blink). Cannot be stunned mid-blink. Weak point: the void-shimmer at its core, visible for 1 second after a blink-landing.
- [🧭] **Floor compass symbol** — standing on the inlaid symbol with the Void Compass equipped displays a faint directional overlay on the dungeon map (for 30 seconds) — showing which room connections are real versus mirror-false throughout the lower half of the dungeon. A significant navigational reward for standing in the right spot.
- [★] **🧭 COMPASS charge** — the Void Compass can be recharged here at the floor symbol. In this dungeon the Compass has a limited number of uses per room before it needs a moment to recalibrate (3 uses per room — generous enough to not feel restrictive but teaches that the Compass is a finite resource per room, not spammable).
- [?] **Lore Tablet** (center of room, embedded in the floor symbol): *"This atrium was built as an orientation point. The compass symbol shows true north. True north in this sanctum is not a direction — it is a decision. The sanctum tests whether you choose correctly."*
- [★] **Gold Pouch (35 coins)** — on a raised stone plinth on the east side, accessible without tools.
- All four exits are clearly labeled in carved stone above the doorways — unusual for a dungeon. The labels read: NORTH (R08), EAST (R05), WEST (R03), SOUTH (R02). The labels are accurate. The sanctum tests navigation, not basic literacy.

**Enemies:** 2× Shadow Clone, 1× Void Wraith
**Secrets:** None.

---

### R05 — FALSE HALL
**Shape:** Long horizontal rectangle. Identical in every visual detail to R03 — same cracks, same sconces, same floor texture. This is intentional. A player moving quickly might assume they have looped back to R03. The Void Compass distinguishes them: in R05 the needle points west (back toward R04, the real exit). Proceeding east from R05 leads to a mirror trap.
**Size:** Medium-Large
**Connections:** West → R04 (real — open) | East → *(MIRROR TRAP — loops back to R04 east entrance, depositing player as if coming from R05's direction)* | North → R06 *(LOCKED — Small Key B required)*

**Contents:**
- ⚔️ **2× Shadow Clone** — same as R03 encounter. Designed to reinforce the "this is R03" illusion. A player who fights them the same way as R03 will feel confirmed in the false identity. The Compass is the only corrective.
- [🧭] **Void Compass check** — east exit causes needle to spin. North exit causes needle to pulse steadily (indicating a locked real exit). West exit causes needle to point clearly (real return path).
- [★] **Small Key B** — on a high ledge above the north door. Ladder required to reach the ledge. Key B unlocks the north door to R06.
- [★] **Gold Pouch (25 coins)** — on the floor near the east wall, placed to tempt players toward the mirror trap exit.
- [?] **Lore Tablet** (south wall, subtle — easy to miss): *"This is not R03. You are further east than you think. The cracks look the same because the cracks were made by the same event. But the room is different. Look at where the north door is."* A fourth-wall-adjacent hint — the tablet acknowledges the player's potential confusion. Slightly uncanny.
- The north door is in a slightly different position than R03's north door — 1 tile further east. Detectable if the player studies the room carefully rather than reacting to its visual similarity.

**Enemies:** 2× Shadow Clone
**Secrets:** None.

---

### R06 — THE REFLECTION ROOM
**Shape:** Square. Every wall is a mirror — floor-to-ceiling polished obsidian. The player's reflection multiplies endlessly in every direction. The actual room is small; the reflections make it feel infinite. One reflection is wrong — it doesn't move when the player moves. That is the Shadow Boss of this room.
**Size:** Medium
**Connections:** South → R05 (locked entry, Key B used) | North → R09 *(LOCKED — Small Key C required)*

**Contents:**
- ⚔️ **1× Mirror Shade** — new enemy. Appears as one of the player's reflections but does not mimic correctly — it is 1 second behind on movement and stands in a slightly wrong position. When identified and struck it becomes a physical entity and fights back. Strikes deal double damage because it knows the player's attack patterns (it has been watching). Weak point: its back — the only surface without a mirror-reflection.
- [🪞] **Mirror walls** — all four walls. Breaking a mirror (Hammer) reveals either a solid wall behind it or a small niche. Breaking the north wall mirror reveals a niche containing **Small Key C**. Breaking the south wall mirror reveals a niche containing **Gold Pouch (40 coins)**. Breaking east or west mirrors reveals solid stone — nothing behind them. Player must break all four to find both rewards. Each Hammer strike shatters one full mirror wall (4 strikes total).
- [★] **Small Key C** — behind north mirror wall (Hammer required).
- [★] **Gold Pouch (40 coins)** — behind south mirror wall (Hammer required).
- [?] **Audio lore** — no lore tablet in this room. Instead, a faint voice speaks when all four mirrors are broken: *"You broke every mirror. Most people only break one. The sanctum respects thoroughness."* This is the sanctum itself speaking — the only time it communicates directly. Unsettling and memorable.
- ░░░ **Unstable floor sections** — two tiles flicker between solid and void. Standing on a void-state tile deals minor fall damage (1 tile drop, catches back up). Adds movement pressure during the Mirror Shade fight.

**Enemies:** 1× Mirror Shade
**Secrets:**
- 🔍 All four mirror walls — Hammer to break. Key C and gold hidden in two of them.

---

### R07 — SHADOW HALL
**Shape:** Wide horizontal rectangle. The room is divided down the middle by a wall of pure darkness — not a physical wall but a void-shadow, 1 tile thick. Walking through it is instantaneous but disorienting (brief black screen, player emerges on the other side). Enemies on the far side are audible but not visible until the player crosses.
**Size:** Large
**Connections:** South → R03 (locked entry, Key A used) | North → R08 (open — far side of shadow wall, north exit)  | East → R08 (open — near side, east wall exit before shadow wall)

**Contents:**
- ⚔️ **2× Void Wraith** (far side of shadow wall) + **1× Shadow Clone** (near side).
- ▓▓▓ **Void-shadow wall** — crossing it requires walking directly into the dark. The Void Compass continues to function on the far side (it does not rely on light). Enemies on the far side cannot target the player until the player crosses (they hear but cannot pinpoint).
- [★] **Small Key D** — far side of the shadow wall, on the north wall beside the exit. Behind one of the Void Wraiths.
- [★] **Gold Pouch (35 coins)** — near side, east wall, beside the east exit.
- [★] **Healing Herb ×2** — far side, west alcove.
- [?] **Lore Tablet** (near side, south wall): *"The shadow wall is not dangerous. It is merely uncomfortable. Most of the sanctum's defences work this way — not lethal, just wrong enough to make you hesitate. Hesitation is the trap."*
- [≋] **Environmental rift** (far side, west wall) — a passive rift. Entering it transports the player to a brief void-space then deposits them back on the near side of the shadow wall. A one-way shortcut back, useful for retreating.
- [🪜] **Ladder climb point** — near side, east wall. Reaches a ledge above the shadow wall with a **Gold Pouch (20 coins)** and a clear view across both sides of the room simultaneously — spatial context for how the shadow wall actually works.

**Enemies:** 2× Void Wraith, 1× Shadow Clone
**Secrets:**
- 🔍 Ladder ledge — gold and above-wall perspective.

---

### R08 — CENTRAL NEXUS
**Shape:** Large octagon. The hub of the dungeon's upper half. Eight exits — but only five lead to real destinations. Three are mirror traps. The Void Compass is essential here. The floor has a dimensional rift running diagonally across it — a visible crack of blue-white light, 1 tile wide. Stepping on it deals no damage but briefly phases the player (1 second of translucency — enemies cannot target the player during this phase window).
**Size:** Very Large
**Connections:** South → R04 (open — real) | South-West → R07 (open — real) | North → R11 (open — real) | North-East → R12 (open — real) | East → R09 (open — real) | North-West → *(MIRROR TRAP)* | West → *(MIRROR TRAP)* | South-East → *(MIRROR TRAP)*

**Contents:**
- ⚔️ **2× Void Wraith** + **2× Shadow Clone** — densest enemy encounter in the dungeon outside the boss. The octagonal room gives enemies and player equal movement options — the most tactically complex fight so far.
- [🧭] **Void Compass critical zone** — in this room the Compass is mandatory. The three mirror-trap exits are visually indistinguishable from real exits. The needle spins erratically when facing a trap exit; pulses when facing a real one. Teaches the player to check the Compass before every exit choice.
- [!] **Pressure plate** (center of the diagonal rift) — standing on it while the rift is in its pulse state (every 8 seconds the rift brightens — a 2-second window) opens a wall panel on the north wall, revealing **Small Key E** inside.
- [★] **Small Key E** — north wall panel, revealed by timed pressure plate.
- [?] **Lore Tablet** (south wall): *"The nexus was designed with more exits than necessary. The designers believed that choice was a form of honesty — if you offer enough false choices, the true choice becomes visible by elimination. The compass is faster."*
- [★] **Gold Pouch (45 coins)** — on a raised platform accessible only by climbing the diagonal rift edge (1-tile hop onto the rift itself — phases the player briefly, but reachable). Platform is above the main floor by 2 tiles.
- The three mirror-trap exits look like doorways but lead to void-space corridors that loop back to R08's south entrance after 10 seconds of walking. No damage, no punishment — just wasted time and mild disorientation.

**Enemies:** 2× Void Wraith, 2× Shadow Clone
**Secrets:** None — the room's complexity IS the puzzle.

---

### R09 — RIFT ROOM
**Shape:** Wide square. Three large environmental rifts occupy the east, west, and north walls — passive portals, each one connected to a different room in the sanctum. The connections shift every 60 seconds (a low resonant hum announces the shift). The Void Compass indicates which room each rift currently connects to.
**Size:** Large
**Connections:** West → R08 (open — standard corridor exit) | Rift connections (rotating): East rift cycles between R03 / R06 / R12 | West rift cycles between R04 / R07 / R14 | North rift cycles between R11 / R13 / R15a

**Contents:**
- ⚔️ **1× Void Wraith** + **2× Mirror Shade** — the Mirror Shades here spawn from the rift walls rather than reflections. They emerge as silhouettes stepping out of the rift light.
- [≋] **Three large environmental rifts** — rotating connections, 60-second cycle. The Compass shows the current destination of each rift (needle points direction, Compass face shows room symbol). Using rifts is optional but provides significant navigation shortcuts. This is the dungeon's sandbox mechanic — experienced players can use rift-hopping to bypass large sections.
- [★] **Small Key F** — on the east wall between two rifts, on a stone bracket. Standard retrieve.
- [★] **Gold Pouch (30 coins)** — on the floor near the west corridor exit.
- [?] **Lore Tablet** (south wall): *"The rifts were not built. They formed. The sanctum generates them the way a wound generates heat — as a byproduct of something that is trying to heal itself and cannot. The dragon's piece is the wound. The rifts are the fever."*
- [★] **Healing Herb ×2** — deposited near the north rift by whatever room it was last connected to. Changes position every 60 seconds as the rift reconnects (the herbs slide back through). Slightly comedic if the player notices — the dungeon is briefly a slot machine for healing herbs.

**Enemies:** 1× Void Wraith, 2× Mirror Shade
**Secrets:** None — the rift mechanic is the room's entire design.

---

### R10 — THE LOOP CORRIDOR
**Shape:** Long horizontal rectangle — a corridor. The east and west ends both look identical: same torchlight, same stone, same slight curve. It appears to be a straight corridor connecting two rooms. It is not. The corridor loops — entering from the west deposits the player back at the west entrance after 20 seconds of walking. The same happens from the east.
**Size:** Medium (corridor)
**Connections:** West → R08 (real — escape from the loop, Compass required) | East → R12 (real — forward progress, Compass required) | [LOOP] — entering incorrectly loops back to entry point

**Contents:**
- **The Loop** — the corridor's walls curve imperceptibly back on themselves. Without the Void Compass, the player cannot tell they are looping until they see the entrance again. With the Compass: the needle points consistently east (toward R12). If the player is looping, the needle eventually points back west — the signal to stop, reverse, and re-enter from a slightly different angle (hugging the north wall rather than the center breaks the loop — a subtle spatial trick that the Compass's directional pulse indicates).
- ⚔️ **3× Shadow Clone** — appearing at intervals along the corridor. Their positions repeat on each loop (they are generated by the loop itself). Fighting the same Clones twice without progress is a gameplay signal that something is wrong.
- [🧭] **Void Compass is mandatory** — the only mechanic in the room. Teaches the player to trust the Compass absolutely.
- [★] **Gold Pouch (50 coins)** — appears in the center of the corridor once the player navigates correctly (breaks the loop). Reward for persisting.
- [?] **Lore Tablet** (center of corridor — only visible when the loop is broken): *"You broke the loop. Most don't. The corridor has held seventeen navigators in permanent repetition. We considered it a kindness — better to loop than to arrive unprepared."*
- No enemies once the loop is broken — the corridor becomes a normal passageway.

**Enemies:** 3× Shadow Clone (per loop iteration)
**Secrets:** None — the loop itself is the puzzle.

---

### R11 — MIRROR HALL
**Shape:** Wide horizontal rectangle. Both the north and south walls are covered in large mirror panels — 6 panels each, floor to ceiling. Every reflection shows a subtly different version of the room — some with different numbers of exits, some with enemies that aren't present, some with treasure that isn't there. The real room has one real exit (north) and one mirror-trap exit (east — R11m).
**Size:** Large
**Connections:** South → R08 (open — real) | North → R14 *(LOCKED — Small Key E required)* | East → R11m *(MIRROR TRAP — loops back to R07 south entrance)*

**Contents:**
- ⚔️ **2× Mirror Shade** + **1× Void Wraith** — the Mirror Shades spawn from the mirror panels. Each panel can spawn one Shade. Breaking a panel (Hammer) prevents that panel from spawning. 12 panels total — player need not break all of them, just the ones currently spawning Shades.
- [🪞] **Mirror panels** — 12 total. Hammer to break individually. Breaking the correct panel (third from left on the north wall) reveals a niche containing **Gold Pouch (55 coins)**. Breaking the fifth panel on the south wall reveals **Healing Herb ×3**. Other panels reveal solid wall. The Void Compass pulses when aimed at a panel with something behind it.
- [🧭] **Void Compass use** — east exit causes needle to spin (mirror trap). North exit causes needle to pulse steadily (real exit, locked). Compass can also identify which mirror panels have niches behind them.
- [★] **Gold Pouch (55 coins)** — third north panel niche (Hammer).
- [★] **Healing Herb ×3** — fifth south panel niche (Hammer).
- [?] **Lore Tablet** (west wall, the only non-mirror surface): *"Twelve mirrors. Two truths. Ten lies. The compass finds the truths. Or you could break all twelve. Either approach is valid. The sanctum does not judge methodology."*

**Enemies:** 2× Mirror Shade, 1× Void Wraith
**Secrets:**
- 🔍 Mirror panels — Compass identifies, Hammer breaks. Two contain items.

---

### R11m — MIRROR TRAP: FALSE MIRROR HALL
**Shape:** Appears identical to R11 in every way. Same dimensions, same mirror panels, same lighting. The differences: the north door is 2 tiles further left than in R11, the lore tablet is missing, and the Void Compass needle spins constantly (no real exit in this room — only the south exit, which loops back to R07's south entrance).
**Size:** Large
**Connections:** North → *(FALSE — solid wall, appears to be a door)* | South → R07 south entrance *(loop exit — deposits player as if arriving from R07's south)*

**Contents:**
- ⚔️ **2× Mirror Shade** — same as R11. Designed to keep the illusion intact.
- [🪞] **Mirror panels** — identical appearance to R11. Breaking them all reveals solid wall behind every single one. No niches, no rewards. The emptiness is the tell.
- [🧭] **Void Compass** — needle spins throughout. Clear signal this is a trap room.
- [?] **No lore tablet** — its absence is intentional. A player who remembers there was a tablet in R11 will notice its absence here. Environmental storytelling.
- The north door appears identical to R11's real north door — same frame, same keyhole. But it does not open. The keyhole is a facade (no actual mechanism behind it). Small Key E does not work here — the key simply does not fit. Another tell.
- After 30 seconds in this room with no progress, a faint voice (the sanctum): *"This is not the way. You know this. Leave."*

**Enemies:** 2× Mirror Shade
**Secrets:** None — this IS the trap.

---

### R12 — COMPASS ROOM
**Shape:** Square. The room is dominated by a massive floor-inlaid Void Compass — a 5-tile-diameter carved circle with directional markings, cardinal symbols, and a rotating indicator needle (mechanical, set into the stone, slowly turning on its own). The room's exits are unlabeled. The floor compass shows which exit leads where — but only if the player's Void Compass is calibrated (done in R01).
**Size:** Large
**Connections:** South → R08 (open — real) | North → R14 *(LOCKED — Small Key F required)* | West → R12m *(MIRROR TRAP — loops back to R08 south entrance)* | East → R13 (open — real)

**Contents:**
- ⚔️ **1× Void Wraith** + **1× Mirror Shade** — both spawn from the floor compass design itself (rising from the inlaid stonework as void-forms). Defeated normally.
- [🧭] **Floor Void Compass** — the mechanical floor compass rotates slowly. When the player's Void Compass is held over the center, the floor compass locks onto the player's Compass bearing. For 60 seconds all exits display faint colored outlines: green for real exits, red for mirror traps. A massive navigational aid — the most informative single room in the dungeon.
- [!] **Pressure plate** (center of floor compass) — standing on it while carrying a Small Key charges the Key with compass energy. A charged Key can open any standard lock in the dungeon regardless of which door it belongs to (one-time use). Allows the player to use any spare key on any locked door — powerful flexibility reward for thorough exploration.
- [★] **Gold Pouch (45 coins)** — on a stone plinth on the east side.
- [?] **Lore Tablet** (north wall): *"The floor compass was calibrated to the dragon's piece. The piece is north of here. It is always north of here, regardless of which north you are currently in. Trust that."*

**Enemies:** 1× Void Wraith, 1× Mirror Shade
**Secrets:** None — the floor compass mechanic is the room's entire gift.

---

### R12m — MIRROR TRAP: FALSE COMPASS ROOM
**Shape:** Appears identical to R12. Same floor compass, same room dimensions, same stone plinths. Differences: the floor compass rotates in the wrong direction (counter-clockwise instead of clockwise — very subtle), the plinth is empty, and the Compass needle spins.
**Size:** Large
**Connections:** East → *(FALSE — appears to lead to R13, solid wall)* | North → *(FALSE — appears to lead to R14, solid wall)* | West → R08 south entrance *(loop exit)*

**Contents:**
- ⚔️ **1× Void Wraith** + **1× Mirror Shade** — same as R12.
- [🧭] **Floor compass rotates wrong direction** — the only physical tell besides the Compass needle spinning. Easily missed on first visit; obvious in retrospect.
- [?] **No lore tablet** — absent, same as R11m.
- Empty plinth where the gold was in R12. No reward anywhere in the room.
- After 30 seconds, the sanctum voice: *"Wrong compass. Wrong room. You are closer than you think, but in entirely the wrong direction."*

**Enemies:** 1× Void Wraith, 1× Mirror Shade
**Secrets:** None — this IS the trap.

---

### R13 — RIFT BRIDGE
**Shape:** Long vertical rectangle. A stone bridge spans the room lengthwise over a void — the floor below the bridge is not stone or water but pure dimensional rift (deep blue-black, faintly luminescent). Falling off the bridge drops the player into the rift and deposits them back at the bridge's south entrance (no damage — pure repositioning). The bridge is 2 tiles wide with no railings.
**Size:** Large
**Connections:** South → R12 (open — bridge south end) | North → R14 (open — bridge north end)

**Contents:**
- ⚔️ **2× Void Wraith** — patrol the bridge. Their rift-step blink ability combined with the narrow bridge is the room's combat challenge. A Void Wraith blinking behind the player on a 2-tile-wide bridge with void on both sides is genuinely dangerous.
- [≋] **Void floor** — falling off the bridge (pushed by a Wraith blink-impact or misstepped during combat) deposits the player back at the south entrance. No damage but positional reset — potentially costly mid-combat.
- [★] **Small Key G** — on the bridge, midway, on a raised 1-tile platform (a stone pillar-top). Stepping from the bridge onto the pillar-top requires a precise 1-tile step sideways — possible without tools, but the Strong Arm Glove can pull it toward the player from the bridge (Glove reach mechanic).
- [★] **Gold Pouch (40 coins)** — on the bridge near the north end, dropped by a Wraith on death.
- [?] **Lore Tablet** (south end of bridge, at the entrance): *"The bridge crosses the void between what the sanctum is and what it was trying to be. On the other side: the room closest to the dragon's piece. Do not fall. The void is patient but it is not kind."*
- The void below the bridge is beautiful in a terrifying way — depth that has no bottom, faint light moving in patterns far below. Looking down is optional but atmospheric.

**Enemies:** 2× Void Wraith
**Secrets:** None — the bridge is the hazard.

---

### R14 — THE VOID CHAMBER
**Shape:** Very large octagon. The innermost chamber before the boss. The walls are not stone — they are solid void, black and shifting. The floor is cracked and partially translucent (the void visible beneath). The Boss Door is on the north wall, framed in white stone — the only bright surface in the room. Four void-column pillars ring the center. The room hums.
**Size:** Very Large
**Connections:** South → R11 (locked entry, Key E used) | South → R12 (locked entry, Key F used) | South → R13 (open) | North → R18 *(BOSS DOOR — requires Boss Key)* | West hidden rift → R15a *(SECRET)* | East hidden rift → R15b *(SECRET)*

**Contents:**
- ⚔️ **2× Void Wraith** + **2× Mirror Shade** + **1× Shadow Boss** — new enemy. The Shadow Boss is a full-size void entity — 3 tiles tall, slow but extremely powerful. Immune to normal attacks — must be struck with a charged Void Compass strike (equip Compass, hold attack button for 2 seconds to charge, release — fires a compass-energy beam that damages void entities). Dies in 5 charged strikes.
- **Void Altar Puzzle** — four void-column pillars, each with a directional symbol. The player must face each pillar and input the correct directional command (shown on the pillar face) while the Void Compass is equipped — this calibrates each column. When all four are calibrated: the Boss Key rises from the floor on a stone plinth.
- [★] **Boss Key** — rises from floor plinth after four-column calibration.
- [?] **Lore Tablet** (east wall, pale stone): *"Duplexis was not created here. It arrived. It followed the dimensional rifts inward like a current follows a river. It found the dragon's piece and it stayed. It does not guard. It mirrors. Everything it encounters, it becomes a copy of. It has been copying the dragon's piece for so long that it has forgotten what it originally was."*
- **West hidden rift → R15a:** One of the void-wall panels on the west side shimmers slightly differently from the others — a slightly lighter shade of black, if that makes sense. Standing near it with the Void Compass causes the needle to point directly at it. Stepping through opens the rift to R15a.
- **East hidden rift → R15b:** A rift-crack on the east floor (visible as a thin blue-white line) — walking along the crack with the Compass equipped causes the crack to widen into a passage to R15b. Requires the Compass to trigger.
- ░░░ **Partially translucent floor sections** — 6 tiles flicker between solid and void-visible. Standing on a void-state tile deals minor damage. The pattern is predictable — same 6 tiles on a 4-second cycle.

**Enemies:** 2× Void Wraith, 2× Mirror Shade, 1× Shadow Boss
**Secrets:**
- 🔍 **West hidden rift → R15a:** Lighter void-panel, Compass points to it.
- 🔍 **East hidden rift → R15b:** Floor rift-crack widens with Compass equipped.

---

### R15a — SECRET ROOM: THE STILL POINT *(West)*
**Shape:** Small square. The void walls here are completely still — no shimmer, no shift. The first truly quiet place in the dungeon. A single stone bench. A single candle, lit.
**Size:** Small
**Connections:** East → R14 (rift passage — Compass equipped to open)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **📜 Lore Scroll — "The Dragonbinder's Sixth Entry"** — on the bench beside the candle. Reads: *"The sixth fragment is a wing. The right wing. When I hold the left wing piece and stand near this room I can feel them pulling toward each other. They remember being connected. I wonder if all eight pieces feel the others — if the dragon, wherever it is, feels them being gathered. I wonder if it is frightened. I wonder if I should be."*
- [★] **Rare Healing Herb** — full HP restore, beside the lore scroll.
- [★] **Gold Pouch (95 coins)** — highest single gold reward in the dungeon, under the bench (placed there by someone who sat here and left something behind).
- The candle has been burning for an unknowable amount of time. It is not melted down. It simply burns. No explanation offered.
- Mira's drawing — age 8 this time. A dragon with both wings spread, whole, in flight. Below it: *"I think it wants to come back."*

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R15b — SECRET ROOM: THE ECHO CACHE *(East)*
**Shape:** Small irregular space — not quite a room, more a dimensional pocket. The walls are inconsistent — they shift slightly as the player looks at them. No fixed geometry. The floor is solid, at least.
**Size:** Small
**Connections:** West → R14 (floor rift-crack — Compass required to widen)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **Gold Pouch (70 coins)** — on a stone surface that appears from the shifting wall.
- [★] **Healing Herb ×3** — floating, slowly rotating in the air. Collectable normally.
- [★] **📜 Lore Scroll — "A Letter, Unsent"** — readable item. Handwriting identical to the adventurer's journal from Level 1. *"I made it to the sixth dungeon. I don't think I'll make it to the seventh. I'm leaving this here for whoever comes after. The pieces want to be together. The dragon wants to be whole. But I've read enough of the old texts to know: the seal was put there for a reason. When you find all eight pieces — and you will — think very carefully before you place the last one. That's all. Good luck."* The letter is signed with an initial: M.
- The pocket space begins to collapse after 60 seconds (walls close inward — non-lethal, just a time pressure prompt to collect and leave). Player is deposited back at R14's east floor rift if they stay too long.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R18 — BOSS CHAMBER: THE MIRROR SANCTUM
**Shape:** Very large circle. A perfect circle — the most geometrically precise room in the dungeon. The walls are entirely covered in mirrors, floor to ceiling, all the way around. In the center: a raised circular platform, 3 tiles in diameter. The Dragon Piece is suspended above the platform inside a void-crystal containment — slowly rotating, visibly a wing shape, casting fractured light across every mirror surface. Duplexis is inside the containment, coiled around the Dragon Piece. It has been here so long the containment has partially merged with it.
**Size:** Very Large — biggest room in the dungeon.
**Connections:** South → R14 *(BOSS DOOR — Boss Key required)*

**Contents:**
- 🔒 **BOSS DOOR** — south wall. Boss Key required. The door is a mirror itself — the player's reflection stares back as they approach. The reflection opens the door from the inside. Unsettling symmetry.
- **BOSS: Duplexis, the Mirror Lich** — a void entity that has spent centuries absorbing the identity of everything it encounters. It has copied the Dragon Piece so completely that it now appears as a dragon — but wrong, reversed, made of mirrors and void rather than scale and bone. It has no fixed form; it shifts between copied forms mid-fight.
- [★] **🧩 Dragon's Right Wing (Piece 6/8)** — released from the void-crystal containment on boss defeat. The containment shatters, the piece floats down to the platform floor.
- [★] **🌀 Portal / Warp Tool** — appears on the platform after the containment shatters, materialising from the void-crystal itself. The crystal was, apparently, a proto-portal device — the dragon's piece transformed it into the Portal Tool over centuries of proximity. It is now usable.
- The mirror walls reflect the fight — every attack, every movement, every phase of the boss is reflected infinitely. Duplexis uses these reflections to spawn false copies.
- ░░░ **Unstable floor sections** — 8 tiles around the platform edge, flickering on a 5-second cycle. Falling into a void-state tile during combat is a genuine hazard — deposits player at the south wall (near the Boss Door), requiring repositioning.

**Boss — Duplexis, the Mirror Lich:**
- **Phase 1 (100%–65% HP):** Duplexis creates **3 Mirror Copies** of itself — all visually identical. The Void Compass identifies the real Duplexis: the needle locks onto the real one (the copies cause needle spin). Only the real Duplexis takes damage. Copies dissolve on hit (1 hit each) and new ones reform after 10 seconds. Real Duplexis attacks: void-beam (straight line, 3-tile reach, 1.5 sec telegraph), mirror-slam (slaps the floor, shockwave in a ring around it, dodge by rolling away), copy-swap (Duplexis and one copy swap positions — the Compass immediately re-locks to the new real position).
- **Phase 2 (65%–35% HP):** Copies increase to **5**. Duplexis begins mimicking the player's attack pattern — if the player uses the same attack 3 times in a row, the next copy uses that exact attack. Forces varied combat. Also begins using mirror-walls offensively: void-beams that bounce off mirrors change trajectory. The Compass still tracks the real Duplexis but the bouncing beams are harder to predict.
- **Phase 3 (35%–0% HP):** Copies increase to **7** — filling the arena. Duplexis drops its copied dragon form and reveals its original void-state: formless, vast, occupying the entire ceiling. Attacks now come from above (void-drops — circular shadow telegraphs landing zone, dodge out of shadow). The Compass in this phase must be aimed at the ceiling to track the real Duplexis above. Charged Compass strikes (same as Shadow Boss in R14) deal triple damage in this phase. Defeat: Duplexis contracts — all copies collapsing inward into the real one. The void-form compresses, crystallises, and shatters. The Dragon Piece drops. The mirrors crack simultaneously. The room goes quiet. The Portal Tool forms from the shards.
- **Defeat:** Every mirror in the room shows the same image: the full dragon, unbroken, all six pieces gathered so far assembled into a partial form. It is the first time the player sees what they are building toward. Then the mirrors go dark. The Portal Tool lies on the platform, waiting.

**Secrets:** None — boss rooms are always clean and readable.

---

## DUNGEON FLOW SUMMARY

```
CRITICAL PATH:
R01 (Compass calibrated, map) → R02 (Compass navigation, fight Clones)
→ R04 (hub, floor compass — fight) → R08 (Compass mandatory, fight, get Key E via timed plate)
→ R10 (loop corridor — Compass to break loop) → R11 (mirror hall, Key E used, fight)
→ R13 (rift bridge, fight) → R14 (void altar puzzle, get Boss Key)
→ R18 (boss, get Piece 6 + Portal Tool)

KEY LOCATIONS:
  Small Key A — R03 (dimensional crack, west wall)
  Small Key B — R05 (high ledge, Ladder required)
  Small Key C — R06 (north mirror wall, Hammer required)
  Small Key D — R07 (far side of shadow wall, behind Wraith)
  Small Key E — R08 (timed pressure plate panel)
  Small Key F — R09 (east wall bracket)
  Small Key G — R13 (bridge midpoint pillar-top, Glove reach or step)
  Boss Key    — R14 (floor plinth after void altar puzzle)

OPTIONAL PATHS:
  R04 → R03 (Key A + healing — opens R07 approach)
  R04 → R05 (Key B via Ladder — opens R06)
  R05 → R06 (all mirrors broken — Key C + gold)
  R06 → R09 (Key C → R09 rift room, Key F, rift shortcuts)
  R07 → R08 via shadow wall (Key D shortcut to upper dungeon)
  R09 → Rift shortcuts (rotating rifts provide optional fast-travel)
  R12 → floor compass key-charge (any spare key becomes universal)
  R14 → R15a (west void panel, Compass to open — Dragonbinder 6 + gold)
  R14 → R15b (east floor rift, Compass to widen — unsent letter + gold)

VOID COMPASS USAGE SUMMARY:
  R01 — calibration (mandatory, one-time)
  R02 — navigate 4-door room (tutorial)
  R05 — distinguish R05 from R03 (false hall identification)
  R08 — navigate 8-exit nexus (mandatory)
  R10 — break loop corridor (mandatory)
  R11 — identify real exit, find mirror niches (optional for niches)
  R12 — floor compass interaction (navigational bonus)
  R14 — find hidden rifts to secrets (optional)
  R18 — track real Duplexis in boss fight (mandatory for damage)

MIRROR TRAP SUMMARY:
  R02 east/west doors — loop back to R02 (tutorial — low penalty)
  R05 east door — loops back to R04 east entrance
  R08 NW/W/SE exits — loops back to R08 south entrance
  R11 east door (R11m) — loops back to R07 south entrance
  R12 west door (R12m) — loops back to R08 south entrance

BLOCKED UNTIL FUTURE TOOLS:
  Portal Tool earned here — Level 7 is first dungeon requiring it
  All prior tool blocks still apply and are usable throughout
```

---

## FULL ITEM & TOOL CHECKLIST

| Item | Location | Required? |
|------|----------|-----------|
| 🗺️ Dungeon Map | R01 — east wall geometric carving | Auto-collected |
| 🧭 Void Compass | World — dropped by wandering Shade | Yes (navigation + boss fight) |
| 🗝️ Small Key A | R03 — dimensional crack, west wall | Optional (opens R07 from south) |
| 🗝️ Small Key B | R05 — high ledge (Ladder required) | Optional (opens R06) |
| 🗝️ Small Key C | R06 — north mirror wall (Hammer) | Optional (opens R09 from R06) |
| 🗝️ Small Key D | R07 — far side of shadow wall | Optional (opens R08 west approach) |
| 🗝️ Small Key E | R08 — timed pressure plate panel | Yes (opens R11 north door) |
| 🗝️ Small Key F | R09 — east wall bracket | Yes (opens R12 north door) |
| 🗝️ Small Key G | R13 — bridge midpoint pillar-top | Optional (universal if charged at R12) |
| 🔑 Boss Key | R14 — floor plinth after void puzzle | Yes (opens R18) |
| 📜 Lore Scroll (Dragonbinder 6) | R15a — bench beside candle | Optional (collectible) |
| 📜 Lore Scroll (Unsent Letter — M) | R15b — dimensional pocket | Optional (collectible — critical lore) |
| 🧩 Piece 6/8 | R18 — platform after boss | Main Objective |
| 🌀 Portal / Warp Tool | R18 — forms from shattered crystal | Yes (needed for Level 7) |
| 💰 Gold (total ~680) | Throughout | Optional |
| 🌿 Healing Herbs | R03, R05, R07, R09, R11, R13, R15a, R15b | Optional |

---
---

*[Level VII — The Aetherian Spire will be added to a new document upon completion]*
