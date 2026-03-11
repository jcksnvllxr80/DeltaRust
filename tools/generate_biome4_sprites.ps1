Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome4"
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
# BIOME 4 — THE SUNKEN COAST — TILE PALETTE
# =============================================================================
# Tone: Brine and beauty. Tidal rhythm. Clear deep-blue sky, calm sea,
#        flooded ruins, stone causeways, sandy tidal islands.
#
# Key mapping (case-insensitive safe):
#   . = transparent    w = deep ocean       s = shallow water    f = foam/wavecap
#   b = beach sand     n = wet sand         c = causeway stone   d = causeway dark
#   r = ruin stone     k = ruin dark        g = seagrass/kelp    m = dock wood
#   h = highlight      a = iron gate        e = lighthouse white p = roof red
#   t = tidal algae    y = lantern glow
$b4TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "w" = [System.Drawing.Color]::FromArgb(255, 42, 82, 128)
    "s" = [System.Drawing.Color]::FromArgb(255, 68, 118, 158)
    "f" = [System.Drawing.Color]::FromArgb(255, 148, 192, 212)
    "b" = [System.Drawing.Color]::FromArgb(255, 208, 192, 152)
    "n" = [System.Drawing.Color]::FromArgb(255, 178, 164, 128)
    "c" = [System.Drawing.Color]::FromArgb(255, 148, 142, 128)
    "d" = [System.Drawing.Color]::FromArgb(255, 108, 104, 92)
    "r" = [System.Drawing.Color]::FromArgb(255, 122, 118, 104)
    "k" = [System.Drawing.Color]::FromArgb(255, 72, 68, 62)
    "g" = [System.Drawing.Color]::FromArgb(255, 58, 108, 78)
    "m" = [System.Drawing.Color]::FromArgb(255, 108, 82, 52)
    "h" = [System.Drawing.Color]::FromArgb(255, 182, 198, 218)
    "a" = [System.Drawing.Color]::FromArgb(255, 62, 58, 52)
    "e" = [System.Drawing.Color]::FromArgb(255, 228, 224, 212)
    "p" = [System.Drawing.Color]::FromArgb(255, 158, 62, 48)
    "t" = [System.Drawing.Color]::FromArgb(255, 52, 92, 68)
    "y" = [System.Drawing.Color]::FromArgb(255, 228, 198, 98)
}

# ---- DEEP OCEAN: Impassable open sea (eastern edge) ----
$deepOcean = @(
    "wwwwwwwwwwwwwwww",
    "wwwwwfwwwwwwwwww",
    "wwwwwwwwwwfwwwww",
    "wwwwwwwwwwwwwwww",
    "wwfwwwwwwwwwwwww",
    "wwwwwwwwwwwwwfww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwfwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwfwwwwwwwfwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwfwwwwwwwwww",
    "wwwwwwwwwwfwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwfwwwwwwwwwwww"
)

# ---- SHALLOW WATER: Wading-depth tidal water around islands ----
$shallowWater = @(
    "sssssssfssssssss",
    "ssssfssssssssssf",
    "ssssssssssfsssss",
    "sfsssssssssssssf",
    "sssssfssssssssss",
    "sssssssssfsssssf",
    "sssfsssssssssss",
    "sssssssssssfssss",
    "sfsssssssssssssf",
    "ssssssfsssssssss",
    "sssssssssssfsssf",
    "ssfsssssssssssss",
    "ssssssssfsssssss",
    "sssfsssssssssssf",
    "sssssssssssfsssf",
    "sssssfssssssssss"
)

