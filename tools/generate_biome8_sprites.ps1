Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome8"
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
# BIOME 8 — THE DRAGON'S APPROACH — TILE PALETTE
# =============================================================================
# Tone: Austere, solemn, narrow mountain valley. Grey-brown stone, carvings on
# walls, pilgrims' road. The last biome — sparse, reverent, quiet.
#
# Key mapping (case-insensitive safe):
#   . = transparent    s = stone grey        d = dark stone       h = highlight
#   c = carved line    w = wall shadow       r = road surface     p = path edge
#   g = grass tuft     b = brown earth       n = name carving     m = memory alcove
#   a = arch stone     e = entrance dark     f = fire glow        k = camp wood
#   t = threshold      y = sky blue          v = valley floor
$b8TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "s" = [System.Drawing.Color]::FromArgb(255, 148, 142, 132)
    "d" = [System.Drawing.Color]::FromArgb(255, 98, 92, 82)
    "h" = [System.Drawing.Color]::FromArgb(255, 178, 172, 162)
    "c" = [System.Drawing.Color]::FromArgb(255, 118, 112, 102)
    "w" = [System.Drawing.Color]::FromArgb(255, 78, 72, 62)
    "r" = [System.Drawing.Color]::FromArgb(255, 168, 158, 142)
    "p" = [System.Drawing.Color]::FromArgb(255, 138, 128, 112)
    "g" = [System.Drawing.Color]::FromArgb(255, 88, 108, 68)
    "b" = [System.Drawing.Color]::FromArgb(255, 128, 108, 82)
    "n" = [System.Drawing.Color]::FromArgb(255, 108, 102, 92)
    "m" = [System.Drawing.Color]::FromArgb(255, 68, 62, 52)
    "a" = [System.Drawing.Color]::FromArgb(255, 158, 152, 142)
    "e" = [System.Drawing.Color]::FromArgb(255, 28, 22, 18)
    "f" = [System.Drawing.Color]::FromArgb(255, 228, 168, 68)
    "k" = [System.Drawing.Color]::FromArgb(255, 108, 78, 48)
    "t" = [System.Drawing.Color]::FromArgb(255, 188, 182, 172)
    "y" = [System.Drawing.Color]::FromArgb(255, 138, 168, 198)
    "v" = [System.Drawing.Color]::FromArgb(255, 138, 132, 118)
}

# ---- VALLEY MOUTH: Wide entry, open sky above, walls beginning to close ----
$valleyMouth = @(
    "ysssssssssssssy",
    "ysssshssshsssy.",
    ".ysssssssssssy.",
    ".yssssssssssy..",
    "..ysssssssssy..",
    "..yssrsrrsssy..",
    "..yssrrrrsssy..",
    "..ysrrrrrrsy...",
    "..ysrrrrrrsy...",
    "..yssrrrrsssy..",
    "..yssrsrrsssy..",
    "..yssssssssy...",
    ".yssssssssssy..",
    ".ysssssssssssy.",
    "ysssshssshssy..",
    "ysssssssssssssy"
)

# ---- PILGRIM'S ROAD: Narrow path with carved names on walls ----
$pilgrimsRoad = @(
    "wncncnrrrrncncnw",
    "wnncnrrrrrncncw",
    "wcncnrrrrrnncnw",
    "wncncrrrrrncncw",
    "wcncnrrrrrncncw",
    "wnncnrrrrrcncnw",
    "wcncnrrrrrncncw",
    "wncncrrrrrncncw",
    "wcncnrrrrrncncw",
    "wnncnrrrrrcncnw",
    "wcncnrrrrrncncw",
    "wncncrrrrrncncw",
    "wcncnrrrrrncncw",
    "wnncnrrrrrcncnw",
    "wncncnrrrrncncnw",
    "wcncnrrrrrncncw"
)

