Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome5"
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
# BIOME 5 — GRIMFORGE APPROACHES — TILE PALETTE
# =============================================================================
# Tone: Hostile beauty. Black basalt, hardened lava flows, fumaroles, ash fall.
# Mountain dominates northern skyline, ember glow in cracks.
#
# Key mapping (case-insensitive safe):
#   . = transparent    b = black basalt     r = basalt cracked   d = dark rock
#   e = ember orange   f = flame glow       s = scrub green      n = scrub dark
#   a = ash grey       w = smoke white      g = smoke grey       c = crystal blue
#   m = molten red     k = charcoal         h = basalt highlight p = path stone
#   t = tent brown     y = ember bright
$b5TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "b" = [System.Drawing.Color]::FromArgb(255, 38, 36, 34)
    "r" = [System.Drawing.Color]::FromArgb(255, 58, 52, 46)
    "d" = [System.Drawing.Color]::FromArgb(255, 28, 26, 24)
    "e" = [System.Drawing.Color]::FromArgb(255, 208, 118, 38)
    "f" = [System.Drawing.Color]::FromArgb(255, 238, 168, 48)
    "s" = [System.Drawing.Color]::FromArgb(255, 68, 82, 48)
    "n" = [System.Drawing.Color]::FromArgb(255, 48, 58, 32)
    "a" = [System.Drawing.Color]::FromArgb(255, 128, 124, 118)
    "w" = [System.Drawing.Color]::FromArgb(255, 198, 198, 192)
    "g" = [System.Drawing.Color]::FromArgb(255, 158, 156, 148)
    "c" = [System.Drawing.Color]::FromArgb(255, 108, 168, 198)
    "m" = [System.Drawing.Color]::FromArgb(255, 178, 42, 28)
    "k" = [System.Drawing.Color]::FromArgb(255, 22, 20, 18)
    "h" = [System.Drawing.Color]::FromArgb(255, 78, 74, 68)
    "p" = [System.Drawing.Color]::FromArgb(255, 68, 64, 58)
    "t" = [System.Drawing.Color]::FromArgb(255, 108, 82, 52)
    "y" = [System.Drawing.Color]::FromArgb(255, 248, 198, 68)
}

# ---- HARDENED LAVA: Black basalt ground, cracked ----
$hardenedLava = @(
    "bbbbrbbbbbbbrbbbb",
    "bbbbbbbbbrbbbbbbb",
    "brbbbbbbbbbbbbbrrb",
    "bbbbbrbbbbbbbbbb",
    "bbbbbbbbbbrbbbb",
    "bbrbbbbbbbbbbbrb",
    "bbbbbbbrbbbbbbbb",
    "bbbbrbbbbbbbrbbbb",
    "brbbbbbbbbbbbbbbb",
    "bbbbbbbbbrbbbbbb",
    "bbbbrbbbbbbbbbrb",
    "bbbbbbbbbbbrbbbb",
    "bbrbbbbbbbbbbbrb",
    "bbbbbbbrbbbbbbbb",
    "bbbbrbbbbbrbbbb",
    "bbbbbbbbbbbbbbbb"
)

# ---- MOLTEN SEAM: Glowing lava crack in basalt — hazard ----
$moltenSeam = @(
    "bbbbbbbbbbbbbbbb",
    "bbbbbembbbbbbbb",
    "bbbbbmebbbbbbbbb",
    "bbbbbbembbbmbbbb",
    "bbbbbbbmebmebbb",
    "bbbbbbbbmembbbb",
    "bbbbbbbbbmebbbbb",
    "bbbbbbbbbbmmbbb",
    "bbbbbbbbbbbmebbb",
    "bbbbbbbbmbbmebbb",
    "bbbbbbbmebbbmbb",
    "bbbbbbmebbbbbmbbb",
    "bbbbbmebbbbbbbbb",
    "bbbbmebbbbbbbbb",
    "bbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb"
)

# ---- FUMAROLE: Steam vent in basalt ground ----
$fumarole = @(
    "bbbbbwgwbbbbbbbb",
    "bbbbwgggwbbbbbbb",
    "bbbwgggggwbbbbbb",
    "bbbbbwwwbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbhhhbbbbbbbb",
    "bbbhhdddhhhbbbb",
    "bbhhddddddhbb",
    "bbhhddddddhbb",
    "bbbhhddddhbbbb",
    "bbbbhhhhhhbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb"
)

