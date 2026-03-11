Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome6"
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
# BIOME 6 — THE VOID WASTES — TILE PALETTE
# =============================================================================
# Tone: Unsettling emptiness. Pale grey stone, dead soil, dimensional rifts.
# Wrong sky, wrong stars. No trees, no water.
#
# Key mapping (case-insensitive safe):
#   . = transparent    p = pale stone       d = dead soil        s = stone dark
#   r = rift purple    n = rift deep        c = rift shimmer     w = wrong star
#   k = void black     m = mirror silver    h = highlight        g = grey road
#   a = road dark      e = rift edge glow   f = faint shimmer    b = boundary
#   t = trace shimmer  y = pale yellow
$b6TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "p" = [System.Drawing.Color]::FromArgb(255, 188, 184, 178)
    "d" = [System.Drawing.Color]::FromArgb(255, 148, 142, 132)
    "s" = [System.Drawing.Color]::FromArgb(255, 118, 114, 108)
    "r" = [System.Drawing.Color]::FromArgb(255, 108, 62, 138)
    "n" = [System.Drawing.Color]::FromArgb(255, 58, 28, 78)
    "c" = [System.Drawing.Color]::FromArgb(255, 168, 128, 198)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 212, 228)
    "k" = [System.Drawing.Color]::FromArgb(255, 18, 12, 22)
    "m" = [System.Drawing.Color]::FromArgb(255, 192, 198, 208)
    "h" = [System.Drawing.Color]::FromArgb(255, 208, 204, 198)
    "g" = [System.Drawing.Color]::FromArgb(255, 158, 154, 148)
    "a" = [System.Drawing.Color]::FromArgb(255, 128, 124, 118)
    "e" = [System.Drawing.Color]::FromArgb(255, 148, 108, 178)
    "f" = [System.Drawing.Color]::FromArgb(255, 178, 168, 198)
    "b" = [System.Drawing.Color]::FromArgb(255, 98, 94, 88)
    "t" = [System.Drawing.Color]::FromArgb(255, 158, 148, 178)
    "y" = [System.Drawing.Color]::FromArgb(255, 208, 198, 168)
}

# ---- PALE STONE: Featureless grey ground ----
$paleStone = @(
    "pppppdpppppppppp",
    "pppppppppppdpppp",
    "ppdppppppppppppp",
    "pppppppppdpppppp",
    "pppppdpppppppppp",
    "pppppppppppppdpp",
    "ppdppppppppppppd",
    "pppppppdpppppppp",
    "ppppppppppdppppp",
    "pppdpppppppppppp",
    "pppppppppppdpppp",
    "ppppppdppppppppp",
    "pppppppppppppdpp",
    "pppdpppppppppppp",
    "pppppppdpppppppp",
    "pppppppppppppppp"
)

# ---- DEAD SOIL: Barren lifeless ground ----
$deadSoil = @(
    "dddsdddddddsdddd",
    "ddddddddsdddddd",
    "dsddddddddddddsd",
    "ddddddsdddddddd",
    "dddsdddddddsdddd",
    "ddddddddsdddddd",
    "dsdddddddddddddd",
    "dddddddddddsddd",
    "dddsdddddddddddd",
    "ddddddsdddddddd",
    "dddddddddsdddddd",
    "dsdddddddddddddd",
    "dddsdddddddsdddd",
    "dddddddddddddsd",
    "ddddddsdddddddd",
    "dddddddddddddddd"
)

# ---- STABLE RIFT: Permanent dimensional tear, purple glow ----
$stableRift = @(
    "pppppppppppppppp",
    "ppppppcrppppppp",
    "pppppcrrnpppppp",
    "ppppcrrrncppppp",
    "pppcrrrnncppppp",
    "pppcrrrkrncpppp",
    "pppcrrkkkrncppp",
    "pppcrrrkrncpppp",
    "ppppcrrnncppppp",
    "pppppcrrncppppp",
    "ppppppcrncppppp",
    "pppppppcrppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp"
)

