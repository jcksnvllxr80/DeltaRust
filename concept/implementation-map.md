# Overworld Implementation Map

This file now reflects the **live overworld implementation** in `src\world_data.rs`.

The actual game world uses a blob-style biome footprint grid with authored landmark screens layered on top. The result is `168` biome screens and `42` connector / path screens across the real `14 x 15` overworld.

## Legend

- `B1`..`B8` = live biome footprint screens
- `PP` = connector / path screen
- `D1`..`D8` = dungeon entrance screen
- `Sw`, `Ak`, `Tc`, `Ec`, `Vc`, `Ss`, `Dc`, `Cs` = authored special landmark screens

## Exact live map

```text
x ->    0    1    2    3    4    5    6    7    8    9   10   11   12   13

y=00   B8   B8   D8   B8   Cs   B7   B7   B7   B7   B7   B7   PP   PP   PP
y=01   B8   B8   B8   B8   B8   B7   B7   B7   B7   Ss   B7   PP   PP   PP
y=02   B8   B8   B8   B8   B8   B7   B7   B7   D7   B7   B7   B7   PP   PP
y=03   B8   Dc   B8   B8   B6   B6   B7   B7   B5   B5   D5   B5   B5   B5
y=04   B8   B8   D6   B6   B6   B6   B6   B5   B5   Ec   B5   B5   B5   PP
y=05   PP   PP   B6   B6   B6   Vc   B6   B5   B5   B5   B5   B5   B5   PP
y=06   PP   PP   B6   B6   B6   B6   B6   B6   B4   Tc   B5   B5   B5   PP
y=07   PP   PP   B6   B6   B4   B4   B4   B4   B4   B4   B4   B4   B4   B4
y=08   PP   PP   B3   B3   D3   B4   B4   B4   B4   B4   B4   B4   D4   B4
y=09   PP   PP   B3   B3   B3   B3   B3   B3   B4   B2   B2   B2   B2   PP
y=10   PP   PP   B3   Ak   B3   B3   B3   B3   B3   B2   B2   B2   B2   PP
y=11   PP   PP   PP   B3   B3   B3   B3   B3   B2   B2   B2   B2   D2   PP
y=12   PP   PP   D1   B1   B1   B1   B1   B1   B2   B2   B2   B2   B2   PP
y=13   PP   PP   B1   B1   B1   B1   Sw   B1   B1   B1   B2   B2   PP   PP
y=14   PP   PP   B1   B1   B1   B1   B1   B1   B1   B1   PP   PP   PP   PP
```

## Landmark and dungeon placements

- `(02,00)` - `D8` - **FINAL APPROACH**
- `(04,00)` - `Cs` - **THRONE NICHE**
- `(09,01)` - `Ss` - **MERCHANT'S CIRCUIT**
- `(08,02)` - `D7` - **SPIRE BASE**
- `(01,03)` - `Dc` - **LAST CAMP**
- `(10,03)` - `D5` - **SUMMIT APPROACH**
- `(02,04)` - `D6` - **SANCTUM APPROACH**
- `(09,04)` - `Ec` - **EMBER GARDEN**
- `(05,05)` - `Vc` - **MIRROR POOL**
- `(09,06)` - `Tc` - **LIGHTHOUSE ISLE**
- `(04,08)` - `D3` - **VAULT APPROACH**
- `(12,08)` - `D4` - **FLOODED RUINS**
- `(03,10)` - `Ak` - **ENGINEER'S TOWER**
- `(12,11)` - `D2` - **WARDEN'S POST**
- `(02,12)` - `D1` - **THE FEN**
- `(06,13)` - `Sw` - **GRANDFATHER TREE**

## Biome key

- `B1` - Mosshaven Wilds
- `B2` - Ashenfall Reaches
- `B3` - Iron Highlands
- `B4` - Sunken Coast
- `B5` - Grimforge Approaches
- `B6` - Void Wastes
- `B7` - Celestial Plateau
- `B8` - Dragon's Approach

## Exact live biome footprints

## B1 - Mosshaven Wilds

Live footprint: 22 screens

Coordinates: `(02,12)`, `(03,12)`, `(04,12)`, `(05,12)`, `(06,12)`, `(07,12)`, `(02,13)`, `(03,13)`, `(04,13)`, `(05,13)`, `(06,13)`, `(07,13)`, `(08,13)`, `(09,13)`, `(02,14)`, `(03,14)`, `(04,14)`, `(05,14)`, `(06,14)`, `(07,14)`, `(08,14)`, `(09,14)`