# ---- EMBER GARDEN: Crystal cluster field, world item location ----
$emberGarden = @(
    "bbbcbbcbbcbbbbbb",
    "bbcccbcccbccbbbb",
    "bbcbcbcbcbcbbbbb",
    "bbbcbbcbbcbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbcfcbbbbbbbb",
    "bbbbcfyfcbbbbbbb",
    "bbbcfyyyfcbbbbbb",
    "bbbbcfyfcbbbbbbb",
    "bbbbbcfcbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbcbbcbbcbbbbbb",
    "bbcccbcccbccbbbb",
    "bbcbcbcbcbcbbbbb",
    "bbbcbbcbbcbbbbbb",
    "bbbbbbbbbbbbbbbb"
)

# ---- ASCENT PATH: Narrow winding mountain path ----
$ascentPath = @(
    "dddpppppppdddddd",
    "ddpppppppppddddd",
    "dppppppppppdddd",
    "dpppppppppdddddd",
    "ddpppppppdddddd",
    "dddpppppppddddd",
    "ddddpppppppddd",
    "dddddpppppppdd",
    "dddddpppppppd",
    "ddddpppppppddd",
    "dddpppppppddddd",
    "ddpppppppdddddd",
    "ddppppppppddddd",
    "dddpppppppddddd",
    "ddddpppppppdddd",
    "dddpppppppdddd"
)

# ---- SUMMIT APPROACH: Open plateau before dungeon entrance ----
$summitApproach = @(
    "pppppppppppppppp",
    "ppppphpppphppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "ppphppppppphpppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppphppphppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "ppphppppppphpppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "ppppphpppphppppp",
    "pppppppppppppppp",
    "pppppppppppppppp"
)

# ---- FORGE ENTRANCE: The vent mouth dungeon entry, hot air glow ----
$forgeEntrance = @(
    "dddddddkddddddd",
    "dddddkkkkkddddd",
    "ddddkkkkkkkkdddd",
    "dddkkkeeekkkddd",
    "ddkkkeeeeeekkkdd",
    "ddkkeeefeeekkd",
    "dkkeeeffeeekkdd",
    "dkkeeefeeeekkkd",
    "ddkkeeeeeekkdd",
    "dddkkkeeeekkkdd",
    "dddkkkkkkkkdddd",
    "dddddkkkkdddddd",
    "ddddddkkddddddd",
    "dddddddddddddddd",
    "dddddddddddddddd",
    "dddddddddddddddd"
)

# ---- MOUNTAINEER CAMP: Dax's camp, sheltered behind boulders ----
$mountaineerCamp = @(
    "bbbbbbbbbbbbbbbb",
    "bbbbbttttbbbbbbb",
    "bbbbttttttbbbbbb",
    "bbbttteettbbbbb",
    "bbbttttttttbbbb",
    "bbbbttttttbbbbbb",
    "bbbbbttttbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbhhhhhhhhbbbbb",
    "bbhbbbbbbbbhbbbb",
    "bbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb"
)

# ---- CALDERA VIEW: Crack showing interior glow ----
$calderaView = @(
    "dddddddddddddddd",
    "dddddkkkdddddddd",
    "ddddkkkkkkdddddd",
    "dddkkkeeekkkddd",
    "ddkkkeyyeekkkdd",
    "ddkkeyyyyyykkdd",
    "dkkeyyyyyeekk",
    "dkkeyyyyyyekkdd",
    "ddkkeeyyyeekdd",
    "dddkkkeeekkdddd",
    "dddddkkkkkddddd",
    "dddddddkddddddd",
    "dddddddddddddddd",
    "dddddddddddddddd",
    "dddddddddddddddd",
    "dddddddddddddddd"
)

# ---- SCRUB: Sparse volcanic scrub vegetation ----
$volcanicScrub = @(
    "bbbbbbsbbbbbbbbb",
    "bbbbbnsnbbbbbbbb",
    "bbbbbsbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbsbbbbbbbbsbbb",
    "bbnsnbbbbbbnsnbb",
    "bbbsbbbbbbbsbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbsbbbbbbbb",
    "bbbbbbnsnbbbbbb",
    "bbbbbbsbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbsbbbbbbbbsbbbb",
    "bnsnbbbbbbbnsnbb",
    "bbsbbbbbbbbbsbbb",
    "bbbbbbbbbbbbbbbb"
)