# ---- BEACH SAND: Sandy tidal island surface ----
$beachSand = @(
    "bbbbnbbbbbbbnbbb",
    "bbbbbbbbbnbbbbbb",
    "bnbbbbbbbbbbbbbn",
    "bbbbbbbnbbbbbbbbb",
    "bbbnbbbbbbbbbnbb",
    "bbbbbbbbbnbbbbbb",
    "bbbbbnbbbbbbbbbb",
    "bnbbbbbbbbbbnbbb",
    "bbbbbbbbbnbbbbbb",
    "bbbbnbbbbbbbbbbbb",
    "bbbbbbbbbbbnbbbb",
    "bnbbbbbnbbbbbbbn",
    "bbbbbbbbbbbbnbbb",
    "bbbbbnbbbbbbbbbb",
    "bbnbbbbbbbnbbbbb",
    "bbbbbbbbbbbbbbbb"
)

# ---- STONE CAUSEWAY: The raised paths connecting tidal islands ----
$stoneCauseway = @(
    "ddccccccccccccdd",
    "dccccccccccccccd",
    "cccccccccccccccc",
    "ccccchccccchcccc",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "dccccccccccccccd",
    "ddccccccccccccdd",
    "ddccccccccccccdd",
    "dccccccccccccccd",
    "cccccccccccccccc",
    "cccchccccchccccc",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "dccccccccccccccd",
    "ddccccccccccccdd"
)

# ---- FLOODED RUIN WALL: Partially submerged ruin structures ----
$floodedRuin = @(
    "rrkrrrkrrrrkrrrr",
    "rrrrrrrrrkrrrrkr",
    "rrrkrrrrrrrrrrrr",
    "rrrrrrkrrrrrrkrr",
    "ssssssssssssssss",
    "ssssssssssssssss",
    "rrkrrrkrrrrkrrrr",
    "rrrrrrrrrkrrrrkr",
    "rrrkrrrrrrrrrrrr",
    "rrrrrrkrrrrrrkrr",
    "ssssssssssssssss",
    "ssssgsssssgssssss",
    "ssssssgsssssgsss",
    "ssssssssssssssss",
    "ssssgsssssssgss",
    "ssssssssssssssss"
)

# ---- LIGHTHOUSE: The abandoned lighthouse — red roof, white tower ----
$lighthouse = @(
    "......pppp......",
    ".....pppppp.....",
    "......yyyy......",
    ".....eeeeee.....",
    "....eeeeeeee....",
    "....eeeeeeee....",
    "....eeeeeeee....",
    "....eeeeeeee....",
    "...eeeeeeeeee...",
    "...eeeeeeeeee...",
    "...eeeeeeeeee...",
    "...eeeeeeeeee...",
    "..eeeeeeeeeeee..",
    "..dddddddddddd..",
    ".dddddddddddddd.",
    "bbbbbbbbbbbbbbbb"
)

# ---- TIDAL GATE: Massive stone gate, opens/closes with tide ----
$tidalGate = @(
    "ddaaaaaaaaaaadd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "daaakkkkkkaaaddd",
    "ddaaddddddaadddd",
    "dddddddddddddddd",
    "ssssssssssssssss",
    "ssssssssssssssss"
)

# ---- SEA STACK: Isolated rock pillar, Sael lives here ----
$seaStack = @(
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwddddwwwwwww",
    "wwwwddrrddwwwwww",
    "wwwddrrrrdddwwww",
    "wwdddrrrrrdddwww",
    "wwddrrrrrrdddwww",
    "wwddrrrrrrddwwww",
    "wwddrrrrrrdddwww",
    "wwdddrrrrrdddwww",
    "wwwdddrrddddwwww",
    "wwwwddddddwwwwww",
    "wwwwwwddwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww"
)

# ---- DOCK / VILLAGE: Wooden dock with pilings over water ----
$dockVillage = @(
    "ssssmmmmmmssssss",
    "ssssmmmmmmsssss",
    "mmmmmmmmmmmmmmms",
    "mmmmmmmmmmmmmmms",
    "ssmsssmsssmsssmss",
    "ssmsssmsssmsssmss",
    "ssssssssssssssss",
    "ssssssssssssssss",
    "ssssmmmmmmssssss",
    "ssssmmmmmmssssss",
    "mmmmmmmmmmmmmmms",
    "mmmmmymmmymmmms",
    "ssmsssmsssmsssmss",
    "ssmsssmsssmsssmss",
    "ssssssssssssssss",
    "ssssssssssssssss"
)