# ---- MEMORY ALCOVE: Small carved recess in valley wall ----
$memoryAlcove = @(
    "wwwwwwwwwwwwwwww",
    "wwwwmmmmmmwwwwww",
    "wwwmeeeeemwwwww",
    "wwwmehhhhemwwwww",
    "wwmehcccchemwwww",
    "wwmehcccchemwwww",
    "wwmehcccchemwwww",
    "wwmehcccchemwwww",
    "wwmehhhhemwwwww",
    "wwwmeeeeemwwwww",
    "wwwwmmmmmmwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwrrrrrrrrwwwwww",
    "wwrrrrrrrrwwwwww",
    "wwwwwwwwwwwwwwww"
)

# ---- THRESHOLD STONE: Flat stone at the narrowest point ----
$thresholdStone = @(
    "wssssssssssssssw",
    "wsstttttttttssw",
    "wsttttttttttttsw",
    "wstthtthtthtttsw",
    "wstttttttttttsw",
    "wstthttttthtttsw",
    "wstttttttttttsw",
    "wstthtthtthtttsw",
    "wstttttttttttsw",
    "wstthttttthtttsw",
    "wstttttttttttsw",
    "wstthtthtthtttsw",
    "wsttttttttttttsw",
    "wsstttttttttssw",
    "wssssssssssssssw",
    "wssssssssssssssw"
)

# ---- FINAL APPROACH: Valley widens, Throne entrance ahead ----
$finalApproach = @(
    "wsssssssssssssw",
    "wssssrsrsssssw.",
    ".wsssrrrrssssw.",
    "..wssrrrrssw...",
    "..wsrrrrrrsw...",
    "..wsrrrrrrrsw..",
    ".wsrrrrrrrrssw.",
    "wsrrrrhrrrrrsw.",
    "wsrrrrrrrrrrsw.",
    ".wsrrrrrrrrssw.",
    "..wsrrrrrrrsw..",
    "..wsrrrrrrsw...",
    "..wssrrrrssw...",
    ".wsssrrrrssssw.",
    "wssssrsrsssssw.",
    "wsssssssssssssw"
)

# ---- THRONE ENTRANCE: The wound in the mountain, dark opening ----
$throneEntrance = @(
    "ddddddddddddddd",
    "ddddaaaaaaaddddd",
    "dddaeeeeeeadddd",
    "ddaeeeeeeeaaddd",
    "ddaeeeeeeeeaddd",
    "ddaeeeeeeeeeddd",
    "ddaeeeeeeeeeddd",
    "ddaeeeeeeeeeddd",
    "ddaeeeeeeeeeddd",
    "ddaeeeeeeeeddd",
    "ddaeeeeeeeeaddd",
    "ddaaeeeeeeadddd",
    "dddaaeeeeaadddd",
    "ddddaaaaaaaddddd",
    "ddddddddddddddd",
    "dddrrrrrrrddddd"
)

# ---- VALLEY WALL CARVINGS: Dense carved names section ----
$valleyWallCarvings = @(
    "wnncncncncncnnw",
    "wcnncnncncnncnw",
    "wncncncncncncnw",
    "wcnncncncncncnw",
    "wnncnncncncncnw",
    "wcncncncncnncnw",
    "wncncncncncncnw",
    "wcnncncncnncnw",
    "wnncncncncncnnw",
    "wcncncncncncncw",
    "wncnncncncncnw",
    "wcncncncnncncnw",
    "wnncncncncncncw",
    "wcncnncncncncnw",
    "wnncncncncncnnw",
    "wcncncncncncncw"
)

# ---- LAST CAMP: Sheltered area mid-valley, Wren's home ----
$lastCamp = @(
    "wssssssssssssssw",
    "wsssssssssssssw",
    "wsskkkkssssssw.",
    "wskkffkkssssssw",
    "wskffffkssssssw",
    "wskkffkkssssssw",
    "wsskkkkssssssw.",
    "wsssssssssssssw",
    "wssrrrrrrrsssw.",
    "wsssrrrrrsssssw",
    "wsssssrsssssssw",
    "wssssssssssssw.",
    "wssbbbbssssssw.",
    "wssbhbhbsssssw",
    "wssbbbbssssssw.",
    "wssssssssssssssw"
)