# Write tile sheet — 10 tiles: 160 x 16
$b5TileSheet = New-Sheet -Width 160 -Height 16
Paint-Sprite $b5TileSheet $hardenedLava    0   0 $b5TilePalette
Paint-Sprite $b5TileSheet $moltenSeam      16  0 $b5TilePalette
Paint-Sprite $b5TileSheet $fumarole        32  0 $b5TilePalette
Paint-Sprite $b5TileSheet $emberGarden     48  0 $b5TilePalette
Paint-Sprite $b5TileSheet $ascentPath      64  0 $b5TilePalette
Paint-Sprite $b5TileSheet $summitApproach  80  0 $b5TilePalette
Paint-Sprite $b5TileSheet $forgeEntrance   96  0 $b5TilePalette
Paint-Sprite $b5TileSheet $mountaineerCamp 112 0 $b5TilePalette
Paint-Sprite $b5TileSheet $calderaView     128 0 $b5TilePalette
Paint-Sprite $b5TileSheet $volcanicScrub   144 0 $b5TilePalette
$b5TilePath = Join-Path $outDir "tiles.png"
$b5TileSheet.Save($b5TilePath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 5 — GRIMFORGE APPROACHES — ENEMY PALETTE
# =============================================================================
# Enemies: Lava Beetle, Basalt Crawler, Ash Drifter, Magma Spawn
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         r = beetle red       n = beetle dark
#   s = shell shine    w = white eye       k = black            b = basalt brown
#   d = dark body      e = ember glow      f = flame            a = ash grey
#   g = ash light      m = magma red       c = magma core       h = highlight
#   p = purple heat    t = claw
$b5EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 22, 18, 16)
    "r" = [System.Drawing.Color]::FromArgb(255, 168, 52, 28)
    "n" = [System.Drawing.Color]::FromArgb(255, 118, 38, 22)
    "s" = [System.Drawing.Color]::FromArgb(255, 198, 88, 42)
    "w" = [System.Drawing.Color]::FromArgb(255, 228, 228, 232)
    "k" = [System.Drawing.Color]::FromArgb(255, 18, 16, 14)
    "b" = [System.Drawing.Color]::FromArgb(255, 72, 62, 52)
    "d" = [System.Drawing.Color]::FromArgb(255, 48, 42, 36)
    "e" = [System.Drawing.Color]::FromArgb(255, 218, 138, 42)
    "f" = [System.Drawing.Color]::FromArgb(255, 248, 188, 58)
    "a" = [System.Drawing.Color]::FromArgb(255, 148, 144, 138)
    "g" = [System.Drawing.Color]::FromArgb(255, 178, 174, 168)
    "m" = [System.Drawing.Color]::FromArgb(255, 188, 48, 28)
    "c" = [System.Drawing.Color]::FromArgb(255, 238, 128, 38)
    "h" = [System.Drawing.Color]::FromArgb(255, 198, 178, 148)
    "p" = [System.Drawing.Color]::FromArgb(255, 138, 58, 88)
    "t" = [System.Drawing.Color]::FromArgb(255, 92, 78, 62)
}

# ---- LAVA BEETLE: Small, very fast, red-shelled, swarms ----
$lavaBeetle1 = @(
    "................",
    "................",
    "................",
    "......orro......",
    ".....orrrrro....",
    "....orrssrrro...",
    "....orrsrrrro...",
    "....orrrrrrro...",
    "....orrssrrro...",
    ".....orrrrro....",
    "......orro......",
    "....tt.oo.tt....",
    "...t........t...",
    "................",
    "................",
    "................"
)

$lavaBeetle2 = @(
    "................",
    "................",
    "................",
    "......orro......",
    ".....orrrrro....",
    "....orrssrrro...",
    "....orrsrrrro...",
    "....orrrrrrro...",
    "....orrssrrro...",
    ".....orrrrro....",
    "......orro......",
    "...tt..oo..tt...",
    "..t..........t..",
    "................",
    "................",
    "................"
)

