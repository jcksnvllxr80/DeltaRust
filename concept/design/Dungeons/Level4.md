# 🐉 THE DRAGON SEAL — Dungeon Design Document
*Visual ASCII Maps + Room-by-Room Notes*
*Scale: Small early → Large late | Format: ASCII map + notes*

---
---

# LEVEL IV — 🌊 THE SUNKEN CITADEL
**Theme:** A drowned coastal fortress, half-submerged beneath a tidal underground lake. Stone battlements slick with algae, flooded lower chambers, rising water sections, barnacle-crusted iron gates, and cold bioluminescent light filtering through cracks in the submerged walls. The Raft is the master tool — used to cross flooded chambers, reach elevated platforms from the water, and ferry heavy objects across gaps. The Tide Chart (found in the overworld at the lighthouse ruins) is required to enter — it reveals the tidal window during which the entrance is above water.
**Difficulty:** Mid. Teaches: Raft navigation, rising water sequences, water current mechanics, submerged tunnels (hold breath — limited time), ferry puzzles (moving objects on the Raft), tidal gate timing.
**Total Rooms:** 15 *(9 main + 2 secret + 2 sub-chambers + 1 flooded corridor + 1 boss)*
**Puzzle Piece:** 🧩 Dragon's Right Hind Leg *(Piece 4/8)*
**Tool Earned:** 💪 Strong Arm Glove
**World Item Required:** 🌊 Tide Chart *(found at lighthouse ruins in the overworld — reveals safe entry tidal window)*
**New Mechanics Introduced:** Raft navigation · Rising water sequences · Breath timer (submerged tunnels) · Ferry puzzle (Raft + heavy object) · Tidal gate timing · Current push (water flow redirects Raft)

---

## ASCII MAP

**Dungeon silhouette: RIGHT HIND LEG** — reversed-L shape. A vertical column
(the "thigh") rising from entry to the knee, then rooms extend rightward as the
"shin and foot." Mirrors Level 3's left-leaning L.

```
                                                                ┌───────────┐
                                                                │    R15    │
                                                                │   BOSS    │
                                                                │[THALVORN] │
                                                                └─────┬─────┘
                                                                      │ [BOSS DOOR]
                                                                ┌─────┴─────┐
                                                      ◄══════════    R12     ══════════►
                                                   [SECRET]     │  The Tide  │   [SECRET]
                                                                │  Altar     │
                                                                └════┬═══┬══┘
                                                                     │   │
                                                                ┌────┴┐ ┌┴───────┐
                                                                │ R11 │ │  R10   │
                                                                │Upper│ │Current │
                                                                │Ramp.│ │Channel │
                                                                └─────┘ └────┬───┘
                                                                             │
                                                                        ┌────┴────┐
                                                                        │  R10b   │
                                                                        │SUBMERGED│
                                                                        │ TUNNEL  │
                                                                        └─────────┘
                                                          ┌─────────────────┘
                                                          │
                                            ┌─────────────┴────────────┐
                                            │                          │
                                       ┌────┴────┐               ┌────┴────┐
                                       │   R08   │               │   R09   │
                                       │ Sunken  │               │ Flooded │
                                       │ Armory  │               │Barracks │
                                       └─────────┘               └────┬────┘
                                                                      │
                                                                 ┌────┴────┐
                                                                 │  R09b   │
                                                                 │SUB-CHMBR│
                                                                 │(sunken) │
                                                                 └─────────┘
               ┌────────────────────────────────────────┘
               │
          ┌────┴────┐
          │   R07   │
          │ Great   │
          │ Hall    │
          └────┬────┘
               │
     ┌─────────┼──────────────┐
     │         │              │
┌────┴────┐┌───┴─────┐  ┌────┴────┐
│   R03   ││   R04   │  │   R05   │
│ Docking ││ Flooded │  │  Salt   │
│  Cave   ││  Court  │  │ Cellar  │
└─────────┘└────┬────┘  └─────────┘
                │
          ┌─────┴─────┐
          │    R02    │
          │  Tidal    │
          │ Antechamber│
          └─────┬─────┘
                │
          ┌─────┴────┐
          │    R06   │
          │  Tidal   │
          │  Gate    │
          └─────┬────┘
                │
          ┌─────┴─────┐
          │    R01    │
          │   ENTRY   │
          │  (Start)  │
          └───────────┘


SECRET ROOMS (not on dungeon map):
  R13a ◄═══ west hidden wall of R12
  R13b ═══► east hidden wall of R12

SUB-CHAMBERS:
  R09b ↓ below R09 (sunken chamber — Raft to float down, swim back up)
  R10b ↔ lateral from R10 (submerged tunnel — breath timer, swim through)

FLOODED CORRIDOR:
  R10b connects R10 to the lower half of R09b via submerged passage
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
  [B]           = Heavy object (pushable / ferryable on Raft)
  [🪜]          = Ladder climb point (up or down)
  [🔨]          = Hammer-smashable wall or panel
  [🛶]          = Raft launch / dock point
  ~~~           = Water / flooded floor (wadeable)
  ≋≋≋           = Deep water (Raft required — cannot wade)
  ░░░           = Collapsing floor tiles
  ^^^           = Steam / water jet hazard
  〰〰〰         = Water current (pushes Raft in current direction)
  ▓▓▓           = Dark zone
  ↑ ↓           = Upper / lower level transition
  [~]           = Tidal gate (opens / closes on tidal cycle)
  [breath]      = Submerged section — breath timer active
```

