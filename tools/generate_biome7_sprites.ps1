Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome7"
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
# BIOME 7 — THE CELESTIAL PLATEAU — TILE PALETTE
# =============================================================================
# Tone: Rarefied calm. Smooth white stone, perpetually clear sky, the Spire
# visible from everywhere. No weather, no cover. Cleanest biome.
#
# Key mapping (case-insensitive safe):
#   . = transparent    w = white stone      s = stone shadow     h = highlight
#   c = carved line    d = dark accent      b = blue sky         n = night blue
#   g = gold star      r = spire white      k = spire shadow     m = moat water
#   a = arch glow      e = star bright      f = lantern blue     p = platform
#   t = star chart     y = celestial glow
$b7TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "w" = [System.Drawing.Color]::FromArgb(255, 232, 228, 222)
    "s" = [System.Drawing.Color]::FromArgb(255, 198, 194, 188)
    "h" = [System.Drawing.Color]::FromArgb(255, 248, 246, 242)
    "c" = [System.Drawing.Color]::FromArgb(255, 168, 164, 158)
    "d" = [System.Drawing.Color]::FromArgb(255, 138, 134, 128)
    "b" = [System.Drawing.Color]::FromArgb(255, 108, 158, 208)
    "n" = [System.Drawing.Color]::FromArgb(255, 28, 38, 68)
    "g" = [System.Drawing.Color]::FromArgb(255, 228, 208, 108)
    "r" = [System.Drawing.Color]::FromArgb(255, 242, 238, 232)
    "k" = [System.Drawing.Color]::FromArgb(255, 208, 204, 198)
    "m" = [System.Drawing.Color]::FromArgb(255, 128, 178, 218)
    "a" = [System.Drawing.Color]::FromArgb(255, 218, 228, 248)
    "e" = [System.Drawing.Color]::FromArgb(255, 248, 238, 168)
    "f" = [System.Drawing.Color]::FromArgb(255, 108, 148, 218)
    "p" = [System.Drawing.Color]::FromArgb(255, 218, 214, 208)
    "t" = [System.Drawing.Color]::FromArgb(255, 178, 174, 168)
    "y" = [System.Drawing.Color]::FromArgb(255, 228, 238, 248)
}

# ---- WHITE PLAIN: Smooth white stone, open, unobstructed ----
$whitePlain = @(
    "wwwwswwwwwwwswww",
    "wwwwwwwwwswwwwww",
    "wswwwwwwwwwwwwww",
    "wwwwwwwswwwwwsww",
    "wwwwswwwwwwwwwww",
    "wwwwwwwwwwswwwww",
    "wswwwwwwwwwwwwsw",
    "wwwwwwwswwwwwwww",
    "wwwwwwwwwwswwwww",
    "wwswwwwwwwwwwwww",
    "wwwwwwwwswwwwwww",
    "wwwwwswwwwwwswww",
    "wwwwwwwwwwwwwwww",
    "wswwwwwwwwwwwsww",
    "wwwwwswwwwwwwwww",
    "wwwwwwwwwswwwwww"
)

# ---- SPIRE BASE: The platform at the Aetherian Spire's base ----
$spireBase = @(
    "pppppppppppppppp",
    "pppphpppppphpppp",
    "pppppppppppppppp",
    "pppppprrrrpppppp",
    "ppppprkkkrppppp",
    "ppppprkakrppppp",
    "ppppprkkrkppppp",
    "pppppprrrrpppppp",
    "pppppppppppppppp",
    "pppphpppppphpppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp"
)

# ---- SPIRE ARCHWAY: The grand sealed dungeon entry, glowing ----
$spireArchway = @(
    "......yyyy......",
    ".....yaaaay.....",
    "....yaddday.....",
    "....yaddddy.....",
    "...yaddddday....",
    "...yaddddddy....",
    "...yaddddddy....",
    "...yaddddddy....",
    "...yaddddddy....",
    "...yaddddddy....",
    "...yaddddddy....",
    "...yaddddddy....",
    "..pyadddddayp...",
    ".ppyadddddaypp..",
    "pppyaaaaaaypppp",
    "pppppppppppppppp"
)

