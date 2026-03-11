# 🐉 THE DRAGON SEAL — Dungeon Design Document
*Visual ASCII Maps + Room-by-Room Notes*
*Scale: Small early → Large late | Format: ASCII map + notes*

---
---

# LEVEL V — 🔥 GRIMFORGE DEPTHS
**Theme:** A vast volcanic forge complex buried deep beneath an active mountain. Rivers of lava flow through channels cut into the black basalt floors. Enormous furnaces, massive iron casting frames, chains thick as tree trunks, and the oppressive heat of a place that has been burning for centuries without pause. The Strong Arm Glove is the master tool — used to lift and throw heavy objects, pull massive chains, hurl enemy weapons back at them, and grip scorching-hot handles that would destroy unprotected hands. The Ember Crystal (found in the overworld at the volcanic outcrop) is required to relight the Master Forge in R12, which is the dungeon's central puzzle mechanic.
**Difficulty:** Mid-high. Teaches: Strong Arm Glove usage on chains / heavy objects / throw mechanic, lava channel hazards, heat zones (slowed movement, gradual damage), forge-lighting puzzle, weight-based pressure plates, anvil throwing in combat.
**Total Rooms:** 15 *(9 main + 2 secret + 2 sub-chambers + 1 boss)*
**Puzzle Piece:** 🧩 Dragon's Left Wing *(Piece 5/8)*
**Tool Earned:** *(No new tool — mastery check level. All prior tools tested.)*
**World Item Required:** 🔥 Ember Crystal *(mined at volcanic outcrop south of the forge entrance in the overworld)*
**New Mechanics Introduced:** Strong Arm Glove (lift/throw/pull) · Lava channel hazards · Heat zones · Forge-lighting sequence · Chain-pull mechanisms · Anvil throw combat · Weight plates (heavier objects required than standard boulders)

---

## ASCII MAP

**Dungeon silhouette: LEFT WING** — a narrow right-side spine with the dungeon
fanning broadly leftward into layered "feathers." The boss sits at the upper wing
root while the lower half spreads outward in longer horizontal spans.

```
                ┌───────────┐
                │    R15    │
                │   BOSS    │
                │[MOLTENFIST│
                └─────┬─────┘
                │ [BOSS DOOR]
           ◄══════════════╧══════════════►
               ┌───────────┐
           [SECRET]  │    R12    │  [SECRET]
               │  Master   │
               │   Forge   │
               └─┬────┬──┬─┘
              │    │  │
            ┌──────────┘    │  └──────────┐
          ┌────┴────┐     ┌────┴────┐   ┌────┴────┐
          │   R09   │     │   R10   │   │   R11   │
          │ Chain   │     │  Lava   │   │ Casting │
          │  Hall   │     │  River  │   │  Floor  │
          └────┬────┘     └────┬────┘   └────┬────┘
            │               │             │
          ┌────┴────┐     ┌────┴────┐        │
          │  R09b   │     │  R10b   │        │
          │SUB-CHMBR│     │SUB-CHMBR│        │
          │(chain   │     │ (lava   │        │
          │ vault)  │     │ tunnel) │        │
          └─────────┘     └─────────┘        │
            ▲               ▲              │
            │               │              │
          ┌────┴────┐     ┌────┴────┐   ┌────┴────┐
          │   R06   ├─────┤   R07   ├───┤   R08   │
          │ Bellows │     │ Central │   │Smelting │
          │  Room   │     │  Works  │   │  Hall   │
          └────┬────┘     └────┬────┘   └─────────┘
            ▲               │
            │               │
          ┌────┴────┐     ┌────┴────┐
          │   R05   │     │   R04   │
          │ Cooling │─────┤  Forge  │
          │  Vats   │     │  Floor  │
          └────┬────┘     └────┬────┘
            │               │
            └──────┐   ┌────┘
             │   │
             ┌──┴───┴──┐    ┌─────────┐
             │   R02   ├────┤   R03   │
             │ Descent │    │ Intake  │
             │Antechmb.│    │  Shaft  │
             └────┬────┘    └────┬────┘
               │              │
             ┌────┴────┐         │
             │   R01   │         │
             │  ENTRY  │         │
             │ (Start) │         │
             └─────────┘         │
                     ▼
                  to R06


SECRET ROOMS (not on dungeon map):
  R13a ◄═══ west hidden wall of R12
  R13b ═══► east hidden wall of R12

SUB-CHAMBERS:
  R09b ↓ below R09 (chain vault — Strong Arm Glove required to open hatch)
  R10b ↓ below R10 (lava tunnel — heat-shielded passage, timed crossing)
```

```
LEGEND
══════════════════════════════════════════
  │ ─ ┌ ┐ └ ┘ ┤ ├  = Tunnel / corridor
  ◄═══►         = Hidden / secret wall passage
  [BOSS DOOR]   = Requires Boss Key to open
  [LOCKED]      = Requires Small Key to open
  [★]           = Chest or item location
  [!]           = Pressure plate / weight plate
  [?]           = Lore tablet / hint stone
  [B]           = Heavy object (liftable with Strong Arm Glove)
  [🪜]          = Ladder climb point (up or down)
  [🔨]          = Hammer-smashable wall or panel
  [💪]          = Strong Arm Glove required interaction
  [⛓️]          = Chain-pull point (Glove required)
  ~~~           = Shallow water / cooling fluid
  ≋≋≋           = Deep water (Raft required)
  🔥🔥🔥         = Lava channel (instant death on contact)
  ░░░           = Heat zone (gradual damage, slowed movement)
  ▓▓▓           = Dark zone
  ↑ ↓           = Upper / lower level transition
```

---

## ROOM-BY-ROOM NOTES

---