# ---- BASALT CRAWLER: Lizard, clings to walls, drops on player ----
$basaltCrawler1 = @(
    "................",
    "................",
    "....ob...bo.....",
    "...obdddddbo....",
    "..obdwkdwkdbo...",
    "..obdddddddbo..",
    "..obdbddbddbo...",
    "...obdddddbo....",
    "....obdddboo....",
    "....t.bdb.t.....",
    "...t..bdb..t....",
    "..t...bdb...t...",
    "......bdb.......",
    ".......b........",
    "................",
    "................"
)

$basaltCrawler2 = @(
    "................",
    "................",
    "....ob...bo.....",
    "...obdddddbo....",
    "..obdwkdwkdbo...",
    "..obdddddddbo..",
    "..obdbddbddbo...",
    "...obdddddbo....",
    "....obdddboo....",
    "...t..bdb..t....",
    "..t...bdb...t...",
    ".t....bdb....t..",
    "......bdb.......",
    ".......b........",
    "................",
    "................"
)

# ---- ASH DRIFTER: Ash-cloud entity, moves with wind, unavoidable ----
$ashDrifter1 = @(
    "....aagg........",
    "...aagggga......",
    "..aaggggggaa....",
    ".aagggaagggaa...",
    "aagggaaagggaa...",
    "aggggggggggga...",
    ".aggggggggga....",
    "..aggggggga.....",
    "...agggaggga....",
    "..aagggggggaa...",
    ".aagaaggaaggaa..",
    "agggggggggggaa..",
    ".aggggggggga....",
    "..aggggggga.....",
    "...aagga........",
    "....aa.........."
)

$ashDrifter2 = @(
    ".....aagg.......",
    "....aagggga.....",
    "...aaggggggaa...",
    "..aagggaagggaa..",
    ".aagggaaagggaa..",
    ".aggggggggggga..",
    "..aggggggggga...",
    "...aggggggga....",
    "..aagggaggga....",
    ".aagggggggggaa..",
    "aaggaaggaaggaa..",
    ".agggggggggga...",
    "..aggggggggga...",
    "...aagggga......",
    "....aagg........",
    ".....aa........."
)

# ---- MAGMA SPAWN: Slow, heavy, emerges from molten seams ----
$magmaSpawn1 = @(
    "................",
    "......ommo......",
    ".....ommmmmo....",
    "....ommcmcmmo...",
    "...ommmmmmmmmo..",
    "...ommcemmcemo..",
    "...ommmmmmmmmo..",
    "..ommmmmmmmmmmo.",
    "..ommmcmmcmmmo..",
    "..ommmmmmmmmmmo.",
    "...ommmmmmmmmo..",
    "...ommmmmmmmmo..",
    "....omm..ommo...",
    "...omm....mmo...",
    "....oo....oo....",
    "................"
)

$magmaSpawn2 = @(
    "................",
    "......ommo......",
    ".....ommmmmo....",
    "....ommcmcmmo...",
    "...ommmmmmmmmo..",
    "...ommcemmcemo..",
    "...ommmmmmmmmo..",
    "..ommmmmmmmmmmo.",
    "..ommmcmmcmmmo..",
    "..ommmmmmmmmmmo.",
    "...ommmmmmmmmo..",
    "...ommmmmmmmmo..",
    "....ommo.mmo....",
    "...omm...ommo...",
    "....oo....oo....",
    "................"
)