# ---- RUIN ROOFTOP: Accessible rooftop of flooded building ----
$ruinRooftop = @(
    "ssssssssssssssss",
    "sssrrrrrrrrrsss",
    "ssrrpppppprrrsss",
    "srrrpppppprrrrss",
    "srrrrpppprrrrrss",
    "srrrrrpprrrrrrss",
    "srrrrrrrrrrrrrs",
    "srrrrrrrrrrrrrs",
    "srrrrrrrrrrrrrss",
    "srrrrhrrrrhrrrss",
    "srrrrrrrrrrrrrss",
    "srrrrrrrrrrrrrs",
    "ssrrrrrrrrrrrsss",
    "sssrrrrrrrrsssss",
    "ssssssssssssssss",
    "ssssssssssssssss"
)

# Write the biome 4 tile sheet — 10 tiles in a 160x16 strip
$b4TileSheet = New-Sheet -Width 160 -Height 16
Paint-Sprite $b4TileSheet $deepOcean       0   0 $b4TilePalette
Paint-Sprite $b4TileSheet $shallowWater    16  0 $b4TilePalette
Paint-Sprite $b4TileSheet $beachSand       32  0 $b4TilePalette
Paint-Sprite $b4TileSheet $stoneCauseway   48  0 $b4TilePalette
Paint-Sprite $b4TileSheet $floodedRuin     64  0 $b4TilePalette
Paint-Sprite $b4TileSheet $lighthouse      80  0 $b4TilePalette
Paint-Sprite $b4TileSheet $tidalGate       96  0 $b4TilePalette
Paint-Sprite $b4TileSheet $seaStack        112 0 $b4TilePalette
Paint-Sprite $b4TileSheet $dockVillage     128 0 $b4TilePalette
Paint-Sprite $b4TileSheet $ruinRooftop     144 0 $b4TilePalette
$b4TilePath = Join-Path $outDir "tiles.png"
$b4TileSheet.Save($b4TilePath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 4 — THE SUNKEN COAST — ENEMY PALETTE
# =============================================================================
# Enemies from doc:
#   Tideclaw (crab) — scuttles sideways, pincer grab
#   Reef Stalker (octopus) — tentacle grabs from water
#   Stormkite (bird) — aerial, dives and wind-buffets
#   Saltform (water humanoid) — merges with water on flooded tiles
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         r = crab red        n = crab dark
#   s = shell pink     w = white eye       k = black pupil     b = body blue
#   d = dark blue      t = tentacle purple g = green tint      m = grey beak
#   f = feather blue   a = wing brown      c = crest yellow    e = water bright
#   h = highlight      p = pale blue
$b4EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 28, 28, 36)
    "r" = [System.Drawing.Color]::FromArgb(255, 188, 72, 48)
    "n" = [System.Drawing.Color]::FromArgb(255, 138, 48, 32)
    "s" = [System.Drawing.Color]::FromArgb(255, 218, 128, 98)
    "w" = [System.Drawing.Color]::FromArgb(255, 228, 228, 232)
    "k" = [System.Drawing.Color]::FromArgb(255, 22, 22, 28)
    "b" = [System.Drawing.Color]::FromArgb(255, 68, 108, 148)
    "d" = [System.Drawing.Color]::FromArgb(255, 38, 62, 98)
    "t" = [System.Drawing.Color]::FromArgb(255, 112, 68, 128)
    "g" = [System.Drawing.Color]::FromArgb(255, 72, 128, 92)
    "m" = [System.Drawing.Color]::FromArgb(255, 138, 134, 122)
    "f" = [System.Drawing.Color]::FromArgb(255, 88, 138, 178)
    "a" = [System.Drawing.Color]::FromArgb(255, 118, 98, 72)
    "c" = [System.Drawing.Color]::FromArgb(255, 218, 198, 78)
    "e" = [System.Drawing.Color]::FromArgb(255, 128, 178, 208)
    "h" = [System.Drawing.Color]::FromArgb(255, 188, 208, 228)
    "p" = [System.Drawing.Color]::FromArgb(255, 158, 188, 208)
}

