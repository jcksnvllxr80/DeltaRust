# DeltaRust

Native Rust version of the Delta game, kept separate from the original browser implementation in `Delta/`.

## Run

```sh
cargo run
```

## In-game console

During gameplay, press `~` to open the console at the bottom of the screen.

- `help` lists the supported console commands
- `teleport x,y` moves you to an overworld screen coordinate such as `teleport 3,4`
- `god_mode 0/1` disables or enables player invulnerability
- `get_item <item_name>` adds an item to your inventory
- `show_map_poi` toggles the map points-of-interest overlay (same as launching with `--dev-mode`)

Press `Enter` to run the current command, and `Esc` or `~` to close the console.

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

## License

Include your project license here, for example MIT or Apache-2.0. Adjust as needed.