# ---- SKY MOAT: Water around the Spire base, Raft required ----
$skyMoat = @(
    "mmmmmmmmmmmmmmm",
    "mmmmhmmmmmhmmmmm",
    "mmmmmmmmmmmmmmm",
    "mmhmmmmmmmmhmmmm",
    "mmmmmmmmmmmmmmm",
    "mmmmmhmmmhmmmmm",
    "mmmmmmmmmmmmmmm",
    "mmhmmmmmmmmhmmmm",
    "mmmmmmmmmmmmmmm",
    "mmmmmhmmmhmmmmm",
    "mmmmmmmmmmmmmmm",
    "mmhmmmmmmmmhmmm",
    "mmmmmmmmmmmmmmm",
    "mmmmmhmmmmhmmmm",
    "mmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmm"
)

# ---- STAR MAP PLAZA: Carved star chart in plateau surface ----
$starMapPlaza = @(
    "cccccccccccccccc",
    "cwtcwwtcwwwtcwwc",
    "cwwcwwwcwwwwcwwc",
    "cttwctwtwttctwtc",
    "cwwwcwwwcwwwcwwc",
    "cwtwcwtwtcwwctwc",
    "cwwwcwwwwcwwcwwc",
    "cttwcttwtcttcttc",
    "cwwwcwwwcwwwcwwc",
    "cwtwcwtwcwwwctwc",
    "cwwwcwwwcwwwcwwc",
    "cwtcwwwtcwwtcwtc",
    "cwwcwwwwcwwwcwwc",
    "cttwcttwcttcttcc",
    "cwwwcwwwcwwwcwwc",
    "cccccccccccccccc"
)

# ---- MERCHANT LANTERN: Blue lantern at north edge, nightfall marker ----
$merchantLantern = @(
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwffwwwwwwww",
    "wwwwwffffwwwwwww",
    "wwwwwfbbfwwwwwww",
    "wwwwwfbbfwwwwwww",
    "wwwwwffff wwwwww",
    "wwwwwwffwwwwwwww",
    "wwwwwwddwwwwwwww",
    "wwwwwwddwwwwwwww",
    "wwwwwwddwwwwwwww",
    "wwwwwwddwwwwwwww",
    "wwwwwddddwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww"
)

# ---- NIGHT SKY: Dark sky tile for night cycle, stars correct ----
$nightSky = @(
    "nnnnennnnnnennnn",
    "nnnnnnnnnnnnnnen",
    "nennnnnnnnnnnnnn",
    "nnnnnnnennnnnnnn",
    "nnnnennnnnnnnnnn",
    "nnnnnnnnnnnennnn",
    "nennnnnnnnnnnnnn",
    "nnnnnnnnennnnnn",
    "nnnnnnnnnnnnenn",
    "nennnnnnnnnnnnnn",
    "nnnnnennnnnnnnn",
    "nnnnnnnnnennnnn",
    "nnnnnnnnnnnnennn",
    "nennnnnnnnnnnnnn",
    "nnnnennnnnnnnnn",
    "nnnnnnnnnnnennn"
)

# ---- PLATEAU EDGE: Sheer drop, the horizon curves ----
$plateauEdge = @(
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "ssssssssssssssss",
    "dddddddddddddddd",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbb"
)

# ---- DRAGON CARVING: Full dragon image at Spire base, water level ----
$dragonCarving = @(
    "mmmmmmmmmmmmmmmm",
    "mmmdddddddddmmm",
    "mmdcccccccccddmm",
    "mmcgcccccccgcdm",
    "mmdccgccgcccddm",
    "mmccccccccccccdm",
    "mmdccccccccddmm",
    "mmmdddddddddmmmm",
    "mmmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmmm",
    "mmmmmmmmmmmmmmmm"
)

