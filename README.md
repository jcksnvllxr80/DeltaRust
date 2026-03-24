# DeltaRust

Native Rust version of the Delta game, kept separate from the original browser implementation in `Delta/`.

## Run

```sh
cargo run
```

## Character Customization

Before starting a new game, the character creator lets you build your hero's appearance. Use the arrow keys to cycle through each field.

**Gender** is the first option and acts as a preset:

- **Male** — default color palette (green shirt, brown pants, grey boots)
- **Female** — pink clothing palette and long hair style applied automatically

All other fields (hair style, hair color, shirt, pants, boots) can be adjusted independently after selecting a gender, so the preset is just a starting point.

## Day / Night Cycle

Time passes automatically at one in-game minute per 60 frames (roughly one full day every 24 real minutes at default speed). The world reacts to the current time:

| Time | Atmosphere |
|---|---|
| 12:00 AM – 5:00 AM | Night — dark blue overlay |
| 5:00 – 6:30 AM | Pre-dawn — fading night |
| 6:30 – 8:30 AM | Sunrise — warm orange tint |
| 8:30 AM – 5:30 PM | Day — no overlay, full brightness |
| 5:30 – 6:30 PM | Sunset — orange-red tint |
| 6:30 – 8:00 PM | Dusk — purple haze |
| 8:00 – 10:00 PM | Evening — darkening |
| 10:00 PM – 12:00 AM | Night |

The night overlay darkness depends on the **moon phase**, which runs on an 8-day cycle:

- **Full moon** (days 0–2) — lighter night, more visibility
- **Half moon** (days 3–4) — moderate darkness
- **New moon** (days 5–7) — darkest nights

Dungeons and interiors are unaffected by the cycle and always render at full brightness.

### HUD

The current time is displayed centered at the bottom of the HUD (e.g. `6:00 AM`).

A **celestial chip** in the HUD chip row shows the sun or moon:

- **Sun** — visible during daytime; appears orange-gold near dawn and dusk, bright yellow midday
- **Moon** — visible at night; the circle shape reflects the current moon phase (full circle, gibbous, half, crescent, or dark on a new moon)

### Per-dungeon boss keys

Each dungeon has its own boss key. Collecting a boss key in dungeon 3 does not carry over to dungeon 4. The inventory shows each dungeon's boss key as **Boss Key 1** through **Boss Key 8**, and the HUD chip highlights only the key for the dungeon you are currently in.

## Gnomes

### Nip (Wandering Gnome)

A small gnome named **Nip** spawns on each overworld screen. Nip wanders aimlessly but flees when the player gets close. If you catch Nip (walk into them), you gain **+5 HP**.

Nip is purely overworld — does not appear in dungeons or interiors, and a fresh Nip spawns each time you enter a screen.

### Pip (Healer Gnomes)

Four sleepy Pips are seated in fixed spots across the overworld, one per quadrant. Press `Z` / `Enter` while facing one to interact. Pip will say a kind word and **fully restore your HP**. Can be revisited as many times as needed.

## In-game console

During gameplay, press `~` to open the console at the bottom of the screen.

- `help` — lists all supported commands
- `teleport x,y` — moves you to an overworld screen coordinate (e.g. `teleport 3,4`)
- `god_mode 0/1` — disables or enables player invulnerability
- `get_item <item_name>` — adds an item to your inventory
- `set_time HH:MM` — sets the clock to a specific time (e.g. `set_time 18:30`)
- `time_speed <n>` — sets the clock speed multiplier; `time_speed 4` runs time 4× faster. Always multiplies the base rate — calling it again sets a new multiplier rather than stacking
- `show_map_poi` — toggles map points-of-interest markers (same effect as `--dev-mode`)

Press `Enter` to execute, `Esc` or `~` to close.

### get_item

`get_item <item_name>` grants the specified item. If you already own it, the command tells you so. Some items have special behavior:

| Item | Notes |
|------|-------|
| `sword` | Auto-equips to Main slot |
| `bombs` | Auto-equips to Side slot; if already owned, refills ammo to max |
| `hammer` | Auto-equips to Side slot |
| `keys` | Adds 1 key each time |
| `gems` | Adds 10 gems each time |
| `dragon_pieces` | Sets count to 7 (all pieces) |
| `boss_key_1` … `boss_key_8` | Grants the boss key for the specified dungeon (e.g. `boss_key_3`) |
| `ladder` | Lets you climb at ladder markers |
| `lantern` | Casts a warm circle of light around you at night |
| `raft` | Lets you cross water tiles |
| `strong_arm_glove` | Also accepted as `strong_glove` |
| `portal_tool` | Grants the portal attunement focus |
| `ancient_key` | Opens the Iron Highlands vault |
| `tide_chart` | Marks safe Sunken Coast routes |
| `ember_crystal` | Requires `strong_arm_glove` first |
| `void_compass` | Stabilizes fractured sanctum routes |
| `star_sigil` | Merchant seal for Aetherian ascent |
| `dragon_codex` | Lore needed for the final approach |
| `crystal_of_seeing` | Reveals the hidden path in the last ascent |

## Editable Sprites

The game loads character art from `assets/sprites` at runtime.

Files:

- `assets/sprites/hero.png`
- `assets/sprites/enemies.png`
- `assets/sprites/tiles.png`
- `assets/sprites/items.png`
- `assets/sprites/layout.toml`

You can open the PNGs in any pixel editor, save over them, and rerun the game. If you change the sheet layout, update `layout.toml` to match. The code is no longer hardcoded to one sprite-sheet arrangement.

Default layout:

- `hero.png`: 2 columns x 4 rows of `16x16` frames
  - row 1: down
  - row 2: up
  - row 3: left
  - row 4: right
- `enemies.png`: stacked rows
  - row 1: slime `16x16`, 2 frames
  - row 2: octorok `16x16`, 2 frames
  - row 3: bat `16x16`, 2 frames
  - row 4: darknut `16x16`, 2 frames
  - row 5: boss `24x24`, 2 frames
- `tiles.png`: 5 columns x 4 rows of `16x16` tiles for terrain, dungeon pieces, doors, chest, goal
- `items.png`: 4 columns x 4 rows of `16x16` icons for pickups, bombs, projectiles, and HUD icons

To regenerate the starter sheets:

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\generate_sprites.ps1
```

## Credits

Game sound effects sourced from various authors on [freesound.org](https://freesound.org).

## License

This project is released into the public domain under the [Unlicense](LICENSE). Do whatever you want with it.