# ---- PALE ROAD: Ancient stone road, may loop ----
$paleRoad = @(
    "ppaaggggggggaapp",
    "ppagggggggggappp",
    "ppaggggggggggapp",
    "pagggghgggghggap",
    "paggggggggggggap",
    "pagggggggggggapp",
    "ppagggggggggappp",
    "ppaggggggggaappp",
    "ppaaggggggggaapp",
    "ppagggggggggappp",
    "pagggghgggghggap",
    "paggggggggggggap",
    "pagggggggggggapp",
    "ppagggggggggappp",
    "ppaggggggggaappp",
    "ppaaggggggaaappp"
)

# ---- SANCTUM APPROACH: Stable geometry near dungeon ----
$sanctumApproach = @(
    "ssssssssssssssss",
    "ssshssssssshssss",
    "ssssssssssssssss",
    "ssssssssssssssss",
    "ssshssssssshssss",
    "ssssssssssssssss",
    "ssssssssssssssss",
    "ssshssssssshssss",
    "ssssssssssssssss",
    "ssssssssssssssss",
    "ssshssssssshssss",
    "ssssssssssssssss",
    "ssssssssssssssss",
    "ssshssssssshssss",
    "ssssssssssssssss",
    "ssssssssssssssss"
)

# ---- SANCTUM ENTRANCE: Vertical rift, 3 tiles tall (icon) ----
$sanctumEntrance = @(
    "ppppppppppppppp",
    "ppppppcrcppppppp",
    "pppppcrrrcpppppp",
    "ppppcrrrrrncpppp",
    "pppcrrknkrncppp",
    "ppcrrkkkknrncpp",
    "ppcrrkkkknrncpp",
    "ppcrrkkkknrncpp",
    "pppcrrknkrncppp",
    "ppppcrrrrrncpppp",
    "pppppcrrrncppppp",
    "ppppppcrcpppppp",
    "ppppppppppppppp",
    "ppppppppppppppp",
    "ppppppppppppppp",
    "ppppppppppppppp"
)

# ---- MIRROR POOL: Silver liquid pool in center ----
$mirrorPool = @(
    "ppppppppppppppp",
    "pppppmmmmppppp",
    "ppppmmmmmmppppp",
    "pppmmmmmmmmppp",
    "pppmmwmmwmmppp",
    "ppmmmmmmmmmmpp",
    "ppmmmmmmmmmmppp",
    "ppmmmmmmmmmmpp",
    "pppmmmmmmmmmppp",
    "pppmmmmmmmmppp",
    "ppppmmmmmmppppp",
    "pppppmmmmppppp",
    "ppppppppppppppp",
    "ppppppppppppppp",
    "ppppppppppppppp",
    "ppppppppppppppp"
)

# ---- VOID SHIMMER TRACE: Faint trail left by Wandering Shade ----
$voidShimmer = @(
    "ppppppppppppppp",
    "ppppppfppppppp",
    "pppppftfppppppp",
    "ppppppfppppppp",
    "ppppppppfppppp",
    "pppppppftfpppp",
    "ppppppppfppppp",
    "ppppppppppfppp",
    "pppppppppftfpp",
    "ppppppppppfppp",
    "ppppppppppppfpp",
    "pppppppppppftpp",
    "ppppppppppppfpp",
    "ppppppppppppppp",
    "ppppppppppppppp",
    "ppppppppppppppp"
)

# ---- WRONG SKY: Background tile with wrong stars ----
$wrongSky = @(
    "kkkkkkkwkkkkkkk",
    "kkkkkkkkkkkkwkkk",
    "kwkkkkkkkkkkkkk",
    "kkkkkkkkkwkkkkk",
    "kkkkwkkkkkkkkkkk",
    "kkkkkkkkkkkwkkkk",
    "kkwkkkkkkkkkkkkk",
    "kkkkkkkkwkkkkkk",
    "kkkkkkkkkkkkwkk",
    "kwkkkkkkkkkkkkkk",
    "kkkkkwkkkkkkkkk",
    "kkkkkkkkkwkkkkkk",
    "kkkkkkkkkkkkwkk",
    "kwkkkkkkkkkkkkkk",
    "kkkkwkkkkkkkkkk",
    "kkkkkkkkkkkwkkkk"
)