# Enemy sheet: 4 enemies x 2 frames = 32 x 64
$b5EnemySheet = New-Sheet -Width 32 -Height 64
Paint-Sprite $b5EnemySheet $lavaBeetle1    0  0  $b5EnemyPalette
Paint-Sprite $b5EnemySheet $lavaBeetle2    16 0  $b5EnemyPalette
Paint-Sprite $b5EnemySheet $basaltCrawler1 0  16 $b5EnemyPalette
Paint-Sprite $b5EnemySheet $basaltCrawler2 16 16 $b5EnemyPalette
Paint-Sprite $b5EnemySheet $ashDrifter1    0  32 $b5EnemyPalette
Paint-Sprite $b5EnemySheet $ashDrifter2    16 32 $b5EnemyPalette
Paint-Sprite $b5EnemySheet $magmaSpawn1    0  48 $b5EnemyPalette
Paint-Sprite $b5EnemySheet $magmaSpawn2    16 48 $b5EnemyPalette
$b5EnemyPath = Join-Path $outDir "enemies.png"
$b5EnemySheet.Save($b5EnemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 5 — GRIMFORGE APPROACHES — NPC PALETTE
# =============================================================================
# Dax the Mountaineer: Professional climber, rugged gear, climbing rope.
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         s = skin            a = skin shadow
#   h = hair brown     m = hair dark       e = eyes            b = boots heavy
#   c = coat green     f = coat dark       w = white undershirt k = belt/rope
#   d = dark gear      g = goggles/strap   p = pants khaki     n = bandana red
#   r = rope tan       t = trim
$b5NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 22, 22, 28)
    "s" = [System.Drawing.Color]::FromArgb(255, 188, 152, 118)
    "a" = [System.Drawing.Color]::FromArgb(255, 152, 118, 88)
    "h" = [System.Drawing.Color]::FromArgb(255, 82, 58, 38)
    "m" = [System.Drawing.Color]::FromArgb(255, 52, 38, 28)
    "e" = [System.Drawing.Color]::FromArgb(255, 38, 38, 42)
    "b" = [System.Drawing.Color]::FromArgb(255, 58, 48, 38)
    "c" = [System.Drawing.Color]::FromArgb(255, 72, 92, 62)
    "f" = [System.Drawing.Color]::FromArgb(255, 48, 64, 42)
    "w" = [System.Drawing.Color]::FromArgb(255, 208, 202, 188)
    "k" = [System.Drawing.Color]::FromArgb(255, 128, 98, 58)
    "d" = [System.Drawing.Color]::FromArgb(255, 48, 44, 38)
    "g" = [System.Drawing.Color]::FromArgb(255, 148, 142, 128)
    "p" = [System.Drawing.Color]::FromArgb(255, 138, 128, 98)
    "n" = [System.Drawing.Color]::FromArgb(255, 168, 58, 42)
    "r" = [System.Drawing.Color]::FromArgb(255, 158, 132, 88)
    "t" = [System.Drawing.Color]::FromArgb(255, 178, 148, 98)
}

# ---- DAX, THE MOUNTAINEER: Rugged climber, green coat, climbing gear ----
$dax1 = @(
    "................",
    ".....omnmo......",
    "....omhhhmo.....",
    "....omhhhmo.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffrrffcco...",
    "..ocffkkffcco...",
    "...ocffffcco....",
    "...oppppppo.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$dax2 = @(
    "................",
    ".....omnmo......",
    "....omhhhmo.....",
    "....omhhhmo.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffrrffcco...",
    "..ocffkkffcco...",
    "...ocffffcco....",
    "...oppppppo.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# NPC sheet: 1 NPC x 2 frames = 32 x 16 (padded to 32x32 for consistency)
$b5NpcSheet = New-Sheet -Width 32 -Height 16
Paint-Sprite $b5NpcSheet $dax1   0  0  $b5NpcPalette
Paint-Sprite $b5NpcSheet $dax2   16 0  $b5NpcPalette
$b5NpcPath = Join-Path $outDir "npcs.png"
$b5NpcSheet.Save($b5NpcPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 5 — GRIMFORGE APPROACHES — ITEM PALETTE
# =============================================================================
# Items: Ember Crystal (world item), Heating Tonic, Climbing Rope,
#         Lava Beetle Shell, Ember Crystal Shard, Forgemaster's Postcard
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         e = ember orange     f = flame yellow
#   r = crystal red    d = dark            s = steel grey       w = white
#   b = bottle glass   t = tonic red       g = gold             n = gold dark
#   p = parchment      a = parchment dark  c = copper shell     m = brown rope
#   h = highlight      k = charcoal
$b5ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 22, 22, 28)
    "e" = [System.Drawing.Color]::FromArgb(255, 218, 128, 38)
    "f" = [System.Drawing.Color]::FromArgb(255, 248, 198, 68)
    "r" = [System.Drawing.Color]::FromArgb(255, 188, 58, 32)
    "d" = [System.Drawing.Color]::FromArgb(255, 58, 52, 44)
    "s" = [System.Drawing.Color]::FromArgb(255, 148, 144, 132)
    "w" = [System.Drawing.Color]::FromArgb(255, 228, 224, 212)
    "b" = [System.Drawing.Color]::FromArgb(255, 88, 142, 128)
    "t" = [System.Drawing.Color]::FromArgb(255, 198, 82, 42)
    "g" = [System.Drawing.Color]::FromArgb(255, 208, 178, 58)
    "n" = [System.Drawing.Color]::FromArgb(255, 168, 138, 38)
    "p" = [System.Drawing.Color]::FromArgb(255, 188, 172, 132)
    "a" = [System.Drawing.Color]::FromArgb(255, 148, 132, 98)
    "c" = [System.Drawing.Color]::FromArgb(255, 158, 88, 48)
    "m" = [System.Drawing.Color]::FromArgb(255, 138, 108, 68)
    "h" = [System.Drawing.Color]::FromArgb(255, 238, 218, 168)
    "k" = [System.Drawing.Color]::FromArgb(255, 42, 38, 34)
}

