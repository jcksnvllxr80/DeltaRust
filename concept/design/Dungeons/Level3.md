# 🐉 THE DRAGON SEAL — Dungeon Design Document
*Visual ASCII Maps + Room-by-Room Notes*
*Scale: Small early → Large late | Format: ASCII map + notes*

---
---

# LEVEL III — ⚙️ THE IRONCLAD VAULT
**Theme:** A deep mechanical fortress buried beneath a mountain. Iron walls, grinding gears, steam vents, pressure-sealed chambers, and ancient clockwork mechanisms. Everything is locked, bolted, or welded shut. The Hammer is the master key — used to smash sealed panels, break cracked walls, trigger stuck mechanisms, and destroy armored enemies. The Ancient Key (found in the world, in a merchant's locked chest) is required to enter at all.
**Difficulty:** Mid-early. Teaches: Hammer usage on walls / panels / enemies, steam vent hazards, gear-lock puzzles, multi-key gating, combat against armored enemy types.
**Total Rooms:** 15 *(10 main + 2 secret + 2 sub-chambers + 1 boss)*
**Puzzle Piece:** 🧩 Dragon's Left Hind Leg *(Piece 3/8)*
**Tool Earned:** 🛶 Raft
**World Item Required:** 🗝️ Ancient Key *(obtained outside the dungeon — merchant's locked chest in the overworld)*
**New Mechanics Introduced:** Hammer wall-breaks · Steam vent hazards · Gear-lock puzzles · Cracked panel smashing · Armored enemy weak points

---

## ASCII MAP

**Dungeon silhouette: LEFT HIND LEG** — L-shaped. A vertical column (the "thigh")
rising from entry to the knee, then rooms extend leftward as the "shin and foot."
The boss sits at the foot's end.

```
    ┌───────────┐
    │    R15    │
    │   BOSS    │
    │[IRONJAW]  │
    └─────┬─────┘
          │ [BOSS DOOR]
    ┌─────┴─────┐
    │    R12    ├══════════►
    │  Gear Lock│   [SECRET R13b]
    │   Chamber │
◄═══    ════════┘
[SECRET       │
  R13a]  ┌────┴────┐
         │   R11   │
         │ Furnace │
         │  Hall   │
         └────┬────┘
              │
    ┌─────────┴─────────┐
    │                   │
┌───┴─────┐        ┌────┴────┐
│  R09    │        │   R10   │
│ Steam   │        │ Pressure│
│ Tunnels │        │  Vault  │
└────┬────┘        └────┬────┘
     │                  │
┌────┴────┐        ┌────┴────┐
│  R09b   │        │  R10b   │
│SUB-CHMBR│        │SUB-CHMBR│
│(steam)  │        │(sealed) │
└─────────┘        └─────────┘
    ┌──────────────────────┘
    │             ┌─────────────────────────────────────┐
    │             │                                     │
┌───┴─────┐  ┌───┴─────┐                          ┌────┴────┐
│  R06    │  │   R08   │                          │   R07   │
│  Bolt   │  │  Iron   │                          │ Central │
│ Gallery │  │ Library │                          │  Works  │
└─────────┘  └─────────┘                          └────┬────┘
                                                       │
                                   ┌───────────────────┼──────────────┐
                                   │                   │              │
                              ┌────┴────┐         ┌────┴────┐   ┌────┴────┐
                              │   R03   │         │   R04   │   │   R05   │
                              │ Intake  │         │  Gear   │   │ Welding │
                              │  Hall   │         │  Floor  │   │  Bay    │
                              └─────────┘         └────┬────┘   └─────────┘
                                                       │
                                                  ┌────┴────┐
                                                  │   R02   │
                                                  │Antechamber│
                                                  │(Entry Lock)│
                                                  └────┬────┘
                                                       │
                                                  ┌────┴────┐
                                                  │   R01   │
                                                  │  ENTRY  │
                                                  │ (Start) │
                                                  └─────────┘


SECRET ROOMS (not on dungeon map):
  R13a ◄═══ west hidden panel of R12
  R13b ═══► east hidden panel of R12

SUB-CHAMBERS:
  R09b ↓ below R09 (steam vent shaft — Hammer to open grate)
  R10b ↓ below R10 (sealed pressure chamber — Hammer to break panel)
```

```
LEGEND
══════════════════════════════════════════
  │ ─ ┌ ┐ └ ┘ ┤ ├  = Tunnel / corridor
  ◄═══►         = Hidden / secret panel passage
  [BOSS DOOR]   = Requires Boss Key to open
  [LOCKED]      = Requires Small Key to open
  [★]           = Chest or item location
  [!]           = Pressure plate
  [?]           = Lore tablet / hint stone
  [B]           = Boulder / heavy object (pushable)
  [🪜]          = Ladder climb point (up or down)
  [🔨]          = Hammer-smashable wall or panel
  ~~~           = Water / flooded floor
  ░░░           = Collapsing floor tiles
  ▓▓▓           = Dark zone (torch / lantern required)
  ^^^           = Steam vent (periodic hazard — burns on contact)
  ⚙️            = Gear mechanism (interactive)
  ↑ ↓           = Upper / lower floor transition
```

---

## ROOM-BY-ROOM NOTES

---

### R01 — ENTRY CHAMBER
**Shape:** Wide horizontal rectangle. The entrance is a massive iron door set into the mountainside — already ajar, forced open by some previous visitor who never returned. The interior is dim, lit by residual heat glow from deep within the vault.
**Size:** Large
**Connections:** North → R02 *(LOCKED — Ancient Key required)*

**Contents:**
- 🗺️ **DUNGEON MAP** — etched into a brass plate bolted to the east wall. Precise, mechanical, clearly made by the vault's original builders. Shows R01–R12 and R15. Sub-chambers R09b and R10b and secret rooms R13a/R13b are NOT shown.
- [?] **Lore Tablet** (west wall, iron-cast text): *"The Ironclad Vault was built to contain what could not be destroyed. Entry requires authorisation. Unauthorised access will be met with the Lockkeeper."*
- The Ancient Key slot is visible beside the north door — an ornate iron keyhole that clearly matches the Ancient Key obtained in the overworld. Inserting it permanently unlocks the north door. The key stays in the slot (cannot be retrieved).
- A row of iron lockers along the south wall — all welded shut. Three have been forced open by the previous visitor. Empty. One still sealed. **[🔨] Hammer-smashable** — breaking it open reveals **Gold Pouch (30 coins)** inside. First Hammer use in the dungeon, immediately on entry. Teaches the mechanic gently.
- A dead mechanical construct on the floor — a small gear-driven sentinel, broken. Cogs spill from its chest. Foreshadows the enemy type inside.
- Residual heat shimmer in the air — the vault runs hot. Atmospheric.

**Enemies:** None.
**Secrets:**
- 🔍 Sealed locker — Hammer required. Rewards players who test the mechanic immediately.

---

### R02 — ANTECHAMBER / ENTRY LOCK
**Shape:** Square. Feels deliberately transitional — a decompression chamber between the outside world and the vault proper. Iron walls, no decoration. A large mechanical gate on the north wall with a visible gear mechanism beside it.
**Size:** Medium
**Connections:** South → R02 (Ancient Key used, permanently open) | North → R04 *(gear-locked — must activate mechanism to open)* | West → R03 (open) | East → R05 (open)

**Contents:**
- ⚙️ **Gear Mechanism** (east wall) — a large hand-crank. Turning it (hold interact for 3 seconds) opens the north gate to R04. If the player releases the crank, the gate begins to close again. A **wedge object** (an iron bar on the floor nearby) can be jammed into the crank to hold it — teaches the "jam the mechanism" trick used again in R07 and R12.
- ⚔️ **2× Iron Sentinel** — new enemy. Small gear-driven constructs. Slow but deal heavy damage on contact. Immune to normal weapons from the front (iron plating). **Weak point: the gear cluster on their back** — exposed when they turn to track the player. Die in 2 Hammer hits to the back, or 6 normal hits anywhere.
- [★] **Iron Bar** — on the floor near the south wall. Used to jam the gear crank. Cannot leave the room (too heavy — flavor text if player tries).
- [?] **Lore Tablet** (north wall, beside the gate): *"Primary access: Central Works. Secondary access: Intake Hall (west), Welding Bay (east). All sectors report to the Lockkeeper."*
- A pressure gauge on the north wall slowly rises and falls — decorative, but establishes the steam-pressure atmosphere.

**Enemies:** 2× Iron Sentinel
**Secrets:** None.

---

### R03 — INTAKE HALL
**Shape:** Long vertical rectangle. A processing corridor — conveyor track runs down the center (inactive, belts stopped). Iron crates stacked along both walls.
**Size:** Medium-Large
**Connections:** East → R02 (open) | North → R06 *(LOCKED — Small Key A required)* | West dead-end alcove

**Contents:**
- ⚔️ **3× Iron Sentinel** — one patrols the conveyor track length, two guard the north door. Conveyor-patrol enemy moves faster than normal Sentinels (carried momentum) — new behavior variant.
- [★] **Small Key A** — in the east dead-end alcove, inside a sealed iron crate. **[🔨] Hammer required to break the crate open.** Second mandatory Hammer use. The crate is visually distinct from decorative crates (a red X painted on it).
- [★] **Gold Pouch (25 coins)** — atop a stacked crate on the west wall. Requires climbing (no Ladder needed — crates are stacked like steps, climbable).
- [★] **Healing Herb ×2** — behind a crate on the east wall near the alcove.
- [?] **Lore Tablet** (east wall): *"Intake processed 400 units in the vault's first year. Records cease after the sealing event. The conveyor has not moved since."*
- The conveyor track has a large gear wheel at each end — decorative, but one gear is cracked and half-separated from the wall. **[🔨] Hammer-smashable** — breaking it causes the conveyor to lurch and briefly move, knocking any enemy standing on it into the east wall (stun). Optional combat trick.

**Enemies:** 3× Iron Sentinel
**Secrets:**
- 🔍 Cracked conveyor gear — optional Hammer use to stun enemies.

---

### R04 — GEAR FLOOR
**Shape:** Wide square. The floor is a grid of interlocking gear teeth — large flat cogs embedded in the stone, slowly rotating. Walking across them is safe but noisy (enemies in adjacent rooms get a stealth alert). Three elevated platforms dot the room, each 2 tiles high.
**Size:** Large
**Connections:** South → R02 (gear-locked entry, mechanism jammed open) | North → R07 (open) | West → R03 (via R02 branch) | East → R05 (via R02 branch)

**Contents:**
- ⚔️ **2× Iron Sentinel** + **1× Gear Brute** — new enemy. The Gear Brute is large, slow, and covered in spinning gear teeth that damage the player on contact (must not touch its body — only safe to attack from range or with a timed roll). Walks in wide patrol arcs. Weak point: the exhaust pipe on its back — Hammer hit stuns it for 3 seconds.
- [★] **🧭 COMPASS** — on the central elevated platform (middle of the room). Requires climbing up (iron ladder rungs on the platform side).
- [!] **Pressure Plate** on the northeast platform — stepping on it temporarily stops all gear rotation on the floor (3 seconds of silence — no stealth alert noise). Useful to know for sneaking. Resets after 3 seconds.
- [?] **Lore Tablet** (west wall, platform level): *"The gear floor was a security measure. Motion sensors beneath the cogs detect unauthorised foot traffic. The Lockkeeper is always listening."*
- ⚙️ **Gear rotation** — walking on the active gears does not damage the player but creates loud clanking. Enemies in R07 will be alerted if the player crosses without using the pressure plate to silence the floor first.
- The three platforms are islands of silence — standing on them generates no noise.

**Enemies:** 2× Iron Sentinel, 1× Gear Brute
**Secrets:** None.

---

### R05 — WELDING BAY
**Shape:** Wide horizontal rectangle. Welding equipment lines the walls — iron frames, tool racks, dead forges. The ceiling is lower than other rooms. Two large iron doors on the far wall are welded shut and cannot be opened from this side.
**Size:** Medium
**Connections:** East → R02 (open) | North → R08 *(LOCKED — Small Key B required)*

**Contents:**
- ⚔️ **2× Iron Sentinel** + **1× Ash Drifter** *(carried over from Level 2 — a scout unit that wandered in through a ventilation shaft)*. The Ash Drifter moves unpredictably among the iron Sentinels — disruptive, fragile, fast.
- [★] **Small Key B** — welded to a bracket on the north wall. **[🔨] Hammer required to knock it free.** Third mandatory Hammer use.
- [★] **Gold Pouch (20 coins)** — inside a half-open forge on the east wall. Reachable by hand.
- [★] **Gold Pouch (15 coins)** — on a tool rack. Optional.
- [?] **Lore Tablet** (south wall): *"Bay 4: Fabrication of containment housing for the sealed artefact. Project lead: Ironjaw Unit 01. Completion date: pre-sealing. Current status: operational."*
- **[🔨] Two welded iron doors** on the north-east wall — both are decorative dead ends even after Hammer use. One reveals an empty alcove (disappointing — teaches that not every breakable surface holds something). The other reveals **Healing Herb ×2** (rewards persistence).
- A half-finished iron construct on a workbench — clearly an early prototype of the Iron Sentinel enemies. One arm still twitches (ambient animation).

**Enemies:** 2× Iron Sentinel, 1× Ash Drifter
**Secrets:**
- 🔍 Two welded doors — one empty, one rewarding. Teaches variance in breakable surfaces.

---

### R06 — BOLT GALLERY
**Shape:** Long vertical rectangle, high ceiling. Iron bolts the size of tree trunks line both walls horizontally — giant locking mechanisms for something enormous and unseen. They extend from the walls and retract on a slow cycle, creating periodic obstacles across the corridor.
**Size:** Medium-Large
**Connections:** South → R03 (locked entry, Key A used) | North → R09 (open)

**Contents:**
- ⚔️ **2× Iron Sentinel** — stationed between bolt cycles. Timing fights around the moving bolts is the room's combat puzzle.
- **Moving Bolts** — 4 pairs of iron bolts extend from opposite walls on a 5-second cycle (2 seconds extended, 3 seconds retracted). Walking into an extended bolt deals moderate damage and pushes the player back. The pattern is learnable — consistent timing.
- [★] **Gold Pouch (35 coins)** — in a wall alcove accessible only when the bolt beside it is retracted. Requires timing.
- [★] **Torch ×2** — in a sconce on the east wall, midway through the gallery.
- [?] **Lore Tablet** (north wall near exit): *"The bolt gallery secures the vault's primary containment. Should the bolts retract simultaneously, the inner vault will open. This has never happened. This must never happen."*
- **[🔨] Cracked bolt housing** (west wall, third bolt from north) — Hammer smash jams that bolt permanently retracted. Optional quality-of-life shortcut — makes the north half of the gallery permanently safe to cross. Not required but rewards Hammer experimentation.

**Enemies:** 2× Iron Sentinel
**Secrets:**
- 🔍 Cracked bolt housing — Hammer to jam bolt permanently. Rewards tool use.

---

### R07 — CENTRAL WORKS
**Shape:** Large cross-shaped room. The mechanical heart of the vault. Pipes run floor to ceiling. Gear clusters cover the walls. A massive central piston rises and falls on a 4-second cycle — 3 tiles tall, blocks north-south movement when raised. Two raised catwalks run east-west at mid-height.
**Size:** Very Large
**Connections:** South → R04 (open) | West → R08 (open) | West → R06 (via R09 branch) | North → R10 *(LOCKED — Small Key C required)* | Upper catwalks West → R08 upper level | Upper catwalks West → R09 upper level

**Contents:**
- ⚔️ **1× Gear Brute** + **2× Iron Sentinel** — Brute patrols around the central piston. Sentinels on the catwalks above.
- **Central Piston** — rises and falls on a 4-second cycle. When raised: blocks direct north-south path through the center. When lowered: player can run across. Timing required. The piston also creates a shockwave on impact with the floor — knocks player back if standing in the center tile when it lands.
- [🪜] **Ladder climb points** — both east and west walls have iron rungs leading to the catwalks.
- ⚙️ **Gear mechanism** (east catwalk) — a secondary crank. Turning it and jamming it with a nearby **iron pin** (item on the catwalk floor) slows the piston cycle to 8 seconds. Makes crossing significantly easier. Optional but teaches the jam mechanic again.
- [★] **Small Key C** — on the north side of the piston, floor level. Accessible only when the piston is raised (or by crossing during the lowered window). Requires timing.
- [★] **Gold Pouch (40 coins)** — on the west catwalk.
- [?] **Lore Tablet** (south wall): *"The central piston regulates pressure across all seven vault sectors. Manual override requires two simultaneous inputs. Ironjaw holds the override authority."*
- **^^^  Steam vents** — two vents on the north wall floor, flanking the locked north door. Active on a 3-second cycle. Stepping into the steam stream deals burn damage. Must be timed to approach Key C and the north door.

**Enemies:** 1× Gear Brute, 2× Iron Sentinel
**Secrets:** None.

---

### R08 — IRON LIBRARY
**Shape:** Wide square. Shelves of iron-bound records line every wall floor to ceiling — schematics, construction logs, maintenance reports. A reading desk in the center. A locked iron cabinet on the north wall.
**Size:** Medium
**Connections:** East → R07 (open, floor level) | East → R07 (open, catwalk level) | North → R11 *(LOCKED — Small Key D required)*

**Contents:**
- ⚔️ **2× Iron Sentinel** — one on the floor level, one on a catwalk-height shelf walkway (accessed via iron rungs on the east wall).
- [★] **Small Key D** — inside the locked iron cabinet on the north wall. **[🔨] Hammer required to break the cabinet lock.** Fourth mandatory Hammer use. Cabinet door swings open after strike.
- [★] **📜 Lore Scroll — "The Lockkeeper's Directive"** — on the reading desk. Reads: *"Ironjaw Unit 01 — Primary containment directive: the sealed artefact must not leave the vault. Secondary directive: any entity attempting removal is to be neutralised. Tertiary directive: if the vault is breached, seal yourself inside with the artefact and wait. We are still waiting."*
- [★] **Blueprint Fragment** — collectible schematic page. Flavor text: *"Partial schematic for the Raft Containment System — Vault Sector 7. The raft was used to transport the sealed artefact across the underground reservoir during construction."* Foreshadows the Raft tool reward.
- [?] **Lore Tablet** (east wall): *"All vault records are stored in triplicate. Two copies were destroyed in the sealing event. This is the third copy. It is the only copy."*
- **[🔨] Cracked shelf section** (west wall, lower) — Hammer breaks it open, revealing a hidden alcove behind the shelving: **Gold Pouch (45 coins)** and **Healing Herb ×1**.

**Enemies:** 2× Iron Sentinel
**Secrets:**
- 🔍 Cracked shelf — Hammer to reveal alcove behind it. Gold + healing reward.

---

### R09 — STEAM TUNNELS
**Shape:** Irregular, branching. A maintenance tunnel network — low ceilings, narrow passages, constant hissing of steam pipes. Multiple small alcoves branch off the main corridor. The floor is hot metal grating.
**Size:** Large (sprawling, but narrow passages make it feel small)
**Connections:** South → R06 (open) | North → R12 (open) | Down via grate → R09b *(sub-chamber — Hammer required to break grate)*

**Contents:**
- ⚔️ **3× Steam Wraith** — new enemy. Emerge from steam vents as half-formed entities. Cannot be damaged while in steam form — must wait for them to fully materialise (2 second window after emerging) then strike. Die in 2 hits. Respawn at their vent after 20 seconds if not permanently sealed.
- **^^^ Steam vents** — 5 active vents throughout the tunnel network. Fixed positions, cycle every 4 seconds (2 on, 2 off). Wraiths emerge from them.
- **[🔨] Floor grate** — in the northeast alcove, a heavy iron grate covers a shaft leading down to R09b. Hammer breaks the bolts holding it. Drops player into the sub-chamber below. Iron rungs on the shaft wall for climbing back up.
- [★] **Gold Pouch (30 coins)** — in a west alcove, behind a steam vent (must time approach).
- [★] **Healing Herb ×2** — in a south alcove near the entry.
- [?] **Maintenance Log** (scratched into the pipe wall, readable): *"Vent 3 and Vent 5 are showing pressure anomalies. Request maintenance team. Request denied — all personnel reassigned to Lockkeeper containment duty. Log ends."*
- **[🔨] Cracked pipe junction** (east wall) — Hammer strike bursts a steam pipe, permanently disabling Vent 4 and Vent 5. Removes two Steam Wraith spawn points. Optional but significant quality-of-life improvement.

**Enemies:** 3× Steam Wraith
**Secrets:**
- 🔍 Floor grate — Hammer to open, descend to R09b.
- 🔍 Cracked pipe junction — Hammer to permanently disable two vents and two Wraith spawns.

---

### R09b — SUB-CHAMBER: STEAM RESERVOIR
**Shape:** Small square. Below the tunnel grating. A sealed maintenance reservoir — pipes feed in from every wall. The floor is a shallow pool of hot (but not damaging) water. Steam fills the upper half of the room visually.
**Size:** Small
**Connections:** Up → R09 (shaft with iron rungs — Hammer to open grate from above, can climb back freely)

**Contents:**
- ⚔️ **1× Steam Wraith** — spawns from the central pipe fitting. Cannot leave this room.
- ~~~ **Shallow hot water floor** — wade movement. Not damaging (below boiling). Visual steam effect obscures the room slightly.
- [★] **Gold Pouch (55 coins)** — on a pipe bracket on the south wall, above water level. Reachable by jumping up to it (1-tile hop).
- [★] **Rare Healing Herb** — on a sealed maintenance shelf on the east wall. Full HP restore.
- [★] **Pressure Valve** — collectible item. Can be used once in R12 to reduce steam hazards in the gear-lock puzzle. Optional but helpful.
- [?] **Maintenance Tag** (on pipe): *"Reservoir 9-B: Do not enter without pressure-rated equipment. Pressure rating: extreme. Equipment available: none."*

**Enemies:** 1× Steam Wraith
**Secrets:** None — this IS the sub-chamber.

---

### R10 — PRESSURE VAULT
**Shape:** Tall vertical rectangle. Divided into two halves by a reinforced iron wall with a single sliding panel in the center. North half is sealed until the pressure puzzle is solved. South half is accessible immediately. High ceiling with pressure gauges visible on every surface.
**Size:** Large
**Connections:** South → R07 (locked entry, Key C used) | North half → R12 (open, once panel slides) | Down via sealed floor hatch → R10b *(sub-chamber — Hammer required to break hatch bolts)*

**Contents:**
- ⚔️ **2× Iron Sentinel** — south half only. North half is sealed until puzzle solved.
- **Pressure Puzzle** — four pressure gauges on the south wall, each with a dial and a target zone marked in red. Player must turn each dial to the correct pressure level using a nearby hand-crank. Each crank affects two gauges simultaneously (interdependent — adjusting one shifts another). Solving requires working through the interdependencies methodically. When all four gauges are in the red target zone simultaneously — the central iron panel slides open, revealing the north half.
- [★] **Small Key E** — north half, on a pedestal behind the now-open panel. Required for... a later room (carried forward to R11).
- **[🔨] Floor hatch bolts** — in the southwest corner of the south half. Four iron bolts seal a floor hatch. Hammer breaks all four (four Hammer strikes). Opens access to R10b below.
- [?] **Lore Tablet** (south wall, beside gauges): *"Pressure equalisation required before inner vault access. Incorrect calibration will trigger lockdown. Lockdown has been active for an indeterminate period."*
- ^^^ **Two steam vents** flank the central panel — active until the puzzle is solved. Solving the pressure puzzle also shuts the vents down permanently (pressure equalises).

**Enemies:** 2× Iron Sentinel
**Secrets:**
- 🔍 Floor hatch — Hammer to open, descend to R10b.

---

### R10b — SUB-CHAMBER: SEALED PRESSURE ROOM
**Shape:** Small horizontal rectangle. Hermetically sealed — no pipes, no vents. Pristine compared to the rust and grime above. Clearly built to protect something specific.
**Size:** Small
**Connections:** Up → R10 (floor hatch — Hammer used to open from above, iron rungs to climb back)

**Contents:**
- ⚔️ No enemies — this room has never been entered before. Untouched.
- [★] **📜 Lore Scroll — "The Dragonbinder's Third Entry"** — third page of the Dragon Codex. Reads: *"The third fragment is deeper in than I expected. The vault was built around it — not to display it, but to bury it. Someone wanted this piece forgotten specifically. That tells me it is the most important one yet."*
- [★] **Gold Pouch (70 coins)** — sealed inside a pristine lockbox. No lock — it was sealed by pressure alone. Opens freely now that the hatch is broken.
- [★] **Iron Magnet** — collectible tool upgrade. Attached to the Compass — makes the Compass more accurate in magnetically noisy environments (mechanically: stabilises the compass needle in R12's gear-heavy environment).
- [?] **Builder's Note** (pinned to the wall with an iron nail): *"If you are reading this, the vault has been breached. You have found what we hid here. We ask only that you consider what you are about to do with it. The dragon's pieces are not trophies. They are a warning."*

**Enemies:** None.
**Secrets:** None — this IS the sub-chamber.

---

### R11 — FURNACE HALL
**Shape:** Wide square. A massive furnace dominates the north wall — cold and dead, its fuel long exhausted. The furnace door is sealed. Iron cauldrons on iron frames fill the room. The floor is scorched black.
**Size:** Medium-Large
**Connections:** South → R08 (locked entry, Key D used) | North → R12 *(LOCKED — Small Key E required)*

**Contents:**
- ⚔️ **1× Gear Brute** + **2× Steam Wraith** — Brute guards the furnace door. Wraiths emerge from two cold vents (they still function despite the furnace being dead — connected to a separate pipe network).
- **[🔨] Furnace door** — sealed shut. Hammer breaks the seal. Inside the furnace: **Gold Pouch (50 coins)** and a **Gold Pouch (20 coins)**. The furnace interior is safe — cold for centuries.
- [★] **Small Key E** — wait, Key E was found in R10 north half. Key E unlocks the north door here. *(No second Key E spawns — player must have retrieved it from R10 first. If they haven't visited R10 yet, this door remains locked until they do. Creates a deliberate non-linear dependency.)*
- [★] **Healing Herb ×3** — in iron cauldrons on the west side. Visible on approach.
- [?] **Lore Tablet** (east wall): *"The furnace powered the vault for sixty years without interruption. It went cold on the day of the sealing. We have not been able to relight it. We have not tried very hard."*
- **[🔨] Cracked cauldron** (northeast corner) — Hammer strike shatters it, spilling its contents: **Gold Pouch (15 coins)** and an unexpected **live coal** that briefly lights the room brighter (3 seconds — cosmetic only).

**Enemies:** 1× Gear Brute, 2× Steam Wraith
**Secrets:**
- 🔍 Sealed furnace door — Hammer to open. Gold + armor material inside.
- 🔍 Cracked cauldron — Hammer to shatter. Small gold reward.

---

### R12 — GEAR LOCK CHAMBER
**Shape:** Large octagon. The mechanical showpiece of the dungeon. Eight gear clusters — one on each wall — are all interconnected by iron shafts. A massive central lock mechanism sits in the floor — a 3-tile-wide gear assembly that, when solved, opens the floor and reveals the Boss Key on a rising plinth. The room is loud. Gears turn constantly. The air shakes.
**Size:** Very Large
**Connections:** South → R09 (open, from steam tunnels) | South → R10 (open, from pressure vault north half) | North → R15 *(BOSS DOOR — requires Boss Key)* | West hidden panel → R13a *(SECRET)* | East hidden panel → R13b *(SECRET)*

**Contents:**
- ⚔️ **1× Gear Brute** + **2× Iron Sentinel** + **2× Steam Wraith** — heaviest enemy presence in the dungeon. The Gear Brute patrols the outer ring of the octagon. Sentinels are stationary near the gear clusters. Wraiths emerge from two vents on the south wall.
- **Gear Lock Puzzle** — the central floor assembly has 8 gear slots (matching the 8 wall clusters). Each wall cluster has a numbered dial (1–8). The player must set each dial to its correct position. The correct positions are hinted at by symbols on the floor around the central assembly — each symbol corresponds to a wall cluster number and a gear-tooth count visible on that cluster. The player must count gear teeth on each cluster and set the dial to that number. When all 8 are correct: the central assembly unlocks, rises, and reveals the **Boss Key** on the plinth.
- If the player has the **Iron Magnet** from R10b, the Compass points toward the correct dial setting for each cluster (glows when facing the right direction) — significantly simplifies the puzzle for thorough explorers.
- If the player has the **Pressure Valve** from R09b, they can use it on the south wall vents — permanently shuts down both Steam Wraith spawns, making the fight manageable before starting the puzzle.
- ^^^ **Steam vents** (south wall, 2 active) — suppressible with Pressure Valve from R09b.
- **[🔨] Cracked west wall panel** (mid-height) — behind a gear cluster. Hammer clears it, reveals tunnel to R13a.
- **[🔨] Cracked east wall panel** (floor level) — Hammer clears it, reveals tunnel to R13b.
- [?] **Lore Tablet** (north wall, beside the Boss Door): *"The Lockkeeper's chamber lies beyond. Ironjaw Unit 01 has protected the vault's innermost secret since the sealing. It has never failed. It does not know how."*

**Enemies:** 1× Gear Brute, 2× Iron Sentinel, 2× Steam Wraith
**Secrets:**
- 🔍 **West cracked panel → R13a:** A gear cluster on the west wall is mounted slightly lower than the others and has visible cracking in the stone panel behind it. Hammer reveals the tunnel. Requires noticing the inconsistency.
- 🔍 **East cracked panel → R13b:** The east wall panel has a hairline fracture running vertically beside a gear cluster. Hammer breaks it open. Again — requires close observation.

---

### R13a — SECRET ROOM: THE BUILDER'S ALCOVE *(West)*
**Shape:** Small rectangle. Clean stone, no gears — this predates the vault's mechanical fitting. Older construction.
**Size:** Small
**Connections:** East → R12 (cracked panel, Hammer used)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **📜 Lore Scroll — "The Dragonbinder's Third Entry — Addendum"** — a second fragment of the third Codex page, found separately. Reads: *"I found a builder's note in the sub-chamber. They knew what they were hiding. They built this place not to keep thieves out — but to give the piece somewhere worthy to rest. I am not sure I am worthy. I will take it anyway."*
- [★] **Rare Healing Herb** — full HP restore.
- [★] **Gold Pouch (90 coins)** — highest single gold reward in the dungeon.
- A child's drawing scratched into the stone — a dragon, crude but recognisable, surrounded by 8 circles. Below it, a name: *"Mira, age 6."* Who was she? Why was she here?
- A small stone bench — the only furniture in the entire dungeon. Someone sat here, often, for a long time.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R13b — SECRET ROOM: THE SCHEMATIC VAULT *(East)*
**Shape:** Small square. Shelves of rolled iron-tube schematics line the walls — more organised than the Iron Library in R08. These were the originals.
**Size:** Small
**Connections:** West → R12 (cracked panel, Hammer used)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **Gold Pouch (60 coins)** — in a sealed iron tube on the top shelf, alongside rolled schematics.
- [★] **Healing Herb ×3** — in a sealed preservation case on the bottom shelf (still fresh after centuries — the case kept them).
- [★] **Master Schematic — "The Raft Transport System"** — readable item. Full schematic for the underground reservoir raft system used to transport the vault's sealed artefact during construction. Flavor text: *"Raft capacity: 2 personnel + 1 sealed containment unit. Reservoir depth: 40 fathoms. Current raft status: stored in containment chamber beyond the Lockkeeper's room."* Directly foreshadows the Raft tool reward location in R15 and its use in Level 4.
- [?] **Engraved wall text** (no tablet — etched directly): *"Everything in this vault was built by hand. Every gear, every bolt, every wall. We built it to last. We built it to be forgotten. We succeeded at both."*

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R15 — BOSS CHAMBER: THE LOCKKEEPER'S SANCTUM
**Shape:** Large circle. The vault's innermost chamber. The walls are entirely covered in interlocking gears — all slowly turning, all connected. The floor is polished iron. In the center, a heavy iron containment frame stands empty — this is where the Dragon Piece was kept. The piece itself sits atop a sealed altar on the north wall, behind a thick glass panel. The Raft is folded and stored in a large iron rack on the east wall.
**Size:** Very Large — biggest room in the dungeon.
**Connections:** South → R12 *(BOSS DOOR — Boss Key required)*

**Contents:**
- 🔒 **BOSS DOOR** south side — Boss Key required. The door is a 12-bolt iron seal — all bolts retract simultaneously when the key is inserted.
- **BOSS: Ironjaw Unit 01 (The Lockkeeper)** — a massive gear-driven mechanical beast. Four-legged, the size of a cart horse, with a jaw that opens to reveal a gear-toothed grinding mechanism (its namesake). Has been sealed in this room since the vault was built. When the Boss Door opens, it activates instantly — no dormancy delay. It was waiting.
- [★] **🧩 Dragon's Left Hind Leg (Piece 3/8)** — behind the glass panel on the north altar. Glass shatters automatically on boss defeat (shockwave from the Ironjaw's collapse breaks it). Piece is then freely collectible.
- [★] **🛶 RAFT** — folded in the iron rack on the east wall. Collectible after boss defeat. The schematic in R13b already described exactly this rack.
- The empty iron containment frame in the center is where the Ironjaw has stood for centuries. The floor beneath it is worn into a perfect ring — it paced in circles. For centuries. Alone.
- Four gear-cluster pillars at the cardinal points — destructible (Hammer or combat damage). Destroying a pillar briefly stuns Ironjaw. Each can only be destroyed once.
- ^^^ **Steam vents** in the floor — 6 of them, in a ring around the center. Active in Phase 2 only. Fixed positions, 3-second cycle.
- No lore tablet. No note. The room speaks for itself.

**Boss — Ironjaw Unit 01 (The Lockkeeper):**
- **Phase 1 (100%–65% HP):** Charges in straight lines across the room (predictable — wide turning radius, telegraphed by revving gear sound 1.5 sec before charge). Jaw-snap attack at close range (must dodge sideways — deals heavy damage). **Armored front plating** — immune to damage from the front. Must be struck on the flanks or rear. Weak point: the exhaust pipe cluster on the back (same as Gear Brute, but larger target window — 3 seconds after each failed charge into a wall).
- **Phase 2 (65%–30% HP):** Gear pillars become live hazards (spinning gear arms extend from them — avoid). Floor vents activate. Ironjaw gains a sweeping jaw-drag attack (drags its jaw along the floor in an arc — jump or dodge over it). Charge speed increases. Back window shortens to 2 seconds.
- **Phase 3 (30%–0% HP):** Front plating cracks and falls away — now vulnerable from all directions. Ironjaw becomes erratic, non-linear. Begins ricocheting off walls unpredictably. Its jaw-snap gains a shockwave (small radius). Gear pillars (if not already destroyed) begin spinning independently, moving slowly around the room.
- **Defeat:** Ironjaw skids to a stop facing the south wall (the Boss Door). Its gears wind down one by one. Its jaw drops open. The grinding stops. The entire room goes quiet for the first time in centuries. The glass panel cracks. The Raft rack unlocks.

**Secrets:** None — boss rooms are always clean and readable.

---

## DUNGEON FLOW SUMMARY

```
CRITICAL PATH (ascends the thigh, then extends left along the leg):
R01 (Ancient Key used, get map, break locker) → R02 (fight, jam crank) → R04 (fight, get Compass)
→ R07 (fight, time piston, get Key C) → R10 (pressure puzzle, get Key E, open hatch to R10b)
→ R11 (fight, use Key E) → R12 (gear-lock puzzle, get Boss Key) → R15 (boss, get Piece 3 + Raft)

NOTE: The layout forms an L-shape — the vertical entry column bends left
at the mid-point, evoking the Dragon's Left Hind Leg. The boss sits at
the "foot" end of the L.

KEY LOCATIONS:
  Small Key A — R03 (sealed crate, Hammer required)
  Small Key B — R05 (welded bracket, Hammer required)
  Small Key C — R07 (north side of piston, timing required)
  Small Key D — R08 (iron cabinet lock, Hammer required)
  Small Key E — R10 (north half, after pressure puzzle)
  Boss Key    — R12 (after gear-lock puzzle)

OPTIONAL PATHS:
  R02 → R03 (Key A + gold — opens R06 route to steam tunnels)
  R02 → R05 (Key B + gold — opens R08 route to Iron Library and R11)
  R06 → R09 (Steam Tunnels → R12 second southern entry)
  R09 → R09b (Hammer grate — Pressure Valve + gold + Lore Scroll)
  R07 → R08 (Iron Library — Key D + Lore Scroll + blueprint)
  R10 → R10b (Hammer hatch — Lore Scroll + Iron Magnet + gold)
  R12 → R13a (west cracked panel, Hammer — gold + lore)
  R12 → R13b (east cracked panel, Hammer — Raft schematic + gold)

TOOL SYNERGIES (optional items aiding puzzle):
  Iron Magnet (R10b) + Compass → simplifies gear-lock puzzle in R12
  Pressure Valve (R09b) → suppresses Steam Wraiths in R12

BLOCKED UNTIL FUTURE TOOLS:
  R03 east rubble wall (from Level 2 callback) — N/A this dungeon
  All [🔨] surfaces are clearable here — Hammer is available from Level 2
  No Level-4-tool blocks yet — those begin in Level 4
```

---

## FULL ITEM & TOOL CHECKLIST

| Item | Location | Required? |
|------|----------|-----------|
| 🗺️ Dungeon Map | R01 — brass plate on entry wall | Auto-collected |
| 🗝️ Ancient Key | World — merchant's locked chest | Yes (to enter dungeon) |
| 🧭 Compass | R04 — central platform | Optional |
| 🧲 Iron Magnet | R10b — sealed shelf | Optional (simplifies R12 puzzle) |
| 🔧 Pressure Valve | R09b — maintenance shelf | Optional (suppresses R12 Wraiths) |
| 🗝️ Small Key A | R03 — sealed crate (Hammer) | Optional (opens R06 route) |
| 🗝️ Small Key B | R05 — welded bracket (Hammer) | Optional (opens R08 route) |
| 🗝️ Small Key C | R07 — floor level north of piston | Yes (opens R10) |
| 🗝️ Small Key D | R08 — iron cabinet (Hammer) | Yes (opens R11) |
| 🗝️ Small Key E | R10 — north half after puzzle | Yes (opens R11 north door to R12) |
| 🔑 Boss Key | R12 — revealed after gear-lock puzzle | Yes (opens R15) |
| 📜 Lore Scroll (Lockkeeper's Directive) | R08 — reading desk | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 3) | R10b — sealed shelf | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 3 Addendum) | R13a — secret alcove | Optional (collectible) |
| 📋 Blueprint Fragment | R08 — reading desk | Optional (lore item) |
| 📋 Master Schematic | R13b — iron tube | Optional (lore item) |
| 🧩 Piece 3/8 | R15 — north altar after boss | Main Objective |
| 🛶 Raft | R15 — east wall rack after boss | Yes (needed for Level 4) |
| 💰 Gold (total ~610) | R01–R13b | Optional |
| 🌿 Healing Herbs | R03, R05, R07, R09, R09b, R11, R13a, R13b | Optional |

---
---

*[Level IV — The Sunken Citadel will be added to a new document upon completion]*