# ---- CARVED NAMES: Detailed wall section with Mira's carvings ----
$carvedNames = @(
    "wnnncncncncnnnw",
    "wcncnhncncncncw",
    "wncncncncncncnw",
    "wcncnhncnhncnw",
    "wnncncncncncnnw",
    "wcncncncncncncw",
    "wncnhncncnhcnw",
    "wcncncncncncncw",
    "wnncnhncncnnnw",
    "wcncncncncncncw",
    "wncncncnhncncnw",
    "wcncncncncncncw",
    "wnnhncncncnhnnw",
    "wcncncncncncncw",
    "wncncncncncncnw",
    "wcncncncncncncw"
)

# ---- VALLEY MOUTH OPEN: Wide valley mouth with sky above ----
$valleyMouthOpen = @(
    "yyyyyyyyyyyyyyyy",
    "yyyyyyyyyyyyyyyy",
    "yyssssssssssssyy",
    "yssssssssssssssy",
    "ssssssssssssssss",
    "sssssrrrrrrsssss",
    "ssssrrrrrrrrsss",
    "sssrrrrrrrrrrss",
    "sssrrrrrrrrrrss",
    "ssssrrrrrrrrsss",
    "sssssrrrrrrsssss",
    "ssssssssssssssss",
    "yssssssssssssssy",
    "yyssssssssssssyy",
    "yyyyyyyyyyyyyyyy",
    "yyyyyyyyyyyyyyyy"
)

# ---- ASSEMBLE TILE SHEET (10 tiles, 160x16) ----
$b8TileSheet = New-Sheet 160 16
Paint-Sprite $b8TileSheet $valleyMouth       0   0 $b8TilePalette
Paint-Sprite $b8TileSheet $pilgrimsRoad      16  0 $b8TilePalette
Paint-Sprite $b8TileSheet $memoryAlcove      32  0 $b8TilePalette
Paint-Sprite $b8TileSheet $thresholdStone    48  0 $b8TilePalette
Paint-Sprite $b8TileSheet $finalApproach     64  0 $b8TilePalette
Paint-Sprite $b8TileSheet $throneEntrance    80  0 $b8TilePalette
Paint-Sprite $b8TileSheet $valleyWallCarvings 96  0 $b8TilePalette
Paint-Sprite $b8TileSheet $lastCamp          112 0 $b8TilePalette
Paint-Sprite $b8TileSheet $carvedNames       128 0 $b8TilePalette
Paint-Sprite $b8TileSheet $valleyMouthOpen   144 0 $b8TilePalette

$b8TileSheet.Save((Join-Path $outDir "tiles.png"), [System.Drawing.Imaging.ImageFormat]::Png)
$b8TileSheet.Dispose()

# =============================================================================
# BIOME 8 — ENEMIES (3 enemies × 2 frames = 32×48)
# =============================================================================
# Valley Sentinel — slow, heavy stone guard
# Highrock Eagle — aerial, dive-attack bird
# Drift Shadow — weak alcove ghost
$b8EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "s" = [System.Drawing.Color]::FromArgb(255, 108, 102, 92)
    "d" = [System.Drawing.Color]::FromArgb(255, 68, 62, 52)
    "h" = [System.Drawing.Color]::FromArgb(255, 148, 142, 132)
    "r" = [System.Drawing.Color]::FromArgb(255, 178, 58, 48)
    "g" = [System.Drawing.Color]::FromArgb(255, 198, 168, 48)
    "b" = [System.Drawing.Color]::FromArgb(255, 88, 72, 52)
    "w" = [System.Drawing.Color]::FromArgb(255, 198, 192, 182)
    "k" = [System.Drawing.Color]::FromArgb(255, 48, 42, 32)
    "n" = [System.Drawing.Color]::FromArgb(255, 28, 22, 18)
    "e" = [System.Drawing.Color]::FromArgb(255, 218, 208, 188)
    "a" = [System.Drawing.Color]::FromArgb(255, 128, 122, 148)
    "p" = [System.Drawing.Color]::FromArgb(255, 88, 82, 108)
    "v" = [System.Drawing.Color]::FromArgb(255, 58, 52, 78)
}