# ---- EMBER CRYSTAL: Glowing orange-red crystal — world item ----
$emberCrystal = @(
    "................",
    ".......f........",
    "......fef.......",
    ".....feeef......",
    "....feeeref.....",
    "....fereref.....",
    "...feereeef.....",
    "....feeeref.....",
    "....fereeef.....",
    "....fereref.....",
    ".....feeef......",
    "......fef.......",
    ".......f........",
    "................",
    "................",
    "................"
)

# ---- HEATING TONIC: Warm potion, orange glow ----
$heatingTonic5 = @(
    "................",
    ".......ww.......",
    "......owwo......",
    "......oddo......",
    ".....odttdo.....",
    "....odtttttdo...",
    "....odttetttdo..",
    "....odtttttdo...",
    "....odtttttdo...",
    "....odttetttdo..",
    "....oddddddo....",
    ".....oooooo.....",
    "................",
    "................",
    "................",
    "................"
)

# ---- CLIMBING ROPE: Coiled brown rope ----
$climbingRope = @(
    "................",
    "......mm........",
    ".....mmmm.......",
    "....mm..mm......",
    "...mm....mm.....",
    "...mm....mm.....",
    "....mm..mm......",
    ".....mmmm.......",
    "......mmmm......",
    ".....mm..mm.....",
    "....mm....mm....",
    "....mm....mm....",
    ".....mm..mm.....",
    "......mmmm......",
    ".......mm.......",
    "................"
)

# ---- LAVA BEETLE SHELL: Dropped crafting material ----
$lavaBeetleShell = @(
    "................",
    "................",
    "......ooo.......",
    ".....occco......",
    "....occcccco....",
    "....occeccco....",
    "....occcccco....",
    "....occeccco....",
    "....occcccco....",
    ".....occco......",
    "......ooo.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- EMBER CRYSTAL SHARD: Small sellable shard ----
$emberShard = @(
    "................",
    "................",
    "........f.......",
    ".......fef......",
    "......feeef.....",
    ".......fef......",
    "........f.......",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- FORGEMASTER'S POSTCARD: Lore collectible ----
$forgemasterPostcard = @(
    "................",
    "....oaao........",
    "...oappao.......",
    "..oappppapo.....",
    "..oappdppapo....",
    "..oapppppapo....",
    "..oappdppapo....",
    "..oaaaaaao......",
    "...oooooo.......",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# Item sheet: 6 items at 16x16 = 48 x 32
$b5ItemSheet = New-Sheet -Width 48 -Height 32
Paint-Sprite $b5ItemSheet $emberCrystal        0  0  $b5ItemPalette
Paint-Sprite $b5ItemSheet $heatingTonic5       16 0  $b5ItemPalette
Paint-Sprite $b5ItemSheet $climbingRope        32 0  $b5ItemPalette
Paint-Sprite $b5ItemSheet $lavaBeetleShell     0  16 $b5ItemPalette
Paint-Sprite $b5ItemSheet $emberShard          16 16 $b5ItemPalette
Paint-Sprite $b5ItemSheet $forgemasterPostcard 32 16 $b5ItemPalette
$b5ItemPath = Join-Path $outDir "items.png"
$b5ItemSheet.Save($b5ItemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated Biome 5 (Grimforge Approaches) sprite sheets:"
Write-Host " - $b5TilePath"
Write-Host " - $b5EnemyPath"
Write-Host " - $b5NpcPath"
Write-Host " - $b5ItemPath"
