# 🐉 THE DRAGON SEAL — Dungeon Design Document
*Visual ASCII Maps + Room-by-Room Notes*
*Scale: Small early → Large late | Format: ASCII map + notes*

---
---

# LEVEL II — 🏚️ RUINS OF ASHENFALL
**Theme:** Crumbling stone fortress overtaken by ash, rot, and time. Collapsed floors, exposed ledges, vertical shafts, hanging chains, and broken staircases. The Ladder is the key tool — used constantly to reach upper levels and descend into collapsed sub-levels.
**Difficulty:** Early. Teaches: vertical navigation with Ladder, collapsing floor tiles, patrol enemy patterns, torch-lit dark rooms, multi-floor layouts.
**Total Rooms:** 13 *(9 main + 2 secret + 1 sub-level + 1 boss)*
**Puzzle Piece:** 🧩 Dragon's Tail Body *(Piece 2/8)*
**Tool Earned:** 🔨 Hammer
**New Mechanics Introduced:** Ladder traversal (up/down ledges) · Collapsing floor tiles · Dark rooms (torch required) · Patrol enemy routes · Upper / lower floor splits

---

## ASCII MAP

**Dungeon silhouette: TAIL BODY** — elongated serpentine S-curve. The path
winds back and forth, 1–2 rooms wide, evoking a sinuous tail segment.

```
                                        ┌───────────┐
                                        │    R13    │
                                        │   BOSS    │
                                        │ [WARDEN]  │
                                        └─────┬─────┘
                                              │ [BOSS DOOR]
                                        ┌─────┴─────┐
                              ◄══════════    R10     ══════════►
                           [SECRET]     │  The Ash   │   [SECRET]
                                        │  Belltower │
                                        └──┬─────┬──┘
                                  ┌────────┘     └────────┐
                             ┌────┴────┐             ┌────┴────┐
                             │   R08   │             │   R09   │
                             │Collapsed│             │  Chain  │
                             │  Hall   │             │  Hall   │
                             └────┬────┘             └─────────┘
                                  │
                             ┌────┴────┐
                             │  R08b   │
                             │SUB-LEVEL│
                             │ (lower) │
                             └─────────┘
                        ┌─────────┘
                        │
                   ┌────┴────┐
                   │   R07   │
                   │ Warden  │
                   │ Barracks│
                   └────┬────┘
                        │
              ┌─────────┴─────────┐
              │                   │
         ┌────┴────┐         ┌────┴────┐
         │   R05   │         │   R06   │
         │  Dark   │         │  Guard  │
         │ Armory  │         │  Post   │
         └─────────┘         └────┬────┘
                                  │
                        ┌─────────┴─────────┐
                        │                   │
                   ┌────┴────┐         ┌────┴────┐
                   │   R03   │         │   R04   │
                   │ Fallen  │         │  Ash    │
                   │  Nave   │         │  Court  │
                   └────┬────┘         └─────────┘
                        │
                   ┌────┴────┐
                   │   R02   │
                   │Gatehouse│
                   │  Entry  │
                   └────┬────┘
                        │
                   ┌────┴────┐
                   │   R01   │
                   │  ENTRY  │
                   │ (Start) │
                   └─────────┘

SECRET ROOMS (not on dungeon map):
  R11a ◄═══ west wall of R10
  R11b ═══► east wall of R10

SUB-LEVEL:
  R08b ↓ below R08 (Ladder required to descend)
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
  [🪜]          = Ladder climb point (up or down)
  ///           = Cracked / breakable wall (Hammer needed)
  ~~~           = Water / flooded floor
  ░░░           = Collapsing floor tiles (fall through on 2nd step)
  ▓▓▓           = Dark zone (torch / lantern required to navigate)
  ↑ ↓           = Upper / lower floor transition
```

---

## ROOM-BY-ROOM NOTES

---

### R01 — ENTRY CHAMBER
**Shape:** Wide horizontal rectangle. A collapsed outer wall forms the entrance — open sky visible through the breach. Ash drifts in from outside.
**Size:** Large
**Connections:** North → R02 (open archway)