# ---- VALLEY SENTINEL: Hulking stone guardian, slow ----
$valleySentinel1 = @(
    "....ssssss......",
    "...sddddds.....",
    "..sddrrddds....",
    "..sdddddddds...",
    "..sdddddddds...",
    ".sddddddddds..",
    ".sdbdddddbds..",
    ".sdddddddds...",
    "..sdddddds.....",
    "..sddssddds....",
    "..sdds.sdds....",
    "..sdds.sdds....",
    "..sdds.sdds....",
    "..sdds.sdds....",
    "..sdds.sdds....",
    "..sdds.sdds...."
)
$valleySentinel2 = @(
    "....ssssss......",
    "...sddddds.....",
    "..sddrrddds....",
    "..sdddddddds...",
    "..sdddddddds...",
    ".sddddddddds..",
    ".sdbdddddbds..",
    ".sdddddddds...",
    "..sdddddds.....",
    "...sddddds.....",
    "..sdds.sdds....",
    ".sdds...sdds...",
    "..sdds.sdds....",
    ".sdds...sdds...",
    "..sdds.sdds....",
    "..sdds.sdds...."
)

# ---- HIGHROCK EAGLE: Brown raptor, wings spread ----
$highrockEagle1 = @(
    "......bb........",
    ".....beb........",
    "....bebb........",
    "...beebb........",
    "..beegbb........",
    ".beeeebb........",
    "beeeeebbb......",
    ".beeeebb.bb.....",
    "..beeebb..bb....",
    "...beebb........",
    "....bebb........",
    ".....bbb........",
    "......bb........",
    "......bb........",
    ".....bddb.......",
    "......dd........"
)
$highrockEagle2 = @(
    "................",
    "......bb........",
    ".....beb........",
    "....beebb.......",
    "...beeegbb......",
    "..beeeeebb......",
    ".beeeeeeebb.....",
    "beeeeeeeebbb...",
    ".beeeeeebb......",
    "..beeeeebb......",
    "...beeebb.......",
    "....beebb.......",
    ".....bbb........",
    "......bb........",
    ".....bddb.......",
    "......dd........"
)

# ---- DRIFT SHADOW: Weak wispy spectre, alcove guard ----
$driftShadow1 = @(
    "......pp........",
    ".....paap.......",
    "....paavap......",
    "...paavvap......",
    "...pavvvap......",
    "...pavvvap......",
    "...pavvvvap.....",
    "...paavvap......",
    "....pavvap......",
    "....pavap.......",
    ".....pap........",
    ".....pap........",
    "......p.........",
    "......p.........",
    "................",
    "................"
)
$driftShadow2 = @(
    "................",
    "......pp........",
    ".....paap.......",
    "....paavap......",
    "...paavvap......",
    "...pavvvvap.....",
    "...pavvvap......",
    "...paavvap......",
    "....pavvap......",
    "....pavap.......",
    ".....pap........",
    "......p.........",
    "......p.........",
    "................",
    "................",
    "................"
)

# ---- ASSEMBLE ENEMY SHEET (3 enemies, 32×48) ----
$b8EnemySheet = New-Sheet 32 48
Paint-Sprite $b8EnemySheet $valleySentinel1  0  0  $b8EnemyPalette
Paint-Sprite $b8EnemySheet $valleySentinel2  16 0  $b8EnemyPalette
Paint-Sprite $b8EnemySheet $highrockEagle1   0  16 $b8EnemyPalette
Paint-Sprite $b8EnemySheet $highrockEagle2   16 16 $b8EnemyPalette
Paint-Sprite $b8EnemySheet $driftShadow1     0  32 $b8EnemyPalette
Paint-Sprite $b8EnemySheet $driftShadow2     16 32 $b8EnemyPalette

$b8EnemySheet.Save((Join-Path $outDir "enemies.png"), [System.Drawing.Imaging.ImageFormat]::Png)
$b8EnemySheet.Dispose()