### R01 — ENTRY CHAMBER
**Shape:** Wide horizontal rectangle. The entrance is a natural volcanic vent — a crack in the mountainside forced wider by centuries of heat expansion. The air shimmers with residual warmth even here, far from the active forge below. Black basalt walls, smooth and glassy. The smell of sulfur and old iron.
**Size:** Large
**Connections:** North → R02 (open — descent begins)

**Contents:**
- 🗺️ **DUNGEON MAP** — etched into a slab of cooled obsidian propped against the east wall. Reflects the light strangely. Shows R01–R12 and R15. Sub-chambers R09b and R10b and secret rooms R13a/R13b are NOT shown.
- [?] **Lore Tablet** (west wall, basalt-carved): *"Grimforge was built by the first smiths who found the dragon's piece here — already half-melted into the rock. They built their forge around it. They never stopped forging. They are still here."*
- The Ember Crystal slot is visible on the south wall — a carved crystal-shaped recess beside a dormant pipe system. The Ember Crystal from the overworld must be inserted here to prime the Master Forge in R12. Inserting it now triggers a low rumble through the dungeon — the pipe system begins carrying heat upward. The slot accepts the crystal permanently (it stays inserted).
- [💪] **Heavy iron blast door** on the north wall — partially ajar, forced open by a previous explorer. The Strong Arm Glove can wrench it fully open (first Glove use — optional, as it is already partially ajar and passable by squeezing through). Wrenching it fully open makes it permanently wide and reveals a **Gold Pouch (25 coins)** that was wedged behind the door.
- A dead forge-worker slumped against the east wall — petrified by heat centuries ago. Still holding a hammer. The hammer is fused to the hand. Not interactable. Atmospheric detail.
- Heat shimmer in the air increases noticeably toward the north.

**Enemies:** None.
**Secrets:**
- 🔍 Heavy iron door — Glove wrench reveals hidden gold.

---

### R02 — DESCENT ANTECHAMBER
**Shape:** Tall vertical rectangle — a descending ramp room. The floor slopes steeply downward from south to north. Iron railing on both sides (partially collapsed). The heat increases sharply as the player descends. The ceiling glows faintly orange from the forge light below.
**Size:** Medium
**Connections:** South → R01 (open — up the ramp) | North → R04 (open — bottom of ramp) | East → R03 *(LOCKED — Small Key A required)* | West → R05 (open)

**Contents:**
- ⚔️ **2× Forge Hound** — new enemy. Quadrupedal constructs of iron and hardened slag. Fast, low to the ground. Attack by ramming. Weak point: the cooling vent on their spine — glows orange. Hammer strikes or Strong Arm Glove throws stagger them. Die in 3 Glove-assisted hits or 6 normal hits.
- [★] **Small Key A** — on a wall bracket on the east wall, mid-ramp height, behind the iron railing. Reachable by leaning over the railing (standard interact — no tool needed). Key A is easily spotted; teaches players to check walls while descending.
- [?] **Lore Tablet** (west wall, mid-ramp): *"The descent to the forge floor takes eleven minutes at walking pace. The smiths made it in four. They stopped noticing the heat after the first year."*
- ░░░ **Heat zone** — the lower third of the ramp is in a heat zone. Movement slows, gradual tick damage begins. First heat zone introduction — mild, short duration. Player exits it quickly by entering R04.
- Iron railing on the west side has a section that has collapsed outward — a gap. Falling through the gap drops the player 2 tiles to a ledge below (minor fall damage) that connects to R05. Shortcut for observant players.
- [🪜] **Ladder climb point** — east wall at the bottom of the ramp. Leads to a high ledge overlooking R04. Gold reward on the ledge (**Gold Pouch 20 coins**).

**Enemies:** 2× Forge Hound
**Secrets:**
- 🔍 Collapsed railing gap — drop to R05 ledge shortcut.
- 🔍 Ladder ledge — overlook of R04 and gold reward.

---

### R03 — INTAKE SHAFT
**Shape:** Tall narrow rectangle — a vertical shaft. Iron grating at multiple levels forms a series of platforms. The shaft descends 6 tile-heights. A chain runs through the center from top to bottom — the original lift mechanism for raw ore. The chain is still intact.
**Size:** Medium (narrow, but tall)
**Connections:** West → R02 (locked entry, Key A used) | Bottom of shaft → R06 (open)

**Contents:**
- ⚔️ **2× Forge Hound** — one on the second grating platform, one on the fourth. They cannot climb the chain — only the player can.
- [⛓️] **Central chain** — Strong Arm Glove required to grip and climb. Without the Glove the chain is too hot to hold bare-handed. With the Glove the player can climb freely up and down. First mandatory chain-climb in the dungeon.
- [🪜] **Ladder climb points** — iron rungs on the west wall allow descending to the second platform. Below the second platform, the rungs stop — chain required.
- [★] **Small Key B** — on the third grating platform (halfway down), wedged between the grating bars. Requires chain descent to reach.
- [★] **Gold Pouch (30 coins)** — at the bottom of the shaft on the floor, beside the R06 exit.
- [★] **Healing Herb ×2** — on the first grating platform (top — immediately accessible from R02 entry).
- [?] **Carved shaft wall text** (scratched by a worker, readable on the third platform): *"Day 1: Started the descent. Day 4: Reached the bottom. Day 4 also: Found what they built this around. Did not come back up. Decided not to."*
- ░░░ **Heat zone** — the bottom two platforms and the shaft floor. Gradual tick damage — encourages moving quickly through the lower shaft.

**Enemies:** 2× Forge Hound
**Secrets:** None.

---