# Write tile sheet — 10 tiles
$b7TileSheet = New-Sheet -Width 160 -Height 16
Paint-Sprite $b7TileSheet $whitePlain      0   0 $b7TilePalette
Paint-Sprite $b7TileSheet $spireBase       16  0 $b7TilePalette
Paint-Sprite $b7TileSheet $spireArchway    32  0 $b7TilePalette
Paint-Sprite $b7TileSheet $skyMoat         48  0 $b7TilePalette
Paint-Sprite $b7TileSheet $starMapPlaza    64  0 $b7TilePalette
Paint-Sprite $b7TileSheet $merchantLantern 80  0 $b7TilePalette
Paint-Sprite $b7TileSheet $nightSky        96  0 $b7TilePalette
Paint-Sprite $b7TileSheet $plateauEdge     112 0 $b7TilePalette
Paint-Sprite $b7TileSheet $dragonCarving   128 0 $b7TilePalette
Paint-Sprite $b7TileSheet $whitePlain      144 0 $b7TilePalette  # alternate
$b7TilePath = Join-Path $outDir "tiles.png"
$b7TileSheet.Save($b7TilePath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 7 — THE CELESTIAL PLATEAU — ENEMY PALETTE
# =============================================================================
# Enemies: Star Sentinel (light-lance), Night Drifter (night only, passes through),
#           Comet Shard (day only, aerial projectile)
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         w = white body       s = silver
#   g = gold glow      d = dark accent     k = black            b = blue glow
#   n = night dark     r = red core        c = comet trail      h = highlight
#   a = armor white    e = eye bright      m = moon pale        f = frost blue
#   p = purple tint    t = star yellow
$b7EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 48, 48, 58)
    "w" = [System.Drawing.Color]::FromArgb(255, 232, 228, 222)
    "s" = [System.Drawing.Color]::FromArgb(255, 198, 194, 188)
    "g" = [System.Drawing.Color]::FromArgb(255, 228, 208, 108)
    "d" = [System.Drawing.Color]::FromArgb(255, 138, 134, 128)
    "k" = [System.Drawing.Color]::FromArgb(255, 28, 28, 38)
    "b" = [System.Drawing.Color]::FromArgb(255, 108, 148, 218)
    "n" = [System.Drawing.Color]::FromArgb(255, 38, 42, 68)
    "r" = [System.Drawing.Color]::FromArgb(255, 198, 68, 58)
    "c" = [System.Drawing.Color]::FromArgb(255, 178, 198, 228)
    "h" = [System.Drawing.Color]::FromArgb(255, 248, 246, 242)
    "a" = [System.Drawing.Color]::FromArgb(255, 218, 214, 208)
    "e" = [System.Drawing.Color]::FromArgb(255, 248, 238, 168)
    "m" = [System.Drawing.Color]::FromArgb(255, 188, 188, 208)
    "f" = [System.Drawing.Color]::FromArgb(255, 148, 178, 218)
    "p" = [System.Drawing.Color]::FromArgb(255, 148, 128, 178)
    "t" = [System.Drawing.Color]::FromArgb(255, 238, 228, 128)
}

# ---- STAR SENTINEL: White/gold armored, straight-line, light-lance ----
$starSentinel1 = @(
    "................",
    ".....oaaao......",
    "....oawwwao.....",
    "....oawgwao.....",
    "...oawekewao....",
    "...oawwwwwao....",
    "...oaaaaaao.....",
    "..oawwwwwwao....",
    "..oawwgwwwao....",
    "..oawwwwwwao....",
    "...oawwwwao.....",
    "...oawwwao......",
    "....oa.ao.......",
    "...oa...ao......",
    "....o...o.......",
    "................"
)

$starSentinel2 = @(
    "................",
    "......oaaao.....",
    ".....oawwwao....",
    ".....oawgwao....",
    "....oawekewao...",
    "....oawwwwwao...",
    "....oaaaaaao....",
    "...oawwwwwwao...",
    "...oawwgwwwao...",
    "...oawwwwwwao...",
    "....oawwwwao....",
    "....oawwwao.....",
    ".....oa.ao......",
    "....oa...ao.....",
    ".....o...o......",
    "................"
)

