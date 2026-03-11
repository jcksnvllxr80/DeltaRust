Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome3"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

function New-Sheet {
    param([int]$Width, [int]$Height)
    $bmp = New-Object System.Drawing.Bitmap($Width, $Height)
    $clear = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    for ($y = 0; $y -lt $Height; $y++) {
        for ($x = 0; $x -lt $Width; $x++) {
            $bmp.SetPixel($x, $y, $clear)
        }
    }
    return $bmp
}

function Paint-Sprite {
    param(
        [System.Drawing.Bitmap]$Bitmap,
        [string[]]$Rows,
        [int]$OffsetX,
        [int]$OffsetY,
        [hashtable]$Palette
    )
    for ($y = 0; $y -lt $Rows.Length; $y++) {
        $row = $Rows[$y]
        for ($x = 0; $x -lt $row.Length; $x++) {
            $ch = [string]$row[$x]
            $targetX = $OffsetX + $x
            $targetY = $OffsetY + $y
            if (
                $Palette.ContainsKey($ch) -and
                $targetX -ge 0 -and $targetY -ge 0 -and
                $targetX -lt $Bitmap.Width -and $targetY -lt $Bitmap.Height
            ) {
                $Bitmap.SetPixel($targetX, $targetY, $Palette[$ch])
            }
        }
    }
}

# =============================================================================
# BIOME 3 — THE IRON HIGHLANDS — TILE PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    r = dark iron rock  s = iron stone lt   d = dark plateau
#   g = sparse grass   m = rust metal      n = rust dark       p = iron plate
#   k = pipe interior  c = charcoal        w = steam white     a = steam grey
#   e = ember orange   t = rivet/bolt      b = beam iron       h = highlight
#   f = furnace glow   y = yellow warning
$b3TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "r" = [System.Drawing.Color]::FromArgb(255, 72, 68, 62)
    "s" = [System.Drawing.Color]::FromArgb(255, 108, 102, 92)
    "d" = [System.Drawing.Color]::FromArgb(255, 52, 48, 44)
    "g" = [System.Drawing.Color]::FromArgb(255, 86, 112, 68)
    "m" = [System.Drawing.Color]::FromArgb(255, 142, 98, 62)
    "n" = [System.Drawing.Color]::FromArgb(255, 108, 72, 44)
    "p" = [System.Drawing.Color]::FromArgb(255, 118, 114, 106)
    "k" = [System.Drawing.Color]::FromArgb(255, 36, 34, 32)
    "c" = [System.Drawing.Color]::FromArgb(255, 44, 42, 38)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 222, 228)
    "a" = [System.Drawing.Color]::FromArgb(255, 172, 178, 184)
    "e" = [System.Drawing.Color]::FromArgb(255, 198, 112, 42)
    "t" = [System.Drawing.Color]::FromArgb(255, 88, 84, 76)
    "b" = [System.Drawing.Color]::FromArgb(255, 78, 76, 72)
    "h" = [System.Drawing.Color]::FromArgb(255, 148, 144, 134)
    "f" = [System.Drawing.Color]::FromArgb(255, 218, 148, 52)
    "y" = [System.Drawing.Color]::FromArgb(255, 196, 178, 68)
}

# ---- IRON ROCK: Dark iron-bearing plateau ground ----
$ironRock = @(
    "rrsrrrdrrrsrrrdr",
    "rrrrrsrrrrrrrsrr",
    "rsrrrrrrrdrrrrrr",
    "rrrrdrrrrrrrrsrr",
    "rrrsrrrrrsrrrrrr",
    "rrrrrrsrrrrrrdrd",
    "rdrrrrrrrrrsrrrr",
    "rrrrsrdrrrrrrrsr",
    "rrrrrrrrrsrrrrrr",
    "rsrrrrrrrrrrdrrr",
    "rrrdrsrrrrrrrrsr",
    "rrrrrrrrdrrrrrrr",
    "rrrsrrrrrrrsrrdr",
    "rrrrrrdrrrrrrrrs",
    "rdrrrrrrrrdrrsrr",
    "rrrrsrrrrrrrrrrr"
)

# ---- IRON PLATE ROAD: Maintained iron-plate sections, riveted ----
$ironPlateRoad = @(
    "ppptpppppptppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "ptppppppppppppptp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "ppptpppppptppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "ptppppppppppppptp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "ppptpppppptppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp"
)