### R04 — FORGE FLOOR
**Shape:** Very wide square. The main working floor of the forge — the largest open space in the dungeon. Lava channels cut across the floor in a grid pattern, dividing the room into stone island-sections. The channels are 2 tiles wide — too wide to jump across unaided. Iron bridges (2 tiles wide, no rails) span the channels at intervals — some intact, some collapsed. The ceiling is vaulted and high, lost in smoke and heat haze.
**Size:** Very Large
**Connections:** South → R02 (open — top of ramp) | North → R07 (open — far north) | West → R05 (open — lower wing span) | North-West → R06 (open — upper wing span)

**Contents:**
- 🔥🔥🔥 **Lava channels** — crossing them deals instant death. Intact bridges are safe. Collapsed bridges leave a 2-tile gap — a [B] **iron beam** nearby can be lifted with the Glove and laid across the gap as a makeshift bridge. First mandatory Glove-lift-and-place puzzle.
- ⚔️ **2× Forge Hound** + **1× Slag Golem** — new enemy. The Slag Golem is large, slow, and made of partially cooled lava-rock. Cannot be damaged normally — must be struck with the Strong Arm Glove (the impact shatters its outer cooling shell, exposing the molten core, which is then vulnerable to 2 more Glove strikes or 4 weapon strikes). Alternatively, luring it into a lava channel kills it instantly.
- [B] **3× Iron Beams** — scattered across the island sections. Each is heavy — Glove required to lift. Used to bridge collapsed channel crossings. One beam can bridge two gaps simultaneously if placed diagonally (optional clever solution).
- [!] **Weight Plate** — on the central island section. Requires an iron beam placed on it (too heavy for boulders). When activated: a wall panel on the north wall slides open, revealing **Small Key C** behind it.
- [★] **Small Key C** — revealed by weight plate. Required for R07's west branch.
- [?] **Lore Tablet** (south island, near entry): *"The forge floor was redesigned three times. Each time the lava channels moved. Each time a smith fell in. We stopped redesigning after the third time."*
- [★] **Gold Pouch (35 coins)** — on an isolated island section in the northeast, reachable only by bridging two channel gaps in sequence.

**Enemies:** 2× Forge Hound, 1× Slag Golem
**Secrets:** None — the beam puzzle is overt and central.

---

### R05 — COOLING VATS
**Shape:** Wide horizontal rectangle. A row of enormous iron vats along the north wall, each filled with dark cooling fluid — the contrast to the heat of the rest of the dungeon. The fluid is not water — it is black, viscous, and faintly luminescent. The floor is dry but cool. The southern half is an open workspace with workbenches and tool racks (all empty).
**Size:** Medium-Large
**Connections:** East → R02 (open — ramp level, via the collapsed railing shortcut OR standard entry) | East → R04 (open — forge floor span) | North-East → R08 *(LOCKED — Small Key B required)*

**Contents:**
- ⚔️ **2× Forge Hound** + **1× Slag Golem** — Golem patrols between the cooling vats.
- [★] **🧭 COMPASS** — submerged in one of the cooling vats (the third from the left), visible as a glow beneath the black fluid. Retrievable by reaching into the vat — requires the Strong Arm Glove (fluid is too hot for bare hands despite being a cooling agent — the irony is noted in a nearby label). First Glove-required retrieval.
- [B] **Cooling Ingot** — a large iron ingot sitting in the first vat, partially cooled and solidified. Liftable with the Glove. No immediate use here — can be carried to R07 where a weight plate accepts it. Optional ferry challenge (cross-room carry, not required for critical path).
- [★] **Gold Pouch (25 coins)** — on a workbench on the south wall, east side.
- [★] **Healing Herb ×2** — in a sealed ceramic jar on the west workbench. Jar lid is fused shut — Hammer or Glove to open.
- [?] **Lore Tablet** (north wall, between vats): *"The cooling vats were filled with volcanic brine. It cools metal faster than water and leaves a harder finish. It also dissolves bone faster than acid. We lost two apprentices before we put up the signs. We put up the signs after the third one."*
- ~~~ **Cooling fluid in vats** — entering a vat is possible but deals gradual damage (too hot). Not lethal fast — discourages loitering, not exploration.

**Enemies:** 2× Forge Hound, 1× Slag Golem
**Secrets:** None.

---

### R06 — BELLOWS ROOM
**Shape:** Large square. The room that feeds air into the forge — six enormous iron bellows, each the size of a small house, are mounted on the walls. Three are active (rhythmically pumping, creating wind gusts that push the player sideways). Three are inactive and still. The floor has grating over lava beneath — the grating is intact but warm.
**Size:** Large
**Connections:** South → R03 (open — top of intake shaft, via floor-level exit) | East → R07 *(LOCKED — Small Key C required from the central works side)* | South-East → R04 (open — channel bridge) | North → R09 *(LOCKED — Small Key C required)*

**Contents:**
- ⚔️ **3× Forge Hound** — two navigate the bellows gusts (they lean into the wind, unfazed — the player must time movement through gusts to avoid being pushed into the hot grating).
- **Active Bellows** — three bellows pump on a 6-second cycle (3 seconds pushing, 3 seconds drawing back). Standing in front of a bellows during the push phase shoves the player 3 tiles sideways. The grating is hot — being pushed onto it deals minor burn damage.
- [⛓️] **Chain-pull mechanism** (north wall) — a thick chain runs from the ceiling to a floor anchor. Pulling it with the Strong Arm Glove deactivates all three active bellows permanently. Makes the room safe and static. Optional but significant quality-of-life.
- [★] **Gold Pouch (30 coins)** — behind the westernmost active bellows. Accessible only when the bellows is in the drawing-back phase (3-second window) or after pulling the chain to deactivate it.
- [★] **Heating Tonic** — new consumable item. Temporarily makes the player immune to heat zone tick damage for 30 seconds. Found on a shelf on the east wall.
- [?] **Lore Tablet** (south wall): *"The bellows feed the forge. The forge feeds the furnace. The furnace feeds the Master Forge. The Master Forge feeds something older than all of us. We chose not to ask what."*
- ░░░ **Heat zone** — the eastern third of the room, near the R04 exit. Gradual damage. Short traversal — not punishing.