**Contents:**
- 🗺️ **DUNGEON MAP** — pinned to a rotting wooden board just inside the entrance, held in place by a rusted dagger. Shows R01–R10 and R13. Sub-level R08b and secret rooms R11a/R11b are NOT shown.
- [?] **Lore Tablet** (east wall, cracked but readable): *"Ashenfall stood for three centuries before the dragon's sealing shook its foundations. The Warden never left. He would not abandon his post."*
- A collapsed staircase on the west side leads nowhere — the upper floor has caved in entirely. Atmospheric ruin detail.
- Ash drifts across the floor in slow diagonal lines — wind effect, purely visual.
- Two rusted iron sconces on the walls — torches long burned out. Establishes the "dead fortress" tone.
- A crumbled statue near the south wall, too eroded to identify. One hand still raised, pointing north.

**Enemies:** None.
**Secrets:** None.

---

### R02 — GATEHOUSE ENTRY
**Shape:** Tall vertical rectangle — a gatehouse corridor. High ceiling with a portcullis chain mechanism hanging above. Feels imposing and narrow.
**Size:** Medium
**Connections:** South → R01 (open) | North → R03 *(LOCKED — Small Key A required)* | East alcove (dead end — Ladder climb point up to ledge)

**Contents:**
- ⚔️ **2× Ashwalker** — new enemy. Slow, armored in rusted plate. Takes 5 hits normally, or 2 hits from behind. Patrols a fixed 4-tile route. Introduces patrol mechanics — player can observe and time approach.
- [★] **Small Key A** — hanging on a hook on the east wall inside the alcove, on a raised ledge 2 tiles above floor. Requires **🪜 Ladder** to reach. First mandatory Ladder use in the dungeon.
- [🪜] **Ladder climb point** — east alcove wall has iron rungs. Player places their Ladder here to reach the Key A ledge.
- [?] **Lore Tablet** (west wall): *"The gatehouse was the last to fall. They say the soldiers kept their posts even as the walls crumbled around them."*
- Portcullis on north side — locked. Chains hang slack. Small Key A opens the winch mechanism beside it.
- A broken sword lies on the floor near the portcullis — remnant of the fortress's last stand. Decorative.

**Enemies:** 2× Ashwalker
**Secrets:** None.

---

### R03 — FALLEN NAVE
**Shape:** Large cross-shaped room — the remnants of a grand hall. The east arm of the cross has fully collapsed, creating a rubble wall. The west arm is accessible. A wide open central space with a partially caved-in roof.
**Size:** Large
**Connections:** South → R02 (locked entry, key used) | North → R06 (open) | East → R04 (open) | East arm → rubble wall *(blocked — Hammer needed, future)*

**Contents:**
- ⚔️ **3× Ashwalker** — two patrol the central nave on fixed routes. One stands still guarding the north exit. Can be snuck past or fought.
- [★] **🧭 COMPASS** — on a stone altar in the western arm. Surrounded by melted candles. Still faintly glowing.
- [★] **Gold Pouch (40 coins)** — behind the standing guard near the north exit. Requires defeating or distracting the guard.
- [?] **Lore Tablet** (south wall, near entry): *"This was the hall of oaths. Every soldier swore here before taking their post. Some oaths outlast the soldiers who made them."*
- Collapsed roof section in the center lets in pale grey light — the only natural light in the dungeon. A column of ash drifts through it.
- /// **East rubble wall** — cracked stone blocking the east arm. Cannot be cleared yet. Hammer required. A visible glint of gold beyond the rubble hints at a reward inside.

**Enemies:** 3× Ashwalker
**Secrets:**
- 🔍 **East rubble wall** — blocked now, but observant players will note the glow beyond it. Hammer (Level 3 tool) will clear it on a return visit, revealing a hidden room with bonus loot.

---

### R04 — ASH COURT
**Shape:** Wide square. An open courtyard, once used for drills. The floor is thick with grey ash — ankle-deep in places. Ash muffles sound; the room feels heavy and quiet.
**Size:** Medium-Large
**Connections:** West → R03 (open, lateral branch). Dead end — no exit other than the way in.

**Connections (corrected):** North → R06 *(LOCKED — Small Key B required)* | No other exits — dead-end branch.