---

## ROOM-BY-ROOM NOTES

---

### R01 — ENTRY CHAMBER
**Shape:** Wide horizontal rectangle. A sea cave worn smooth by centuries of tide. The entrance is a crack in the cliff face — barely wide enough to enter sideways. The Tide Chart showed this opening is only above water for two hours per tidal cycle. Salt crust on every surface. Kelp draped from the ceiling like curtains. The smell of brine and old stone.
**Size:** Large
**Connections:** North → R02 (open)

**Contents:**
- 🗺️ **DUNGEON MAP** — scratched into a flat piece of driftwood, wedged into a wall crack at eye level just inside the entrance. Shows R01–R12 and R15. Sub-chambers R09b, R10b and secret rooms R13a/R13b are NOT shown.
- [?] **Lore Tablet** (east wall, barnacle-encrusted, partially readable): *"The Citadel was built on the lake's edge. The lake had other ideas. The tide came in one century and never fully left."*
- A natural stone shelf runs along the south wall — above the high-tide line. Dry. A good place to leave things, though nothing is here yet.
- [🛶] **First Raft dock point** — a natural stone ledge on the north wall. The Raft can be deployed here for the first time. A rope ring is hammered into the stone — someone used this dock before.
- ~~~ **Shallow tidal pool** — the eastern third of the floor is submerged under a few inches of water. Wadeable. Crabs skitter away from the player's feet. Harmless.
- Two rusted iron rings embedded in the south wall — old mooring points from when the citadel was accessible from the sea.

**Enemies:** None.
**Secrets:** None.

---

### R02 — TIDAL ANTECHAMBER
**Shape:** Square, vaulted ceiling. The first proper citadel room — dressed stone, iron fittings, the ghost of military order under heavy water damage. A tidal gate dominates the north wall — a heavy iron portcullis with barnacle-studded crossbars.
**Size:** Medium
**Connections:** South → R01 (open) | North → R04 *([~] Tidal Gate — opens at low tide, closes at high tide — tidal cycle: 40 seconds open, 20 seconds closed)* | East → R03 (open) | West → R05 *(LOCKED — Small Key A required)*

**Contents:**
- ⚔️ **2× Tide Crawler** — new enemy. Crustacean-like. Heavy shell on top (immune to attacks from above — must roll to the side and strike the exposed underbelly). Slow. Patrols the room perimeter.
- **[~] Tidal Gate** — the north portcullis rises and falls on a fixed 40/20-second cycle (40 open, 20 closed). The cycle is visible — the player can hear the groan of the mechanism and see the gate beginning to move before it completes. Forces timed movement or waiting. First introduction of the tidal gate mechanic — stakes are low here (being caught by a closing gate just blocks progress, no damage).
- [★] **Small Key A** — on a high shelf on the west wall, above the water line. Requires either the Ladder or climbing via wall brackets (iron footholds on the wall, climbable without the Ladder — teaches that not all height needs the Ladder).
- [?] **Lore Tablet** (south wall): *"Tidal Gate Protocol: Do not attempt to hold the gate open manually. The mechanism does not care. Three soldiers tried. The mechanism did not care about them either."*
- ~~~ **Flooded floor** — the western third is knee-deep water. Wading slow. The tidal gate leads to deeper water beyond.
- A waterlogged wooden chair floats lazily in the flooded section. Purely atmospheric.

**Enemies:** 2× Tide Crawler
**Secrets:** None.

---

### R03 — DOCKING CAVE
**Shape:** Large irregular oval. A natural cave that was converted into an internal dock. A wide stone quay runs along the west wall. The rest of the room is deep water — Raft required to cross.
**Size:** Large
**Connections:** West → R02 (open, quay level) | North → R06 *([~] Tidal Gate — 30 seconds open, 30 seconds closed)* | ≋≋≋ Deep water throughout (east side fully submerged)

**Contents:**
- ≋≋≋ **Deep water** — covers the east half and center. Raft required to cross. Cannot be waded. First mandatory Raft use in the dungeon.
- [🛶] **Raft dock point** — quay on the west wall. Raft deploys here. Iron dock ring on the quay edge.
- ⚔️ **2× Sea Phantom** — new enemy. Translucent, floats above the water surface. Cannot be hit by melee while over deep water — must be lured to the quay or platform edges. Drifts slowly in patrol arcs over the water.
- [★] **🧭 COMPASS** — on a raised stone platform in the center of the deep water section. Accessible only by Raft. Platform is just above water level — player can step off the Raft onto it.
- [★] **Resonance Shell** — on the same central platform as the Compass. Key optional item — used in the boss fight to stun Thalvorn. Not mandatory but dramatically eases the boss encounter.
- [?] **Lore Tablet** (west wall, quay level): *"The dock once received supply ships from the surface. The passage collapsed in the third decade. We have been resupplying ourselves from what was already inside."*
- **[~] Tidal Gate** (north wall) — 30/30 cycle. Leads to R06. Accessible only by Raft (gate is in a deep water wall — no quay on the north side).
- [🔨] **Cracked stone pillar** (northeast, mid-water) — Hammer from the Raft. Breaking it causes the pillar to topple into the water, creating a permanent stepping-stone path across the northeast corner. Optional movement shortcut.