# ---- NIGHT DRIFTER: Dark ethereal, appears only at night ----
$nightDrifter1 = @(
    "................",
    "....onnno.......",
    "...onmmmmno.....",
    "..onmmmmmmmno...",
    "..onmfmfmmno....",
    "..onmmmmmmmno...",
    ".onmmmmmmmmno...",
    "..onmmmmmmmno...",
    "...onmmmmmno....",
    "....onmmmno.....",
    ".....onmno......",
    "......ono.......",
    ".......o........",
    "................",
    "................",
    "................"
)

$nightDrifter2 = @(
    "................",
    ".....onnno......",
    "....onmmmmno....",
    "...onmmmmmmmno..",
    "...onmfmfmmno...",
    "...onmmmmmmmno..",
    "..onmmmmmmmmno..",
    "...onmmmmmmmno..",
    "....onmmmmmno...",
    ".....onmmmno....",
    "......onmno.....",
    ".......ono......",
    "................",
    "................",
    "................",
    "................"
)

# ---- COMET SHARD: Fast aerial projectile, day only ----
$cometShard1 = @(
    "................",
    "................",
    "................",
    "................",
    "..........ot....",
    ".........oteto..",
    "........oteeto..",
    ".......oteteo...",
    "......oteto.....",
    ".....oteto......",
    "....ote.........",
    "...oc...........",
    "..c.............",
    "................",
    "................",
    "................"
)

$cometShard2 = @(
    "................",
    "................",
    "................",
    "................",
    "...........ot...",
    "..........oteto.",
    ".........oteeto.",
    "........oteteo..",
    ".......oteto....",
    "......oteto.....",
    ".....ote........",
    "....oc..........",
    "...c............",
    "................",
    "................",
    "................"
)

# Enemy sheet: 3 enemies x 2 frames = 32 x 48
$b7EnemySheet = New-Sheet -Width 32 -Height 48
Paint-Sprite $b7EnemySheet $starSentinel1  0  0  $b7EnemyPalette
Paint-Sprite $b7EnemySheet $starSentinel2  16 0  $b7EnemyPalette
Paint-Sprite $b7EnemySheet $nightDrifter1  0  16 $b7EnemyPalette
Paint-Sprite $b7EnemySheet $nightDrifter2  16 16 $b7EnemyPalette
Paint-Sprite $b7EnemySheet $cometShard1    0  32 $b7EnemyPalette
Paint-Sprite $b7EnemySheet $cometShard2    16 32 $b7EnemyPalette
$b7EnemyPath = Join-Path $outDir "enemies.png"
$b7EnemySheet.Save($b7EnemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 7 — THE CELESTIAL PLATEAU — NPC PALETTE
# =============================================================================
# Celestial Merchant: Ancient trader, night only, unsettling calm.
# Senna: Young astronomer, lives on plateau year-round.
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         s = skin            a = skin shadow
#   h = hair silver    m = hair dark       e = eyes            b = boots
#   c = robe white     f = robe shadow     w = undershirt      k = belt gold
#   d = cloak blue     g = cloak dark      p = pants           n = scarf
#   r = staff wood     t = trim
$b7NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 28, 28, 38)
    "s" = [System.Drawing.Color]::FromArgb(255, 208, 182, 148)
    "a" = [System.Drawing.Color]::FromArgb(255, 178, 152, 118)
    "h" = [System.Drawing.Color]::FromArgb(255, 198, 194, 188)
    "m" = [System.Drawing.Color]::FromArgb(255, 58, 48, 38)
    "e" = [System.Drawing.Color]::FromArgb(255, 38, 38, 48)
    "b" = [System.Drawing.Color]::FromArgb(255, 52, 48, 42)
    "c" = [System.Drawing.Color]::FromArgb(255, 228, 224, 218)
    "f" = [System.Drawing.Color]::FromArgb(255, 198, 194, 188)
    "w" = [System.Drawing.Color]::FromArgb(255, 238, 234, 228)
    "k" = [System.Drawing.Color]::FromArgb(255, 198, 178, 88)
    "d" = [System.Drawing.Color]::FromArgb(255, 68, 98, 148)
    "g" = [System.Drawing.Color]::FromArgb(255, 42, 68, 108)
    "p" = [System.Drawing.Color]::FromArgb(255, 88, 82, 72)
    "n" = [System.Drawing.Color]::FromArgb(255, 148, 178, 208)
    "r" = [System.Drawing.Color]::FromArgb(255, 128, 98, 62)
    "t" = [System.Drawing.Color]::FromArgb(255, 168, 148, 88)
}