# =============================================================================
# BIOME 8 — NPC (1 NPC × 2 frames = 32×16)
# =============================================================================
# Wren, the Pilgrim Keeper — elderly woman, warm tones
$b8NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "s" = [System.Drawing.Color]::FromArgb(255, 208, 188, 158)
    "h" = [System.Drawing.Color]::FromArgb(255, 198, 178, 148)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 208, 198)
    "d" = [System.Drawing.Color]::FromArgb(255, 128, 98, 68)
    "b" = [System.Drawing.Color]::FromArgb(255, 148, 108, 78)
    "r" = [System.Drawing.Color]::FromArgb(255, 168, 88, 58)
    "g" = [System.Drawing.Color]::FromArgb(255, 158, 148, 128)
    "k" = [System.Drawing.Color]::FromArgb(255, 48, 38, 28)
    "e" = [System.Drawing.Color]::FromArgb(255, 38, 28, 22)
    "n" = [System.Drawing.Color]::FromArgb(255, 88, 78, 68)
    "a" = [System.Drawing.Color]::FromArgb(255, 228, 208, 168)
    "f" = [System.Drawing.Color]::FromArgb(255, 228, 168, 68)
}

# ---- WREN: Elderly woman, grey hair, warm brown shawl ----
$wren1 = @(
    "......ww........",
    ".....wwww.......",
    "....wwssww......",
    "....wseksw......",
    "....wssssw......",
    ".....wwww.......",
    "....ddrrdd......",
    "...ddrrrdd.....",
    "...ddrrrrdd.....",
    "...ddrrrrdd.....",
    "...ddrrrdd.....",
    "....drrrd.......",
    "....drrrd.......",
    ".....nnnn.......",
    "....nn..nn......",
    "....nn..nn......"
)
$wren2 = @(
    "......ww........",
    ".....wwww.......",
    "....wwssww......",
    "....wseksw......",
    "....wssssw......",
    ".....wwww.......",
    "....ddrrdd......",
    "...ddrrrdd.....",
    "...ddrrrrdd.....",
    "...ddrrrrdd.....",
    "...ddrrrdd.....",
    "....drrrd.......",
    "....drrrd.......",
    "....nnnn........",
    "...nn..nn.......",
    "....nn..nn......"
)

# ---- ASSEMBLE NPC SHEET (1 NPC, 32×16) ----
$b8NpcSheet = New-Sheet 32 16
Paint-Sprite $b8NpcSheet $wren1  0  0  $b8NpcPalette
Paint-Sprite $b8NpcSheet $wren2  16 0  $b8NpcPalette

$b8NpcSheet.Save((Join-Path $outDir "npcs.png"), [System.Drawing.Imaging.ImageFormat]::Png)
$b8NpcSheet.Dispose()

# =============================================================================
# BIOME 8 — ITEMS (6 items in 3×2 grid = 48×32)
# =============================================================================
# dragon_codex, crystal_of_seeing, healing_herb, rare_healing_herb, torch, eagle_feather
$b8ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "b" = [System.Drawing.Color]::FromArgb(255, 98, 58, 38)
    "d" = [System.Drawing.Color]::FromArgb(255, 68, 42, 28)
    "g" = [System.Drawing.Color]::FromArgb(255, 198, 168, 48)
    "r" = [System.Drawing.Color]::FromArgb(255, 178, 58, 48)
    "w" = [System.Drawing.Color]::FromArgb(255, 228, 218, 198)
    "h" = [System.Drawing.Color]::FromArgb(255, 68, 148, 68)
    "k" = [System.Drawing.Color]::FromArgb(255, 38, 108, 38)
    "p" = [System.Drawing.Color]::FromArgb(255, 168, 88, 188)
    "s" = [System.Drawing.Color]::FromArgb(255, 148, 142, 132)
    "c" = [System.Drawing.Color]::FromArgb(255, 128, 178, 228)
    "a" = [System.Drawing.Color]::FromArgb(255, 168, 208, 248)
    "f" = [System.Drawing.Color]::FromArgb(255, 228, 168, 68)
    "e" = [System.Drawing.Color]::FromArgb(255, 248, 208, 88)
    "n" = [System.Drawing.Color]::FromArgb(255, 108, 78, 48)
    "m" = [System.Drawing.Color]::FromArgb(255, 88, 72, 52)
}