**Enemies:** 3× Forge Hound
**Secrets:**
- 🔍 Chain-pull — Glove to deactivate all bellows. Rewards tool use.

---

### R07 — CENTRAL WORKS
**Shape:** Large cross-shaped room. The mechanical hub of the forge. A massive iron anvil — 4 tiles across, glowing orange — dominates the center. This is where the forge-work happened. Catwalks run at mid-height along the north and south arms. A weight plate sits on the west arm floor.
**Size:** Very Large
**Connections:** South → R04 (open) | North → R10 (open) | East → R08 (open) | West → R06 *(LOCKED — Small Key C required)*

**Contents:**
- ⚔️ **1× Slag Golem** + **2× Forge Hound** — Golem is stationary at the central anvil (it has been standing there for a very long time). Hounds patrol the catwalk arms.
- [B] **Central Anvil** — the massive glowing anvil. Too heavy for the Glove to lift normally. However, it sits on a tilting iron frame — the Glove can grip the frame's handle and wrench the anvil sideways, tipping it off the frame. Tipped anvil becomes a liftable object (now single-piece, cooler). Used for the weight plate on the west arm. Introduces the concept of repositioning objects before lifting them.
- [!] **Weight Plate** (west arm floor) — accepts the tipped anvil only (too heavy for beams or boulders). When activated: a wall panel on the north arm slides open, revealing **Small Key D** inside.
- If player brought the Cooling Ingot from R05 — it also fits the weight plate (same weight class). Alternative solution.
- [★] **Small Key D** — north arm wall panel, revealed by weight plate.
- [🪜] **Ladder climb points** — both east and west catwalk arm walls. Iron rungs to mid-height catwalk level.
- [★] **Gold Pouch (40 coins)** — on the south catwalk arm, behind the Forge Hound patrol.
- [?] **Lore Tablet** (center of room, mounted on the anvil frame): *"Every piece forged here was tested against the dragon's fragment. If the fragment rejected it, the piece was remelted. The fragment rejected everything. We kept forging anyway."*
- [🔨] **Cracked east wall** (catwalk level) — Hammer reveals a hidden alcove behind the wall: **Healing Herb ×2** and **Gold Pouch (20 coins)**.

**Enemies:** 1× Slag Golem, 2× Forge Hound
**Secrets:**
- 🔍 Cracked east wall (catwalk) — Hammer for healing and gold.

---

### R08 — SMELTING HALL
**Shape:** Wide horizontal rectangle. A row of smelting furnaces along the south wall — all cold except the easternmost, which still burns (it has been burning, unattended, for centuries). Iron molds on the floor, filled with hardened slag. A raised walkway on the north wall runs the full length of the room.
**Size:** Medium-Large
**Connections:** West → R07 (open) | North → R11 *(LOCKED — Small Key D required)* | East dead-end alcove

**Contents:**
- ⚔️ **2× Forge Hound** + **1× Slag Golem** — Golem patrols between the furnaces. Its proximity to the active furnace makes it slightly faster than normal (the heat keeps its slag-body more fluid).
- [★] **Small Key E** — in the east dead-end alcove, inside an iron mold filled with hardened slag. **[💪] Glove required** to smash the mold and extract the key. Mold is too dense for normal attacks.
- [⛓️] **Chain-pull mechanism** (east wall, above the active furnace) — pulling the chain with the Glove releases a water-quench mechanism. The active furnace goes out permanently. Removes the heat zone in the eastern half of the room. Optional but reduces hazard.
- ░░░ **Heat zone** — eastern third of the room (around the active furnace). Gradual damage. Quenchable with chain pull.
- [★] **Gold Pouch (35 coins)** — inside the second furnace (cold, safe to enter). Crouch-walk inside the furnace mouth to retrieve it.
- [★] **Forge Tongs** — collectible tool. Flavor text: *"Heavy-duty forge tongs. Handles objects too hot for even the Strong Arm Glove without risk of damaging the Glove's lining."* Mechanically allows retrieving one object in R10b that is otherwise inaccessible. Optional.
- [?] **Lore Tablet** (north wall, walkway level): *"The easternmost furnace has not been shut down since the forge was built. We lost the shutdown key in the third decade. We have been looking for it since. It is fine. Probably."*

**Enemies:** 2× Forge Hound, 1× Slag Golem
**Secrets:** None.

---

### R09 — CHAIN HALL
**Shape:** Tall vertical rectangle. The entire ceiling is covered in hanging chains of varying thickness — some decorative, some functional. The thickest chains connect to massive iron counterweights on the east and west walls. The floor has a large iron hatch in the center, sealed with four chain-wrapped locks.
**Size:** Large
**Connections:** South → R06 (locked entry, Key C used) | North → R12 (open — left wing root approach) | Down via floor hatch → R09b *(sub-chamber — four chain locks, Glove required)*

**Contents:**
- ⚔️ **2× Slag Golem** — one on each side of the central hatch. Stationary. Guardians.
- [⛓️] **Four chain-wrapped locks** on the floor hatch — each requires the Strong Arm Glove to grip and unwind. Each unwinding takes 2 seconds and produces a satisfying mechanical groan. Unwinding all four opens the hatch. Cannot be done mid-combat safely — the Golems must be dealt with first or the player risks being hit mid-interaction.
- [★] **📜 Lore Scroll — "The Dragonbinder's Fifth Entry"** — hanging from a chain on the west wall, at mid-height. Climbable chain to reach it. Reads: *"The fifth fragment is the hardest one to look at. It is a wing. I know it is a wing because when I stand near it, I feel — not a pull, not heat — but weight. The weight of something that can no longer fly. I did not expect the dragon's pieces to make me feel grief."*
- [★] **Gold Pouch (45 coins)** — hanging in a sack from a ceiling chain. Must be pulled down with the Glove — a throw or a yank (either works). The sack drops and spills.
- [?] **Lore Tablet** (east wall): *"The chains hold the counterweights. The counterweights balance the forge pressure. If the counterweights drop, the forge pressure spikes. If the forge pressure spikes... we have a sign about this somewhere."*
- [🪜] **Ladder climb point** — west wall. Reaches a high ceiling ledge where a **Heating Tonic** sits beside an old lantern (still lit, fuel unknown).