**Contents:**
- ⚔️ **2× Ash Drifter** — new enemy. Thin, fast, difficult to track in the ash-covered floor. Leaves a visible disturbance trail in the ash as it moves — telegraphs position. Dies in 2 hits but moves quickly.
- [★] **Small Key B** — buried in the ash near the center of the room. A faint glimmer visible. Player must walk over it to trigger a pickup prompt. Teaches: the ash floor hides things.
- [★] **Healing Herb ×2** — against the south wall, laid at the base of a crumbled fountain.
- [B] **1× Ash-Covered Boulder** — partially buried in ash. Can be pushed and reveals **Gold Pouch (20 coins)** beneath it. Optional discovery.
- [?] **Lore Tablet** (east wall, barely readable through ash coating): *"The court remembers the weight of boots. The ash holds the shape of every soldier who ever drilled here."*
- The fountain in the south corner is dry and cracked — a skeletal hand reaches out from the ash beside it. Morbid atmospheric detail.

**Enemies:** 2× Ash Drifter
**Secrets:**
- 🔍 Boulder pushed reveals coins beneath — rewards exploration of the ash floor.

---

### R05 — DARK ARMORY
**Shape:** Long horizontal rectangle. No windows, no light sources — completely dark. Shelves, weapon racks (empty), and storage crates line the walls.
**Size:** Medium
**Connections:** West → R06 (open) | No other exits — dead-end branch.

**Contents:**
- ▓▓▓ **DARK ZONE** — room is pitch black without a light source. Without the Lantern (from Level 1 R06) or a torch, the player navigates blind (can only see 1-tile radius around themselves). Enemies are invisible until adjacent.
- ⚔️ **2× Ashwalker** — stationary in the dark. Placed near the item chests. Dangerous without light.
- [★] **Torch ×3** — on a shelf near the entrance. Found immediately even in the dark (placed at entry within 1-tile reach). Torches are consumable light sources usable in dark zones.
- [★] **Gold Pouch (35 coins)** — in a crate on the north wall, visible with any light source.
- [★] **Gold Pouch (25 coins)** — on a weapon rack. Optional.
- [?] **Lore Tablet** (south wall, find-able only with light): *"The armory was stripped before the end. Whatever was left behind was left on purpose."*
- Empty weapon racks and broken crates line every wall — all looted long ago. Atmosphere of abandonment.

**Enemies:** 2× Ashwalker *(hidden in dark)*
**Secrets:** None — the dark itself is the challenge.

---

### R06 — GUARD POST
**Shape:** Compact square. A central room with corridors branching in four directions — a hub and chokepoint. A raised watchtower platform in the center (2 tiles high) with a Ladder climb point.
**Size:** Medium
**Connections:** South → R03 (open) | East → R04 *(LOCKED — Small Key B required)* | West → R05 (open) | North → R07 (open)

**Contents:**
- ⚔️ **1× Ashwalker Captain** — new, stronger enemy variant. Wears a full helm (immune to frontal hits — must circle behind). Patrols the raised platform and descends on a set route. Drops **Gold Pouch (30 coins)** on death.
- [🪜] **Ladder climb point** — iron rungs on the central platform. Climbing to the top reveals a **Lore Scroll** on the platform surface.
- [★] **📜 Lore Scroll — "The Warden's Standing Order"** — atop the watchtower platform. Reads: *"The Warden's last written order, still pinned to the post board: 'Hold the inner keep. Do not open the Boss Door under any circumstance. The thing inside must not be woken.' It has been woken."*
- [?] **Lore Tablet** (north wall, between the three exits): *"Three roads. The barracks, the great hall, the chain yard. All lead to the inner keep. None lead out."*
- A cracked bell hanging from the ceiling on a rusted chain — if struck (any attack), it rings and causes all Ashwalkers in R07 to become temporarily alert (patrol speed doubles for 30 seconds). Optional hazard — teaches environmental awareness.

**Enemies:** 1× Ashwalker Captain
**Secrets:** None — but the bell mechanic rewards players who notice and avoid it.

---

### R07 — WARDEN'S BARRACKS
**Shape:** Wide horizontal rectangle. Rows of collapsed bunk frames line the walls. A raised sleeping loft runs along the north wall, 3 tiles high, accessible only by Ladder.
**Size:** Large
**Connections:** South → R06 (open) | North → R08 (open) | East → R09 (open, lateral branch)