**Enemies:** 2× Sea Phantom
**Secrets:** None — the central platform is visible and intentional, not hidden.

---

### R04 — FLOODED COURT
**Shape:** Wide square. Once an open-air courtyard — now the roof has partially collapsed and the floor is entirely submerged. Deep water throughout. Three stone archways remain standing above water level. The collapsed roof section lets in dim natural light from far above — the only outside light in the dungeon.
**Size:** Large
**Connections:** South → R02 (via tidal gate, Raft for deep water approach) | North → R07 (open, arch-level — Raft required to reach) | West → R06 (open, arch-level) | East → R05 (open, archway, wadeable threshold)

**Contents:**
- ≋≋≋ **Deep water** — entire floor submerged. Raft required throughout. The three standing archways are above water and provide rest points where the player can step off the Raft.
- ⚔️ **3× Tide Crawler** — on the three archway platforms (they cannot enter the water). Must be fought on the platforms — player steps off the Raft onto the platform, fights, then re-boards.
- [★] **Small Key B** — on the northernmost archway platform, behind a Tide Crawler.
- [B] **Stone Urn** (on west archway platform) — heavy, ferryable on the Raft. Needed for the ferry puzzle in R07. Can be loaded onto the Raft by approaching and pressing interact. Player must carry it north to R07 on the Raft. First ferry puzzle introduction — low stakes.
- [?] **Lore Tablet** (on central archway, partially submerged — readable by leaning over Raft edge): *"The courtyard flooded slowly. We moved the training to the upper ramparts. We told ourselves it was temporary."*
- The collapsed roof section above — natural light filters down in a single column, illuminating the center of the water. Bioluminescent algae ring the walls just below the waterline, visible from the Raft. Beautiful.
- [🔨] **Cracked archway keystone** (east arch) — Hammer strike from the Raft. Collapsing the arch drops stones into the water, creating a wading path to R05 without needing the Raft. Optional shortcut for the return journey.

**Enemies:** 3× Tide Crawler
**Secrets:** None.

---

### R05 — SALT CELLAR
**Shape:** Long horizontal rectangle. Below the main courtyard level — accessible via a low archway that dips below water before rising again (a natural siphon, like a U-bend). The floor on the far side is above water — a dry room reached by ducking under and through. The siphon itself is a brief submerged section (3-4 tiles, fast to swim — no breath timer here, too short).
**Size:** Medium
**Connections:** East → R04 (siphon tunnel — brief submerged passage, no breath timer) | North → R08 *(LOCKED — Small Key B required)*

**Contents:**
- ⚔️ **2× Tide Crawler** — on the dry floor beyond the siphon.
- [★] **Small Key A** is used here — wait. Key A opens R05's north door to R08? No — Key A opens R02's west door. *Correction: Key B (from R04) opens the north door of R05 to R08.*
- [★] **Gold Pouch (40 coins)** — in a salt-crusted chest on the east wall. The chest is warped but not locked — just stiff. Opens with force (standard interact).
- [★] **Healing Herb ×3** — in a ceramic jar on a shelf (the only intact shelf in the room — everything else has rotted). The jar kept them sealed and preserved.
- [?] **Lore Tablet** (south wall): *"Salt cellar inventory as of last count: 400 barrels of preserved fish, 200 barrels of grain, 60 barrels of fresh water. Current inventory: 0. We ate well, for a while."*
- The walls are heavily salt-crusted — white and crystalline, catching light. The driest, most preserved-feeling room in the dungeon. Contrast with the oppressive wet of everything else.
- [🔨] **Cracked north wall section** — Hammer opens a secondary passage into R08's lower section, bypassing the Key B door. Optional shortcut. The Key B door still useful if not found this way.

**Enemies:** 2× Tide Crawler
**Secrets:**
- 🔍 Cracked north wall — Hammer bypass to R08.

---

### R06 — TIDAL GATE CONTROL
**Shape:** Compact square. A control room — iron levers, pipe fittings, pressure dials. The mechanisms that control all tidal gates in the citadel are routed through this room. A raised iron catwalk runs around the upper third of the walls.
**Size:** Medium
**Connections:** South → R03 (via tidal gate, deep water) | East → R04 (open, arch-level, deep water) | North → R09 *(LOCKED — Small Key C required)*

**Contents:**
- ⚔️ **1× Tide Crawler** + **1× Sea Phantom** — Crawler on the catwalk (patrols its length), Phantom drifts in the flooded lower section.
- ~~~ **Flooded lower section** — the floor is knee-deep water. The catwalk is dry and 3 tiles above the floor. Iron ladder rungs on the south wall lead up to it.
- [🪜] **Ladder climb point** — south wall. Access to catwalk.
- ⚙️ **Tidal Gate Control Lever** — on the catwalk. A large iron lever with three positions: FULL OPEN (all tidal gates stay open permanently), CYCLE (normal tidal operation — default), FULL CLOSED (all gates close permanently). Switching to FULL OPEN removes all tidal gate timing pressure for the rest of the dungeon. Switching to FULL CLOSED would lock the player out of certain rooms — the game prevents this with a warning prompt. The lever defaults to CYCLE. Most players will switch to FULL OPEN once they find it, rewarding thorough exploration.
- [★] **Small Key C** — on the catwalk, behind the Tide Crawler. Unlocks north door to R09.
- [?] **Lore Tablet** (catwalk wall): *"Tidal gate override is reserved for emergency use. Emergency defined as: rising water, structural collapse, or Thalvorn breach. Current status: Thalvorn has breached. No one is left to pull the lever."*
- [★] **Gold Pouch (35 coins)** — in the flooded lower section, on a submerged shelf visible from above. Requires wading to retrieve.