# ---- BOUNDARY EDGE: The eerily regular biome border ----
$boundaryEdge = @(
    "bbbbbbbbbbbbbbb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bpppppppppppppb",
    "bbbbbbbbbbbbbbb"
)

# Write tile sheet — 10 tiles
$b6TileSheet = New-Sheet -Width 160 -Height 16
Paint-Sprite $b6TileSheet $paleStone       0   0 $b6TilePalette
Paint-Sprite $b6TileSheet $deadSoil        16  0 $b6TilePalette
Paint-Sprite $b6TileSheet $stableRift      32  0 $b6TilePalette
Paint-Sprite $b6TileSheet $paleRoad        48  0 $b6TilePalette
Paint-Sprite $b6TileSheet $sanctumApproach 64  0 $b6TilePalette
Paint-Sprite $b6TileSheet $sanctumEntrance 80  0 $b6TilePalette
Paint-Sprite $b6TileSheet $mirrorPool      96  0 $b6TilePalette
Paint-Sprite $b6TileSheet $voidShimmer     112 0 $b6TilePalette
Paint-Sprite $b6TileSheet $wrongSky        128 0 $b6TilePalette
Paint-Sprite $b6TileSheet $boundaryEdge    144 0 $b6TilePalette
$b6TilePath = Join-Path $outDir "tiles.png"
$b6TileSheet.Save($b6TilePath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 6 — THE VOID WASTES — ENEMY PALETTE
# =============================================================================
# Enemies: Void Shade (mirrors player), Shadow Clone (looks like player),
#           Wandering Shade (world item carrier), Rift Watcher (eyes in rift)
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         n = void dark        r = rift purple
#   c = shimmer light  k = deep void       w = eye white        e = eye pupil
#   d = dark body      s = shadow body     g = glow purple      m = mid purple
#   b = clone green    a = clone dark      h = highlight        f = beam glow
#   p = pale           t = trace
$b6EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 18, 12, 22)
    "n" = [System.Drawing.Color]::FromArgb(255, 38, 28, 48)
    "r" = [System.Drawing.Color]::FromArgb(255, 108, 62, 138)
    "c" = [System.Drawing.Color]::FromArgb(255, 168, 128, 198)
    "k" = [System.Drawing.Color]::FromArgb(255, 12, 8, 16)
    "w" = [System.Drawing.Color]::FromArgb(255, 228, 218, 238)
    "e" = [System.Drawing.Color]::FromArgb(255, 148, 48, 178)
    "d" = [System.Drawing.Color]::FromArgb(255, 48, 38, 58)
    "s" = [System.Drawing.Color]::FromArgb(255, 68, 58, 78)
    "g" = [System.Drawing.Color]::FromArgb(255, 138, 98, 168)
    "m" = [System.Drawing.Color]::FromArgb(255, 88, 68, 108)
    "b" = [System.Drawing.Color]::FromArgb(255, 68, 98, 72)
    "a" = [System.Drawing.Color]::FromArgb(255, 42, 62, 48)
    "h" = [System.Drawing.Color]::FromArgb(255, 178, 168, 198)
    "f" = [System.Drawing.Color]::FromArgb(255, 188, 148, 218)
    "p" = [System.Drawing.Color]::FromArgb(255, 158, 148, 168)
    "t" = [System.Drawing.Color]::FromArgb(255, 128, 118, 148)
}

# ---- VOID SHADE: Stationary near rifts, mirrors player movement ----
$voidShade1 = @(
    "................",
    ".....onnno......",
    "....onddddno....",
    "...ondddddno....",
    "...ondwedweno...",
    "...onddddddno..",
    "..onddddddddno.",
    "..onddddddddno.",
    "..ondddddddno..",
    "...ondddddno...",
    "...onddddno....",
    "....ondno.......",
    "....on.no.......",
    "...on...no......",
    "................",
    "................"
)