# ---- TIDECLAW: A reddish crab — scuttles sideways, pincers ----
$tideclaw1 = @(
    "................",
    "................",
    "..r...rrrr...r..",
    "..rr.rrssrr.rr..",
    "...rrsswwssr....",
    "...rrsssssr.....",
    "..rrrssssrrrr...",
    "..rrsssssssrr...",
    ".rrssssssssrrr..",
    ".rrsssssssssrr..",
    "..rrssssssrr....",
    "...rrssssrr.....",
    "....rr..rr......",
    "...rr....rr.....",
    "................",
    "................"
)

$tideclaw2 = @(
    "................",
    "................",
    "...r..rrrr..r...",
    "...rr.rssr.rr...",
    "....rrsswwsr....",
    "....rrssssr.....",
    "..rrrrssssrrrr..",
    "..rrssssssssrr..",
    "..rrsssssssrr...",
    ".rrsssssssssrr..",
    "..rrsssssssrr...",
    "....rrssrr......",
    "....rr..rr......",
    "..rr......rr....",
    "................",
    "................"
)

# ---- REEF STALKER: Octopus-like, tentacles from below, purple-blue ----
$reefStalker1 = @(
    "................",
    "....odddddo.....",
    "...odddddddo....",
    "..odddwkdwkdo...",
    "..odddddddddo...",
    "...odddddddo....",
    "..otttddddttto..",
    ".ottttddddtttto.",
    ".ot.ttttttttt.o.",
    "..ot.ttttttt.o..",
    "..ot..ttttt..o..",
    "...ot.ttt.to....",
    "....ot.t.to.....",
    ".....ooooo......",
    "................",
    "................"
)

$reefStalker2 = @(
    "................",
    "....odddddo.....",
    "...odddddddo....",
    "..odddwkdwkdo...",
    "..odddddddddo...",
    "...odddddddo....",
    "..ottdddddtto...",
    ".otttdddddttto..",
    "..ottttttttto...",
    ".ot.ttttttttto..",
    "..o.ttttttt.o...",
    "...otttttt.o....",
    "....ot.t.to.....",
    ".....ooooo......",
    "................",
    "................"
)

# ---- STORMKITE: Seabird, dives from height, wind-buffet attack ----
$stormkite1 = @(
    "................",
    "................",
    ".......ff.......",
    "......ffff......",
    ".....ffffff.....",
    "..aaffffffaa....",
    ".aaaffwkffaaa...",
    "aaafffffffaaaa..",
    ".aaffmffmffaa...",
    "..afffffffa.....",
    "...afffffaa.....",
    "....fffff.......",
    ".....faf........",
    "......f.........",
    "................",
    "................"
)

$stormkite2 = @(
    "................",
    "................",
    ".......ff.......",
    "......ffff......",
    ".....ffffff.....",
    "....afffffffaa..",
    "...aafffwkffaaa.",
    "..aaafffffffaaa.",
    "...aaffmffmffa..",
    "....afffffffaa..",
    ".....afffffa....",
    "......fffff.....",
    ".......faf......",
    "........f.......",
    "................",
    "................"
)

# ---- SALTFORM: Humanoid water construct, translucent blue body ----
$saltform1 = @(
    "................",
    ".....oeeo.......",
    "....oepbeo......",
    "....oewkeo......",
    "....oebbeo......",
    "....oeppeo......",
    "...oebbbeo......",
    "...oebbbbeo.....",
    "...oebbbeo......",
    "..oebbbbbeo.....",
    "...oebbeo.......",
    "...oeppeo.......",
    "...oeb.beo......",
    "..oeb...beo.....",
    "...oo...oo......",
    "................"
)