# ---- PIPE FIELD: Maze of rusted pipes at ground level ----
$pipeField = @(
    "rrnmmmmmmmmnrrrr",
    "rrnkkkkkkkkknrrr",
    "rrnmmmmmmmmnrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrrrnmmmmmmnrrrr",
    "rrrrnkkkkkkknrrr",
    "rrrrnmmmmmmnrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrrnmmmmmmmnrrrr",
    "rrrnkkkkkkkknrrr",
    "rrrnmmmmmmmnrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrnmmmmmmmmnrrrr",
    "rrnkkkkkkkkknrrr",
    "rrnmmmmmmmmnrrrr",
    "rrrrrrrrrrrrrrrr"
)

# ---- ACTIVE VENT: Steam vent hazard, periodic eruption ----
$activeVent = @(
    "rrrrrwwwwrrrrrrr",
    "rrrrwaaaawrrrrrr",
    "rrrwaaaaaawrrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrrrddddddrrrrr",
    "rrrddeeeedddrrr",
    "rrrdeeeeeeddrrr",
    "rrrdeeeeeeddrrr",
    "rrrddeeeedddrrr",
    "rrrrddddddrrrrr",
    "rrrrrddddrrrrr",
    "rrrrrrddrrrrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrrrrrrrrrrrrrrr"
)

# ---- GREAT AQUEDUCT: Massive iron aqueduct cross-section ----
$greatAqueduct = @(
    "bbbbbbbbbbbbbbbb",
    "btbbbbbbbbbbbtbb",
    "bbbbbbbbbbbbbbbb",
    "bbnmmmmmmmmnbbbb",
    "bbnkkkkkkkkknbbb",
    "bbnkkkkkkkkknbbb",
    "bbnmmmmmmmmnbbbb",
    "bbbbbbbbbbbbbbbb",
    "btbbbbbbbbbbbtbb",
    "bbbbbbbbbbbbbbbb",
    "bbnmmmmmmmmnbbbb",
    "bbnkkkkkkkkknbbb",
    "bbnkkkkkkkkknbbb",
    "bbnmmmmmmmmnbbbb",
    "bbbbbbbbbbbbbbbb",
    "btbbbbbbbbbbbtbb"
)

# ---- ENGINEER TOWER: Structurally sound 4-tile tower ----
$engineerTower = @(
    "....ssssssss....",
    "...ssbbbbbbss...",
    "..ssbbbbbbbbs...",
    "..sbb......bbs..",
    "..sbb.hh.h.bbs..",
    "..sbb......bbs..",
    "..sbbbbbbbbbs...",
    "..sbb......bbs..",
    "..sbb.h..h.bbs..",
    "..sbb......bbs..",
    "..sbbbbbbbbbs...",
    "..sbb....bbbs..",
    "..sbb....bbbs..",
    "..sbbbbbbbbs....",
    "...ssbbbbss.....",
    "....ssssss......"
)

# ---- VAULT DOOR: Massive iron door, 3 tiles tall (icon representation) ----
$vaultDoor = @(
    "ddbbbbbbbbbbbbdd",
    "dbtbbbbbbbbbtbdd",
    "dbbbbbbbbbbbbbdd",
    "dbbbnnnnnnnbbdd",
    "dbbnkkkkkknbbdd",
    "dbbnkkkkkknbbdd",
    "dbbnkkttkknbbdd",
    "dbbnkkkkkknbbdd",
    "dbbnkkkkkknbbdd",
    "dbbbnnnnnnnbbdd",
    "dbbbbbbbbbbbbbdd",
    "dbbbyyyyyybbbdd",
    "dbbbbbbbbbbbbbdd",
    "dbtbbbbbbbbbtbdd",
    "ddbbbbbbbbbbbbdd",
    "dddddddddddddddd"
)

# ---- SPARSE GRASS: Tough highland grass on dark rock ----
$sparseGrass = @(
    "rrrrgrrrrrrrrrrr",
    "rrrrrrrrrgrrrrrr",
    "rrrrrrrrrrrrrrrd",
    "rrgrrrrrrrrrrgrd",
    "rrrrrrrrrrrrrrrr",
    "rrrrrrgrrrrrrrrr",
    "rrrrrrrrrrgrrrrd",
    "rrrrrrrrrrrrrrrr",
    "rrgrrrrrrrrrrrrr",
    "rrrrrrrrrgrrrrrd",
    "rrrrrrrrrrrrrrrr",
    "rrrrgrrrrrrrrrrr",
    "rrrrrrrrrrrgrrrr",
    "rrrrrrrrrrrrrrrr",
    "rrgrrrrrrrrrrgrd",
    "rrrrrrrrrrrrrrrr"
)