**Enemies:** 2× Slag Golem
**Secrets:**
- 🔍 Ceiling-height chain sack — Glove pull to drop gold.
- 🔍 Ladder ceiling ledge — Heating Tonic reward.

---

### R09b — SUB-CHAMBER: THE CHAIN VAULT
**Shape:** Small square. Below the Chain Hall. A sealed vault — the counterweight mechanism originates here. Two enormous iron counterweights hang from the ceiling (connected to the chains above). The room is surprisingly cool — the weight mass absorbs the heat.
**Size:** Small
**Connections:** Up → R09 (floor hatch — Glove to unwind chains from below to re-lock, or leave open)

**Contents:**
- ⚔️ No enemies — the vault has never been entered.
- [B] **The two iron counterweights** — massive hanging masses. Liftable with the Glove if the chain is released (a pin mechanism on each — Glove to pull the pin, weight drops to floor level, then liftable). No reason to move them — but players who try are rewarded with flavor text: *"You have no idea why you're holding this. You put it down."* And then a **Gold Pouch (15 coins)** falls from the chain housing above.
- [★] **Gold Pouch (70 coins)** — in a sealed strongbox on the north wall. Glove to wrench open (lid is fused by age).
- [★] **Rare Healing Herb** — full HP restore, on a shelf beside the strongbox.
- [★] **📜 Lore Scroll — "The Forgemaster's Personal Notes"** — on a small writing desk squeezed into the southeast corner. Reads: *"I built this vault to hold something of my own. Not the dragon's piece — that belongs to everyone now, or no one. I built it to hold the name of the person who told us to build around it instead of move it. The name is: The Dragonbinder. They were here before any of us."*
- The writing desk is oddly domestic — a candle, an inkwell (dry for centuries), a chair. The Forgemaster sat here, often. To think. To be away from the heat.

**Enemies:** None.
**Secrets:** None — this IS the sub-chamber.

---

### R10 — LAVA RIVER
**Shape:** Wide horizontal rectangle. The floor is divided by a single massive lava river — not a channel, a true river. The river is 4 tiles wide and flows east to west. A collapsed iron bridge spans the river in the center — the bridge is broken in the middle (a 2-tile gap). The north half of the room is accessible only by crossing.
**Size:** Large
**Connections:** South → R07 (open — south bank) | North → R12 (open — upper wing root, past river) | Down via lava tube → R10b *(sub-chamber — heat-shielded passage, timed crossing with Heating Tonic)*