$saltform2 = @(
    "................",
    "......oeeo......",
    ".....oepbeo.....",
    ".....oewkeo.....",
    ".....oebbeo.....",
    ".....oeppeo.....",
    "....oebbbeo.....",
    "....oebbbbeo....",
    "....oebbbeo.....",
    "...oebbbbbeo....",
    "....oebbeo......",
    "....oeppeo......",
    "....oe.beo......",
    "...oeb...beo....",
    "....oo...oo.....",
    "................"
)

# Enemy sheet: 4 enemies x 2 frames = 32 x 64
$b4EnemySheet = New-Sheet -Width 32 -Height 64
Paint-Sprite $b4EnemySheet $tideclaw1      0  0  $b4EnemyPalette
Paint-Sprite $b4EnemySheet $tideclaw2      16 0  $b4EnemyPalette
Paint-Sprite $b4EnemySheet $reefStalker1   0  16 $b4EnemyPalette
Paint-Sprite $b4EnemySheet $reefStalker2   16 16 $b4EnemyPalette
Paint-Sprite $b4EnemySheet $stormkite1     0  32 $b4EnemyPalette
Paint-Sprite $b4EnemySheet $stormkite2     16 32 $b4EnemyPalette
Paint-Sprite $b4EnemySheet $saltform1      0  48 $b4EnemyPalette
Paint-Sprite $b4EnemySheet $saltform2      16 48 $b4EnemyPalette
$b4EnemyPath = Join-Path $outDir "enemies.png"
$b4EnemySheet.Save($b4EnemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 4 — THE SUNKEN COAST — NPC PALETTE
# =============================================================================
# Captain Aldric: Retired coastal captain. Weathered, navy coat, captain's hat.
# Sael: Eccentric sea trader on an isolated sea stack. Wrap/shawl, windswept.
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         s = skin            a = skin shadow
#   h = hair grey      m = hat dark navy   e = eyes            b = boots dark
#   c = coat navy      f = coat dark navy  w = shirt white     k = belt brown
#   n = shawl teal     r = shawl dark      d = dress sand      g = gold earring
#   p = pants grey     t = trim brass
$b4NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 28, 28, 36)
    "s" = [System.Drawing.Color]::FromArgb(255, 198, 168, 132)
    "a" = [System.Drawing.Color]::FromArgb(255, 168, 138, 108)
    "h" = [System.Drawing.Color]::FromArgb(255, 152, 148, 138)
    "m" = [System.Drawing.Color]::FromArgb(255, 32, 42, 62)
    "e" = [System.Drawing.Color]::FromArgb(255, 42, 42, 48)
    "b" = [System.Drawing.Color]::FromArgb(255, 48, 42, 36)
    "c" = [System.Drawing.Color]::FromArgb(255, 48, 68, 98)
    "f" = [System.Drawing.Color]::FromArgb(255, 32, 48, 72)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 218, 212)
    "k" = [System.Drawing.Color]::FromArgb(255, 112, 82, 48)
    "n" = [System.Drawing.Color]::FromArgb(255, 68, 128, 118)
    "r" = [System.Drawing.Color]::FromArgb(255, 42, 88, 78)
    "d" = [System.Drawing.Color]::FromArgb(255, 188, 172, 132)
    "g" = [System.Drawing.Color]::FromArgb(255, 208, 188, 68)
    "p" = [System.Drawing.Color]::FromArgb(255, 82, 78, 68)
    "t" = [System.Drawing.Color]::FromArgb(255, 168, 142, 72)
}