**Contents:**
- ⚔️ **3× Ashwalker** — two on the ground floor (patrol fixed routes between bunk rows), one on the loft above (stationary, watching the room below). Loft guard has a wider attack range — drops rocks on the player below if alerted.
- [🪜] **Ladder climb point** — west end of the north wall. Required to reach the loft.
- [★] **Gold Pouch (25 coins)** — on the loft level, near the loft guard. Incentivises climbing.
- [★] **Healing Herb ×2** — on the ground floor, behind a collapsed bunk frame in the southwest corner. Partially hidden.
- [B] **2× Collapsed Bunk Frame** — pushable. Moving them reveals **cracked floor tiles** beneath (decorative, not functional yet — seeds Hammer expectation).
- [?] **Lore Tablet** (east wall): *"The soldiers ate, slept, and drilled until there was nothing left to protect. Then they drilled some more. Duty is a cage some men build around themselves."*
- ░░░ **Collapsing floor tiles** — a 3-tile section in the center of the room. First step causes them to crack and shake (audio/visual warning). Second step causes them to fall, dropping the player to a 1-tile-fall damage pit below (minor damage). A plank on the west wall can be pushed across the gap as a bridge — optional puzzle.

**Enemies:** 3× Ashwalker
**Secrets:** None.

---

### R08 — COLLAPSED HALL
**Shape:** Irregular — a grand hall mid-collapse. The floor has partially given way in the center, creating a large pit. The pit is 3 tiles wide and spans the full east-west width of the room. The only way across is via a narrow intact ledge on the east wall OR by descending into the sub-level below.
**Size:** Large
**Connections:** South → R07 (open) | North → R10 *(LOCKED — Small Key C required)* | Down via pit → R08b *(sub-level, Ladder required to descend safely)*

**Contents:**
- ⚔️ **2× Ash Drifter** — fast, on the south side of the pit only. Do not cross the pit.
- ░░░ **Collapsed pit** — center of the room. Falling in (without Ladder) deals moderate damage and deposits the player in R08b below. Climbing back out requires the Ladder (iron rungs on north pit wall).
- [🪜] **Ladder climb point** — north wall of the pit interior. Used to descend safely into R08b and climb back out.
- The narrow east-wall ledge is 1 tile wide and runs along the pit's length — player can edge across it to reach the north side without descending. No enemies on the ledge.
- [★] **Small Key C** — on the north side of the pit, past the ledge traverse, on the floor near the locked north door. Requires getting across the pit (ledge walk or sub-level route) to reach.
- [?] **Lore Tablet** (south wall): *"The floor fell during the third tremor. Three soldiers were on the other side when it happened. They never came back across. We assumed they found another way."*

**Enemies:** 2× Ash Drifter
**Secrets:** None — the pit and sub-level are visible and intended, not hidden.

---

### R08b — SUB-LEVEL: THE LOWER HALL
**Shape:** Wide irregular rectangle — the collapsed underside of R08. Rubble piles everywhere. Low ceiling (2 tiles high). Feels claustrophobic versus the grandeur above.
**Size:** Medium
**Connections:** Up → R08 (via Ladder on north wall — iron rungs) | West dead-end alcove | East dead-end alcove

**Contents:**
- ⚔️ **2× Bog Lurker** — carried up from Level 1's flooded grotto concept. Water seeps through the collapsed floor from an underground source. Lurkers inhabit the shallow water pooling in the sub-level's low points.
- ~~~ **Shallow water** — covers the eastern half of the sub-level floor. Slow wade movement. Lurkers hide in it.
- [★] **Gold Pouch (60 coins)** — in the west dead-end alcove, inside a cracked strongbox. Largest single gold reward in the dungeon outside of secrets.
- [★] **Healing Herb ×3** — scattered across the sub-level floor, partially buried in rubble.
- [★] **Gold Pouch (20 coins)** — in the east alcove on a collapsed shelf. Optional.
- [?] **Lore Tablet** (west alcove wall, surprisingly intact): *"The three soldiers who fell were found down here, still at their posts. They had made new posts. Some oaths are older than the buildings that heard them."*
- /// **North rubble wall** — a cracked wall in the northeast corner. Hammer required to break. Cannot be cleared yet. A faint orange glow pulses through the cracks — something is sealed beyond it.