Named landmark screens:
- `(02,12)` - `D1` - THE FEN
- `(06,13)` - `Sw` - GRANDFATHER TREE
## B2 - Ashenfall Reaches

Live footprint: 20 screens

Coordinates: `(09,09)`, `(10,09)`, `(11,09)`, `(12,09)`, `(09,10)`, `(10,10)`, `(11,10)`, `(12,10)`, `(08,11)`, `(09,11)`, `(10,11)`, `(11,11)`, `(12,11)`, `(08,12)`, `(09,12)`, `(10,12)`, `(11,12)`, `(12,12)`, `(10,13)`, `(11,13)`

Named landmark screens:
- `(12,11)` - `D2` - WARDEN'S POST
## B3 - Iron Highlands

Live footprint: 21 screens

Coordinates: `(02,08)`, `(03,08)`, `(04,08)`, `(02,09)`, `(03,09)`, `(04,09)`, `(05,09)`, `(06,09)`, `(07,09)`, `(02,10)`, `(03,10)`, `(04,10)`, `(05,10)`, `(06,10)`, `(07,10)`, `(08,10)`, `(03,11)`, `(04,11)`, `(05,11)`, `(06,11)`, `(07,11)`

Named landmark screens:
- `(04,08)` - `D3` - VAULT APPROACH
- `(03,10)` - `Ak` - ENGINEER'S TOWER
## B4 - Sunken Coast

Live footprint: 22 screens

Coordinates: `(08,06)`, `(09,06)`, `(04,07)`, `(05,07)`, `(06,07)`, `(07,07)`, `(08,07)`, `(09,07)`, `(10,07)`, `(11,07)`, `(12,07)`, `(13,07)`, `(05,08)`, `(06,08)`, `(07,08)`, `(08,08)`, `(09,08)`, `(10,08)`, `(11,08)`, `(12,08)`, `(13,08)`, `(08,09)`

Named landmark screens:
- `(09,06)` - `Tc` - LIGHTHOUSE ISLE
- `(12,08)` - `D4` - FLOODED RUINS
## B5 - Grimforge Approaches

Live footprint: 21 screens

Coordinates: `(08,03)`, `(09,03)`, `(10,03)`, `(11,03)`, `(12,03)`, `(13,03)`, `(07,04)`, `(08,04)`, `(09,04)`, `(10,04)`, `(11,04)`, `(12,04)`, `(07,05)`, `(08,05)`, `(09,05)`, `(10,05)`, `(11,05)`, `(12,05)`, `(10,06)`, `(11,06)`, `(12,06)`

Named landmark screens:
- `(10,03)` - `D5` - SUMMIT APPROACH
- `(09,04)` - `Ec` - EMBER GARDEN
## B6 - Void Wastes

Live footprint: 20 screens

Coordinates: `(04,03)`, `(05,03)`, `(02,04)`, `(03,04)`, `(04,04)`, `(05,04)`, `(06,04)`, `(02,05)`, `(03,05)`, `(04,05)`, `(05,05)`, `(06,05)`, `(02,06)`, `(03,06)`, `(04,06)`, `(05,06)`, `(06,06)`, `(07,06)`, `(02,07)`, `(03,07)`

Named landmark screens:
- `(02,04)` - `D6` - SANCTUM APPROACH
- `(05,05)` - `Vc` - MIRROR POOL
## B7 - Celestial Plateau

Live footprint: 21 screens

Coordinates: `(05,00)`, `(06,00)`, `(07,00)`, `(08,00)`, `(09,00)`, `(10,00)`, `(05,01)`, `(06,01)`, `(07,01)`, `(08,01)`, `(09,01)`, `(10,01)`, `(05,02)`, `(06,02)`, `(07,02)`, `(08,02)`, `(09,02)`, `(10,02)`, `(11,02)`, `(06,03)`, `(07,03)`

Named landmark screens:
- `(09,01)` - `Ss` - MERCHANT'S CIRCUIT
- `(08,02)` - `D7` - SPIRE BASE
## B8 - Dragon's Approach

Live footprint: 21 screens

Coordinates: `(00,00)`, `(01,00)`, `(02,00)`, `(03,00)`, `(04,00)`, `(00,01)`, `(01,01)`, `(02,01)`, `(03,01)`, `(04,01)`, `(00,02)`, `(01,02)`, `(02,02)`, `(03,02)`, `(04,02)`, `(00,03)`, `(01,03)`, `(02,03)`, `(03,03)`, `(00,04)`, `(01,04)`

Named landmark screens:
- `(02,00)` - `D8` - FINAL APPROACH
- `(04,00)` - `Cs` - THRONE NICHE
- `(01,03)` - `Dc` - LAST CAMP
