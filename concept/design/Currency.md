# 🐉 THE DRAGON SEAL — Currency System
*Gems · Three Denominations · Ground Scatter + Enemy Drops · Auto-Collect*

---

## THE THREE GEMS

The world of The Dragon Seal uses **dragonstones** — fragments of crystallised energy left behind by the sealing event. They come in three varieties, distinguished by colour and size. Every merchant in the game prices in the same system. Every dungeon, every biome, every enemy uses the same three gems.

---

### 🟤 SHARD — Value: 1
**Full name:** Earthshard
**Colour:** Warm brown with a faint amber inner glow — the colour of old wood or dried clay. Small, irregular, fits easily between finger and thumb.
**Description:** The most common dragonshard. Found in shallow soil, loose stone, and ordinary places throughout the world. The sealing event scattered millions of these — they worked their way into the ground over centuries and have been turning up in fields, riverbeds, and dungeon floors ever since. Locals use them as small change. Children collect them.
**In-world logic:** The sealing fractured the dragon's energy into the earth. Earthshards are the smallest such fragments — too small to be remarkable, too numerous to be rare. They are the world's way of saying the dragon was here.
**Visual on ground:** A small brown glint, barely distinguishable from gravel. Clusters of 3–8 appear together. Easy to walk past at speed; obvious when looking.
**Enemy drop:** Small enemies (Bogwolves, Thornvine Crawlers, Forge Hounds) drop 1–3 Earthshards.

---

### 🔵 SHARD — Value: 5
**Full name:** Tideshard
**Colour:** Deep ocean blue with a slow internal pulse — like light through water. Medium sized, smooth-faced, slightly translucent.
**Description:** Tideshards formed where the sealing energy met underground water — rivers, springs, coastal aquifers. The water shaped them into smooth, rounded forms over centuries. They are more deliberately collected than Earthshards — merchants prize them, travellers seek them, and dungeons near water reliably contain them.
**In-world logic:** The dragon's energy interacted differently with different materials. Water slowed the crystallisation, producing larger, calmer stones. The blue colour is the trapped memory of movement — the water the shard formed inside is still visible as the internal pulse.
**Visual on ground:** A steady blue glow, clearly visible in dim dungeon lighting. Usually found in clusters of 2–4. Unmistakable once the player knows what to look for.
**Enemy drop:** Medium enemies (Ruin Golems, Slag Golems, Star Sentinels) drop 1–2 Tideshards. Bosses always drop a handful on defeat.

---

### 🔴 SHARD — Value: 25
**Full name:** Hearthshard
**Colour:** Deep crimson with a warm, steady inner light — not a flicker, not a pulse, just a constant warmth, like a coal that will never go out. Large, faceted naturally, heavy for its size.
**Description:** The rarest dragonstone. Hearthshards formed at the sealing's epicentre — the Dragon's Throne — and radiated outward through the deepest rock. They are found far underground, in the hearts of dungeons, in places that were closest to the dragon at the moment of sealing. They do not form in the overworld. Every Hearthshard came from deep inside something.
**In-world logic:** The heart of the sealing produced the most concentrated energy. Hearthshards carry the most of it. They are warm to the touch — not hot, just warm, like a sleeping animal. Scholars debate whether they are currency or relics. Merchants do not debate this.
**Visual on ground:** A strong red glow visible from several tiles away. Never in clusters larger than 2. Often found alone — a single Hearthshard in a sealed chest or a dark corner, clearly significant.
**Enemy drop:** Major enemies and dungeon bosses only. Each boss drops 2–4 Hearthshards on defeat, guaranteed.

---

## VISUAL IDENTITY SUMMARY

| Gem | Name | Value | Colour | Size | Glow | Found |
|-----|------|-------|--------|------|------|-------|
| 🟤 | Earthshard | 1 | Warm brown / amber | Small, irregular | Faint flicker | Ground scatter, small enemies |
| 🔵 | Tideshard | 5 | Ocean blue | Medium, smooth | Slow pulse | Ground scatter, medium enemies |
| 🔴 | Hearthshard | 25 | Deep crimson | Large, faceted | Constant warm glow | Dungeon depths, bosses |

---

## COLLECTION BEHAVIOUR

**Auto-collect on contact** — walking over a gem picks it up immediately. No button press, no prompt. The gem rises slightly as the player approaches (within 1 tile) and snaps to the player on contact. The animation is quick and satisfying — a soft chime sound per gem, a brief counter increment on the HUD.