**Enemies:** 1× Tide Crawler, 1× Sea Phantom
**Secrets:** None — the control lever is the room's feature. Its existence and effect are overt.

---

### R07 — GREAT HALL
**Shape:** Very large rectangle. The citadel's main hall — high vaulted ceiling, two rows of stone columns running north-south, a raised dais at the north end. The lower half is deep water. The column tops and dais are above water — islands in the flooded interior.
**Size:** Very Large
**Connections:** South → R04 (open, water level) | North → R10 (open, dais level) | East → R08 *(LOCKED — Small Key B required — door is above water on the east wall, accessible from column tops)* | West → R06 (open, water level)

**Contents:**
- ≋≋≋ **Deep water** — entire lower half. Column tops and dais are above water (2-3 tiles above water surface). Raft required to move between columns.
- ⚔️ **2× Sea Phantom** + **1× Tidal Brute** — new enemy. The Tidal Brute is a large armored cephalopod — tentacles, heavy body, slow. Lives in the deep water, cannot leave it. Attacks by slamming tentacles onto column tops and the Raft surface. Must be struck on the head (above water) — only exposed when it surfaces to attack. Can capsize the Raft with a tentacle slam (player falls into water — brief swim-to-Raft scramble, no damage but disorienting).
- **Ferry Puzzle** — the Stone Urn from R04 must be ferried on the Raft to the dais at the north end. A stone socket on the dais accepts the Urn — placing it triggers a mechanism that lowers a bridge from the dais east wall down to the east door level, providing a stable dry path to R08. Without this, R08 is inaccessible from the east (Key B door requires the bridge to reach the door's height). This is the dungeon's main ferry puzzle.
- [!] **Pressure Plate on dais** — the Urn-socket. Placing the Urn on it is the pressure plate activation. Irreversible — Urn stays on the dais.
- [🛶] **Raft dock points** — column bases (iron rings on each column base). Player can moor the Raft at any column.
- [★] **Gold Pouch (45 coins)** — on the third column top from the south (west row). Requires Raft to reach.
- [?] **Lore Tablet** (dais, above water): *"The great hall hosted the garrison's last meal. The water came in during the third course. The Tidal Brute came in during the fourth."*

**Enemies:** 2× Sea Phantom, 1× Tidal Brute
**Secrets:** None.

---

### R08 — SUNKEN ARMORY
**Shape:** Wide horizontal rectangle. Split into two levels — the upper armory (dry, above water line) and the lower rack section (submerged). The upper level holds weapon racks and storage. The lower level is visible through iron grating in the upper floor.
**Size:** Medium-Large
**Connections:** West → R07 (bridge from dais, upper level) | West → R05 (Hammer bypass, lower level — cracked wall) | North → R11 *(LOCKED — Small Key D required)*

**Contents:**
- ⚔️ **2× Tide Crawler** — upper armory level. One guards the north door.
- [★] **Small Key D** — on a weapon rack on the north wall, upper level. Behind the north-door-guarding Tide Crawler.
- [★] **Gold Pouch (30 coins)** — on the upper armory shelf, east side.
- [★] **Gold Pouch (25 coins)** — on a weapon rack.
- [🔨] **Iron grating in floor** — Hammer breaks it open. Drops access to the submerged lower rack section.
- ~~~ **Submerged lower section** — visible through the grating before breaking it. Knee-deep water (not deep water — no Raft needed). Contains: **Healing Herb ×2** on a submerged shelf and a **sealed iron box** (Hammer to open) holding **Gold Pouch (25 coins)**.
- [?] **Lore Tablet** (upper armory, east wall): *"Armory inventory was last audited seventeen years before the flood. No audit has been conducted since. We know what we have. We have less of it every year."*

**Enemies:** 2× Tide Crawler
**Secrets:**
- 🔍 Iron floor grating — Hammer to open, access submerged lower section.

---

### R09 — FLOODED BARRACKS
**Shape:** Large rectangle. A soldiers' barracks, now heavily flooded. Upper bunks are above water; lower bunks are fully submerged. The room has a distinct two-level feel — the submerged floor and the surviving upper bunk level (1 tile above water surface, climbable from Raft).
**Size:** Large
**Connections:** South → R06 (locked entry, Key C used) | North → R12 (open) | Down via submerged hatch → R09b *(sub-chamber — submerged, Raft floats down)*

**Contents:**
- ≋≋≋ **Deep water** — entire floor submerged. Upper bunk frames are above water. Raft required for transit.
- ⚔️ **3× Sea Phantom** — patrol between bunk frame islands. One guards the north exit passage.
- [🛶] **Raft dock points** — iron rings on bunk frame uprights. Player can moor at any bunk.
- [★] **📜 Lore Scroll — "The Dragonbinder's Fourth Entry"** — on an upper bunk mattress (somehow still mostly dry — oilskin wrapping). Reads: *"The fourth fragment is the deepest I have had to go. This place was built on water — or drowned by it. I cannot tell which came first. The piece is below the water line. I will need to go under."*
- [★] **Gold Pouch (30 coins)** — on a bunk frame on the west side.
- **Submerged floor hatch** — visible through the water (bioluminescent light from below). Diving down reveals a hatch handle. Player interacts with it while swimming (free dive — no Raft) to open the hatch. The Raft can then be angled over the hatch and slowly sinks into R09b below (gravity + water flow pulls it down — the Raft floats to the surface of the sub-chamber below). Player rides the Raft down. Novel traversal moment.
- [?] **Lore Tablet** (on a bunk frame above water): *"We moved to the upper bunks when the water reached the lower ones. We have not moved again. There is nowhere left to move to."*

**Enemies:** 3× Sea Phantom
**Secrets:** None — the hatch is visible and intentional.

---

### R09b — SUB-CHAMBER: THE SUNKEN VAULT
**Shape:** Small square. A sealed storage vault below the barracks — pressure-sealed, which is why it's still mostly intact despite the flooding. The Raft floats on the water surface when it arrives (ceiling is 4 tiles above water — room is not fully submerged, just flooded to 3/4 depth).
**Size:** Small
**Connections:** Up → R09 (submerged hatch — swim up, Raft floats back up on its own when player surfaces)

**Contents:**
- ⚔️ **1× Tidal Brute** — in the water below. Smaller variant than R07's — a juvenile. Still dangerous. Must be struck on the head when it surfaces.
- ≋≋≋ **Deep water** — entire floor. Raft floats on entry. Player arrives on Raft.
- [★] **Gold Pouch (65 coins)** — in a sealed iron strongbox on a stone shelf just above the water line (wall shelf, reachable from Raft). Hammer to open the box.
- [★] **Rare Healing Herb** — full HP restore, on the same shelf.
- [★] **Coral Fragment** — collectible curiosity item. Flavor text: *"A piece of living coral growing inside a sealed vault. How did it get here? How long has it been growing? It's beautiful."* No mechanical use — pure lore flavor.
- [?] **Builder's Plaque** (above water, on the wall): *"Vault 9-B: Emergency Provisions. Sealed against flood, fire, and failure. If you are reading this, all three have occurred."*

**Enemies:** 1× Tidal Brute (juvenile)
**Secrets:** None — this IS the sub-chamber.

---

### R10 — CURRENT CHANNEL
**Shape:** Long vertical rectangle. A water channel — the entire floor is deep water moving in a visible current from south to north. The current pushes the Raft northward at a moderate speed. Two stone ledges run along the east and west walls (1 tile wide each, above water). Enemies on the ledges, player on the Raft navigating the current.
**Size:** Large
**Connections:** South → R07 (open, water level — current begins here) | North → R12 (open — current deposits Raft at north end) | West ledge → R09 (open gap in west wall at ledge level) | East side passage → R10b *(submerged tunnel entrance — breath timer)*

**Contents:**
- 〰〰〰 **Water current** — northward flow. Pushes Raft at moderate speed. Player can paddle against it (slow) or work with it (fast). Introduces current mechanic — the Raft handles differently in current versus still water.
- ⚔️ **2× Sea Phantom** — one on each stone ledge. Attack the player on the Raft with ranged water-jet attacks (new attack type — pushes Raft sideways, potentially into ledge walls, minor collision damage). Must be dealt with from the Raft (no melee reach from the Raft to ledge — requires Raft to moor at ledge to fight) or lured off the ledge into the water.
- [🛶] **Raft moor points** — iron rings on both ledge walls at intervals. Player can briefly moor against the current to fight or explore.
- **[breath] Submerged tunnel entrance** (east wall, below water line) — visible as a dark gap in the wall below the water surface. Swimming through leads to R10b. Breath timer active — 15 seconds to reach the other side before the player must surface.
- [★] **Gold Pouch (40 coins)** — on the west ledge, midway along.
- [?] **Lore Tablet** (east ledge, north end): *"The channel was dug to manage water flow through the lower citadel. It managed it perfectly. The water is now flowing in the wrong direction."*

**Enemies:** 2× Sea Phantom
**Secrets:** None — the submerged tunnel is visible and intended.

---

### R10b — SUB-CHAMBER: SUBMERGED TUNNEL
**Shape:** Irregular corridor — a natural crack in the rock rather than a built passage. Fully submerged. Narrow, twisting. Opens into a small air pocket chamber at the far end.
**Size:** Small (narrow tunnel + small air pocket room)
**Connections:** South → R10 (submerged tunnel entrance — breath timer, 15 seconds) | The air pocket room has no other exits — dead end

**Contents:**
- **[breath] Entire tunnel is submerged** — player swims through. 15-second breath timer shown on screen. Tunnel is navigable — no enemies, no obstacles, just darkness and the timer. Tense but manageable first time.
- **Air pocket chamber** at the end — a small dome of trapped air above the waterline. Player surfaces and can breathe. Timer resets. Room is small and enclosed.
- [★] **📜 Lore Scroll — "The Citadel's Last Census"** — wedged into a crack in the air pocket ceiling (someone placed it here above the waterline deliberately). Reads: *"Current garrison: 1. The water took the rest. I have decided to stay. I was not sure why, until I found what was in the deepest vault. Now I understand. I am the last guard of something very important. I will do my job."*
- [★] **Gold Pouch (55 coins)** — in a waterproof satchel clipped to the tunnel wall near the air pocket entrance. Visible on approach.
- [★] **Waterproofing Oil** — collectible item. Flavor text: *"Keeps leather and iron rust-free in sustained submersion. The satchel has been here a long time. The oil still works."* Optional collectible.
- Swimming back out restarts the breath timer — same 15 seconds. If the player fails (runs out of breath), they take damage and are deposited back at the R10 tunnel entrance.

**Enemies:** None.
**Secrets:** None — this IS the sub-chamber.

---

### R11 — UPPER RAMPARTS
**Shape:** Wide horizontal rectangle. The highest accessible point in the citadel — a battlement walkway above the water level. Dry floor. Wind whistles through arrow slits in the outer wall. Far below, through the slits, the underground lake is visible.
**Size:** Medium-Large
**Connections:** South → R08 (locked entry, Key D used) | West → R12 (open)

**Contents:**
- ⚔️ **2× Ashwalker** *(carried over from Level 2 — former garrison soldiers, undead, still at their posts on the ramparts)* + **1× Sea Phantom** *(drifted up from below through an arrow slit — it can pass through gaps)*.
- [★] **Small Key E** — on a weapon rack beside the westernmost arrow slit.
- [★] **Gold Pouch (35 coins)** — in a guard post alcove on the north wall.
- [★] **Healing Herb ×2** — in a supply crate near the east entrance.
- [?] **Lore Tablet** (south wall, between arrow slits): *"The ramparts were the last dry place. We watched the water rise from up here for eleven years. We had a very good view."*
- Arrow slits in the north wall overlook R12 from above — the player can see the Tide Altar room below before entering. The Boss Door is visible. Thalvorn is NOT visible yet — it is below the water in R12 until the fight begins.
- [🔨] **Cracked battlement section** (east end, outer wall) — Hammer opens a gap in the outer wall. Not an exit — behind the gap is a small external ledge overlooking the underground lake. On the ledge: **Gold Pouch (20 coins)** and a breathtaking atmospheric view of the submerged citadel exterior.

**Enemies:** 2× Ashwalker, 1× Sea Phantom
**Secrets:**
- 🔍 Cracked battlement — Hammer for external ledge view and gold.

---

### R12 — THE TIDE ALTAR
**Shape:** Large octagon. A ritual chamber — the citadel's innermost sanctum. The floor is entirely deep water. A stone altar platform stands in the center, 2 tiles above water level, accessible only by Raft. Four stone pillars ring the altar, each carved with sea-serpent reliefs. The ceiling is domed — the highest point in the dungeon. The Boss Door is in the north wall, just above water level.
**Size:** Very Large
**Connections:** South → R09 (open, water level — current from R10 also deposits here) | South → R10 (open, current arrival) | West → R11 (open, rampart level — above water, requires Ladder down to water, or jump) | North → R15 *(BOSS DOOR — requires Boss Key)* | West hidden wall → R13a *(SECRET — wall panel, above water)* | East hidden wall → R13b *(SECRET — submerged panel, Raft required to reach)*

**Contents:**
- ≋≋≋ **Deep water** — entire floor. Altar platform and pillar tops above water.
- ⚔️ **2× Sea Phantom** + **1× Tide Crawler** (on altar platform) — moderate pre-boss encounter.
- [🛶] **Raft dock points** — iron rings on all four pillar bases and the altar platform edge.
- **Altar Puzzle** — the altar platform has 4 carved sea-serpent heads, each facing a cardinal direction. Each head has a gem eye that can be rotated (interact). The correct gem orientation is hinted at by the four wall carvings in the room — each wall shows a serpent facing a specific direction with its eye at a specific angle. Solving all four orients them correctly — the Boss Key rises from a hidden compartment in the altar's base (a stone panel slides open below water level, visible from the Raft).
- [★] **Boss Key** — revealed in the altar base compartment after puzzle solved. Player must reach from the Raft to retrieve it (lean-over interaction).
- [?] **Lore Tablet** (altar platform, face-up on the stone): *"Thalvorn was not always here. It followed the dragon's piece down into the deep water centuries ago. It has been circling the altar ever since. It does not understand what it guards. It only knows it must."*
- **[~] Boss Door** — north wall, just above water line. Accessible by Raft.
- **West hidden wall → R13a:** A section of the west wall above water has a slightly different stone colour — older masonry, predating the citadel. Pushing a loose stone block reveals the tunnel. Observation required.
- **East hidden wall → R13b:** A submerged section of the east wall has a gap — visible from Raft as a faint bioluminescent glow through the crack. Player must lean over the Raft edge and push the panel inward. Requires being directly adjacent on the Raft.

**Enemies:** 2× Sea Phantom, 1× Tide Crawler
**Secrets:**
- 🔍 **West hidden wall → R13a:** Above-water panel, older masonry — observation and push.
- 🔍 **East hidden wall → R13b:** Submerged panel — Raft required to reach, bioluminescent glow hints at it.

---

### R13a — SECRET ROOM: THE OLD SHRINE *(West, Above Water)*
**Shape:** Small rectangle. Pre-citadel construction — this was here before the fortress was built around it. Rough natural stone, no iron fittings. A simple carved stone basin in the center, long dry.
**Size:** Small
**Connections:** East → R12 (hidden wall panel, above water)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **📜 Lore Scroll — "The Dragonbinder's Fourth Entry — Addendum"** — Reads: *"I found a shrine behind the altar. Older than the citadel by centuries. Someone was worshipping the dragon piece long before the soldiers ever arrived. The piece was sacred here. I am starting to wonder if 'sealed' is the wrong word. Perhaps it was 'enshrined.'"*
- [★] **Rare Healing Herb** — full HP restore.
- [★] **Gold Pouch (85 coins)** — highest single gold reward in the dungeon, sealed in a ceremonial clay pot.
- The stone basin has a faded carving at the bottom — a dragon shape, but different from the one in Level 1's secret room. This dragon is whole, unbroken. Pre-sealing imagery.
- Four small carved fish around the basin rim — decorative, but the same four fish appear on the Tide Altar's serpent heads in R12. A connection between the shrine and the altar, separated by centuries.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R13b — SECRET ROOM: THE DEPTH CACHE *(East, Submerged Access)*
**Shape:** Small square. Fully submerged entry (player pushes panel from Raft, swims in briefly) — but the room itself has an air pocket above the waterline. Player surfaces inside.
**Size:** Small
**Connections:** West → R12 (submerged wall panel — Raft to reach, brief swim through)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **Gold Pouch (60 coins)** — in a sealed clay amphora resting on a submerged shelf, just at the waterline. Reachable by hand while treading water.
- [★] **Healing Herb ×3** — bundled and sealed in wax, floating just above the waterline on a wooden plank. Still usable.
- [★] **Sea Captain's Log** — readable item. Final entry: *"We found it on the third dive — a piece of something ancient, glowing faintly on the lake floor. We brought it up. The water followed it. I think the water was its. I think we made a mistake."* Connects the flooding of the citadel directly to the Dragon Piece.
- Bioluminescent algae cover every surface — the room glows soft blue-green. The most visually striking room in the dungeon. No enemies, no danger. A moment of quiet beauty after the tension of the submerged entry.
- A child's drawing scratched into the wall — similar to the one in Level 3's R13a. Same hand? The same Mira? A dragon, this time surrounded by waves.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R15 — BOSS CHAMBER: THE DEEP ALTAR POOL
**Shape:** Large circle. Entirely flooded — deep water wall to wall. No platforms, no islands. The player fights entirely from the Raft. The ceiling is low — just above the Raft surface, creating a claustrophobic cave-feel. Bioluminescent veins in the walls provide the only light. The Boss Door is flush with the water surface on the south wall.
**Size:** Very Large
**Connections:** South → R12 *(BOSS DOOR — Boss Key required)*

**Contents:**
- 🔒 **BOSS DOOR** — south wall, at water level. Boss Key required. The door slides sideways into the wall rather than rising — a smooth, silent motion, unlike the grinding of every other gate in the dungeon.
- **BOSS: Thalvorn, Tide Serpent** — enormous water serpent. Fully aquatic. Never leaves the water. The player fights from the Raft — the entire encounter is a naval battle. Thalvorn surfaces to attack, submerges to reposition, and attempts to capsize, crush, and drag the Raft under.
- [★] **🧩 Dragon's Right Hind Leg (Piece 4/8)** — embedded in a glowing node on the chamber floor, visible through the water (bioluminescent). Retrieved automatically after boss defeat — it floats to the surface and can be grabbed from the Raft.
- [★] **💪 Strong Arm Glove** — sealed inside a barnacle-encrusted chest that rises from the water on a stone plinth after Thalvorn is defeated. The plinth was submerged until the boss's death disrupts the water pressure in the chamber.
- The chamber walls are carved with enormous serpent-scale patterns — the entire room was built around Thalvorn, or Thalvorn shaped itself to this room. Impossible to tell which came first.
- **4× Stone Pillar stumps** — just above water level, barely. The Raft can be wedged against them for stability. Thalvorn will smash them in Phase 2 — the Raft becomes harder to stabilise.
- No torches, no iron, no construction. This room was never built. It was found.

**Boss — Thalvorn, Tide Serpent:**
- **Phase 1 (100%–60% HP):** Thalvorn circles the Raft slowly just below the surface (visible as a dark shape in the water). Surfaces to strike — head lunges upward with a bite attack (1.5 sec telegraph — the water ripples in a line toward the Raft before it breaks surface). Dodge the Raft sideways to avoid. Exposed head above water for 2 seconds — strike repeatedly. Also uses a tail-slap (comes from behind — water disturbance on the far side of the room telegraphs it — pivot and dodge forward). If Resonance Shell (from R03) was collected, player can use it once per phase to stun Thalvorn for 3 seconds — full damage window.
- **Phase 2 (60%–30% HP):** Begins Raft-capsize attempt — wraps body partially around the Raft and squeezes (rapid interact-button tap to resist — if failed, Raft capsizes and player swims for 5 seconds before Raft rights itself, taking periodic damage). Smashes the 4 pillar stumps (loses them as stabilisers). Adds a water-jet attack (inhales then blasts a column of water — dodge 90° to the side). Speed increases.
- **Phase 3 (30%–0% HP):** Thalvorn partially surfaces — its full body length visible for the first time. Thrashes wildly, creating waves that push the Raft around the room. Player must compensate for constant Raft drift. Final attack is a full-body breach (launches entirely out of the water, arcs overhead, crashes back down — massive shockwave, dodge to the room's edge and hold).
- **Defeat:** Thalvorn sinks slowly, coiling around the chamber floor. Its bioluminescence dims. The Dragon Piece detaches from the floor and rises. The water stills. The plinth rises. The room is silent except for the sound of water settling.

**Secrets:** None — boss rooms are always clean and readable.

---

## DUNGEON FLOW SUMMARY

```
CRITICAL PATH (ascends the thigh, then extends right along the leg):
R01 (get map, deploy Raft) → R02 (fight, time tidal gate) → R04 (fight, get Key B, get Stone Urn)
→ R07 (fight, ferry Stone Urn to dais — opens bridge to R08) → R08 (fight, get Key D)
→ R11 (fight, get Key E) → R12 (altar puzzle, get Boss Key) → R15 (boss, get Piece 4 + Glove)

NOTE: The layout forms a reversed-L — the vertical entry column bends right
at the mid-point, mirroring Level 3's left-leaning leg. The boss sits at
the "foot" end of the reversed-L.

KEY LOCATIONS:
  Small Key A — R02 (high shelf, wall brackets to climb)
  Small Key B — R04 (north archway platform, behind enemy)
  Small Key C — R06 (catwalk, behind Tide Crawler)
  Small Key D — R08 (weapon rack, upper armory)
  Small Key E — R11 (weapon rack, west arrow slit)
  Boss Key    — R12 (altar base compartment, after puzzle)

OPTIONAL PATHS:
  R02 → R03 (Compass + Resonance Shell — aids boss fight significantly)
  R02 → R05 (via Key A — Salt Cellar, healing + gold)
  R03 → R06 (via tidal gate — Tidal Gate Control, lever to unlock all gates permanently)
  R06 → R09 (Flooded Barracks — Lore Scroll + sub-chamber access)
  R09 → R09b (submerged hatch — Sunken Vault, gold + healing)
  R10 → R10b (submerged tunnel — breath timer, Lore Scroll + gold)
  R11 → external ledge (Hammer battlement — view + gold)
  R12 → R13a (west hidden panel — Lore Scroll + gold)
  R12 → R13b (east submerged panel — Raft required, Sea Captain's Log + gold)

TIDAL GATE NOTE:
  If player finds R06 and pulls lever to FULL OPEN, all [~] tidal gates
  remain permanently open for the rest of the dungeon. Rewards exploration
  of the optional R03 → R06 path.

BLOCKED UNTIL FUTURE TOOLS:
  R03 cracked pillar — Hammer (already available). Can break immediately.
  R04 cracked archway keystone — Hammer (already available).
  R05 cracked north wall — Hammer (already available).
  R08 iron floor grating — Hammer (already available).
  R11 cracked battlement — Hammer (already available).
  No Level-5-tool blocks in this dungeon — those begin in Level 5.
```

---

## FULL ITEM & TOOL CHECKLIST

| Item | Location | Required? |
|------|----------|-----------|
| 🗺️ Dungeon Map | R01 — driftwood slab on entry wall | Auto-collected |
| 🌊 Tide Chart | World — lighthouse ruins | Yes (to enter dungeon) |
| 🧭 Compass | R03 — central platform (Raft required) | Optional |
| 🐚 Resonance Shell | R03 — central platform (Raft required) | Optional (aids boss fight) |
| 🗝️ Small Key A | R02 — high shelf (wall brackets) | Optional (opens R05) |
| 🗝️ Small Key B | R04 — north archway, behind enemy | Yes (opens R05 north / R07 east) |
| 🗝️ Small Key C | R06 — catwalk, behind enemy | Optional (opens R09 direct route) |
| 🗝️ Small Key D | R08 — upper armory weapon rack | Yes (opens R11) |
| 🗝️ Small Key E | R11 — weapon rack by arrow slit | Yes (opens R12 from R11) |
| 🔑 Boss Key | R12 — altar base after puzzle | Yes (opens R15) |
| 🧩 Piece 4/8 | R15 — rises from water after boss | Main Objective |
| 💪 Strong Arm Glove | R15 — chest on risen plinth after boss | Yes (needed for Level 5) |
| 📜 Lore Scroll (Dragonbinder 4) | R09 — upper bunk, oilskin wrapped | Optional (collectible) |
| 📜 Lore Scroll (Last Census) | R10b — air pocket, tunnel end | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 4 Addendum) | R13a — secret shrine | Optional (collectible) |
| 📓 Sea Captain's Log | R13b — secret depth cache | Optional (lore item) |
| 🪸 Coral Fragment | R09b — sunken vault shelf | Optional (curiosity item) |
| 💧 Waterproofing Oil | R10b — tunnel wall satchel | Optional (lore item) |
| 💰 Gold (total ~560) | Throughout | Optional |
| 🌿 Healing Herbs | R05, R08, R09b, R10, R11, R13a, R13b | Optional |

---
---

*[Level V — Grimforge Depths will be added to a new document upon completion]*