**Enemies:** 2× Bog Lurker
**Secrets:**
- 🔍 **North rubble wall** — sealed. Return with Hammer (Level 3). The orange glow hints at something significant — a bonus chamber with high-value loot or deeper lore.

---

### R09 — CHAIN HALL
**Shape:** Tall vertical rectangle. The entire east wall is covered in massive rusted chains hanging from the ceiling — remnants of a portcullis lift system. Several chains hang low enough to climb. The room has two distinct vertical levels: floor level and an upper walkway 4 tiles up.
**Size:** Large
**Connections:** South → R07 (open, lateral branch from walkway) | Dead end.

**Contents:**
- ⚔️ **1× Ashwalker Captain** + **1× Ash Drifter** — Captain on the floor level, Drifter on the upper walkway.
- [🪜] **Ladder climb point** — west wall has iron mounting brackets. Player can prop their Ladder here to reach the upper walkway if they don't want to climb the chains.
- **Climbable chains** — the hanging chains on the east wall can also be climbed (grab + climb mechanic — simpler than Ladder, no setup required but slightly slower). Three chains hang within reach of the floor.
- [★] **🧭 COMPASS upgrade note** — a scrawled note on the upper walkway: *"The compass in this ruin points true, but only when you're above the ash line."* Flavor text — explains why compass sometimes drifts in heavy ash rooms (game mechanic note).
- [★] **Gold Pouch (30 coins)** — on the upper walkway near the north exit.
- [★] **Torch ×2** — on the floor level near the south entrance, in a wall sconce (still lit — the only lit torches in the dungeon). Collectible.
- [?] **Lore Tablet** (floor level, east wall between chains): *"The chains moved the gates. The gates kept things in. Or out. The Warden never specified which."*
- The upper walkway exit north leads directly into R10's upper tier — gives players who use R09 a height advantage entering R10.

**Enemies:** 1× Ashwalker Captain, 1× Ash Drifter
**Secrets:** None.

---

### R10 — THE ASH BELLTOWER
**Shape:** Large vertical rectangle — the base and lower body of a collapsed belltower. The room has three tiers: floor (ground), mid-level platform (3 tiles up), and upper platform (6 tiles up). The actual bell has fallen and crashed into the floor — it lies on its side in the center, a massive rusted object that acts as cover and obstacle simultaneously.
**Size:** Very Large
**Connections:** South → R08 *(LOCKED — Small Key C required, ground level)* | North → R13 *(BOSS DOOR — requires Boss Key)* | West hidden wall → R11a *(SECRET)* | East hidden wall → R11b *(SECRET)*

**Contents:**
- ⚔️ **2× Ashwalker** — ground floor, patrolling around the fallen bell.
- ⚔️ **1× Ashwalker Captain** — mid-level platform, stationary, watching the floor below.
- [🪜] **Ladder climb points** — west wall (floor to mid-level) and east wall (mid-level to upper platform). Both require the Ladder. Chaining both climbs reaches the top.
- [!] **3× Pressure Plates** — one on each tier (floor, mid, upper). All three must be activated simultaneously. Requires placing a boulder on the floor plate, standing on the mid plate, and... the upper plate requires creative thinking: the fallen bell can be rolled (push mechanic) onto the upper plate if the player climbs to the upper tier and pushes it with strong force — **but the bell is too heavy to push from the floor**. The player must reach the upper tier first via Ladder and push the bell down onto the plate from above. This is the dungeon's main puzzle.
- When all 3 plates are activated: a stone panel on the north wall of the upper platform slides open → **Boss Key** revealed on a plinth.
- [B] **The Fallen Bell** — massive pushable object. Can be rolled from upper tier down onto the floor plate. Irreversible once pushed — but the plate stays held.
- [?] **Lore Tablet** (floor level, south wall): *"The bell rang once when the dragon was sealed. Those who heard it said it rang backwards — like a sound being swallowed instead of released."*
- /// **Cracked west wall** (mid-level platform) — blocked, Hammer needed. Visible crack line.
- /// **Cracked east wall** (floor level) — blocked, Hammer needed.