**HUD display:** A small gem icon beside the total. The total is shown as a single number (total value, not broken into denominations) — e.g. *"47"* not *"22 Earthshard, 4 Tidehard, 0 Hearthshard."* The breakdown is visible in the inventory screen.

**Carrying limit:** None. The player can hold as many gems as they find.

**Dropping gems:** The player never drops gems on death (no penalty system). Gems are permanent once collected.

---

## GROUND SCATTER RULES

**Dungeon rooms:**
- Standard rooms: 2–8 Earthshards scattered near enemy patrol areas or in corners
- Secret rooms: always contain at least one Tideshard cluster; usually one Hearthshard
- Boss rooms: no gems on the floor — all gems from a boss come from the boss drop on defeat
- Sub-chambers: one guaranteed Tideshard cluster + variable Earthshards

**Overworld biomes:**
- Gems scatter naturally across all biomes, denser in areas the player has not visited
- Earthshards are found everywhere — in grass, beside rocks, in shallow water
- Tideshards appear near water sources (streams, the Sky Moat, the Sunken Coast pools)
- Hearthshards do not appear in the overworld — underground only

**Density by dungeon tier:**
- Level I–II: Earthshards only on the ground. Tideshards from enemies.
- Level III–V: Earthshards + occasional Tideshards on the ground. Hearthshards from bosses.
- Level VI–VIII: All three on the ground. Hearthshards in secret rooms. Bosses drop Hearthshards guaranteed.

---

## ENEMY DROP TABLES

| Enemy Type | Earthshard | Tideshard | Hearthshard |
|-----------|-----------|----------|------------|
| Small (Bogwolf, Crawler, Forge Hound) | 1–3 | — | — |
| Medium (Golem, Sentinel, Warden) | 2–4 | 0–1 | — |
| Large / Elite (Void Wraith, Shadow Boss) | 3–5 | 1–2 | — |
| Dungeon Boss | 5–10 | 3–5 | 2–4 |
| Wandering Shade (Biome 6 world item carrier) | 8 | 4 | 1 |

---

## ECONOMY CALIBRATION

All shop prices from the item progression document, converted to gem denominations:

| Price | Earthshards | Tideshards | Hearthshards | Breakdown |
|-------|-----------|----------|------------|-----------|
| 50 | 50 | 10 | 2 | Two Hearthshards exactly |
| 75 | 75 | 15 | 3 | Three Hearthshards |
| 90 | 90 | 18 | 3+3 | Three Hearthshards + three Tideshards |
| 130 | 130 | 26 | 5+1 | Five Hearthshards + one Tidshard |
| 200 | 200 | 40 | 8 | Eight Hearthshards exactly |
| 280 | 280 | 56 | 11+1 | Eleven Hearthshards + one Tidshard |
| 300 | 300 | 60 | 12 | Twelve Hearthshards exactly |
| 350 | 350 | 70 | 14 | Fourteen Hearthshards |

**Economy feel:** At the tight 1/5/25 scale, finding a single Hearthshard feels significant. A boss fight rewarding 3 Hearthshards (75 value) is a meaningful chunk toward a 200-coin purchase. The player will spend time in dungeons accumulating currency — nothing is trivially affordable, nothing is impossibly expensive.

**Approximate per-dungeon gem income** (full clear including secrets):
- Level I: ~80–100 value
- Level II: ~120–150 value
- Level III: ~180–220 value
- Level IV: ~200–250 value
- Level V: ~250–300 value
- Level VI: ~300–380 value
- Level VII: ~380–450 value
- Level VIII: ~450–550 value

A player who clears all eight dungeons thoroughly will have accumulated roughly 2,000–2,400 total value — enough to buy most shop items encountered along the way, with meaningful choices about what to prioritise.

---

## GEM LORE

Each gem type has a short entry in the Dragon Codex (Page 1), written in the Dragonbinder's hand:

**On Earthshards:** *"They are everywhere. I keep finding them in my boot. The locals use them to pay for bread. I find this remarkable — they are literally carrying pieces of the dragon's sealing event to the bakery. No one finds this strange. I find it very strange."*

**On Tideshards:** *"The blue ones pulse. I watched one for an hour once, trying to determine if the pulse was regular. It is not quite regular. It is close to regular. Like breathing."*

**On Hearthshards:** *"I found my first Hearthshard in the second dungeon, in the deepest room, behind a wall I almost didn't break. It was alone. It was warm. I held it for a long time before I put it in my pocket. It felt wrong to spend it. I spent it anyway. The merchant didn't comment on the temperature."*

---

*[The Dragon Seal — Currency System — COMPLETE]*