$voidShade2 = @(
    "................",
    "......onnno.....",
    ".....onddddno...",
    "....ondddddno...",
    "....ondwedweno..",
    "....onddddddno.",
    "...onddddddddno",
    "...onddddddddno",
    "...ondddddddno.",
    "....ondddddno..",
    "....onddddno...",
    ".....ondno......",
    ".....on.no......",
    "....on...no.....",
    "................",
    "................"
)

# ---- SHADOW CLONE: Looks like player silhouette, void-shimmer edges ----
$shadowClone1 = @(
    "................",
    ".....cssc.......",
    "....cssssc......",
    "....cssssc......",
    "...cassssac.....",
    "...casssac......",
    "...csssssc......",
    "..cssssssssc....",
    "..csssssssc.....",
    "..cssssssssc....",
    "...csssssc......",
    "...cssssc.......",
    "....cs.sc.......",
    "...cs...sc......",
    "....c...c.......",
    "................"
)

$shadowClone2 = @(
    "................",
    "......cssc......",
    ".....cssssc.....",
    ".....cssssc.....",
    "....cassssac....",
    "....casssac.....",
    "....csssssc.....",
    "...cssssssssc...",
    "...csssssssc....",
    "...cssssssssc...",
    "....csssssc.....",
    "....cssssc......",
    ".....c.ssc......",
    "....cs...sc.....",
    ".....c...c......",
    "................"
)

# ---- WANDERING SHADE: World item carrier, teleports, void-beam ----
$wanderingShade1 = @(
    "................",
    "....oggggo......",
    "...oggmmggoe....",
    "..ogmmmmmgoe....",
    "..ogmwemwego....",
    "..ogmmmmmmmgo...",
    ".ogmmmmmmmmgo...",
    ".ogmmmfmmmmgo...",
    ".ogmmfffmmmgo...",
    "..ogmmmmmmmgo...",
    "..ogmmmmmgoo....",
    "...ogmmmgo......",
    "...ogm.mgo......",
    "..ogm...mgo.....",
    "...oo...oo......",
    "................"
)

$wanderingShade2 = @(
    "................",
    ".....oggggo.....",
    "...eogmmmggoe...",
    "...eogmmmmmgoe..",
    "...ogmwemwego...",
    "...ogmmmmmmmgo..",
    "..ogmmmmmmmmgo..",
    "..ogmmfmmmmmgo..",
    "..ogmmfffmmmgo..",
    "...ogmmmmmmmgo..",
    "...ogmmmmmgoo...",
    "....ogmmmgo.....",
    "....og.mmgo.....",
    "...ogm...mgo....",
    "....oo...oo.....",
    "................"
)