**Enemies:** 2× Ashwalker, 1× Ashwalker Captain
**Secrets:**
- 🔍 **West hidden wall → R11a:** Floor level. A section of the stone wall behind the fallen bell (west side) has faint carved markings — a deliberate pattern unlike the random cracks elsewhere. Pushing it opens a short tunnel. Observation required.
- 🔍 **East hidden wall → R11b:** Upper platform level. A loose stone block at the back of the upper platform wobbles slightly when the player walks near it (visual shake effect). Pushing it opens a tunnel. Requires reaching the upper platform first (Ladder ×2).

---

### R11a — SECRET ROOM: THE ASH RELIQUARY *(West, Ground Level)*
**Shape:** Small square. Walls are lined with stone niches — like a reliquary or memorial. Each niche holds a small ash-covered object.
**Size:** Small
**Connections:** East → R10 (hidden wall, ground floor)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **📜 Lore Scroll — "The Dragonbinder's Second Entry"** — second page of the Dragon Codex. Reads: *"The second fragment was harder to find. The ruins don't give them up easily — it's as if the dragon's pieces resist being gathered. Or perhaps they're testing the one who gathers them."*
- [★] **Rare Healing Herb** — restores full HP.
- [★] **Gold Pouch (80 coins)** — highest gold reward in the dungeon.
- Each stone niche holds a tiny carved figurine — soldiers, animals, symbols. One niche is empty and freshly dusted. The Warden's?
- A name is carved into the floor: **"VAREK — WARDEN OF ASHENFALL — HE HELD."** This is the boss's name, revealed here before the fight.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R11b — SECRET ROOM: THE HIGH CACHE *(East, Upper Platform Level)*
**Shape:** Small rectangle. Rough stone. Feels like a hasty hiding place, not a designed room.
**Size:** Small
**Connections:** West → R10 (hidden wall, upper platform)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **Gold Pouch (50 coins)** — in a lockbox, lid already open. Someone was here before.
- [★] **Healing Herb ×3** — stacked neatly in a corner.
- [★] **Worn Soldier's Badge** — collectible item. Flavor text: *"Ashenfall garrison, third regiment. Name worn away."* Ties into R11a's lore.
- Scratch marks on the walls — frantic, not decorative. Someone was trapped here for a long time. A skeletal hand just visible under a collapsed section of ceiling. The badge's owner, perhaps.
- A narrow slit in the east wall overlooks the outside world — the first glimpse of the exterior environment seen from inside the dungeon. Moody establishing detail.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R13 — BOSS CHAMBER: THE INNER KEEP
**Shape:** Large square with reinforced stone walls — this was the fortress's inner sanctum. High vaulted ceiling. Military banners hang in tatters from iron poles. A cracked throne sits empty at the north end. The floor is clear of ash — swept clean, recently, by habit.
**Size:** Very Large — biggest room in the dungeon.
**Connections:** South → R10 *(BOSS DOOR — Boss Key required)*

**Contents:**
- 🔒 **BOSS DOOR** south side — Boss Key required. The door is a reinforced iron portcullis, not a standard dungeon gate. Heavy, deliberate.
- **BOSS: The Warden of Ash (Varek)** — an enormous armored soldier, impossibly old, still standing at attention in the center of the room. His armor has fused to him over centuries. He does not speak. He does not hesitate. He turns to face the player the moment the portcullis rises.
- [★] **🧩 Dragon's Tail Body (Piece 2/8)** — sealed inside the base of the cracked throne on the north wall. Revealed and accessible only after the boss is defeated.
- [★] **🔨 HAMMER** — mounted on the wall above the throne, behind a broken glass case. Collectible after boss defeat.
- The throne is flanked by two iron torch poles — still lit. Only light source in the room. Cannot be extinguished. This is a boss room — it should feel permanent and deliberate.
- Swept floor with no obstacles — the arena is clean. The Warden maintained this room out of duty even while turning to ash. The respect of that detail should unsettle the player.
- A large crack runs diagonally across the floor from the southeast to the northwest corner — purely aesthetic, but mirrors the crack in the dungeon's entry (R01 collapsed staircase), implying the same seismic event damaged both ends of the fortress.
- No pillars. No cover. Nowhere to hide. The fight is honest.

