# DeltaRust

Native Rust version of the Delta game, kept separate from the original browser implementation in `Delta/`.

## Run

```sh
cargo run
```

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

## Git Setup

To start tracking this project in a repository, initialize git and make an initial commit:

```sh
git init
git add .
git commit -m "Initial commit"
```

A `.gitignore` file is included to exclude build artifacts, editor files, and other temporary data.

## License

Include your project license here, for example MIT or Apache-2.0. Adjust as needed.