# ---- DRAGON CODEX: Ornate book with dragon symbol ----
$dragonCodex = @(
    "................",
    "....dddddd......",
    "...dbbbbbd.....",
    "...dbgggbd.....",
    "...dbgrgbd.....",
    "...dbgrgbd.....",
    "...dbgggbd.....",
    "...dbbbbbbd....",
    "...dbbbbbd.....",
    "...dbgggbd.....",
    "...dbgggbd.....",
    "...dbbbbbd.....",
    "....dddddd......",
    "................",
    "................",
    "................"
)

# ---- CRYSTAL OF SEEING: Glowing blue-white crystal ----
$crystalOfSeeing = @(
    "................",
    "......aa........",
    ".....acca.......",
    "....accca.......",
    "....acccca......",
    "...accccca.....",
    "...acccccca....",
    "...accccca.....",
    "....acccca......",
    "....accca.......",
    ".....acca.......",
    "......aa........",
    "................",
    "................",
    "................",
    "................"
)

# ---- HEALING HERB: Standard green herb ----
$healingHerb8 = @(
    "................",
    "......hh........",
    ".....hhhh.......",
    "....hhkkhh......",
    "...hhkkhh......",
    "....hhhh........",
    ".....kk.........",
    ".....kk.........",
    ".....kk.........",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- RARE HEALING HERB: Purple healing herb ----
$rareHealingHerb = @(
    "................",
    "......pp........",
    ".....pppp.......",
    "....pprrpp......",
    "...pprrpp......",
    "....pppp........",
    ".....kk.........",
    ".....kk.........",
    ".....kk.........",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- TORCH: Wooden stick with flame ----
$torch8 = @(
    "......ef........",
    ".....efef.......",
    ".....efef.......",
    "......ef........",
    "......nn........",
    "......nn........",
    "......nn........",
    "......nn........",
    "......nn........",
    "......nn........",
    "......nn........",
    "......nn........",
    "................",
    "................",
    "................",
    "................"
)

# ---- EAGLE FEATHER: Brown feather, curved ----
$eagleFeather = @(
    "..........mm....",
    ".........mmm....",
    "........mmm.....",
    ".......mmm......",
    "......mmm.......",
    ".....mmmm.......",
    "....mmwmm.......",
    "...mmwwmm.......",
    "..mmwwmm........",
    "..mwwmm.........",
    "..mwmm..........",
    "..mmm...........",
    "..mm............",
    "..m.............",
    "................",
    "................"
)

# ---- ASSEMBLE ITEM SHEET (6 items, 48×32) ----
$b8ItemSheet = New-Sheet 48 32
Paint-Sprite $b8ItemSheet $dragonCodex      0  0  $b8ItemPalette
Paint-Sprite $b8ItemSheet $crystalOfSeeing  16 0  $b8ItemPalette
Paint-Sprite $b8ItemSheet $healingHerb8     32 0  $b8ItemPalette
Paint-Sprite $b8ItemSheet $rareHealingHerb  0  16 $b8ItemPalette
Paint-Sprite $b8ItemSheet $torch8           16 16 $b8ItemPalette
Paint-Sprite $b8ItemSheet $eagleFeather     32 16 $b8ItemPalette

$b8ItemSheet.Save((Join-Path $outDir "items.png"), [System.Drawing.Imaging.ImageFormat]::Png)
$b8ItemSheet.Dispose()

Write-Host "Generated Biome 8 (Dragon's Approach) sprite sheets:"
Write-Host " - $(Join-Path $outDir 'tiles.png')"
Write-Host " - $(Join-Path $outDir 'enemies.png')"
Write-Host " - $(Join-Path $outDir 'npcs.png')"
Write-Host " - $(Join-Path $outDir 'items.png')"