# ---- RIFT WATCHER: Eyes within a stable rift, reaches through ----
$riftWatcher1 = @(
    "................",
    ".....crrc.......",
    "....crrnrc......",
    "...crrnnnrc.....",
    "..crrnnnnnrc....",
    "..crnwewnwrc....",
    "..crnnnnnnrc....",
    "..crrnnnnnrc....",
    "...crrnnnrc.....",
    "....crrnrc......",
    ".....crrc.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$riftWatcher2 = @(
    "................",
    ".....crrc.......",
    "....crrnrc......",
    "...crrnnnrc.....",
    "..crrnnnnnrc....",
    "..crwennwenrc...",
    "..crnnnnnnrc....",
    "..crrnnnnnrc....",
    "...crrnnnrc.....",
    "....crrnrc......",
    ".....crrc.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# Enemy sheet: 4 enemies x 2 frames = 32 x 64
$b6EnemySheet = New-Sheet -Width 32 -Height 64
Paint-Sprite $b6EnemySheet $voidShade1       0  0  $b6EnemyPalette
Paint-Sprite $b6EnemySheet $voidShade2       16 0  $b6EnemyPalette
Paint-Sprite $b6EnemySheet $shadowClone1     0  16 $b6EnemyPalette
Paint-Sprite $b6EnemySheet $shadowClone2     16 16 $b6EnemyPalette
Paint-Sprite $b6EnemySheet $wanderingShade1  0  32 $b6EnemyPalette
Paint-Sprite $b6EnemySheet $wanderingShade2  16 32 $b6EnemyPalette
Paint-Sprite $b6EnemySheet $riftWatcher1     0  48 $b6EnemyPalette
Paint-Sprite $b6EnemySheet $riftWatcher2     16 48 $b6EnemyPalette
$b6EnemyPath = Join-Path $outDir "enemies.png"
$b6EnemySheet.Save($b6EnemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 6 — THE VOID WASTES — NPC PALETTE
# =============================================================================
# Vel the Rift Researcher: Academic, distressed, field equipment.
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         s = skin            a = skin shadow
#   h = hair dark      m = glasses frame   e = eyes            b = boots
#   c = coat purple    f = coat dark       w = white shirt     k = belt
#   d = dark gear      g = goggles lens    p = pants grey      n = notebook
#   r = rift device    t = trim
$b6NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 18, 12, 22)
    "s" = [System.Drawing.Color]::FromArgb(255, 198, 168, 138)
    "a" = [System.Drawing.Color]::FromArgb(255, 168, 138, 108)
    "h" = [System.Drawing.Color]::FromArgb(255, 38, 32, 28)
    "m" = [System.Drawing.Color]::FromArgb(255, 88, 82, 78)
    "e" = [System.Drawing.Color]::FromArgb(255, 42, 38, 48)
    "b" = [System.Drawing.Color]::FromArgb(255, 48, 42, 38)
    "c" = [System.Drawing.Color]::FromArgb(255, 78, 58, 98)
    "f" = [System.Drawing.Color]::FromArgb(255, 52, 38, 68)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 212, 202)
    "k" = [System.Drawing.Color]::FromArgb(255, 108, 88, 68)
    "d" = [System.Drawing.Color]::FromArgb(255, 48, 42, 52)
    "g" = [System.Drawing.Color]::FromArgb(255, 138, 108, 168)
    "p" = [System.Drawing.Color]::FromArgb(255, 82, 78, 72)
    "n" = [System.Drawing.Color]::FromArgb(255, 168, 152, 118)
    "r" = [System.Drawing.Color]::FromArgb(255, 128, 88, 158)
    "t" = [System.Drawing.Color]::FromArgb(255, 148, 118, 88)
}

# ---- VEL, THE RIFT RESEARCHER: Academic with glasses, purple field coat ----
$vel1 = @(
    "................",
    ".....ohhho......",
    "....ohhhhhoo....",
    "....omhmhmo.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffnrffcco...",
    "..ocffkkffcco...",
    "...ocffffcco....",
    "...oppppppo.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$vel2 = @(
    "................",
    ".....ohhho......",
    "....ohhhhhoo....",
    "....omhmhmo.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffnrffcco...",
    "..ocffkkffcco...",
    "...ocffffcco....",
    "...oppppppo.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# NPC sheet: 1 NPC x 2 frames = 32 x 16
$b6NpcSheet = New-Sheet -Width 32 -Height 16
Paint-Sprite $b6NpcSheet $vel1   0  0  $b6NpcPalette
Paint-Sprite $b6NpcSheet $vel2   16 0  $b6NpcPalette
$b6NpcPath = Join-Path $outDir "npcs.png"
$b6NpcSheet.Save($b6NpcPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 6 — THE VOID WASTES — ITEM PALETTE
# =============================================================================
# Items: Void Compass (world item, dropped), Void Compass Map (sold by Vel),
#         Antidote Tonic, Rift Record (lore), Gold Pouch, Healing Herb
#
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         r = compass purple   n = compass dark
#   c = needle red     s = silver          w = white            g = gold
#   d = dark           p = parchment       a = parchment dark   b = bottle glass
#   t = tonic green    e = glow purple     m = map brown        h = highlight
#   f = frame          k = void shimmer
$b6ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 18, 12, 22)
    "r" = [System.Drawing.Color]::FromArgb(255, 108, 62, 138)
    "n" = [System.Drawing.Color]::FromArgb(255, 58, 28, 78)
    "c" = [System.Drawing.Color]::FromArgb(255, 188, 48, 42)
    "s" = [System.Drawing.Color]::FromArgb(255, 178, 178, 188)
    "w" = [System.Drawing.Color]::FromArgb(255, 228, 224, 218)
    "g" = [System.Drawing.Color]::FromArgb(255, 208, 178, 58)
    "d" = [System.Drawing.Color]::FromArgb(255, 48, 42, 52)
    "p" = [System.Drawing.Color]::FromArgb(255, 178, 162, 128)
    "a" = [System.Drawing.Color]::FromArgb(255, 138, 122, 92)
    "b" = [System.Drawing.Color]::FromArgb(255, 88, 128, 108)
    "t" = [System.Drawing.Color]::FromArgb(255, 72, 148, 88)
    "e" = [System.Drawing.Color]::FromArgb(255, 148, 108, 178)
    "m" = [System.Drawing.Color]::FromArgb(255, 128, 104, 68)
    "h" = [System.Drawing.Color]::FromArgb(255, 198, 188, 208)
    "f" = [System.Drawing.Color]::FromArgb(255, 88, 78, 108)
    "k" = [System.Drawing.Color]::FromArgb(255, 168, 148, 198)
}