**Boss — The Warden of Ash (Varek):**
- **Phase 1 (100%–60% HP):** Slow deliberate advance — always walks toward the player. Overhead cleave (wide arc, 3-tile reach, 1.5 sec telegraph). Shield bash (if player is directly in front — forces players to stop frontal approach). **Immune to frontal hits** — full armor. Must be struck from behind or sides. Teaches patience and positioning.
- **Phase 2 (60%–30% HP):** Begins a shield-slam charge — runs 6 tiles in a straight line, crashes into wall if player dodges. Staggered for 2 sec — back exposed. Increases aggression on the advance. Starts doing half-turns mid-patrol to catch players circling.
- **Phase 3 (30%–0% HP):** Armor begins to crack and glow orange — Varek is burning from within. Drops shield entirely. Now vulnerable to frontal hits but significantly faster. Adds a sweeping spin attack (360° arc, requires dodge roll to clear). Emits ash cloud periodically — obscures vision for 3 seconds.
- **Defeat:** Varek stops mid-step, raises a hand — not to attack, but as a salute. He crumbles slowly into ash from the feet up. The throne cracks open. The Hammer case shatters. Music resolves into something mournful.

**Secrets:** None — boss rooms are always clean and readable.

---

## DUNGEON FLOW SUMMARY

```
CRITICAL PATH (follows the S-curve of the tail body):
R01 (get map) → R02 (Ladder to Key A ledge) → R03 (fight, get Compass)
→ R06 (fight Captain, get Lore Scroll from platform) → R07 (fight, cross collapse tiles)
→ R08 (pit traverse, Key C) → R10 (bell puzzle, get Boss Key) → R13 (boss, get Piece 2 + Hammer)

NOTE: The layout serpentines left and right as the player ascends —
evoking the sinuous curve of the Dragon's Tail Body. Side branches
extend at each turn of the S.

KEY LOCATIONS:
  Small Key A — R02 (on raised ledge, Ladder required)
  Small Key B — R04 (buried in ash floor)
  Small Key C — R08 (north side of collapsed pit)
  Boss Key    — R10 (revealed after 3-plate bell puzzle)

OPTIONAL PATHS:
  R03 → R04 (east branch — Key B + healing, opens shortcut only)
  R06 → R05 (west branch — Dark Armory, torches + gold, risky without light)
  R07 → R09 (east branch — Chain Hall, dead end with walkway + items)
  R08 → R08b (sub-level descent via Ladder)
  R10 → R11a (west secret wall — Lore Scroll + gold + boss name reveal)
  R10 → R11b (east secret wall, upper platform — cache + lore)

BLOCKED UNTIL FUTURE TOOLS:
  R03 east rubble wall → requires Hammer (just earned — can revisit immediately after boss)
  R08b north rubble wall → requires Hammer
  R10 cracked west wall (mid-level) → requires Hammer
  R10 cracked east wall (floor level) → requires Hammer
```

---

## FULL ITEM & TOOL CHECKLIST

| Item | Location | Required? |
|------|----------|-----------|
| 🗺️ Dungeon Map | R01 — wooden board on entry | Auto-collected |
| 🧭 Compass | R03 — stone altar, west arm | Optional |
| 🗝️ Small Key A | R02 — raised ledge (Ladder required) | Yes (opens R03 portcullis) |
| 🗝️ Small Key B | R04 — buried in ash floor | Optional (opens R04→R06 path) |
| 🗝️ Small Key C | R08 — north side of collapsed pit | Yes (opens R08→R10 ground path) |
| 🔑 Boss Key | R10 — revealed after bell puzzle | Yes (opens R13) |
| 🏮 Torch ×3 | R05 — shelf near entry | Optional (needed for R05 dark zone) |
| 🏮 Torch ×2 | R09 — wall sconce | Optional |
| 📜 Lore Scroll (Warden's Order) | R06 — atop watchtower platform | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 2) | R11a — secret reliquary | Optional (collectible) |
| 🏅 Worn Soldier's Badge | R11b — secret cache | Optional (lore item) |
| 🧩 Piece 2/8 | R13 — throne base after boss | Main Objective |
| 🔨 Hammer | R13 — wall mount after boss | Yes (needed for Level 3) |
| 💰 Gold (total ~370) | R03–R09, R11a, R11b | Optional |
| 🌿 Healing Herbs | R04, R07, R08b, R11a, R11b | Optional |

---
---

*[Level III — The Ironclad Vault will be added to a new document upon completion]*
