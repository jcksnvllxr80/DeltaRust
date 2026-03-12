# Overworld Implementation Map

This file reflects the **current live generated overworld** in `src\generated_overworld.rs`.

The current overworld is a fully populated `14 x 15` generated map: all `210` coordinates are live screens now, with no connector / filler cells.

Start screen: `(01,13)` - **SOUTHERN MEADOW**.

## Legend

- `B1`..`B8` = generic generated biome screens
- `D1`..`D8` = dungeon entrance / dungeon overworld screen in the generated map
- `Ak` = Engineer's Tower
- `Bc` = Basalt Cache
- `Br` = Belltower Remains
- `Cp` = Collapsed Pipe Cache
- `Cs` = Threshold Stone / Crystal of Seeing cave
- `Da` = Deep Ash Pit
- `Dc` = Last Camp / Dragon Codex cave
- `Ec` = Ember Garden / Ember Crystal cave
- `Fr` = Fungal Ring
- `Ga` = Great Aqueduct
- `Hc` = Hidden Sea Cave
- `Hr` = Hollow Root Clearing
- `Mc` = Mountaineer's Camp
- `Md` = Southern Meadow (start screen)
- `Mp` = Mirror Pool
- `Ps` = Plateau Stair
- `Sc` = Salvager's Camp
- `Se` = Sea Stack
- `Ss` = Merchant Lantern / Star Sigil cave
- `St` = Star Map Plaza
- `Sw` = Grandfather Tree / Sword cave
- `Tc` = Lighthouse Isle / Tide Chart cave
- `Tv` = Tidal Village
- `Vc` = Stable Rift / Void Compass cave
- `Vr` = Vel's Rift Camp

## Exact current generated map

```text
x ->    00   01   02   03   04   05   06   07   08   09   10   11   12   13

y=00   B6   B8   B8   D8   B8   B8   B6   B6   Ss   B7   B7   B7   B7   B7
y=01   B6   B8   B8   B8   Cs   B8   B8   B6   B7   B7   B7   B7   B7   B7
y=02   B6   B6   B8   B8   Dc   B8   Ps   D7   B6   St   B7   B7   B7   B7
y=03   B6   B6   B6   B6   D6   Vc   B8   B8   B6   B7   B7   B7   B7   B7
y=04   B3   B6   B6   D3   B6   B8   B8   B8   D5   B6   B7   B7   B7   B7
y=05   B3   Cp   B3   B6   B6   B6   Mp   B6   Vr   B5   B5   B5   B4   B4
y=06   B3   Ga   B3   B3   B6   B6   B6   Ec   B5   Tc   B5   B4   Hc   B4
y=07   B3   B3   Ak   B3   B3   B6   Mc   B5   Bc   B5   B5   B4   B4   B4
y=08   B3   D1   B3   B3   B3   B3   B5   B5   B5   B5   B5   B4   B4   B4
y=09   B1   B3   B3   B3   B3   B3   Br   B5   B5   Tv   B5   D4   Se   B4
y=10   B1   B1   B3   B3   Hr   B2   B5   B5   B5   B5   B5   B4   B4   B4
y=11   B1   B1   Sw   B1   B1   B2   Da   B2   D2   B2   B5   B4   B4   B4
y=12   B1   B1   B1   B1   Fr   Sc   B2   B2   B2   B2   B2   B4   B4   B4
y=13   B1   Md   B1   B1   B1   B2   B2   B2   B2   B2   B2   B2   B4   B4
y=14   B1   B1   B1   B1   B1   B1   B2   B2   B2   B2   B2   B2   B4   B4
```

## Special generated screens

- `(03,00)` - `D8` - **DRAGON'S ETERNAL THRONE** - biome `8` (dungeon: `8`)
- `(08,00)` - `Ss` - **MERCHANT LANTERN** - biome `7` (cave: `StarSigil`)
- `(04,01)` - `Cs` - **THRESHOLD STONE** - biome `8` (cave: `CrystalOfSeeing`)
- `(04,02)` - `Dc` - **LAST CAMP** - biome `8` (cave: `DragonCodex`)
- `(06,02)` - `Ps` - **PLATEAU STAIR** - biome `7`
- `(07,02)` - `D7` - **SPIRE ARCHWAY** - biome `7` (dungeon: `7`)
- `(09,02)` - `St` - **STAR MAP PLAZA** - biome `7`
- `(04,03)` - `D6` - **SANCTUM APPROACH** - biome `6` (dungeon: `6`)
- `(05,03)` - `Vc` - **STABLE RIFT** - biome `6` (cave: `VoidCompass`)
- `(03,04)` - `D3` - **VAULT APPROACH** - biome `3` (dungeon: `3`)
- `(08,04)` - `D5` - **SUMMIT APPROACH** - biome `5` (dungeon: `5`)
- `(01,05)` - `Cp` - **COLLAPSED PIPE CACHE** - biome `3`
- `(06,05)` - `Mp` - **MIRROR POOL** - biome `6`
- `(08,05)` - `Vr` - **VEL'S RIFT CAMP** - biome `6`
- `(01,06)` - `Ga` - **GREAT AQUEDUCT** - biome `3`
- `(07,06)` - `Ec` - **EMBER GARDEN** - biome `5` (cave: `EmberCrystal`)
- `(09,06)` - `Tc` - **LIGHTHOUSE ISLE** - biome `4` (cave: `TideChart`)
- `(12,06)` - `Hc` - **HIDDEN SEA CAVE** - biome `4`
- `(02,07)` - `Ak` - **ENGINEER'S TOWER** - biome `3` (cave: `AncientKey`)
- `(06,07)` - `Mc` - **MOUNTAINEER'S CAMP** - biome `5`
- `(08,07)` - `Bc` - **BASALT CACHE** - biome `5`
- `(01,08)` - `D1` - **THE FEN** - biome `1` (dungeon: `1`)
- `(06,09)` - `Br` - **BELLTOWER REMAINS** - biome `2`
- `(09,09)` - `Tv` - **TIDAL VILLAGE** - biome `4`
- `(11,09)` - `D4` - **SUNKEN CITADEL** - biome `4` (dungeon: `4`)
- `(12,09)` - `Se` - **SEA STACK** - biome `4`
- `(04,10)` - `Hr` - **HOLLOW ROOT CLEARING** - biome `1`
- `(02,11)` - `Sw` - **GRANDFATHER TREE** - biome `1` (cave: `Sword`)
- `(06,11)` - `Da` - **DEEP ASH PIT** - biome `2`
- `(08,11)` - `D2` - **WARDEN'S POST** - biome `2` (dungeon: `2`)
- `(04,12)` - `Fr` - **FUNGAL RING** - biome `1`
- `(05,12)` - `Sc` - **SALVAGER'S CAMP** - biome `2`
- `(01,13)` - `Md` - **SOUTHERN MEADOW** - biome `1`

## Biome totals in the generated overworld

- `B1` - Mosshaven Wilds: `26` screens
- `B2` - Ashenfall Reaches: `26` screens
- `B3` - Iron Highlands: `26` screens
- `B4` - Sunken Coast: `29` screens
- `B5` - Grimforge Approaches: `26` screens
- `B6` - Void Wastes: `29` screens
- `B7` - Celestial Plateau: `28` screens
- `B8` - Dragon's Approach: `20` screens