# ---- STEAM CLOUD: Obscuring steam effect tile ----
$steamCloud = @(
    "....wwaaaw......",
    "...waaaaaaaw....",
    "..waaaaaaaaww...",
    ".waaaawwaaaaww..",
    "waaaaawwaaaaaw..",
    "waaaaaaaaaaaaw..",
    ".waaaaaaaaaaw...",
    "..waaaaaaaaw....",
    "...waaawaaaw....",
    "..waaaaaaaaww...",
    ".waaawwaaaaaw...",
    "waaaaaaaaaaww...",
    ".waaaaaaaaw.....",
    "..waaaaaaw......",
    "...waaaw........",
    "....ww.........."
)

# Write the biome 3 tile sheet — 10 tiles
$b3TileSheet = New-Sheet -Width 144 -Height 32
Paint-Sprite $b3TileSheet $ironRock         0   0 $b3TilePalette
Paint-Sprite $b3TileSheet $ironPlateRoad    16  0 $b3TilePalette
Paint-Sprite $b3TileSheet $pipeField        32  0 $b3TilePalette
Paint-Sprite $b3TileSheet $activeVent       48  0 $b3TilePalette
Paint-Sprite $b3TileSheet $greatAqueduct    64  0 $b3TilePalette
Paint-Sprite $b3TileSheet $engineerTower    80  0 $b3TilePalette
Paint-Sprite $b3TileSheet $vaultDoor        96  0 $b3TilePalette
Paint-Sprite $b3TileSheet $sparseGrass      112 0 $b3TilePalette
Paint-Sprite $b3TileSheet $steamCloud       128 0 $b3TilePalette
Paint-Sprite $b3TileSheet $ironRock         0   16 $b3TilePalette  # placeholder 10th slot
$b3TilePath = Join-Path $outDir "tiles.png"
$b3TileSheet.Save($b3TilePath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 3 — THE IRON HIGHLANDS — ENEMY PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         m = metal grey      s = metal shine
#   r = red light      d = dark metal      k = black           b = bronze
#   w = white eye      e = ember glow      a = amber           g = green tint
#   p = purple magnet  n = dark body       t = teal circuit    c = copper
#   y = yellow spark   f = flame
$b3EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "m" = [System.Drawing.Color]::FromArgb(255, 128, 124, 116)
    "s" = [System.Drawing.Color]::FromArgb(255, 168, 164, 156)
    "r" = [System.Drawing.Color]::FromArgb(255, 196, 56, 48)
    "d" = [System.Drawing.Color]::FromArgb(255, 68, 66, 60)
    "k" = [System.Drawing.Color]::FromArgb(255, 36, 34, 32)
    "b" = [System.Drawing.Color]::FromArgb(255, 158, 118, 62)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 222, 228)
    "e" = [System.Drawing.Color]::FromArgb(255, 212, 128, 42)
    "a" = [System.Drawing.Color]::FromArgb(255, 192, 156, 52)
    "g" = [System.Drawing.Color]::FromArgb(255, 82, 148, 92)
    "p" = [System.Drawing.Color]::FromArgb(255, 138, 82, 168)
    "n" = [System.Drawing.Color]::FromArgb(255, 52, 48, 44)
    "t" = [System.Drawing.Color]::FromArgb(255, 68, 148, 138)
    "c" = [System.Drawing.Color]::FromArgb(255, 178, 118, 68)
    "y" = [System.Drawing.Color]::FromArgb(255, 228, 218, 72)
    "f" = [System.Drawing.Color]::FromArgb(255, 232, 158, 48)
}

# ---- IRON DRONE: Mechanical construct on patrol, back panel weak point ----
$ironDrone1 = @(
    "................",
    "....odddddo.....",
    "...odmmmmmdoo...",
    "..odmmmmmmmdo...",
    "..odmrttrmdo....",
    "..odmmmmmmdoo...",
    ".odmmmmmmmmdoo..",
    ".odmddddddmdoo..",
    ".odmdmmmmmdmdo..",
    ".odmdmmmmmdmdo..",
    "..odmddddddmdo..",
    "..odmmmmmmmdoo..",
    "...odmmmmdoo....",
    "...oddboobddoo..",
    "....dbb..bbd....",
    "................"
)