# ---- VOID COMPASS: Purple/silver compass — world item ----
$voidCompass = @(
    "................",
    ".....frrrf......",
    "....frrrrrf.....",
    "...frrrcrrrrf...",
    "...frrcccrrrf...",
    "...frrcscrrf....",
    "...frrrcrrrrf...",
    "...frrrrrrrf....",
    "....frrrrrrf....",
    ".....fffff......",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- VOID COMPASS MAP: Shade patrol route map ----
$voidCompassMap = @(
    "................",
    "....oaao........",
    "...oappao.......",
    "..oappppapo.....",
    "..oaperppapo....",
    "..oappreppo.....",
    "..oaperppapo....",
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

# ---- ANTIDOTE TONIC: Green potion ----
$antidoteTonic6 = @(
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

# ---- RIFT RECORD: Lore scroll, void-tinted ----
$riftRecord = @(
    "................",
    "....oaao........",
    "...oappao.......",
    "..oappppapo.....",
    "..oapedppapo....",
    "..oappeppapo....",
    "..oapedppapo....",
    "..oappeppapo....",
    "..oapedpao......",
    "..oaaaaaao......",
    "...oooooo.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- GOLD POUCH: Standard currency drop ----
$voidGoldPouch = @(
    "................",
    "......gg........",
    ".....gggg.......",
    "....gggggg......",
    "...gggggggg.....",
    "...dggggggd.....",
    "...dggggggd.....",
    "...dggggggd.....",
    "...dgggggd......",
    "....dggd........",
    ".....dd.........",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- HEALING HERB: Standard green herb ----
$voidHealingHerb = @(
    "................",
    "......tt........",
    ".....ttbt.......",
    "....tbbbt.......",
    "...tbtttbt......",
    "...tbtttbt......",
    "....tbbbt.......",
    ".....tbt........",
    "......b.........",
    "......b.........",
    ".....bbb........",
    "......b.........",
    "................",
    "................",
    "................",
    "................"
)

# Item sheet: 6 items at 16x16 = 48 x 32
$b6ItemSheet = New-Sheet -Width 48 -Height 32
Paint-Sprite $b6ItemSheet $voidCompass     0  0  $b6ItemPalette
Paint-Sprite $b6ItemSheet $voidCompassMap  16 0  $b6ItemPalette
Paint-Sprite $b6ItemSheet $antidoteTonic6  32 0  $b6ItemPalette
Paint-Sprite $b6ItemSheet $riftRecord      0  16 $b6ItemPalette
Paint-Sprite $b6ItemSheet $voidGoldPouch   16 16 $b6ItemPalette
Paint-Sprite $b6ItemSheet $voidHealingHerb 32 16 $b6ItemPalette
$b6ItemPath = Join-Path $outDir "items.png"
$b6ItemSheet.Save($b6ItemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated Biome 6 (Void Wastes) sprite sheets:"
Write-Host " - $b6TilePath"
Write-Host " - $b6EnemyPath"
Write-Host " - $b6NpcPath"
Write-Host " - $b6ItemPath"