**Contents:**
- 🔥🔥🔥 **Lava river** — 4 tiles wide, instant death. Bridge spans it but has a 2-tile gap in the center.
- ⚔️ **2× Forge Hound** (south bank) + **1× Slag Golem** (north bank — cannot cross the gap either, patrols north bank only).
- [B] **Iron Bridge Section** — on the south bank. A heavy replacement bridge piece, clearly pre-cut to fill the gap. Glove required to lift and carry it onto the broken bridge, then slide it into place. Bridging the gap is the room's central Glove puzzle — carrying the bridge section over the existing bridge while the gap is ahead requires careful approach and precise placement. Satisfying when done correctly.
- **Lava tube entrance** — south bank, near the east wall. A reinforced iron tube descends into the lava flow (heat-shielded — the tube itself is safe). Requires the **Heating Tonic** from R06 or R09 to survive the descent (the tube's lower section is not fully shielded — 20 seconds of heat damage without protection). With the tonic, descent is safe. Without it, manageable but damaging.
- [★] **Small Key E** — north bank, behind the Slag Golem. Required for... wait — Key E was already used/found. *Correction: Key E unlocks R08's north door. The north bank here has **Small Key F** — required for R11.* *(Note: Key F continues the chain.)*
- [★] **Small Key F** — north bank, behind the Slag Golem.
- [★] **Gold Pouch (40 coins)** — on the south bank near the lava tube entrance.
- [?] **Lore Tablet** (south bank, east wall): *"The river was not here when we built the forge. The forge created the river. We consider this a success."*

**Enemies:** 2× Forge Hound, 1× Slag Golem
**Secrets:** None — the lava tube is visible and intended.

---

### R10b — SUB-CHAMBER: LAVA TUBE VAULT
**Shape:** Small oval. The bottom of the lava tube — a heat-shielded cavity in the volcanic rock. The walls glow deep orange. The air is barely breathable. The room has the feeling of being inside something alive.
**Size:** Small
**Connections:** Up → R10 (lava tube — Heating Tonic strongly recommended for return ascent)

**Contents:**
- ⚔️ **1× Slag Golem** — smaller, faster than standard. Compressed into the small space.
- [★] **Gold Pouch (65 coins)** — in a volcanic rock hollow on the east wall. Glove to reach into the hollow safely (too hot bare-handed — same logic as the cooling vat Compass in R05).
- [★] **Forge Gauntlet Lining** — upgrade material for the Strong Arm Glove. Increases grip strength — mechanically allows the player to hold chain-pull mechanisms for 1 extra second before tiring. Optional upgrade.
- If the player has **Forge Tongs** from R08 — they can retrieve an **Ancient Forge Token** from a lava-submerged ledge that would otherwise be inaccessible. The token is a collectible with flavor text: *"A forge token from the original construction crew. Redeemable for one (1) meal at the forge canteen. The canteen closed two hundred years ago."*
- [?] **Scratched wall text** (barely readable through the heat distortion): *"If you made it down here, you are either very brave or very poorly informed. Either way: the piece you are looking for is above you. You passed it on the way down."* (A gentle tease — the Dragon Piece IS in R15, above. The note is technically accurate.)
- ░░░ **Extreme heat zone** — the entire room. Heating Tonic required for comfortable exploration. Without it: aggressive tick damage throughout.

**Enemies:** 1× Slag Golem
**Secrets:** None — this IS the sub-chamber.

---

### R11 — CASTING FLOOR
**Shape:** Large square. The final casting floor — where finished forge-work was inspected. Iron molds in rows, all empty. A raised inspection platform on the north wall. A large sealed iron door on the north wall leads to R12 — sealed with a massive bolt mechanism rather than a key lock.
**Size:** Medium-Large
**Connections:** South → R08 (locked entry, Key D used) | North → R12 *(LOCKED — Small Key F required to release bolt mechanism)*

**Contents:**
- ⚔️ **1× Slag Golem** + **2× Forge Hound** — Golem on the inspection platform, Hounds patrolling between mold rows.
- [⛓️] **Bolt mechanism** on the north door — a large iron bolt, chain-operated. Glove required to grip the chain and pull the bolt. Even with Key F (which disengages the lock cylinder), the bolt itself needs the Glove to slide open. Two-step unlock — Key F + Glove chain pull.
- [★] **Small Key F** *(if not yet retrieved from R10)* — note, Key F comes from R10 north bank. This room requires it. If player hasn't visited R10's north bank, they must backtrack. Deliberate dependency — encourages thorough exploration.
- [B] **Iron Molds ×4** — liftable with Glove. No specific use here. Players can throw them at enemies (they shatter on impact, dealing area damage to nearby enemies). Introduces the throw-for-area-damage combat option. Previews the mechanic used heavily in the boss fight.
- [★] **Gold Pouch (30 coins)** — on the inspection platform, beside the Golem.
- [★] **Healing Herb ×3** — under the inspection platform, in a supply crate.
- [?] **Lore Tablet** (inspection platform): *"Final inspection protocol: the piece is presented, the forge-work is compared, and if the forge-work is found wanting, it is returned for remelting. Every piece was returned. Every single one. The dragon's piece is a standard nothing in this forge has ever met."*

**Enemies:** 1× Slag Golem, 2× Forge Hound
**Secrets:** None.

---

### R12 — THE MASTER FORGE
**Shape:** Very large octagon. The heart of Grimforge — a cathedral of industry. The Master Forge dominates the center: an enormous furnace-altar, 4 tiles across, currently dark and cold (the Ember Crystal inserted in R01 has been carrying heat up through the pipes — but the forge is not yet lit). Four massive chain-pull mechanisms on the cardinal walls. A raised iron walkway rings the upper level. The Boss Door is in the north wall, above the walkway level.
**Size:** Very Large
**Connections:** South-West → R09 (open — chain hall approach) | South → R10 (open — lava river approach) | South-East → R11 *(LOCKED — Small Key F required to release bolt mechanism)* | North → R15 *(BOSS DOOR — requires Boss Key)* | West hidden wall → R13a *(SECRET)* | East hidden wall → R13b *(SECRET)*

**Contents:**
- ⚔️ **1× Slag Golem** + **2× Forge Hound** + **2× Steam Wraith** *(carried over from Level 3 — living in the pipe vents)* — heaviest pre-boss encounter in the dungeon.
- **Master Forge Lighting Puzzle** — the central puzzle of the dungeon. The Ember Crystal (inserted in R01) has primed the heat pipes. The Master Forge must now be lit by pulling all four chain mechanisms simultaneously — but one person cannot pull four chains at once.
  - Solution: Three of the four chains can be held open by jamming them (iron pins on the floor near each mechanism — Glove to pick up and jam into the chain link, holding it taut).
  - The fourth chain must be held by the player manually (Glove required — chains too hot to grip bare-handed).
  - When all four are taut simultaneously, the Master Forge ignites with a deep resonant boom.
  - A stone panel above the forge slides open — **Boss Key** descends on a chain to the walkway level.
- [⛓️] **Four chain mechanisms** — cardinal walls. Each requires Glove to grip.
- [B] **Iron Pins ×4** — on the floor near each mechanism. Glove to pick up and jam.
- [★] **Boss Key** — descends on a chain to the walkway after puzzle solved.
- [?] **Lore Tablet** (south wall, floor level): *"The Master Forge was built to forge one specific thing: a containment for the dragon's wing piece. The containment was never completed. The dragon's piece rejected every attempt. It is still in there. The forge went dark the day we stopped trying."*
- ░░░ **Heat zone** — the entire room becomes a heat zone once the Master Forge ignites. Moderate tick damage — encourages getting the Boss Key quickly and moving on. The room was not designed for long occupation after ignition.
- **[🔨] Cracked west wall panel** (walkway level) — Hammer reveals tunnel to R13a.
- **[💪] Loose east wall block** (floor level) — Glove required to shift the heavy block aside, revealing tunnel to R13b.

**Enemies:** 1× Slag Golem, 2× Forge Hound, 2× Steam Wraith
**Secrets:**
- 🔍 **West cracked wall panel** (walkway) — Hammer reveals R13a tunnel.
- 🔍 **East loose wall block** (floor) — Glove shifts block aside, reveals R13b tunnel.

---

### R13a — SECRET ROOM: THE FORGEMASTER'S STUDY *(West, Walkway Level)*
**Shape:** Small rectangle. Surprisingly liveable — a personal study carved out of the forge wall. Shelves, a desk, a burned-out lantern. Ash on every surface.
**Size:** Small
**Connections:** East → R12 (cracked wall panel, Hammer used, walkway level)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **📜 Lore Scroll — "The Dragonbinder's Fifth Entry — Addendum"** — Reads: *"I found the Forgemaster's study. He left everything behind. His notes describe two hundred years of failed attempts to contain the wing piece. He never gave up. I respect that. I also think he was wrong about what the piece needed. It did not need a container. It needed to be found."*
- [★] **Rare Healing Herb** — full HP restore.
- [★] **Gold Pouch (90 coins)** — highest single gold reward in the dungeon, in a locked iron box on the desk. Hammer to break the lock.
- The Forgemaster's personal journal on the desk — too damaged to read fully, but one page is intact: *"Day 1 of year 200: The piece shifted position in the containment frame today. It has not moved in 199 years. I think it knows someone is coming."*
- A child's drawing on the wall — a dragon, whole and unbroken, with wings spread. Signed: *"Mira, age 7."* Same Mira. One year older than the drawing in Level 3.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R13b — SECRET ROOM: THE CASTING ARCHIVE *(East, Floor Level)*
**Shape:** Small square. An archive room — shelves of iron-tube schematics and casting records. Similar to Level 3's schematic vault, but older and more disorganised.
**Size:** Small
**Connections:** West → R12 (loose wall block, Glove used, floor level)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **Gold Pouch (65 coins)** — in a sealed iron tube on the top shelf.
- [★] **Healing Herb ×3** — in a preservation case on the bottom shelf.
- [★] **Casting Record — "The Wing Piece"** — readable item. Reads: *"Containment attempt #847. The piece again rejected the housing. Upon rejection, the housing melted entirely — not from heat, but from what we can only describe as refusal. The piece does not want to be contained. It wants to be part of something. We do not know what."* Foreshadows the full dragon puzzle.
- [🔨] **Cracked back wall** — Hammer reveals a small nook containing **Gold Pouch (25 coins)** and a **Heating Tonic**. Secret within a secret.
- The same fish carvings from Level 4's shrine appear here — scratched into the iron shelving by someone who was in both places. A thread connecting the dungeons.

**Enemies:** None.
**Secrets:**
- 🔍 Cracked back wall — secret within a secret. Gold and tonic.

---

### R15 — BOSS CHAMBER: THE FORGING PIT
**Shape:** Very large square with rounded corners. The deepest chamber in Grimforge — a raw volcanic cavity shaped into an arena by the forge's builders. The floor is cracked black basalt with lava seeping through the cracks in thin glowing lines — not lethal channels, just ambient heat-light. Four massive iron anvils sit in the corners. The ceiling is open volcanic rock — no construction above. The Dragon Piece hangs in a iron containment frame on the north wall, visible and glowing.
**Size:** Very Large — biggest room in the dungeon.
**Connections:** South → R12 *(BOSS DOOR — Boss Key required)*

**Contents:**
- 🔒 **BOSS DOOR** — south wall. Boss Key required. The door drops into the floor rather than swinging or rising — a heavy thud, a cloud of hot ash, and the arena is revealed.
- **BOSS: Moltenfist, the Forgemaster** — an enormous humanoid figure, more iron than flesh, built up over centuries of constant forge exposure. His hands are literal iron — massive, glowing-hot forge hammers fused to his arms. He stands 4 tiles tall. When the Boss Door drops, he is already looking at it. He was expecting this.
- [★] **🧩 Dragon's Left Wing (Piece 5/8)** — in the containment frame on the north wall. Frame releases automatically on boss defeat. Piece floats to the ground.
- [★] *(No new tool this level — this is the mastery check.)* A **Forge Gauntlet Upgrade** is available on the north altar beside the frame — upgrades the Strong Arm Glove's throw distance and power. Optional but significant.
- **4× Iron Anvils** in the corners — the key mechanic of the fight. Each anvil is liftable with the Strong Arm Glove. Throwing an anvil at Moltenfist deals massive damage and staggers him. This is the primary mechanic the lore has been building toward — the boss fight that demands the player throw his own anvils back at him.
- Moltenfist will periodically attempt to reclaim an anvil — if he reaches one, he lifts it (slower than the player — 3 second wind-up) and hurls it. The player must dodge. Anvils once thrown by the boss can be picked up again by the player.
- ░░░ **Ambient heat zone** — the entire room. Constant mild tick damage. Healing Tonic from earlier in the dungeon is valuable here.

**Boss — Moltenfist, the Forgemaster:**
- **Phase 1 (100%–65% HP):** Slow advance, always toward the player. Iron-fist ground slam (lifts one arm, slams down — shockwave 3 tiles in a cross pattern, dodge sideways). Sweep attack (horizontal arc, 180°, dodge by rolling backward). **Weak point: the cooling vent on his chest** — glows blue when exposed after each failed slam. Striking the vent deals triple damage. Throwing an anvil at his chest hits the vent for massive damage regardless of his facing — and staggers him for 2 seconds.
- **Phase 2 (65%–35% HP):** Moltenfist's hands superheat — contact now deals burn damage (must not touch him at all). Begins grabbing anvils and throwing them (telegraphed by turning to face the nearest anvil — 2 seconds to reposition). Gains a charge attack (runs 5 tiles — large turning radius, crash into wall staggers him for 3 seconds, venting chest exposed). Adds a ground-pound that sends lava seeping through the floor cracks temporarily wider (step off the crack lines for 3 seconds or take burn damage).
- **Phase 3 (35%–0% HP):** Both anvils on the south side have been hurled by Moltenfist — only the north two remain. Moltenfist's chest vent opens permanently (no longer only on slam stumble — now always accessible but he moves faster). He begins a rampage pattern — non-stop advance, no recovery pauses. The player must use the last two anvils decisively. Final throw that kills him causes him to stagger backward into the containment frame — the frame cracks, the Dragon Piece is released.
- **Defeat:** Moltenfist's iron hands cool and go dark. He stands still for a moment. Then he sits down slowly — an enormous figure, suddenly tired. His head drops. The containment frame cracks open. The Dragon Piece drifts down. The lava lines in the floor fade. The forge, for the first time in centuries, begins to cool.

**Secrets:** None — boss rooms are always clean and readable.

---

## DUNGEON FLOW SUMMARY

```
CRITICAL PATH:
R01 (Ember Crystal inserted, map, Glove test) → R02 (fight, get Key A, enter the right-side wing spine)
→ R04 (Glove bridge puzzle, get Key C via weight plate) → R07 (anvil weight plate, get Key D)
→ R08 (fight, get Key E, Forge Tongs) → R10 (bridge lava gap, get Key F north bank)
→ R11 (Key F + Glove chain bolt, enter the upper wing root)
→ R12 (four-chain Forge lighting puzzle, get Boss Key) → R15 (boss, get Piece 5)

KEY LOCATIONS:
  Small Key A — R02 (wall bracket, mid-ramp)
  Small Key B — R03 (third grating platform, chain descent)
  Small Key C — R04 (weight plate panel, iron beam required)
  Small Key D — R07 (weight plate panel, anvil or ingot required)
  Small Key E — R08 (iron mold, Glove to smash)
  Small Key F — R10 (north bank, behind Slag Golem)
  Boss Key    — R12 (descends on chain after forge lighting puzzle)

OPTIONAL PATHS:
  R02 → R03 (Key B + lore — drops into the outer feather at R06)
  R02 → R05 (Compass + Glove retrieval from cooling vat)
  R05 → Cooling Ingot (carry to R07 as alternate weight plate solution)
  R06 → Chain pull (deactivates all bellows — quality of life on the wing's far edge)
  R06 → Heating Tonic (useful for R10b descent)
  R09 → R09b (four chain-unwinding — gold + lore)
  R10 → R10b (lava tube — Heating Tonic recommended, gold + Glove upgrade)
  R12 → R13a (Hammer west wall — Lore Scroll + gold)
  R12 → R13b (Glove east block — Casting Record + secret-within-secret)

TOOL SYNERGIES:
  Heating Tonic (R06 or R09 ledge) → reduces damage in R10b and R15
  Forge Tongs (R08) → retrieves Ancient Forge Token from R10b lava ledge
  Forge Gauntlet Lining (R10b) → increases chain-pull hold time in R12 puzzle
  Cooling Ingot (R05) → alternate weight plate solution in R07 (bypasses anvil tipping)

MASTERY CHECK — ALL PRIOR TOOLS TESTED:
  Ladder  → R02 ledge, R03 shaft, R06 bellows, R09 ceiling ledge
  Hammer  → R07 catwalk wall, R13a box lock, R13b back wall
  Raft    → not needed (no water this dungeon — absence is intentional)
  Glove   → every major puzzle mechanic in the dungeon

BLOCKED UNTIL FUTURE TOOLS:
  No new tool blocks — Portal Tool (Level 6) blocks begin in Level 6
  All existing tool interactions are available here
```

---

## FULL ITEM & TOOL CHECKLIST

| Item | Location | Required? |
|------|----------|-----------|
| 🗺️ Dungeon Map | R01 — obsidian slab on entry wall | Auto-collected |
| 🔥 Ember Crystal | World — volcanic outcrop | Yes (primes Master Forge) |
| 🧭 Compass | R05 — cooling vat (Glove required) | Optional |
| 🗝️ Small Key A | R02 — wall bracket mid-ramp | Optional (opens R03) |
| 🗝️ Small Key B | R03 — third grating platform (chain descent) | Optional (opens R06 from below) |
| 🗝️ Small Key C | R04 — weight plate panel (iron beam) | Yes (opens R06→R09 route) |
| 🗝️ Small Key D | R07 — weight plate panel (anvil/ingot) | Yes (opens R08) |
| 🗝️ Small Key E | R08 — iron mold (Glove smash) | Yes (opens R08 north to R11 path — wait, Key E used for R11 entry)* |
| 🗝️ Small Key F | R10 — north bank behind Golem | Yes (opens R11 bolt mechanism with Glove) |
| 🔑 Boss Key | R12 — descends on chain after puzzle | Yes (opens R15) |
| 🔧 Forge Tongs | R08 — dead-end alcove shelf | Optional (retrieves R10b token) |
| 🧪 Heating Tonic ×2 | R06 shelf + R09 ceiling ledge | Optional (reduces R10b/R15 damage) |
| 💪 Glove Lining | R10b — volcanic hollow | Optional (Glove upgrade) |
| 🪙 Ancient Forge Token | R10b — lava ledge (Forge Tongs req.) | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 5) | R09 — hanging chain | Optional (collectible) |
| 📜 Lore Scroll (Forgemaster Notes) | R09b — writing desk | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 5 Addendum) | R13a — study desk | Optional (collectible) |
| 📋 Casting Record | R13b — iron tube | Optional (lore item) |
| 🧩 Piece 5/8 | R15 — containment frame after boss | Main Objective |
| 🥊 Forge Gauntlet Upgrade | R15 — north altar after boss | Optional (Glove upgrade) |
| 💰 Gold (total ~625) | Throughout | Optional |
| 🌿 Healing Herbs | R03, R05, R07, R09, R09b, R11, R13a, R13b | Optional |

*Key naming consolidated for clarity in final document pass.

---
---

*[Level VI — The Fractured Sanctum will be added to a new document upon completion]*