$ironDrone2 = @(
    "................",
    "....odddddo.....",
    "...odmmmmmdoo...",
    "..odmmmmmmmdo...",
    "..odmrttrmdo....",
    "..odmmmmmmdoo...",
    ".odmmmmmmmmdoo..",
    ".odmddddddmdoo..",
    ".odmdmmmmmdmdo..",
    ".odmdmmmmmdmdo..",
    "..odmddddddmdo..",
    "..odmmmmmmmdoo..",
    "...odmmmmmdoo...",
    "...oddb.obddoo..",
    "....dbb..bbd....",
    "................"
)

# ---- STEAM SPECTER: Ghostly figure made of steam, emerges from vents ----
$steamSpecter1 = @(
    "......ww........",
    ".....wwaw.......",
    "....waaaaw......",
    "...waararaw.....",
    "...waaaaaaw.....",
    "..waaaaaaaaw....",
    "..waaawwaaw.....",
    "..waaaaaaaaw....",
    "...waaaaaaw.....",
    "....waaaaw......",
    ".....waaw.......",
    "......ww........",
    ".......w........",
    "................",
    "................",
    "................"
)

$steamSpecter2 = @(
    ".......ww.......",
    "......waaw......",
    ".....waaaaw.....",
    "....waararaw....",
    "...waaaaaaaaw...",
    "...waaawaaaw....",
    "..waaaaaaaaw....",
    "...waaaaaaw.....",
    "...waaaaaw......",
    "....waaaw.......",
    ".....waw........",
    "......ww........",
    "................",
    "................",
    "................",
    "................"
)