# ---- CELESTIAL MERCHANT: Silver hair, white robe, gold belt, blue cloak ----
$celestialMerchant1 = @(
    "................",
    ".....ohhho......",
    "....ohhhhho.....",
    "....ohhhhho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...odcffcdo.....",
    "..odcfwwfcdoo...",
    "..odcfkkfcdoo...",
    "..odcffffcdoo...",
    "...odcffcdoo....",
    "...odccccdoo....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$celestialMerchant2 = @(
    "................",
    ".....ohhho......",
    "....ohhhhho.....",
    "....ohhhhho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...odcffcdo.....",
    "..odcfwwfcdoo...",
    "..odcfkkfcdoo...",
    "..odcffffcdoo...",
    "...odcffcdoo....",
    "...odccccdoo....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# ---- SENNA, THE STARGAZER: Young astronomer, dark hair, blue scarf ----
$senna1 = @(
    "................",
    ".....ommmo......",
    "....ommmmmoo....",
    "....ommmmmoo....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...onnffnnoo....",
    "..onnfwwfnnoo...",
    "..onfffffnoo....",
    "..onffffffnoo...",
    "...onnffnnoo....",
    "...opppppoo.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$senna2 = @(
    "................",
    ".....ommmo......",
    "....ommmmmoo....",
    "....ommmmmoo....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...onnffnnoo....",
    "..onnfwwfnnoo...",
    "..onfffffnoo....",
    "..onffffffnoo...",
    "...onnffnnoo....",
    "...opppppoo.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# NPC sheet: 2 NPCs x 2 frames = 32 x 32
$b7NpcSheet = New-Sheet -Width 32 -Height 32
Paint-Sprite $b7NpcSheet $celestialMerchant1  0  0  $b7NpcPalette
Paint-Sprite $b7NpcSheet $celestialMerchant2  16 0  $b7NpcPalette
Paint-Sprite $b7NpcSheet $senna1              0  16 $b7NpcPalette
Paint-Sprite $b7NpcSheet $senna2              16 16 $b7NpcPalette
$b7NpcPath = Join-Path $outDir "npcs.png"
$b7NpcSheet.Save($b7NpcPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 7 — THE CELESTIAL PLATEAU — ITEM PALETTE
# =============================================================================
# Items: Star Sigil (world item, purchased), Moon Essence (Portal upgrade),
#         Celestial Map, Healing Herb bundle, Builder's Note, Gold Pouch
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         g = gold star        n = gold dark
#   s = silver         w = white           d = dark             b = blue glow
#   p = parchment      a = parchment dark  c = crystal          e = glow bright
#   m = moon pale      h = herb green      r = herb dark        k = charcoal
#   f = frame          t = trim
$b7ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 28, 28, 38)
    "g" = [System.Drawing.Color]::FromArgb(255, 228, 208, 108)
    "n" = [System.Drawing.Color]::FromArgb(255, 188, 168, 68)
    "s" = [System.Drawing.Color]::FromArgb(255, 198, 194, 188)
    "w" = [System.Drawing.Color]::FromArgb(255, 238, 234, 228)
    "d" = [System.Drawing.Color]::FromArgb(255, 68, 64, 58)
    "b" = [System.Drawing.Color]::FromArgb(255, 108, 148, 218)
    "p" = [System.Drawing.Color]::FromArgb(255, 188, 172, 132)
    "a" = [System.Drawing.Color]::FromArgb(255, 148, 132, 98)
    "c" = [System.Drawing.Color]::FromArgb(255, 168, 188, 228)
    "e" = [System.Drawing.Color]::FromArgb(255, 248, 238, 168)
    "m" = [System.Drawing.Color]::FromArgb(255, 198, 198, 218)
    "h" = [System.Drawing.Color]::FromArgb(255, 72, 138, 82)
    "r" = [System.Drawing.Color]::FromArgb(255, 48, 98, 52)
    "k" = [System.Drawing.Color]::FromArgb(255, 42, 38, 34)
    "f" = [System.Drawing.Color]::FromArgb(255, 148, 144, 138)
    "t" = [System.Drawing.Color]::FromArgb(255, 168, 148, 88)
}

# ---- STAR SIGIL: Glowing gold star emblem — world item ----
$starSigil = @(
    "................",
    ".......e........",
    "......ege.......",
    ".....egege......",
    "....eggngge.....",
    "...eggngnge.....",
    "eeegggngggeeee..",
    "..enngggnnge....",
    "...engggnge.....",
    "...engggne......",
    "....egnge.......",
    "...eg...ge......",
    "..eg.....ge.....",
    "................",
    "................",
    "................"
)

# ---- MOON ESSENCE: Pale glowing orb ----
$moonEssence = @(
    "................",
    "................",
    "......mmm.......",
    ".....mcccm......",
    "....mccccccm....",
    "....mccecccm....",
    "....mccccccm....",
    "....mccceccm....",
    "....mccccccm....",
    ".....mcccm......",
    "......mmm.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- CELESTIAL MAP: Detailed Spire layout scroll ----
$celestialMap = @(
    "................",
    "....oaao........",
    "...oappao.......",
    "..oappppapo.....",
    "..oapbdppapo....",
    "..oappbppapo....",
    "..oapbdppapo....",
    "..oappbppapo....",
    "..oapbdpao......",
    "..oaaaaaao......",
    "...oooooo.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- HEALING HERB BUNDLE: 5-pack green herbs ----
$healingBundle = @(
    "................",
    ".....hh.hh......",
    "....hrh.hrh.....",
    "...hrhh.hrhh....",
    "....hh...hh.....",
    ".hh..........hh.",
    "hrh..........hrh",
    "hrhh........hrhh",
    ".hh..........hh.",
    "......hh........",
    ".....hrh........",
    "....hrhh........",
    ".....hh.........",
    "................",
    "................",
    "................"
)

# ---- BUILDER'S NOTE: Lore item, old parchment ----
$buildersNote = @(
    "................",
    "....oaao........",
    "...oappao.......",
    "..oappppapo.....",
    "..oapdppapo.....",
    "..oapppppo......",
    "..oapddppo......",
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

# ---- GOLD POUCH: Standard gold ----
$celestialGold = @(
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

# Item sheet: 6 items at 16x16 = 48 x 32
$b7ItemSheet = New-Sheet -Width 48 -Height 32
Paint-Sprite $b7ItemSheet $starSigil      0  0  $b7ItemPalette
Paint-Sprite $b7ItemSheet $moonEssence    16 0  $b7ItemPalette
Paint-Sprite $b7ItemSheet $celestialMap   32 0  $b7ItemPalette
Paint-Sprite $b7ItemSheet $healingBundle  0  16 $b7ItemPalette
Paint-Sprite $b7ItemSheet $buildersNote   16 16 $b7ItemPalette
Paint-Sprite $b7ItemSheet $celestialGold  32 16 $b7ItemPalette
$b7ItemPath = Join-Path $outDir "items.png"
$b7ItemSheet.Save($b7ItemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated Biome 7 (Celestial Plateau) sprite sheets:"
Write-Host " - $b7TilePath"
Write-Host " - $b7EnemyPath"
Write-Host " - $b7NpcPath"
Write-Host " - $b7ItemPath"