# ---- CAPTAIN ALDRIC: Retired captain, grey hair, navy coat, captain's hat ----
$aldric1 = @(
    "................",
    "....ommmmmo.....",
    "...ommmmmmmo....",
    "....ohmhmho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocfftffcco....",
    "..ocffwwffcco...",
    "..ocffkkffcco...",
    "...ocffffcco....",
    "...ocppppco.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$aldric2 = @(
    "................",
    "....ommmmmo.....",
    "...ommmmmmmo....",
    "....ohmhmho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocfftffcco....",
    "..ocffwwffcco...",
    "..ocffkkffcco...",
    "...ocffffcco....",
    "...ocppppco.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# ---- SAEL, THE SEA TRADER: Eccentric, teal shawl/wrap, windswept hair, gold earring ----
$sael1 = @(
    "................",
    ".....ohhho......",
    "....ohhghhoo....",
    "....ohhhhho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...onnrrnnoo....",
    "..onnrddrnoo....",
    "..onnddddnnoo...",
    "..onnddddnnoo...",
    "...onnddnnoo....",
    "...oddddddo.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$sael2 = @(
    "................",
    ".....ohhho......",
    "....ohhghhoo....",
    "....ohhhhho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...onnrrnnoo....",
    "..onnrddrnoo....",
    "..onnddddnnoo...",
    "..onnddddnnoo...",
    "...onnddnnoo....",
    "...oddddddo.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# NPC sheet: 2 NPCs x 2 frames = 32 x 32
$b4NpcSheet = New-Sheet -Width 32 -Height 32
Paint-Sprite $b4NpcSheet $aldric1   0  0  $b4NpcPalette
Paint-Sprite $b4NpcSheet $aldric2   16 0  $b4NpcPalette
Paint-Sprite $b4NpcSheet $sael1     0  16 $b4NpcPalette
Paint-Sprite $b4NpcSheet $sael2     16 16 $b4NpcPalette
$b4NpcPath = Join-Path $outDir "npcs.png"
$b4NpcSheet.Save($b4NpcPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 4 — THE SUNKEN COAST — ITEM PALETTE
# =============================================================================
# Items from doc:
#   Tide Chart — reusable reference item, shows tidal pattern (world item)
#   Tidal Passage Map — causeway tidal schedule, sold by Aldric
#   Deep Diving Bell — 120 coins from Sael, extends breath timer
#   Sea Captain's Compass — collectable from lighthouse, sell to Barnett
#   Gold Pouch — coins
#   Healing Herb — coastal variant
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         p = parchment       a = parchment dark
#   b = blue ink       w = white           g = gold            n = gold dark
#   d = dark metal     s = steel grey      r = rust copper     c = compass rose
#   m = map brown      h = herb green      e = herb dark       k = bottle glass
#   t = teal water     f = highlight
$b4ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 28, 28, 36)
    "p" = [System.Drawing.Color]::FromArgb(255, 188, 172, 132)
    "a" = [System.Drawing.Color]::FromArgb(255, 148, 132, 98)
    "b" = [System.Drawing.Color]::FromArgb(255, 48, 78, 128)
    "w" = [System.Drawing.Color]::FromArgb(255, 228, 224, 212)
    "g" = [System.Drawing.Color]::FromArgb(255, 208, 178, 58)
    "n" = [System.Drawing.Color]::FromArgb(255, 168, 138, 38)
    "d" = [System.Drawing.Color]::FromArgb(255, 58, 56, 48)
    "s" = [System.Drawing.Color]::FromArgb(255, 148, 144, 132)
    "r" = [System.Drawing.Color]::FromArgb(255, 168, 108, 62)
    "c" = [System.Drawing.Color]::FromArgb(255, 188, 52, 42)
    "m" = [System.Drawing.Color]::FromArgb(255, 128, 104, 68)
    "h" = [System.Drawing.Color]::FromArgb(255, 72, 138, 82)
    "e" = [System.Drawing.Color]::FromArgb(255, 48, 98, 52)
    "k" = [System.Drawing.Color]::FromArgb(255, 88, 148, 138)
    "t" = [System.Drawing.Color]::FromArgb(255, 68, 118, 148)
    "f" = [System.Drawing.Color]::FromArgb(255, 198, 212, 228)
}

# ---- TIDE CHART: Rolled parchment with blue tidal lines — world item ----
$tideChart = @(
    "................",
    "....oaao........",
    "...oappao.......",
    "..oappppapo.....",
    "..oapbbppapo....",
    "..oappbppapo....",
    "..oapbbppapo....",
    "..oappbppapo....",
    "..oapbbpapo.....",
    "..oaaaaaao......",
    "...oaaaao.......",
    "....oooo........",
    "................",
    "................",
    "................",
    "................"
)

# ---- TIDAL PASSAGE MAP: Folded scroll with causeway routes ----
$tidalPassageMap = @(
    "................",
    "...oaaaao.......",
    "..oappppapo.....",
    "..oapbmppapo....",
    "..oapmpbpapo....",
    "..oapbpmppo.....",
    "..oapppbpapo....",
    "..oapmppapo.....",
    "..oaaaaaao......",
    "...oooooo.......",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- DEEP DIVING BELL: Brass-and-glass diving apparatus ----
$deepDivingBell = @(
    "................",
    ".....dddd.......",
    "....dssssd......",
    "...dssffssdo....",
    "...dssffssd.....",
    "...dssffssdo....",
    "...dssssssdo....",
    "....dssssd......",
    ".....dddd.......",
    "......dd........",
    ".....drrd.......",
    "....drrrrdo.....",
    ".....drrd.......",
    "......dd........",
    "................",
    "................"
)

# ---- SEA CAPTAIN'S COMPASS: Ornate brass compass with red needle ----
$seaCaptainCompass = @(
    "................",
    ".....ngggn......",
    "....ngggggn.....",
    "...ngggcgggn....",
    "...nggcccggn....",
    "...nggcscggn....",
    "...ngggcgggn....",
    "...nggggggn.....",
    "....nggggn......",
    ".....nnnn.......",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- GOLD POUCH: Coastal variant, sea-blue strap ----
$coastGoldPouch = @(
    "................",
    "......tt........",
    ".....tggt.......",
    "....tggggt......",
    "...tgggggt......",
    "...ngggggnt.....",
    "...ngggggnto....",
    "...ngggggnt.....",
    "...ngggggn......",
    "....nggn........",
    ".....nn.........",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- HEALING HERB: Coastal herb, green with teal highlights ----
$coastHealingHerb = @(
    "................",
    "......hh........",
    ".....hehh.......",
    "....heeehh......",
    "...hehheehh.....",
    "...hehhheeh.....",
    "....heeeh.......",
    ".....heh........",
    "......e.........",
    "......e.........",
    ".....eee........",
    "......e.........",
    "................",
    "................",
    "................",
    "................"
)

# Item sheet: 6 items at 16x16 = 48 x 32
$b4ItemSheet = New-Sheet -Width 48 -Height 32
Paint-Sprite $b4ItemSheet $tideChart          0  0  $b4ItemPalette
Paint-Sprite $b4ItemSheet $tidalPassageMap    16 0  $b4ItemPalette
Paint-Sprite $b4ItemSheet $deepDivingBell     32 0  $b4ItemPalette
Paint-Sprite $b4ItemSheet $seaCaptainCompass  0  16 $b4ItemPalette
Paint-Sprite $b4ItemSheet $coastGoldPouch     16 16 $b4ItemPalette
Paint-Sprite $b4ItemSheet $coastHealingHerb   32 16 $b4ItemPalette
$b4ItemPath = Join-Path $outDir "items.png"
$b4ItemSheet.Save($b4ItemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated Biome 4 (Sunken Coast) sprite sheets:"
Write-Host " - $b4TilePath"
Write-Host " - $b4EnemyPath"
Write-Host " - $b4NpcPath"
Write-Host " - $b4ItemPath"