# ---- BOLT CRAWLER: Small mechanical spider, clings to iron surfaces ----
$boltCrawler1 = @(
    "................",
    "................",
    "................",
    "....d.dd.d......",
    "...ddmmmmd......",
    "...dmmtmmd......",
    "...ddmmmmd......",
    "....d.dd.d......",
    "...d..dd..d.....",
    "..d...dd...d....",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$boltCrawler2 = @(
    "................",
    "................",
    "................",
    "...d..dd..d.....",
    "....dmmmmdd.....",
    "...dmmtmmmd.....",
    "....dmmmmdd.....",
    "...d..dd..d.....",
    "....d.dd.d......",
    ".....dddd.......",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- MAGNET HULK: Heavy magnetic construct, pulls player toward it ----
$magnetHulk1 = @(
    "................",
    "....oppppo......",
    "...opddddpo.....",
    "..opdddddddpo..",
    "..opddrdrdddpo..",
    "..opddddddddpo.",
    ".opddddddddddpo",
    ".opdddddddddpo.",
    ".opddddddddpo..",
    "..opddddddpo...",
    "..opddddddpo...",
    "...opdddpo......",
    "...opddbbpo.....",
    "....obb.bbo.....",
    "...bbb...bbb....",
    "................"
)

$magnetHulk2 = @(
    "................",
    "....oppppo......",
    "...opddddpo.....",
    "..opdddddddpo..",
    "..opddrdrdddpo..",
    "..opddddddddpo.",
    ".opddddddddddpo",
    ".opdddddddddpo.",
    ".opddddddddpo..",
    "..opddddddpo...",
    "..opddddddpo...",
    "...opdddpo......",
    "...opdb.bdpo....",
    "....obb.bbo.....",
    "...bbb...bbb....",
    "................"
)

# Enemy sheet: 4 enemies x 2 frames = 32 x 64
$b3EnemySheet = New-Sheet -Width 32 -Height 64
Paint-Sprite $b3EnemySheet $ironDrone1      0  0  $b3EnemyPalette
Paint-Sprite $b3EnemySheet $ironDrone2      16 0  $b3EnemyPalette
Paint-Sprite $b3EnemySheet $steamSpecter1   0  16 $b3EnemyPalette
Paint-Sprite $b3EnemySheet $steamSpecter2   16 16 $b3EnemyPalette
Paint-Sprite $b3EnemySheet $boltCrawler1    0  32 $b3EnemyPalette
Paint-Sprite $b3EnemySheet $boltCrawler2    16 32 $b3EnemyPalette
Paint-Sprite $b3EnemySheet $magnetHulk1     0  48 $b3EnemyPalette
Paint-Sprite $b3EnemySheet $magnetHulk2     16 48 $b3EnemyPalette
$b3EnemyPath = Join-Path $outDir "enemies.png"
$b3EnemySheet.Save($b3EnemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 3 — THE IRON HIGHLANDS — NPC PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         s = skin            a = skin shadow
#   h = hair dark      m = hair mid        e = eyes            b = boots
#   g = goggles brass  r = red bandana     d = dark overalls   w = white undershirt
#   c = coat grey      f = coat dark       k = tool belt       n = notebook tan
#   p = pants brown    t = trim
$b3NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "s" = [System.Drawing.Color]::FromArgb(255, 208, 176, 142)
    "a" = [System.Drawing.Color]::FromArgb(255, 176, 144, 112)
    "h" = [System.Drawing.Color]::FromArgb(255, 48, 38, 28)
    "m" = [System.Drawing.Color]::FromArgb(255, 78, 58, 38)
    "e" = [System.Drawing.Color]::FromArgb(255, 42, 42, 48)
    "b" = [System.Drawing.Color]::FromArgb(255, 58, 48, 36)
    "g" = [System.Drawing.Color]::FromArgb(255, 178, 148, 68)
    "r" = [System.Drawing.Color]::FromArgb(255, 168, 58, 42)
    "d" = [System.Drawing.Color]::FromArgb(255, 72, 68, 58)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 212, 198)
    "c" = [System.Drawing.Color]::FromArgb(255, 128, 122, 108)
    "f" = [System.Drawing.Color]::FromArgb(255, 96, 90, 78)
    "k" = [System.Drawing.Color]::FromArgb(255, 112, 88, 52)
    "n" = [System.Drawing.Color]::FromArgb(255, 188, 168, 118)
    "p" = [System.Drawing.Color]::FromArgb(255, 86, 74, 56)
    "t" = [System.Drawing.Color]::FromArgb(255, 148, 108, 62)
}

# ---- CORVIN, THE SALVAGE ENGINEER: Practical, goggles on forehead, dark overalls ----
$corvin1 = @(
    "................",
    ".....ohhho......",
    "....ohmgmho.....",
    "....ohmmmho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...oddddddoo....",
    "..oddwwwwddoo...",
    "..oddkkkkddoo...",
    "..oddddddddoo...",
    "...oddddddoo....",
    "...oppppppoo....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$corvin2 = @(
    "................",
    ".....ohhho......",
    "....ohmgmho.....",
    "....ohmmmho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...oddddddoo....",
    "..oddwwwwddoo...",
    "..oddkkkkddoo...",
    "..oddddddddoo...",
    "...oddddddoo....",
    "...oppppppoo....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# ---- PETRA, THE GEOLOGIST: Young academic, light hair, field coat ----
$petra1 = @(
    "................",
    ".....onkno......",
    "....onkkkno.....",
    "....onkkkno.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffnnffcco...",
    "..ocffffffcco...",
    "...ocffffcco....",
    "...ocppppco.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$petra2 = @(
    "................",
    ".....onkno......",
    "....onkkkno.....",
    "....onkkkno.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffnnffcco...",
    "..ocffffffcco...",
    "...ocffffcco....",
    "...ocppppco.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# NPC sheet: 2 NPCs x 2 frames = 32 x 32
$b3NpcSheet = New-Sheet -Width 32 -Height 32
Paint-Sprite $b3NpcSheet $corvin1   0  0  $b3NpcPalette
Paint-Sprite $b3NpcSheet $corvin2   16 0  $b3NpcPalette
Paint-Sprite $b3NpcSheet $petra1    0  16 $b3NpcPalette
Paint-Sprite $b3NpcSheet $petra2    16 16 $b3NpcPalette
$b3NpcPath = Join-Path $outDir "npcs.png"
$b3NpcSheet.Save($b3NpcPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 3 — THE IRON HIGHLANDS — ITEM PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         g = gold/brass      n = gold dark
#   k = key teeth      r = rust            d = dark metal      s = steel grey
#   b = bottle glass   t = tonic red       w = white label     p = parchment
#   a = parchment dark c = crystal blue    e = crystal bright   m = map brown
#   f = iron shield    h = highlight
$b3ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "g" = [System.Drawing.Color]::FromArgb(255, 198, 168, 58)
    "n" = [System.Drawing.Color]::FromArgb(255, 158, 128, 38)
    "k" = [System.Drawing.Color]::FromArgb(255, 172, 142, 52)
    "r" = [System.Drawing.Color]::FromArgb(255, 148, 92, 52)
    "d" = [System.Drawing.Color]::FromArgb(255, 72, 68, 58)
    "s" = [System.Drawing.Color]::FromArgb(255, 138, 134, 124)
    "b" = [System.Drawing.Color]::FromArgb(255, 88, 142, 128)
    "t" = [System.Drawing.Color]::FromArgb(255, 186, 82, 48)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 214, 198)
    "p" = [System.Drawing.Color]::FromArgb(255, 178, 158, 118)
    "a" = [System.Drawing.Color]::FromArgb(255, 138, 122, 86)
    "c" = [System.Drawing.Color]::FromArgb(255, 108, 188, 218)
    "e" = [System.Drawing.Color]::FromArgb(255, 158, 218, 238)
    "m" = [System.Drawing.Color]::FromArgb(255, 128, 104, 68)
    "f" = [System.Drawing.Color]::FromArgb(255, 112, 108, 98)
    "h" = [System.Drawing.Color]::FromArgb(255, 178, 174, 164)
}

# ---- ANCIENT KEY: Ornate old key, brass, required for Ironclad Vault ----
$ancientKey = @(
    "................",
    "......gg........",
    ".....gnngg......",
    "......ggng......",
    "......gng.......",
    "......gng.......",
    "......gng.......",
    "...ggggngggg....",
    "...gnngnngngg...",
    "...ggggngggg....",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- HEATING TONIC: Orange-red potion bottle ----
$heatingTonic = @(
    "................",
    ".......ww.......",
    "......owwo......",
    "......oddo......",
    ".....odttdo.....",
    "....odtttttdo...",
    "....odtttttdo...",
    "....odtttttdo...",
    "....odtttttdo...",
    "....odtttttdo...",
    "....oddddddo....",
    ".....oooooo.....",
    "................",
    "................",
    "................",
    "................"
)

# ---- IRON SHIELD: Passive item, reduces Magnet Hulk pull ----
$ironShield = @(
    "................",
    "....offffffo....",
    "...ofsssssfo....",
    "..ofssssssssfo..",
    "..ofsshhhssfo...",
    "..ofssshsssfo...",
    "..ofsshhhssfo...",
    "..ofsssssssfo...",
    "...ofsssssfo....",
    "....ofssssfo....",
    ".....ofssfo.....",
    "......offo......",
    ".......oo.......",
    "................",
    "................",
    "................"
)

# ---- VENT CRYSTAL: Mineral dropped by Steam Specters ----
$ventCrystal = @(
    "................",
    ".......e........",
    "......ecc.......",
    ".....eccce......",
    "....eccccce.....",
    "....ecccccce....",
    "...ecccccce.....",
    "....eccccce.....",
    "....ecccce......",
    ".....ecce.......",
    "......ee........",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- ENGINEER'S REPORT: Old document, industrial script ----
$engineerReport = @(
    "................",
    "....oaao........",
    "...oappao.......",
    "..oappppapo.....",
    "..oappdppapo....",
    "..oapppppapo....",
    "..oappdppapo....",
    "..oapppppapo....",
    "..oappdpapo.....",
    "..oaaaaaao......",
    "...oaaaao.......",
    "....oooo........",
    "................",
    "................",
    "................",
    "................"
)

# ---- MAP FRAGMENT: Partial map showing buried cache ----
$mapFragment = @(
    "................",
    "......oao.......",
    ".....oapao......",
    "....oapppao.....",
    "...oappmpao.....",
    "...oapppppao....",
    "....oappao......",
    ".....oao........",
    "......o.........",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# Item sheet: 6 items at 16x16 = 48 x 32
$b3ItemSheet = New-Sheet -Width 48 -Height 32
Paint-Sprite $b3ItemSheet $ancientKey      0  0  $b3ItemPalette
Paint-Sprite $b3ItemSheet $heatingTonic    16 0  $b3ItemPalette
Paint-Sprite $b3ItemSheet $ironShield      32 0  $b3ItemPalette
Paint-Sprite $b3ItemSheet $ventCrystal     0  16 $b3ItemPalette
Paint-Sprite $b3ItemSheet $engineerReport  16 16 $b3ItemPalette
Paint-Sprite $b3ItemSheet $mapFragment     32 16 $b3ItemPalette
$b3ItemPath = Join-Path $outDir "items.png"
$b3ItemSheet.Save($b3ItemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated Biome 3 (Iron Highlands) sprite sheets:"
Write-Host " - $b3TilePath"
Write-Host " - $b3EnemyPath"
Write-Host " - $b3NpcPath"
Write-Host " - $b3ItemPath"
