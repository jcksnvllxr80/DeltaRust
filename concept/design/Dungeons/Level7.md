# 🐉 THE DRAGON SEAL — Dungeon Design Document
*Visual ASCII Maps + Room-by-Room Notes*
*Scale: Small early → Large late | Format: ASCII map + notes*

---
---

# LEVEL VII — ✨ THE AETHERIAN SPIRE
**Theme:** A celestial tower suspended above everything — floating platforms, impossible architecture, star-mapped ceilings, gravity inversion chambers, and vast open shafts dropping into a sky that has no ground beneath it. The Portal Tool is the master tool — used to bridge platform gaps, redirect across vertical shafts, bypass gravity-inversion zones, and intercept the boss's teleportation routes. The Star Sigil (purchased from the Celestial Merchant in the overworld — appears only at night) is required to activate the Spire's entry mechanism. The Raft is also needed to cross the Sky Moat at the Spire's base. This is the second-to-last dungeon — complexity is near maximum, room count is 20, and every prior tool sees meaningful use.
**Difficulty:** Very High. Teaches: Portal Tool placement and traversal, gravity inversion (upside-down movement in designated zones), Sky Moat Raft navigation (open-air water crossing at extreme height), timed portal sequences, vertical shaft navigation, star-alignment puzzle.
**Total Rooms:** 20 *(12 main + 2 secret + 3 sub-chambers + 1 gravity chamber + 1 sky platform + 1 boss)*
**Puzzle Piece:** 🧩 Dragon's Head & Horns *(Piece 7/8)*
**Tool Earned:** *(No new tool — all six tools used in combination throughout)*
**World Items Required:** ⭐ Star Sigil *(Celestial Merchant, overworld, night only)* · 🛶 Raft *(to cross the Sky Moat at the Spire's base)*
**New Mechanics Introduced:** Portal Tool placement and traversal · Gravity inversion zones · Sky Moat Raft crossing · Timed portal sequences · Vertical shaft navigation · Star-alignment puzzle · Portal interception (boss mechanic)

---

## ASCII MAP

**Dungeon silhouette: HEAD & HORNS** — the lower Spire forms a long vertical
neck, then the upper dungeon splits into two horn-like branches before meeting
again at the crown and boss platform.

```
            ┌───────────┐
            │    R20    │
            │   BOSS    │
            │[AURELION] │
            └─────┬─────┘
               │ [BOSS DOOR]
         ◄══════════════╧══════════════►
             ┌─────────────────┐
          [SECRET] │      R17        │ [SECRET]
             │ Star Alignment  │
             └────────┬────────┘
                │
        ┌────────────────┴────────────────┐
      ┌────┴────┐                      ┌─────┴────┐
      │   R14   │                      │   R16    │
      │Gravity  │                      │   Sky    │
      │Chamber  │                      │ Platform │
      └────┬────┘                      └──────────┘
        │
      ┌────┴────┐
      │   R15   │
      │ Portal  │
      │Gauntlet │
      └─┬────┬──┘
        │    │
      ┌────┘    └────┐
    ┌────┴────┐    ┌────┴────┐
    │  R14b   │    │  R15b   │
    │SUB-CHMBR│    │SUB-CHMBR│
    │(invertd)│    │(midshaft)│
    └─────────┘    └─────────┘
        │
        ┌──┴──┐
        │ R10 │
        │Shaft│
        └──┬──┘
        │
        ┌──┴──┐
        │R10b │
        │sub  │
        └──┬──┘
        │
    ┌─────────┼─────────┐
  ┌────┴────┐ ┌──┴──┐ ┌────┴────┐
  │   R09   │ │ R11 │ │   R05   │
  │  Wind   │ │Astrl│ │ Broken  │
  │ Gallery │ │Hall │ │ Bridge  │
  └────┬────┘ └──┬──┘ └────┬────┘
    │         │         │
  ┌────┴────┐ ┌──┴──┐ ┌────┴────┐
  │   R07   │ │ R06 │ │   R08   │
  │ Mooring │ │Entry│ │ Outer   │
  │  Dock   │ │Atrium││ Rampart │
  └─────────┘ └──┬──┘ └─────────┘
        │
        ┌──┴──┐
        │ R04 │
        │Sky  │
        │Moat │
        └──┬──┘
        │
      ┌────┴────┐
      │   R03   │
      │  Sigil  │
      │  Gate   │
      └────┬────┘
        │
      ┌────┴────┐
      │   R01   │
      │  ENTRY  │
      │ (Start) │
      └─────────┘

Additional lower spur:
  R02 branches east from R01 as the base camp platform.
```


SECRET ROOMS (not on dungeon map):
  R18a ◄═══ west portal-rift of R17
  R18b ═══► east portal-rift of R17

SUB-CHAMBERS:
  R10b ↓ base of central shaft (Portal Tool to descend safely)
  R14b ↓ inverted vault below gravity chamber (gravity-inverted access)
  R15b ↓ mid-shaft platform (Portal shortcut from R15)

SKY PLATFORM:
  R16 — external floating platform, accessed from R15 east exit
```

```
LEGEND
══════════════════════════════════════════
  │ ─ ┌ ┐ └ ┘ ┤ ├  = Tunnel / corridor
  ◄═══►         = Hidden / secret portal-rift passage
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
  [🧭]          = Void Compass check point
  [🌀]          = Portal Tool placement point / portal surface
  [≋]           = Environmental rift / passive portal
  ≋≋≋           = Sky Moat (deep sky-water, Raft required)
  ░░░           = Unstable / flickering floor
  ▽▽▽           = Gravity inversion zone (movement inverted)
  ↑ ↓           = Upper / lower level transition
  ~~~           = Wind current (pushes player laterally)
  ⭐            = Star Sigil activation point
```

---

## ROOM-BY-ROOM NOTES

---

### R01 — ENTRY CHAMBER
**Shape:** Wide horizontal rectangle. The base of the Aetherian Spire — a broad platform of pale white stone suspended in open sky. The sky here is permanent night: deep blue-black, stars visible in every direction including below. Looking down through gaps in the platform edge reveals infinite starfield. The Spire rises from the center of this platform, a slender white tower stretching upward until it is lost in cloud. The entrance is a sealed archway in the Spire's base wall.
**Size:** Large
**Connections:** North → R03 (open — archway into Spire base) | East → R02 (open — base camp platform) | East (far) → R08 (open — outer rampart walkway)

**Contents:**
- 🗺️ **DUNGEON MAP** — carved into a star-chart table on the east platform edge. The map is oriented with north pointing upward, which in this dungeon means toward the top of the Spire. Shows R01–R17 and R20. Sub-chambers R10b, R14b, R15b and secrets R18a/R18b are NOT shown.
- [?] **Lore Tablet** (west wall of archway): *"The Aetherian Spire was built by those who watched the sky the night the dragon was sealed. The sealing was visible from the surface as a star going dark. They built the Spire to reach that dark place. They reached it. The dragon's piece was there, waiting."*
- ⭐ **Star Sigil activation slot** — beside the archway, a carved celestial socket. Inserting the Star Sigil (from the overworld Celestial Merchant) permanently unlocks the archway and sends a pulse of starlight up through the Spire — activating dormant mechanisms in R10, R15, and R17. Without the Sigil the archway does not open. The Sigil stays in the slot.
- The platform edge has no railing — the drop into the starfield below is immediate and visually alarming. Players who approach the edge see a long, dizzying fall. Falling off the platform edge deposits the player back at R01's center (recovery mechanic — no damage, but the visual of falling through stars is striking).
- [★] **Gold Pouch (30 coins)** — on the east edge of the platform, beside the star-chart table.
- The Spire's exterior is covered in carved celestial maps — star positions, constellation lines, angular measurements. Someone spent centuries documenting the sky from this height.

**Enemies:** None.
**Secrets:** None.

---

### R02 — BASE CAMP
**Shape:** Wide irregular platform — a natural rest point established by previous explorers. Supply crates, a bedroll, a lantern still burning. The furthest-from-danger room in the dungeon after R01.
**Size:** Medium
**Connections:** West → R01 (open) | North → R04 (open — Sky Moat approach)

**Contents:**
- ⚔️ **None** — this is a rest room. Deliberately enemy-free.
- [★] **Healing Herb ×4** — the largest healing cache in the dungeon, stacked beside the supply crates. Clearly left by the previous explorer who established this camp.
- [★] **Gold Pouch (25 coins)** — in an open supply crate.
- [★] **Moon Essence** — collectible upgrade item. When applied to the Portal Tool, extends portal range by 50% (portals can be placed further apart). Optional but significant for later portal puzzles.
- [?] **Explorer's Note** (pinned to a crate): *"Camp established at Spire base. Sky Moat crossable at platform R04 — use the Raft, the current runs east-to-west but manageable. The Sigil Gate opened easily once I had the Star Sigil. The hard part starts above the moat. The gravity rooms are on the upper levels. I left supplies here. Use them. You'll need them coming back down if you make it that way."*
- [🔨] **Cracked supply crate** (northwest corner) — Hammer to break open. Contains **Torch ×2** and a **Celestial Lens** collectible. The Celestial Lens when used in R17 reveals the correct star alignment pattern without having to solve it manually — optional puzzle skip.
- The bedroll is the first indication that another explorer reached this point. The same M who left notes throughout the prior dungeons? The handwriting on the note matches.

**Enemies:** None.
**Secrets:**
- 🔍 Cracked supply crate — Hammer for torches and Celestial Lens.

---

### R03 — SIGIL GATE
**Shape:** Tall vertical rectangle — a formal gatehouse built into the Spire's base. High vaulted ceiling with star-glass inlays. The now-open archway (Star Sigil used) leads into the first interior room. A second locked gate on the north wall leads to the Sky Moat approach.
**Size:** Medium
**Connections:** South → R01 (archway — Star Sigil used, permanently open) | North → R04 (open — after Star Sigil activation pulse reaches this gate, unlocking it automatically)

**Contents:**
- ⚔️ **2× Star Sentinel** — new enemy. Constructs of light and stone, angular and precise. Move in straight lines only — never diagonal. Attack with light-lance (a beam that travels in a straight line, 4-tile reach, 1.5 sec telegraph — step perpendicular to avoid). Die in 4 hits. Immune to backstrike bonus (no weak point — damage them from any angle equally).
- [★] **🧭 COMPASS** — on a wall bracket on the east side. Standard retrieve.
- [?] **Lore Tablet** (east wall): *"The Sigil Gate was designed to admit only those carrying a celestial authorisation. The Celestial Merchant holds the authorisations. The Merchant has been circling the Spire's base for three hundred years, selling to anyone who arrives at night. We consider this an acceptable security risk."*
- [★] **Gold Pouch (20 coins)** — on a stone plinth near the north gate.
- The north gate opens automatically once the Star Sigil is inserted in R01 — the pulse of starlight travels through the Spire's walls and unlocks it remotely. Players who explored R02 first and inserted the Sigil in R01 will find this gate already open on arrival. Those who came here first will need to backtrack to R01 to insert the Sigil.

**Enemies:** 2× Star Sentinel
**Secrets:** None.

---

### R04 — SKY MOAT CROSSING
**Shape:** Open-air platform with a wide gap — the Sky Moat. The moat is a channel of sky-water (luminescent, pale blue — visually stunning) that runs east-west across the Spire's base platform, 6 tiles wide and apparently bottomless. The Raft can be deployed here for the Sky Moat crossing — the only body of water in this dungeon.
**Size:** Large (open platform)
**Connections:** South → R02 (open) | South → R03 (open) | North → R06 (open — far bank of Sky Moat) | East → R08 *(LOCKED — Small Key A required, east platform section)*

**Contents:**
- ≋≋≋ **Sky Moat** — 6 tiles wide, luminescent sky-water. Raft required. Cannot be waded or jumped. The water glows faintly — bioluminescent organisms live in it. The crossing is brief but beautiful.
- [🛶] **Raft dock point** — south bank, iron ring hammered into the platform edge.
- ~~~ **Water current** — runs east-to-west at moderate speed. Pushes Raft sideways during crossing. Player must paddle north while compensating for the lateral drift. A diagonal crossing is required — the Raft docks on the northwest corner of the far bank.
- ⚔️ **1× Sky Phantom** — new enemy. Floats above the Sky Moat, cannot be engaged from the south bank (out of range). Attacks the player during the Raft crossing with light-lances from above. Player must dodge while steering the Raft. After crossing, Sky Phantom descends to the north bank and becomes a standard aerial encounter.
- [★] **Small Key A** — east platform section, behind a locked gate. Requires crossing the Sky Moat first (to get to the north bank) and then circling back via R08. Key A unlocks the east approach to R08 from R04's east gate. Non-linear dependency — forces some players to visit R08 via R01's outer rampart path first.
- [?] **Lore Tablet** (south bank, east end): *"The Sky Moat was not planned. The platform settled unevenly after construction and a crack formed. The water came from the storm clouds that pass through at this altitude. We considered filling it. We decided it added character."*
- [🌀] **Portal surface** (north bank wall, marked with a celestial glyph) — the first Portal Tool surface in the dungeon. A paired glyph on the south bank wall. If the player places a portal on each glyph, they create a permanent crossing shortcut — bypassing the Raft for all subsequent crossings. First Portal Tool use in the dungeon; optional but introduces the mechanic clearly.

**Enemies:** 1× Sky Phantom
**Secrets:** None.

---

### R05 — BROKEN BRIDGE
**Shape:** Wide horizontal rectangle with a catastrophic gap — a 4-tile chasm in the center of the room where the bridge floor has collapsed entirely. The south half and north half are separated. The gap is too wide to jump. No physical bridge material nearby. Portal Tool required.
**Size:** Large
**Connections:** West → R06 (open — south half) | North → R09 *(LOCKED — Small Key B required, north half)* | North → R10 (open — north half, east side)

**Contents:**
- ⚔️ **2× Star Sentinel** (south half) + **1× Star Sentinel** (north half — cannot cross the gap, patrols the north side).
- **4-tile chasm** — instant-death fall. No bridge material. No Ladder solution. Portal Tool only. First mandatory Portal Tool use in the dungeon: player places one portal on the south half wall, one on the north half wall, walks through. Simple and clear — teaches basic portal placement.
- [🌀] **Portal surfaces** — marked celestial glyphs on both the south and north walls of the chasm. Glowing faintly, indicating they are valid portal placement points. The glyphs make the mechanic obvious without hand-holding — experienced players will see them and know what to do.
- [★] **Small Key B** — north half, behind the north-side Star Sentinel, near the locked door.
- [★] **Gold Pouch (35 coins)** — south half, on a ledge beside the west exit.
- [?] **Lore Tablet** (south half, west wall): *"The bridge collapsed during the Star Sigil testing. We had not anticipated the resonance effect. Three testing staff were on the north half when it happened. They were fine. They simply could not get back. We resolved this eventually."*
- [🔨] **Cracked south wall section** — Hammer reveals a small alcove: **Healing Herb ×2** and a **Gold Pouch (15 coins)**.

**Enemies:** 3× Star Sentinel
**Secrets:**
- 🔍 Cracked south wall — Hammer for healing and gold.

---

### R06 — ENTRY ATRIUM
**Shape:** Large square. The Spire's interior formal entry — high ceiling, geometric tile floor, four exits. Star-glass windows on the east and west walls (no exterior view — they show star-maps, not the sky outside). A central pillar rises from floor to ceiling, covered in celestial text.
**Size:** Large
**Connections:** South → R04 (open — Sky Moat far bank) | North → R10 (open) | East → R05 (open) | West → R07 (open)

**Contents:**
- ⚔️ **2× Star Sentinel** + **1× Void Wraith** *(carried from Level 6 — drifted in through a dimensional crack in the Spire wall)*.
- [★] **📜 Lore Scroll — "The Dragonbinder's Seventh Entry"** — on the central pillar, rolled and wedged into a carved alcove at eye level. Reads: *"The seventh fragment is the head. The horns. I have been in seven dungeons now and I still cannot explain what it feels like to stand near one of these pieces. It is not warmth. It is not cold. It is recognition — like the piece knows I am the one who has been gathering them. Like it has been waiting. This one more than any of the others."*
- [?] **Lore Tablet** (north wall, beside the R10 exit): *"The atrium was the last room built. Everything above it was constructed from the top down — the builders started at the star and worked their way to the ground. The atrium was the final piece. They built the bottom of the Spire last, standing in open sky, the whole tower above them. They found it a profound experience."*
- [★] **Gold Pouch (35 coins)** — on a tile dais in the southeast corner.
- [🌀] **Portal surface pair** (east and west walls, symmetric) — a paired set of celestial glyphs. If the player places portals on both, they create a permanent east-west shortcut across the atrium — trivial given the room's small size, but teaches that portals can be placed in same-level rooms, not just across gaps.
- The celestial text on the central pillar is in an ancient language — not translatable with current lore. But the number 8 appears repeatedly, and what appears to be a dragon-shaped constellation mark. Foreshadowing in a dead language.

**Enemies:** 2× Star Sentinel, 1× Void Wraith
**Secrets:** None.

---

### R07 — MOORING DOCK
**Shape:** Wide horizontal rectangle. An exterior dock platform on the Spire's west face — open air, wind, stars in all directions. A secondary Raft dock here for approaching from the west (an alternate route to the Spire that bypasses the Sky Moat entirely — for players who circled the Spire's base). Currently empty of use but provides atmospheric breathing room.
**Size:** Medium
**Connections:** East → R06 (open) | North → R09 *(LOCKED — Small Key C required)*

**Contents:**
- ⚔️ **1× Sky Phantom** + **1× Star Sentinel**.
- [🛶] **Secondary Raft dock** — iron ring on the platform edge. Raft can be docked here. No water to cross from this dock — it is an arrival point only.
- [★] **Small Key C** — on the north wall beside the locked door to R09. Behind the Sky Phantom's patrol route.
- [★] **Gold Pouch (30 coins)** — on the platform edge, east side.
- [★] **Healing Herb ×2** — in a supply crate near the Raft dock.
- [?] **Lore Tablet** (west platform wall, facing open sky): *"The view from the west dock is the clearest in the Spire. On a clear night you can see three of the other dungeons from here — the volcanic smoke of the forge, the ruined tower of Ashenfall, the glow of the Mosshaven cavern entrance. They are far away. You have come far."*
- The view from this platform is the first time the player can see all prior dungeons simultaneously — a visual recap of the journey. The volcanic smoke, the ruined coastal battlements, the forest cave entrance. All visible in miniature, far below and far away. A quiet, earned moment.

**Enemies:** 1× Sky Phantom, 1× Star Sentinel
**Secrets:** None.

---

### R08 — OUTER RAMPART
**Shape:** Long vertical rectangle — an exterior walkway around the Spire's base, east face. Open on the right side (east — open sky, no wall, 1-tile-wide walkway with no railing on the right). Wind pushes the player left (into the Spire wall) periodically.
**Size:** Medium-Large
**Connections:** South → R01 (open — outer walkway south end) | North → R04 east gate *(LOCKED — Small Key A required from north side)* | West mid-point → R06 (open — mid-rampart interior entry)

**Contents:**
- ⚔️ **3× Star Sentinel** — on the narrow walkway. Their straight-line movement pattern is genuinely dangerous on a 1-tile-wide walkway — player must dodge laterally (into the Spire wall, the only safe side) to avoid being pushed off the edge.
- ~~~ **Wind current** — periodic gust from the east every 8 seconds (3-second duration). Pushes player 1 tile toward the open edge. Player must brace (hold against the wall) or be repositioned. Does not deal damage if pushed off edge — same recovery mechanic as R01 platform fall. But enemy hits during a gust can combine with the push to knock the player off.
- [★] **Small Key A** — south end of the rampart, in a bracket on the wall. Available immediately on entering from R01. This is the key that unlocks R04's east gate — which means players who do the outer rampart early get the east gate access before crossing the Sky Moat.
- [★] **Gold Pouch (40 coins)** — north end of the rampart, near the R04 east gate.
- [?] **Lore Tablet** (mid-rampart, on the interior wall): *"The outer rampart has claimed eleven builders. All fell east. None fell west — the Spire wall catches you on that side. We consider the eastern fall a design flaw. We have not fixed it. The rampart is otherwise structurally sound."*
- [🔨] **Cracked Spire wall section** (mid-rampart interior) — Hammer reveals a passage into R06's east side. Shortcut from the outer rampart directly into the atrium interior, bypassing the need to walk the full rampart length.

**Enemies:** 3× Star Sentinel
**Secrets:**
- 🔍 Cracked Spire wall — Hammer shortcut into R06.

---

### R09 — WIND GALLERY
**Shape:** Long horizontal rectangle. The Spire's internal wind-management system — pipes and vents run through this room, creating strong lateral wind currents at floor level. The floor is a series of floating stone platforms (4 platforms, 2 tiles each, 1-tile gaps between them). Wind pushes the player sideways mid-jump, making platform crossing without the Portal Tool very difficult.
**Size:** Large
**Connections:** South → R05 (locked entry, Key B used) | South → R07 (locked entry, Key C used) | North → R11 (open) | East → R10 (open — mid-platform, requires crossing to the east wall)

**Contents:**
- ⚔️ **2× Sky Phantom** — hover between platforms, using the wind to their advantage (their movement is unaffected by wind — they are flight-capable).
- ~~~ **Wind current (floor level)** — strong, consistent east-to-west flow. Affects the player during platform jumps — jumping from platform to platform while the wind blows requires angling the jump slightly into the wind to compensate. Without compensation, the player drifts off the platform.
- [🌀] **Portal surfaces** on the west wall (south end) and east wall (north end) — Portal Tool allows skipping the entire platform sequence. Simple portal placement on both glyphs creates a direct crossing. Rewards Portal Tool users; punishes players who forgot to equip it.
- [★] **Small Key D** — on the third platform (second from the north), requiring crossing two gaps to reach.
- [★] **Gold Pouch (45 coins)** — on the fourth platform (northernmost), near the R10 east exit.
- [?] **Lore Tablet** (south wall, near entry): *"The wind gallery manages airflow through the Spire's upper levels. Without it the star-glass windows would shatter from internal pressure. It was not designed as a navigation challenge. It became one anyway."*
- [🔨] **Cracked north wall section** (above the fourth platform) — Hammer reveals a high alcove: **Rare Healing Herb** (full HP) and a **Gold Pouch (25 coins)**. Requires being on the fourth platform and looking up.

**Enemies:** 2× Sky Phantom
**Secrets:**
- 🔍 Cracked north wall alcove — Hammer for rare healing and gold.

---

### R10 — CENTRAL SHAFT
**Shape:** The Spire's central vertical shaft — a 6-tile-wide circular shaft running from the base of the Spire to near its top. The player enters on a ledge partway up. Looking up: the shaft continues upward through 8 floors of the Spire, each level visible as a ring of light. Looking down: the shaft base is visible far below, with a faint blue glow. No floor — only the ledge the player enters on and a series of floating stone platforms spiraling upward.
**Size:** Very Large (vertical)
**Connections:** South → R06 (open — main ledge entry) | West → R09 (open — main ledge, west side) | Up platform spiral → R15 (top of shaft — Portal Tool or platform climb) | Down shaft → R10b *(sub-chamber — Portal Tool to reach safely, or fall and take damage)*

**Contents:**
- ⚔️ **2× Star Sentinel** (main ledge) + **3× Sky Phantom** (spiraling above — patrol the shaft's ascending platforms).
- **Shaft platform spiral** — 12 platforms, each 2 tiles wide, arranged in a loose spiral around the shaft's interior wall. Gaps between platforms range from 2 to 4 tiles. The 4-tile gaps require Portal Tool to cross — jumping alone cannot bridge them.
- [🌀] **Portal surfaces** — glyphs marked on the shaft walls at each major gap point. Paired glyphs on opposite walls allow portal traversal across the shaft. The shaft is the dungeon's primary Portal Tool playground — the mechanic is tested repeatedly at increasing difficulty as the player ascends.
- [★] **Small Key E** — on a platform midway up the shaft (platform 6 of 12). Requires careful ascent.
- **Star Sigil pulse effects** — the Sigil inserted in R01 activated mechanisms throughout the shaft. At every 4th platform, a star-map projection appears on the shaft wall (beautiful, atmospheric). At platform 8, the projection shows the Dragon's head constellation — a direct hint at the dungeon's piece.
- [🪜] **Ladder climb points** — iron rungs on the shaft wall beside each platform. Rungs connect adjacent platforms only — cannot reach the 4-tile gaps. The Ladder gets the player between close platforms; the Portal Tool handles the large gaps. Both tools needed.
- [?] **Lore Tablet** (main ledge, before ascent): *"The shaft was designed for a singular purpose: to connect the ground to the star. Everything in the Spire is in service of that connection. You are currently inside the connection. Look up."*
- Looking up from the main ledge is one of the game's most striking visual moments — the spiral of lit platforms ascending into the starfield visible through the open top of the shaft.

**Enemies:** 2× Star Sentinel, 3× Sky Phantom
**Secrets:** None — the shaft's scale is its entire statement.

---

### R10b — SUB-CHAMBER: SHAFT BASE VAULT
**Shape:** Small circular room at the very base of the central shaft. The blue glow seen from above emanates from here — a sealed star-crystal set into the floor, still active after centuries.
**Size:** Small
**Connections:** Up → R10 (main ledge — Portal Tool to ascend safely, or damage-fall to descend)

**Contents:**
- ⚔️ **None** — the star-crystal's light keeps enemies away. The room is serene.
- [★] **Gold Pouch (75 coins)** — sealed in a crystal housing beside the star-crystal. Hammer to break the housing.
- [★] **Rare Healing Herb** — full HP restore, on a shelf in the curved wall.
- [★] **Astrolabe** — collectible navigation tool. Flavor text: *"A working astrolabe calibrated to the Spire's star-charts. When used in R17 alongside the Celestial Lens, it reduces the star-alignment puzzle to a single correct position immediately — auto-solve of the puzzle. Requires both items."* Optional complete puzzle bypass for players who thoroughly explore.
- [?] **Builder's Inscription** (carved into the floor around the star-crystal): *"The first stone placed. The star-crystal was here before we arrived. We built around it, as one does. It has been glowing since before the dragon was sealed. We believe it will still be glowing after the seal is restored. Or broken. It does not seem to care which."*
- [🌀] **Portal surface** on the north wall — the matching glyph is on the main ledge of R10 above. If the player placed a portal on the ledge before falling/descending, they can portal back up. If they did not, they must climb (Ladder rungs on the shaft wall from base to first platform — tedious but possible).

**Enemies:** None.
**Secrets:** None — this IS the sub-chamber.

---

### R11 — ASTRAL HALL
**Shape:** Wide horizontal rectangle. The Spire's observatory floor — the entire south wall is star-glass, floor to ceiling, looking out at open space. Star-maps cover the floor in inlaid gold lines. Three large orrery models (mechanical star-system representations, slowly rotating) fill the room's center.
**Size:** Large
**Connections:** South → R09 (open) | North → R14 (open) | East → R10 (open — connects to shaft main ledge level)

**Contents:**
- ⚔️ **2× Star Sentinel** + **1× Void Wraith** — the Wraith circles the orrery models, using them as cover.
- **Orreries** — three mechanical star-system models, each 2 tiles wide, slowly rotating. They block movement and line-of-sight. The Slag Golem equivalent of this dungeon — large environmental obstacles. Can be pushed (Strong Arm Glove) to reposition. Pushing one orrery into the Void Wraith deals stagger damage.
- [!] **Pressure plate** (under the central orrery) — the orrery must be moved (Glove) to reveal and activate the plate. Activating it opens a wall panel on the north wall: **Small Key F** inside.
- [★] **Small Key F** — north wall panel, revealed by pressure plate.
- [★] **Gold Pouch (40 coins)** — on the east side, near the R10 connection.
- [?] **Lore Tablet** (east wall): *"The orreries model the star-system as it was at the moment of the dragon's sealing. The seventh planet from the center is dark — not present. That is where the sealing happened. The Spire points at that dark planet. We built an entire tower to point at an absence."*
- [🌀] **Portal surface pair** (north and south walls) — creates a shortcut across the full length of the hall. Useful for retreating quickly if overwhelmed.

**Enemies:** 2× Star Sentinel, 1× Void Wraith
**Secrets:** None.

---

### R14 — GRAVITY CHAMBER
**Shape:** Square. A chamber where gravity has been partially inverted — the south half of the room has normal gravity, the north half has gravity pointing upward (toward the ceiling). The boundary between them is a visible shimmering line across the floor. Crossing it inverts the player's movement — what was floor becomes ceiling, walking now requires navigating the ceiling of the north half (which is the structural floor of R15 above).
**Size:** Large
**Connections:** South → R11 (open — normal gravity half) | Up (north gravity half ceiling) → R15 (open — ceiling of north half IS the floor of R15, player walks through into R15 normally) | Down (special) → R14b *(inverted vault — accessible from the ceiling of the inverted zone)*

**Contents:**
- ⚔️ **2× Star Sentinel** (normal half) + **1× Sky Phantom** (inverted half — floats comfortably in either gravity state).
- ▽▽▽ **Gravity inversion zone** (north half) — crossing the shimmering boundary inverts gravity for the player. Movement controls remain the same (forward is still forward) but up and down are reversed. Items on the "floor" of the inverted half are on what was the ceiling — now at normal reach. First gravity inversion introduction.
- [★] **Small Key G** — in the inverted half, on what appears to be the ceiling (actually the floor in inverted gravity). Straightforward retrieve once inverted.
- [🌀] **Portal surface** (normal half south wall) — paired glyph on the inverted half "ceiling" (structural floor of R15). Portal placed here allows crossing the gravity boundary without experiencing the inversion — stepping into the portal in normal gravity, stepping out in R15 in normal gravity. Optional bypass of the inversion.
- [?] **Lore Tablet** (normal half, near boundary): *"The gravity chamber was an accident. The star-crystal resonance from the shaft below interfered with the room's structural enchantments during construction. We discovered the inversion when the chief architect walked into the north half and didn't come back down. He was fine. He was very confused. We decided to keep it."*
- [🔨] **Cracked ceiling panel** (inverted half — accessible only when inverted, as it is now at floor level) — Hammer reveals a passage downward (in inverted orientation: a passage toward what feels like up). Leads to R14b.

**Enemies:** 2× Star Sentinel, 1× Sky Phantom
**Secrets:**
- 🔍 Cracked ceiling panel (inverted half) — Hammer from below (inverted) reveals R14b.

---

### R14b — SUB-CHAMBER: INVERTED VAULT
**Shape:** Small square. Entirely in inverted gravity — the player walks on what is structurally the ceiling of a chamber below R14's normal half. A sealed vault space, accessible only by the gravity inversion route. Contains items placed upside-down — they sit on the structural ceiling (the inverted floor) naturally.
**Size:** Small
**Connections:** Up (inverted — structurally down) → R14 (cracked panel, Hammer used)

**Contents:**
- ⚔️ **1× Mirror Shade** *(carried from Level 6 — drifted in through a rift in the inverted space)*.
- [★] **Gold Pouch (70 coins)** — in a sealed box on the inverted floor.
- [★] **Healing Herb ×3** — resting naturally on the inverted floor (they were placed here by someone who entered in inverted gravity).
- [★] **📜 Lore Scroll — "The Dragonbinder's Seventh Entry — Addendum"** — Reads: *"I found the vault below the gravity room. I am writing this upside-down relative to how you will read it, unless you are also upside-down, in which case: hello. The piece is above me. Or below. Direction has become complicated. I know I am close. I can feel all seven pieces pulling toward the same point. Including the ones I am carrying. It is like being a magnet surrounded by other magnets. Everything wants to converge."*
- [?] **Scratched wall text** (inverted floor — the structural ceiling, at foot-level when inverted): *"Vault 14-B. Contents: classified. Access: theoretically impossible. If you are reading this: hello, theoretically impossible person."*

**Enemies:** 1× Mirror Shade
**Secrets:** None — this IS the sub-chamber.

---

### R15 — PORTAL GAUNTLET
**Shape:** Very large rectangle. The dungeon's primary Portal Tool challenge room — a series of floating platforms at different heights, connected only by portal glyphs. No Ladder rungs, no climbable chains. The room is entirely vertical traversal via Portal Tool. Platforms range from 1 to 6 tiles above the entry floor. The highest platform has the north exit to R17.
**Size:** Very Large
**Connections:** South → R14 (open — gravity chamber ceiling exit, normal gravity restored) | North (highest platform) → R17 (open) | East (mid platform) → R16 *(sky platform — open sky exit)*  | Down (floor level, portal only) → R15b *(sub-chamber — portal shortcut only)*

**Contents:**
- ⚔️ **3× Sky Phantom** — occupy the mid-height platforms. Must be fought or dodged during vertical traversal.
- **Floating platform network** — 8 platforms at varying heights. Each platform has a glyph on its surface (floor-facing) and a paired glyph on the nearest wall or adjacent platform. The player must place Portal Tool portals to traverse the height differences.
  - Platforms 1–3: 2-tile height differences — Ladder can manage these if player prefers.
  - Platforms 4–6: 4-tile height differences — Portal Tool mandatory.
  - Platforms 7–8: 6-tile height difference — Portal Tool mandatory, and the angle required means placing the exit portal on the underside of the upper platform (a ceiling portal). First ceiling portal placement in the dungeon.
- [🌀] **Ceiling portal glyph** (underside of platform 8) — the most technically demanding portal placement. Player must approach platform 7, aim up at platform 8's underside, place exit portal there, then place entry portal on platform 7's floor, and step through — exiting upward onto platform 8's upper surface. Novel and satisfying when executed.
- [★] **Small Key H** — on platform 5 (mid-height). Standard retrieve once there.
- [★] **Gold Pouch (50 coins)** — on platform 7, near the ceiling-portal challenge.
- [?] **Lore Tablet** (entry floor, south wall): *"The portal gauntlet was designed to test those who carry the Portal Tool. If you do not have the Portal Tool, the gauntlet is impassable. If you do: the gauntlet is a conversation between you and the Spire. The Spire asks a question with each platform. The Portal Tool is your answer."*
- [🌀] **Floor portal glyph** (entry floor level) — paired with a glyph visible in R15b below. Placing a portal on this glyph and the matching glyph in R15b creates a direct floor-level connection to the sub-chamber.

**Enemies:** 3× Sky Phantom
**Secrets:**
- 🔍 Floor portal glyph — Portal Tool to connect to R15b sub-chamber below.

---

### R15b — SUB-CHAMBER: MID-SHAFT PLATFORM
**Shape:** Small platform — an isolated floating disc of stone, suspended in the shaft below R15. Accessible only via the portal shortcut from R15's floor.
**Size:** Very Small
**Connections:** Up → R15 (portal only — glyph on platform surface pairs with R15 floor glyph)

**Contents:**
- ⚔️ **None** — isolated, never reached before.
- [★] **Gold Pouch (60 coins)** — on the platform edge. The only thing here.
- [★] **Rare Healing Herb** — full HP, placed in the center.
- [★] **Star Map Fragment** — collectible item. Flavor text: *"A fragment of the star-chart used to construct the Spire. The chart shows the exact position of the dragon's constellation at the moment of sealing. One star marked in red: the seventh — the one that went dark. A note beside it in old ink: 'It is not gone. It is waiting.'"*
- The platform has a spectacular view — the shaft above and below, the starfield through the open top of the Spire, the base camp far beneath. One of the game's best atmospheric vantage points.

**Enemies:** None.
**Secrets:** None — this IS the sub-chamber.

---

### R16 — SKY PLATFORM
**Shape:** External floating platform, accessible from R15's mid-height east exit. Completely open air — wind, stars, and the vertiginous view of open sky in every direction. The Spire's exterior wall is at the west side; everything else is open space.
**Size:** Medium (open platform)
**Connections:** West → R15 (open — mid-height platform exit)

**Contents:**
- ⚔️ **2× Sky Phantom** — combat is high-risk here (open edges, no railings, strong wind).
- ~~~ **Wind current** — strong east-to-west gusts every 6 seconds. Player must brace or be repositioned toward the platform edge.
- [★] **Gold Pouch (55 coins)** — near the platform's east edge. Collecting it puts the player at the edge during a potential wind gust.
- [★] **Healing Herb ×2** — center of platform, safe zone.
- [★] **Celestial Lens** — *if not already collected from R02's cracked crate* — a second instance is here, on a stone pedestal at the platform's north end. The designers anticipated some players would miss the R02 crate. Only one Lens is usable but having a backup ensures the star-alignment puzzle has a skip option for all players who explore.
- [?] **Lore Tablet** (west wall, platform entry): *"The sky platform was added at the request of the Spire's final builder, who wanted somewhere to stand that was not inside anything. She stood here for three days before returning to work. She said it helped. We believe her."*
- The view from R16 is the game's highest vantage point accessible before the boss. All prior dungeons visible simultaneously: the forge's smoke, the coastal citadel's tower, the fractured sanctum's dimensional shimmer in the landscape, the mosshaven forest, the ruined Ashenfall tower. The entire journey in one vista.

**Enemies:** 2× Sky Phantom
**Secrets:** None — the view is the reward.

---

### R17 — STAR ALIGNMENT CHAMBER
**Shape:** Large octagon. The Spire's apex chamber — just below the final platform and boss room. The ceiling is a perfect dome of star-glass, showing the actual night sky above in real time (the stars visible through it shift slowly as the world turns). In the center of the floor: a star-alignment mechanism — a large circular device with 7 rotating star-rings, each set to a different celestial position.
**Size:** Very Large
**Connections:** South → R15 (open — top platform exit) | North → R20 *(BOSS DOOR — requires Boss Key)* | West hidden portal-rift → R18a *(SECRET)* | East hidden portal-rift → R18b *(SECRET)*

**Contents:**
- ⚔️ **2× Star Sentinel** + **1× Void Wraith** + **1× Shadow Boss** *(carried from Level 6 — the hardest pre-boss encounter in the dungeon)*.
- **Star Alignment Puzzle** — the central mechanism has 7 rotating rings, each representing one of the 7 stars in the dragon's constellation. The player must set each ring to the correct angular position — indicated by the star-chart projections on the dome ceiling (the real star positions overhead, visible through the star-glass). The correct positions are the constellation positions at the exact moment of the dragon's sealing — which the star-charts throughout the dungeon have been documenting.
  - Without tools: the player must study the dome ceiling, compare to the ring positions, and adjust each ring manually (7 rings × 3 possible positions = 21 combinations — manageable with observation).
  - With **Celestial Lens** (from R02 or R16): holding it up to the dome projects the exact correct positions onto the mechanism floor — visual guide that makes adjustment trivial.
  - With **Astrolabe** (from R10b) AND Celestial Lens: the mechanism auto-solves entirely — all 7 rings click into position simultaneously.
  - When all 7 rings are correct: the dome brightens, a star-beam descends through the glass onto the mechanism center — **Boss Key** rises from the mechanism on a stone plinth.
- [★] **Boss Key** — rises on plinth after puzzle solved.
- [?] **Lore Tablet** (north wall, beside the Boss Door): *"Aurelion was the Spire's last guardian. Not a construct, not a soldier — it is a constellation given form. When the dragon's piece arrived here, the seventh star's light bent downward and became Aurelion. It has been here ever since, circling the piece in the pattern of the original constellation. It does not know it is a prison. It believes it is a dance."*
- **West hidden portal-rift → R18a:** A faint portal shimmer on the west wall — visible only when the Portal Tool is equipped and pointed at it. Placing a portal on it and stepping through opens R18a. Requires Portal Tool awareness.
- **East hidden portal-rift → R18b:** Same mechanic on the east wall. Portal Tool required to activate and step through.

**Enemies:** 2× Star Sentinel, 1× Void Wraith, 1× Shadow Boss
**Secrets:**
- 🔍 **West hidden portal-rift → R18a:** Portal Tool equipped — shimmer visible, place portal to open passage.
- 🔍 **East hidden portal-rift → R18b:** Same mechanic, east wall.

---

### R18a — SECRET ROOM: THE SEVENTH STAR *(West)*
**Shape:** Small circular room. The walls glow with the same star-crystal light as R10b's base vault. The ceiling shows a single star — brighter than any other, perfectly centered. This is the seventh star. The one that went dark during the sealing.
**Size:** Small
**Connections:** East → R17 (portal-rift — Portal Tool required)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **📜 Lore Scroll — "The Dragonbinder's Seventh Entry — Final Note"** — Reads: *"I am leaving this in the seventh star's chamber because I believe someone should know: the dragon was not a monster. Every piece I have gathered has shown me something of what it was. The tail that anchored it to the earth. The legs that carried it with patience. The wings that knew both freedom and restraint. The head that saw everything. It was a thinking creature. It made a choice to be sealed. It agreed to this. I have been unsealing something that chose its own fate. I need to think about whether I have the right to finish what I started. I will keep going. I think it wants me to. But I am not certain. And I think that uncertainty is the most honest thing I have written in seven dungeons."*
- [★] **Rare Healing Herb** — full HP.
- [★] **Gold Pouch (100 coins)** — highest single gold reward in the entire dungeon series so far. In a crystal housing on the floor, cracked open with a Hammer strike.
- The seventh star in the ceiling pulses once when the player enters, then stays steady. It has been waiting for someone to come here and see it.
- Mira's drawing — age 9. The dragon is complete, whole, and luminous. Below it, in careful child's handwriting: *"I think it's lonely."* No signature this time.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R18b — SECRET ROOM: THE CONSTELLATION CACHE *(East)*
**Shape:** Small irregular space — carved by light rather than tools. The walls have no right angles.
**Size:** Small
**Connections:** West → R17 (portal-rift — Portal Tool required)
**Does NOT appear on dungeon map.**

**Contents:**
- [★] **Gold Pouch (75 coins)** — on a light-formed shelf.
- [★] **Healing Herb ×3** — floating in a gentle light-current near the ceiling.
- [★] **Celestial Merchant's Note** — readable item. Reads: *"I've been selling Star Sigils for three hundred years. You're the first person who ever bought one and made it this far. Most turn back at the Sky Moat. A few made it to the gravity room. One made it to the portal gauntlet and decided they had proved their point. You're here. Whatever happens next — you earned it. — The Merchant. P.S. If you see Aurelion, tell it the seventh star is still burning. It will know what that means."*
- The note from the Merchant is the first time anyone outside the dungeons has acknowledged the player's journey directly. Humanising in an unexpected way.

**Enemies:** None.
**Secrets:** None within — this IS the secret.

---

### R20 — BOSS CHAMBER: THE APEX PLATFORM
**Shape:** Enormous circular platform at the very top of the Spire — open to the sky on all sides. No walls. No ceiling. Stars in every direction. The platform is 10 tiles in diameter. The Dragon Piece — the Head and Horns — floats in the center inside a slow-rotating constellation of light-formed stars (Aurelion's body, the constellation made physical). The Boss Door emerges into the platform from a stone column at the south edge — the top of the Spire's spine.
**Size:** Very Large — the most open room in the game.
**Connections:** South → R17 *(BOSS DOOR — Boss Key required)*

**Contents:**
- 🔒 **BOSS DOOR** — south stone column, top of the Spire. Boss Key required. The door is a hatch in the column's top, opening upward. The player climbs out into open sky. There is a moment — just a moment — where they stand at the top of the tallest thing they have ever climbed, with nothing above them and everything below them, before the fight begins.
- **BOSS: Aurelion, the Constellation** — a being of pure star-light, vast and non-humanoid. Its form is the dragon's head constellation made physical — seven stars connected by lines of light, constantly moving in the constellation's pattern. It does not attack maliciously; it is fulfilling a pattern it has followed for centuries. It does not understand that the player needs to break the pattern.
- [★] **🧩 Dragon's Head & Horns (Piece 7/8)** — released from the constellation's center on boss defeat. It drops slowly, caught by the platform's edge glow, and rests there waiting to be picked up.
- The platform has no railings, no obstacles, no cover — the fight is pure, open-sky combat. Falling off the platform edge triggers the standard recovery mechanic (repositioned at the platform center — no damage, but the repositioning takes 3 seconds during which Aurelion continues its pattern, potentially damaging the player on reentry).
- [🌀] **Portal surfaces** — six glyphs evenly spaced around the platform edge. The portal mechanic is critical in this fight: Aurelion teleports between these six glyph points. A portal placed between two adjacent glyphs can intercept Aurelion's teleport — catching it mid-transit and staggering it. The fight explicitly requires portal interception to deal damage efficiently.
- The stars visible from the platform are not background decoration — during Phase 3, the real sky responds to Aurelion's damage. Stars visibly dim as Aurelion weakens. When it is defeated, the seventh star — the one that went dark — briefly reilluminates before fading again. A visual callback to every piece of lore the player has read.

**Boss — Aurelion, the Constellation:**
- **Phase 1 (100%–65% HP):** Aurelion traces its constellation pattern — moving between the 6 platform glyphs in a fixed sequence, pausing 2 seconds at each point before teleporting to the next. Attacks: star-lance (beam from current position toward player, dodge perpendicular), light-arc (sweeps along the platform edge, roll inward to avoid). Vulnerable only during the 2-second pause at each glyph point. Portal interception: player places portal between two adjacent glyph points — Aurelion's teleport enters the portal and exits at the player's chosen location, staggered for 3 seconds, fully vulnerable. Portal interception is the most efficient damage window by far.
- **Phase 2 (65%–35% HP):** Glyph sequence becomes irregular — no longer the same fixed order. Player must predict or react to which glyph Aurelion will teleport to next and pre-position the interception portal. Adds a nova attack (expands from current position in all directions, dodge to the platform edges). Adds a star-fall (random platform positions marked by incoming light — dodge off the marked tiles).
- **Phase 3 (35%–0% HP):** Aurelion splits into 3 partial constellation-forms simultaneously — each occupying 2 of the 6 glyphs. Only one is real (the one with the brightest star-glow — Void Compass from Level 6 can confirm, as it still tracks void-adjacency). Portal interception must now target the correct form. The other two forms deal normal damage but take none. Star-fall intensifies (4 simultaneous strikes). Final phase ends when the correct form is intercepted and struck 3 times — Aurelion contracts, the constellation implodes inward, all light extinguishes simultaneously, and the platform is dark for 3 seconds. Then: every star in the visible sky brightens at once. The Dragon Piece falls. The seventh star reilluminates, briefly. Then fades.
- **Defeat:** Silence. The platform at the top of the Spire, open sky in every direction. The Dragon Piece rests on the stone. The player stands alone at the highest point they have reached. Below them: seven dungeons, seven pieces already gathered, countless rooms explored, every tool earned. Above them: one dark star.

**Secrets:** None — boss rooms are always clean and readable.

---

## DUNGEON FLOW SUMMARY

```
CRITICAL PATH:
R01 (Star Sigil inserted, map) → R03 (fight) → R04 (Sky Moat — Raft crossing up the neck)
→ R06 (fight, Lore Scroll) → R05 (Portal Tool — bridge gap, get Key B)
→ R09 (wind gallery — Portal Tool or platform jump) → R10 (shaft ascent — Portal + Ladder)
→ R14 (left horn gravity branch) → R15 (portal gauntlet at the horn split)
→ R17 (star alignment puzzle at the crown, get Boss Key)
→ R20 (boss, get Piece 7)

KEY LOCATIONS:
  Small Key A — R08 (outer rampart south end bracket)
  Small Key B — R05 (north half, behind Sentinel)
  Small Key C — R07 (mooring dock, behind Sky Phantom patrol)
  Small Key D — R09 (third platform, wind gallery)
  Small Key E — R10 (shaft platform 6 of 12)
  Small Key F — R11 (north wall panel, orrery pressure plate)
  Small Key G — R14 (inverted half, on inverted floor)
  Small Key H — R15 (platform 5, portal gauntlet)
  Boss Key    — R17 (star alignment mechanism plinth)

OPTIONAL PATHS:
  R01 → R02 (base camp — major healing cache, Moon Essence, cracked crate)
  R01 → R08 (outer rampart — Key A, Hammer shortcut into R06)
  R04 → R08 east gate (Key A from R08 — non-linear key dependency)
  R04 → Portal glyph pair (permanent Sky Moat portal shortcut)
  R07 → R09 (Key C — alternate left-branch approach to the wind gallery)
  R10 → R10b (fall or Portal — star-crystal vault, Astrolabe)
  R14 → R14b (Hammer inverted ceiling — inverted vault, gold + lore)
  R15 → R15b (floor portal glyph — mid-shaft platform, gold + Star Map)
  R15 → R16 (right horn sky platform — Celestial Lens backup, view, gold)
  R17 → R18a (west portal-rift — seventh star chamber, critical lore)
  R17 → R18b (east portal-rift — constellation cache, Merchant's note)

PUZZLE SKIP OPTIONS:
  Celestial Lens (R02 crate OR R16 pedestal) → visual guide for star alignment
  Astrolabe (R10b) + Celestial Lens → auto-solve star alignment
  Moon Essence (R02) → extends Portal Tool range for gauntlet and boss

ALL SIX TOOLS USED:
  Ladder    → R08 rampart (Key A route), R10 shaft (adjacent platforms)
  Hammer    → R02 crate, R05 south wall, R08 Spire wall, R09 alcove, R14 inverted ceiling
  Raft      → R04 Sky Moat crossing
  Glove     → R11 orrery push, R14 inverted half items
  Void Compass → R06 (Wraith tracking), R17 (Shadow Boss), R20 (real Aurelion in Phase 3)
  Portal Tool → R04 (optional shortcut), R05 (mandatory gap), R09 (wind bypass),
                R10 (shaft traversal), R15 (gauntlet + ceiling portal),
                R17 (secrets), R20 (boss interception — core mechanic)

BLOCKED: None — all tools available, no Level 8 tool blocks until Level 8
```

---

## FULL ITEM & TOOL CHECKLIST

| Item | Location | Required? |
|------|----------|-----------|
| 🗺️ Dungeon Map | R01 — star-chart table | Auto-collected |
| ⭐ Star Sigil | World — Celestial Merchant, night only | Yes (opens Spire entry) |
| 🛶 Raft | Prior dungeon reward | Yes (Sky Moat crossing) |
| 🧭 Compass | R03 — wall bracket | Optional |
| 🌙 Moon Essence | R02 — supply crate | Optional (extends Portal range) |
| 🔭 Astrolabe | R10b — star-crystal vault | Optional (star puzzle auto-solve with Lens) |
| 🔭 Celestial Lens | R02 (cracked crate, Hammer) OR R16 (pedestal) | Optional (star puzzle guide / skip) |
| 🗝️ Small Key A | R08 — rampart south bracket | Optional (opens R04 east gate) |
| 🗝️ Small Key B | R05 — north half, behind Sentinel | Yes (opens R09 from R05) |
| 🗝️ Small Key C | R07 — mooring dock | Optional (opens R09 from R07) |
| 🗝️ Small Key D | R09 — third platform | Optional (opens R10 west approach) |
| 🗝️ Small Key E | R10 — shaft platform 6 | Optional (opens R11 east approach) |
| 🗝️ Small Key F | R11 — orrery pressure plate panel | Yes (opens R14) |
| 🗝️ Small Key G | R14 — inverted half floor | Yes (opens R15) |
| 🗝️ Small Key H | R15 — platform 5 | Yes (opens R17) |
| 🔑 Boss Key | R17 — star alignment mechanism | Yes (opens R20) |
| 📜 Lore Scroll (Dragonbinder 7) | R06 — central pillar alcove | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 7 Addendum) | R14b — inverted vault | Optional (collectible) |
| 📜 Lore Scroll (Dragonbinder 7 Final Note) | R18a — seventh star chamber | Optional (critical lore) |
| 📋 Star Map Fragment | R15b — mid-shaft platform | Optional (lore item) |
| 📋 Celestial Merchant's Note | R18b — constellation cache | Optional (lore item) |
| 🧩 Piece 7/8 | R20 — platform after boss | Main Objective |
| 💰 Gold (total ~800) | Throughout | Optional |
| 🌿 Healing Herbs | R02, R05, R07, R09, R10b, R14b, R15b, R16, R18a, R18b | Optional |

---
---

*[Level VIII — The Dragon's Eternal Throne will be added to a new document upon completion]*
