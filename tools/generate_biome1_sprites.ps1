Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome1"
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
# BIOME 1 — MOSSHAVEN WILDS — TILE PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    g = mossy grass     d = dark moss       m = light moss
#   t = bark brown     k = dark bark       l = leaf green      n = leaf shadow
#   w = fen water dark a = fen water light b = root/wood brown r = stone grey
#   s = stepping stone f = fungal glow     h = fungal bright   c = cave dark
#   y = filtered sun   e = earth brown     p = path dirt
$b1TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "g" = [System.Drawing.Color]::FromArgb(255, 58, 148, 72)
    "d" = [System.Drawing.Color]::FromArgb(255, 38, 108, 52)
    "m" = [System.Drawing.Color]::FromArgb(255, 82, 168, 96)
    "t" = [System.Drawing.Color]::FromArgb(255, 88, 62, 38)
    "k" = [System.Drawing.Color]::FromArgb(255, 64, 46, 28)
    "l" = [System.Drawing.Color]::FromArgb(255, 108, 180, 78)
    "n" = [System.Drawing.Color]::FromArgb(255, 68, 142, 58)
    "w" = [System.Drawing.Color]::FromArgb(255, 48, 106, 78)
    "a" = [System.Drawing.Color]::FromArgb(255, 66, 138, 106)
    "b" = [System.Drawing.Color]::FromArgb(255, 96, 72, 44)
    "r" = [System.Drawing.Color]::FromArgb(255, 106, 106, 96)
    "s" = [System.Drawing.Color]::FromArgb(255, 136, 126, 108)
    "f" = [System.Drawing.Color]::FromArgb(255, 116, 196, 86)
    "h" = [System.Drawing.Color]::FromArgb(255, 180, 240, 120)
    "c" = [System.Drawing.Color]::FromArgb(255, 28, 28, 36)
    "y" = [System.Drawing.Color]::FromArgb(255, 198, 182, 68)
    "e" = [System.Drawing.Color]::FromArgb(255, 142, 124, 76)
    "p" = [System.Drawing.Color]::FromArgb(255, 128, 100, 60)
}

# ---- MOSS GRASS: Dense mossy forest floor with scattered darker patches ----
$mossGrass = @(
    "ggmgggdggmggggdg",
    "gmggggggmggggggg",
    "ggggdggggggdgggg",
    "ggmggggggmgggggg",
    "gggggdgggggggmgg",
    "ggggggmgggdggggg",
    "gdggggggggggggdg",
    "ggggmgdggmgggggg",
    "ggggggggggggdggg",
    "gmggggggmggggggg",
    "ggdggmgggggggmgg",
    "ggggggggdggggggg",
    "gggmggggggmgggdg",
    "ggggggdggggggggg",
    "gdggggggggdggggg",
    "ggggmgggggggmggg"
)

# ---- MOSS TREE: Huge moss-covered trunk, dense canopy (n=leaf shadow, l=leaf, k=dark bark, t=bark) ----
$mossTree = @(
    "gggnllllllnggggg",
    "ggnlllllllllgggg",
    "gnllllnllllngggg",
    "gnlllllllllnnggg",
    "nllllnllnllllngg",
    "gnlllllllllnnggg",
    "ggnnllllllnngggg",
    "gggnlllllngggggg",
    "gggddkkkkkdggggg",
    "ggddmkkkkkmddggg",
    "gggdmkkkkkmddggg",
    "gggddkkkkddggggg",
    "ggggdkkkkkdggggg",
    "ggggdkkkkkdggggg",
    "gggddkkkkddggggg",
    "ggddddddddddgggg"
)

# ---- FEN WATER: Boggy, murky swamp water with algae patches ----
$fenWater = @(
    "wwawwwwawwwwwaww",
    "wawwwwwwwawwwwww",
    "wwwwawwwwwwawwww",
    "wawwwwwawwwwwwww",
    "wwwwwwwwwwawwwaw",
    "wwawwdwwwwwwwwww",
    "wwwwwwwwawwdwwww",
    "wawwwwwwwwwwwwaw",
    "wwwwawwwwwwawwww",
    "wwwwwwwawwwwwwww",
    "wawwwwwwwwawwwww",
    "wwwwdwwwwwwwwaww",
    "wwwwwwawwwwwwwww",
    "wawwwwwwwawwwwww",
    "wwwwwwwwwwwwawww",
    "wwawwwawwwwwwwww"
)

# ---- ROOT BRIDGE: Elevated root formations connecting canopy platforms ----
$rootBridge = @(
    "ggggbbbbbbbbgggg",
    "gggbbbbbbbbbgggg",
    "ggbbkkkkkkkkbbgg",
    "ggbkkbbbbbbkkbgg",
    "gbbkbbbbbbbbkbbg",
    "gbbkbbbbbbbbkbbg",
    "gbbkbbbbbbbbkbbg",
    "gbbkbbbbbbbbkbbg",
    "gbbkbbbbbbbbkbbg",
    "gbbkbbbbbbbbkbbg",
    "gbbkbbbbbbbbkbbg",
    "gbbkbbbbbbbbkbbg",
    "ggbkkbbbbbbkkbgg",
    "ggbbkkkkkkkkbbgg",
    "gggbbbbbbbbbgggg",
    "ggggbbbbbbbbgggg"
)

# ---- MUSHROOM: Glowing mushroom cluster (Fungal Ring element) ----
$mushroom = @(
    "gggggggggggggggg",
    "ggggghhhhggggggg",
    "gggghffffhgggggg",
    "ggghfffffffhgggg",
    "ggghfffffffhgggg",
    "gggghfffffhggggg",
    "gggggfhhhfgggggg",
    "ggggggffgggggggg",
    "ggggggffgggggggg",
    "ggggggffgggggggg",
    "gggggdmmdggggggg",
    "ggggdmggmdgggggg",
    "gggggggggggggggg",
    "ggggfhgghfgggggg",
    "gggggffgffgggggg",
    "gggggggggggggggg"
)

# ---- STEPPING STONE: Stones in Clearwater Stream ----
$steppingStone = @(
    "wwwwwawwwwwwwwww",
    "wawwwwwwwawwwwww",
    "wwwwrrsswwwwawww",
    "wwwwrssrrwwwwwww",
    "wwwwwrrrwwwwwwww",
    "wawwwwwwwwawwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwwawwwwwwawww",
    "wwwwwwwwrrsswwww",
    "wawwwwwwrssrrwww",
    "wwwwwwwwwrrrwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwawwwwwwawwww",
    "wawwwwwwwwwwwwww",
    "wwwwwwwawwwwwwww",
    "wwwawwwwwwwawwww"
)

# ---- CAVE MOSS: Mosshaven Cave entrance — partially concealed by hanging moss ----
$caveMoss = @(
    "ggdmddddddmdgggg",
    "gdmccccccccmdggg",
    "dmcccccccccccmdg",
    "mcccccccccccccmg",
    "cccccccccccccccg",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "cccccccccccccccc",
    "dmcccccccccccmgd",
    "gdmcccccccccmdgm",
    "ggdmmcccccmmdggm",
    "gggdddcccddgggdg"
)

# ---- GRANDFATHER TREE: Massive ancient tree (n=leaf shadow, l=leaf, k=dark bark) ----
$grandfatherTree = @(
    "gdnllllllnndgggg",
    "dnlllnlllllndggg",
    "nlllllllnlllllgg",
    "nlllnlllllnlllgg",
    "nllllllllllllngg",
    "dnlllllllllnddgg",
    "gdnnllllnnddgggg",
    "gggddkkkkddggggg",
    "ggddkkcckkkdgggg",
    "ggdkkcccckkdgggg",
    "ggdkkcccckkdgggg",
    "ggddkkcckkkdgggg",
    "gggdkkkkkkdggggg",
    "gggdkkkkkkdggggg",
    "ggddkkkkkkddgggg",
    "gdddddddddddgggg"
)

# ---- HOLLOW ROOT: Natural root arch — secret passage entrance ----
$hollowRoot = @(
    "gggggggggggggggg",
    "gggdkkkkkkdggggg",
    "ggdkkbbbbkkdgggg",
    "gdkkbbbbbbkkdggg",
    "gkkbb....bbkkggg",
    "gkbb......bbkggg",
    "gkb........bkggg",
    "gkb........bkggg",
    "gkb........bkggg",
    "gkbb......bbkggg",
    "gkkbb....bbkkggg",
    "gdkkbbbbbbkkdggg",
    "ggdkkbbbbkkdgggg",
    "gggdkkkkkkdggggg",
    "ggggddddddgggggg",
    "gggggggggggggggg"
)

# ---- FOREST PATH: Mosshaven winding dirt path ----
$forestPath = @(
    "gdppppppppppgdgg",
    "ggppppppppppgggg",
    "ggppppppppppgggg",
    "ggppppppppppgdgg",
    "gdppppppppppgggg",
    "ggppppppppppgggg",
    "ggppppppppppgggg",
    "ggppppppppppgdgg",
    "gdppppppppppgggg",
    "ggppppppppppgggg",
    "ggppppppppppgggg",
    "ggppppppppppgdgg",
    "gdppppppppppgggg",
    "ggppppppppppgggg",
    "ggppppppppppgggg",
    "ggppppppppppgggg"
)

# Write the biome 1 tile sheet
$b1TileSheet = New-Sheet -Width 144 -Height 32
Paint-Sprite $b1TileSheet $mossGrass        0   0 $b1TilePalette
Paint-Sprite $b1TileSheet $mossTree         16  0 $b1TilePalette
Paint-Sprite $b1TileSheet $fenWater         32  0 $b1TilePalette
Paint-Sprite $b1TileSheet $rootBridge       48  0 $b1TilePalette
Paint-Sprite $b1TileSheet $mushroom         64  0 $b1TilePalette
Paint-Sprite $b1TileSheet $steppingStone    80  0 $b1TilePalette
Paint-Sprite $b1TileSheet $caveMoss         96  0 $b1TilePalette
Paint-Sprite $b1TileSheet $grandfatherTree  112 0 $b1TilePalette
Paint-Sprite $b1TileSheet $hollowRoot       128 0 $b1TilePalette
Paint-Sprite $b1TileSheet $forestPath       0   16 $b1TilePalette
$b1TilePath = Join-Path $outDir "tiles.png"
$b1TileSheet.Save($b1TilePath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 1 — MOSSHAVEN WILDS — ENEMY PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         g = wolf grey       i = wolf light grey
#   w = wolf white     r = red eye         n = nose/dark       v = vine green
#   j = vine dark      t = thorn tip       q = thorn light     p = puffer tan
#   a = puffer dark    s = spore green     z = spore bright    b = lurker teal
#   d = lurker dark    e = lurker eye      y = eye glow
$b1EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "g" = [System.Drawing.Color]::FromArgb(255, 88, 88, 78)
    "i" = [System.Drawing.Color]::FromArgb(255, 118, 118, 108)
    "w" = [System.Drawing.Color]::FromArgb(255, 200, 200, 194)
    "r" = [System.Drawing.Color]::FromArgb(255, 168, 52, 52)
    "n" = [System.Drawing.Color]::FromArgb(255, 36, 36, 36)
    "v" = [System.Drawing.Color]::FromArgb(255, 56, 120, 56)
    "j" = [System.Drawing.Color]::FromArgb(255, 38, 86, 38)
    "t" = [System.Drawing.Color]::FromArgb(255, 96, 168, 52)
    "q" = [System.Drawing.Color]::FromArgb(255, 148, 196, 86)
    "p" = [System.Drawing.Color]::FromArgb(255, 186, 148, 96)
    "a" = [System.Drawing.Color]::FromArgb(255, 156, 118, 72)
    "s" = [System.Drawing.Color]::FromArgb(255, 168, 218, 96)
    "z" = [System.Drawing.Color]::FromArgb(255, 210, 245, 140)
    "b" = [System.Drawing.Color]::FromArgb(255, 68, 96, 78)
    "d" = [System.Drawing.Color]::FromArgb(255, 44, 68, 58)
    "e" = [System.Drawing.Color]::FromArgb(255, 232, 202, 56)
    "y" = [System.Drawing.Color]::FromArgb(255, 222, 178, 48)
}

# ---- BOGWOLF: Grey wolf, hunched, visible fangs, glowing red eyes ----
# g=grey, i=light grey, w=white belly, r=red eyes, n=nose, o=outline
$bogwolf1 = @(
    "................",
    "....oog..goo....",
    "...oggiiiiggoo..",
    "..oiiiiiiiiiigo.",
    "..oiiriiiriiigo.",
    "..oiiiiiiiiiigo.",
    ".oiiiiiiniiiio..",
    ".oiiigwwwwgiio..",
    ".oggiiiiiiiggoo.",
    "..oggiiiiiggoo..",
    "...oiiiiiiio....",
    "..oiio..oiio....",
    "..oio....oio....",
    "..oo......oo....",
    "................",
    "................"
)

$bogwolf2 = @(
    "................",
    "....oog..goo....",
    "...oggiiiiggoo..",
    "..oiiiiiiiiiigo.",
    "..oiiriiiriiigo.",
    "..oiiiiiiiiiigo.",
    ".oiiiiiiniiiio..",
    ".oiiigwwwwgiio..",
    ".oggiiiiiiiggoo.",
    "..oggiiiiiggoo..",
    "...oiiiiiiio....",
    "...oio..oio.....",
    "..oio....oio....",
    "...oo....oo.....",
    "................",
    "................"
)

# ---- THORNVINE CRAWLER: Coiled green vine creature with thorn spikes ----
# v=vine green, j=vine dark, t=thorn tip, q=thorn light, o=outline
$thornvine1 = @(
    "................",
    ".......tq.......",
    "......tvjt......",
    ".....tvjjjt.....",
    "..ttvjjjjjvtt...",
    "..jvjjjjjjjvj...",
    ".jvjjjtjtjjjvj..",
    ".jvjjjjjjjjjvj..",
    "..jvjjjjjjjvj...",
    "...jvvjjjvvj....",
    "....jjvvvjj.....",
    ".....jjjjj......",
    "................",
    "................",
    "................",
    "................"
)

$thornvine2 = @(
    "................",
    "......tqt.......",
    ".....tvjjt......",
    "....tvjjjjt.....",
    "...tjjjjjjjt....",
    "..jvjjjjjjjvj...",
    ".jvjjtjjjtjjvj..",
    ".jvjjjjjjjjjvj..",
    ".jvjjjjjjjjjvj..",
    "..jvjjjjjjjvj...",
    "...jvvvvvvjj....",
    "....jjjjjj......",
    "................",
    "................",
    "................",
    "................"
)

# ---- SPORE PUFFER: Round, puffy mushroom creature that releases spores ----
# p=body tan, a=body dark, r=red eyes, s=spore green, z=spore bright, o=outline
$sporePuffer1 = @(
    "......zz........",
    ".....zsszz......",
    "....zsssz.......",
    "....oaaaao......",
    "...oaaaaaao.....",
    "..oapppppppao...",
    "..oaprppprpao...",
    "..oapppppppao...",
    "..oapppaapaao...",
    "...oaaaaaao.....",
    "....oaaaao......",
    ".....oooo.......",
    "................",
    "................",
    "................",
    "................"
)

$sporePuffer2 = @(
    "...zz...zz......",
    "..zssz.zssz.....",
    "...zz...zz......",
    "....oaaaao......",
    "...oaaaaaao.....",
    "..oapppppppao...",
    "..oaprppprpao...",
    "..oapppppppao...",
    "..oapppaapaao...",
    "...oaaaaaao.....",
    "....oaaaao......",
    ".....oooo.......",
    "................",
    "................",
    "................",
    "................"
)

# ---- FEN LURKER: Reptilian swamp lurker, mostly submerged, yellow eyes ----
# b=lurker teal, d=lurker dark, e=yellow eyes, o=outline
$fenLurker1 = @(
    "................",
    "................",
    "................",
    "................",
    "....oe..eo......",
    "...oddddddoo....",
    "..odbbbbbbbboo..",
    "..odbbbddbbbo...",
    "...odbbbbbboo...",
    "....oooooooo....",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$fenLurker2 = @(
    "................",
    "................",
    "................",
    "................",
    "................",
    "....oe..eo......",
    "...oddddddoo....",
    "..odbbbbbbbboo..",
    "..odbbbddbbbo...",
    "...odbbbbbboo...",
    "....oooooooo....",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# Enemy sheet: 4 enemies x 2 frames = 32 x 64
$b1EnemySheet = New-Sheet -Width 32 -Height 64
Paint-Sprite $b1EnemySheet $bogwolf1       0  0  $b1EnemyPalette
Paint-Sprite $b1EnemySheet $bogwolf2       16 0  $b1EnemyPalette
Paint-Sprite $b1EnemySheet $thornvine1     0  16 $b1EnemyPalette
Paint-Sprite $b1EnemySheet $thornvine2     16 16 $b1EnemyPalette
Paint-Sprite $b1EnemySheet $sporePuffer1   0  32 $b1EnemyPalette
Paint-Sprite $b1EnemySheet $sporePuffer2   16 32 $b1EnemyPalette
Paint-Sprite $b1EnemySheet $fenLurker1     0  48 $b1EnemyPalette
Paint-Sprite $b1EnemySheet $fenLurker2     16 48 $b1EnemyPalette
$b1EnemyPath = Join-Path $outDir "enemies.png"
$b1EnemySheet.Save($b1EnemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 1 — MOSSHAVEN WILDS — NPC PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         s = skin            a = skin shadow
#   h = hair brown     d = hair dark       g = green dress     f = green dress dark
#   j = apron/leather  b = boots           e = eyes            c = coat tan
#   u = coat dark      w = white shirt     r = red scarf       k = sandy hair
#   n = sandy dark     m = map paper
$b1NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "s" = [System.Drawing.Color]::FromArgb(255, 228, 196, 160)
    "a" = [System.Drawing.Color]::FromArgb(255, 198, 162, 126)
    "h" = [System.Drawing.Color]::FromArgb(255, 96, 52, 28)
    "d" = [System.Drawing.Color]::FromArgb(255, 66, 38, 18)
    "g" = [System.Drawing.Color]::FromArgb(255, 72, 132, 62)
    "f" = [System.Drawing.Color]::FromArgb(255, 52, 102, 46)
    "j" = [System.Drawing.Color]::FromArgb(255, 108, 76, 44)
    "b" = [System.Drawing.Color]::FromArgb(255, 74, 52, 32)
    "e" = [System.Drawing.Color]::FromArgb(255, 42, 42, 48)
    "c" = [System.Drawing.Color]::FromArgb(255, 164, 132, 72)
    "u" = [System.Drawing.Color]::FromArgb(255, 134, 106, 56)
    "w" = [System.Drawing.Color]::FromArgb(255, 230, 224, 210)
    "r" = [System.Drawing.Color]::FromArgb(255, 162, 54, 48)
    "k" = [System.Drawing.Color]::FromArgb(255, 168, 88, 48)
    "n" = [System.Drawing.Color]::FromArgb(255, 138, 68, 32)
    "m" = [System.Drawing.Color]::FromArgb(255, 188, 168, 108)
}

# ---- ELARA, THE HERB TRADER: Quiet woman, long brown hair, green forest dress ----
# d=dark hair, h=hair, a=skin shadow, s=skin, e=eyes, g=green dress, f=dark dress, j=apron, b=boots
$elara1 = @(
    "................",
    ".....odddo......",
    "....odhhhdo.....",
    "....odhhhdo.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ogfffggo.....",
    "..ogffjjffgo....",
    "..ogffjjffgo....",
    "..ogfffffggo....",
    "...ogfffggo.....",
    "...ogfffggo.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$elara2 = @(
    "................",
    ".....odddo......",
    "....odhhhdo.....",
    "....odhhhdo.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...ogfffggo.....",
    "..ogffjjffgo....",
    "..ogffjjffgo....",
    "..ogfffffggo....",
    "...ogfffggo.....",
    "...ogfffggo.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# ---- BARNETT, THE CARTOGRAPHER: Enthusiastic mapmaker, tan coat, red scarf ----
# n=sandy dark, k=sandy hair, a=skin shadow, s=skin, e=eyes, r=red scarf,
# c=coat, u=coat dark, w=white shirt, b=boots
$barnett1 = @(
    "................",
    ".....onkno......",
    "....onkkkno.....",
    "....onkkkno.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...orrrrrrro....",
    "..ouuwwwwuuco...",
    "..ouuwwwwuuco...",
    "..ouuuuuuuuco...",
    "...ouuuuuuco....",
    "...ouuuuuuco....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$barnett2 = @(
    "................",
    ".....onkno......",
    "....onkkkno.....",
    "....onkkkno.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...orrrrrrro....",
    "..ouuwwwwuuco...",
    "..ouuwwwwuuco...",
    "..ouuuuuuuuco...",
    "...ouuuuuuco....",
    "...ouuuuuuco....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# NPC sheet: 2 NPCs x 2 frames = 32 x 32
$b1NpcSheet = New-Sheet -Width 32 -Height 32
Paint-Sprite $b1NpcSheet $elara1    0  0  $b1NpcPalette
Paint-Sprite $b1NpcSheet $elara2    16 0  $b1NpcPalette
Paint-Sprite $b1NpcSheet $barnett1  0  16 $b1NpcPalette
Paint-Sprite $b1NpcSheet $barnett2  16 16 $b1NpcPalette
$b1NpcPath = Join-Path $outDir "npcs.png"
$b1NpcSheet.Save($b1NpcPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 1 — MOSSHAVEN WILDS — ITEM PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         g = herb green      d = herb dark
#   l = light leaf     b = pouch leather   u = pouch dark      s = string/tie
#   k = gold coin      n = gold dark       r = stone grey      q = stone dark
#   t = tonic teal     j = tonic dark      w = label white     p = parchment
#   a = parchment dark h = rare herb bright f = rare herb      z = glow
$b1ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "g" = [System.Drawing.Color]::FromArgb(255, 72, 148, 62)
    "d" = [System.Drawing.Color]::FromArgb(255, 52, 112, 46)
    "l" = [System.Drawing.Color]::FromArgb(255, 118, 198, 82)
    "b" = [System.Drawing.Color]::FromArgb(255, 116, 76, 42)
    "u" = [System.Drawing.Color]::FromArgb(255, 86, 56, 30)
    "s" = [System.Drawing.Color]::FromArgb(255, 188, 162, 92)
    "k" = [System.Drawing.Color]::FromArgb(255, 218, 188, 68)
    "n" = [System.Drawing.Color]::FromArgb(255, 188, 158, 48)
    "r" = [System.Drawing.Color]::FromArgb(255, 126, 126, 116)
    "q" = [System.Drawing.Color]::FromArgb(255, 96, 96, 88)
    "t" = [System.Drawing.Color]::FromArgb(255, 64, 128, 100)
    "j" = [System.Drawing.Color]::FromArgb(255, 44, 96, 74)
    "w" = [System.Drawing.Color]::FromArgb(255, 220, 216, 200)
    "p" = [System.Drawing.Color]::FromArgb(255, 178, 148, 108)
    "a" = [System.Drawing.Color]::FromArgb(255, 148, 118, 82)
    "h" = [System.Drawing.Color]::FromArgb(255, 92, 198, 72)
    "f" = [System.Drawing.Color]::FromArgb(255, 62, 158, 52)
    "z" = [System.Drawing.Color]::FromArgb(255, 180, 240, 120)
}

# ---- SEED POUCH: Small leather pouch with seed markings ----
$seedPouch = @(
    "................",
    "......ss........",
    ".....ssss.......",
    "....ouuuuo......",
    "...oubbbuuo.....",
    "...oubglbuo.....",
    "...oubglbuo.....",
    "...oubbbbuo.....",
    "...ouuuuuuo.....",
    "....ouuuuo......",
    ".....oooo.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- GOLD POUCH: Coin pouch for gold rewards ----
$goldPouch = @(
    "................",
    "......ss........",
    ".....ssss.......",
    "....ouuuuo......",
    "...oubbbuuo.....",
    "...oubkkbuo.....",
    "...oubnkbuo.....",
    "...oubbbbuo.....",
    "...ouuuuuuo.....",
    "....ouuuuo......",
    ".....oooo.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- ANTIDOTE TONIC: Small teal bottle for curing Bogwolf poison ----
$antidoteTonic = @(
    "................",
    ".......ww.......",
    "......owwo......",
    "......ojjo......",
    ".....ojttjo.....",
    "....ojttttjo....",
    "....ojttttjo....",
    "....ojttttjo....",
    "....ojttttjo....",
    "....ojttttjo....",
    "....ojjjjjjo....",
    ".....oooooo.....",
    "................",
    "................",
    "................",
    "................"
)

# ---- STONE TABLET FRAGMENT: Illegible stone slab piece ----
$stoneTablet = @(
    "................",
    "....oqqqqo......",
    "...oqrrrrqo.....",
    "..oqrrqqrrqo....",
    "..oqrqrrqrqo....",
    "..oqrrrrrrqo....",
    "..oqrqqqrrqo....",
    "..oqrrrrrrqo....",
    "..oqrrqrrqqo....",
    "...oqrrrrqo.....",
    "....oqqqqo......",
    ".....oooo.......",
    "................",
    "................",
    "................",
    "................"
)

# ---- RARE HEALING HERB: Bright green herb with glow ----
$rareHerb = @(
    "................",
    ".......zh.......",
    "......zhfz......",
    ".....zhffhz.....",
    "....zhffffhz....",
    ".....hffffh.....",
    "......hffh......",
    ".......dd.......",
    ".......dd.......",
    "......gddd......",
    ".......dd.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- MOSSHAVEN MAP: Regional map scroll ----
$mosshavenMap = @(
    "................",
    "....oaao........",
    "...oappapo......",
    "..oappppapo.....",
    "..oappddpapo....",
    "..oappddpapo....",
    "..oappppapo.....",
    "..oappppapo.....",
    "..oapppapapo....",
    "..oaaaaaao......",
    "...oaaaao.......",
    "....oooo........",
    "................",
    "................",
    "................",
    "................"
)

# Item sheet: 6 items at 16x16 = 48 x 32
$b1ItemSheet = New-Sheet -Width 48 -Height 32
Paint-Sprite $b1ItemSheet $seedPouch      0  0  $b1ItemPalette
Paint-Sprite $b1ItemSheet $goldPouch      16 0  $b1ItemPalette
Paint-Sprite $b1ItemSheet $antidoteTonic  32 0  $b1ItemPalette
Paint-Sprite $b1ItemSheet $stoneTablet    0  16 $b1ItemPalette
Paint-Sprite $b1ItemSheet $rareHerb       16 16 $b1ItemPalette
Paint-Sprite $b1ItemSheet $mosshavenMap   32 16 $b1ItemPalette
$b1ItemPath = Join-Path $outDir "items.png"
$b1ItemSheet.Save($b1ItemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated Biome 1 (Mosshaven Wilds) sprite sheets:"
Write-Host " - $b1TilePath"
Write-Host " - $b1EnemyPath"
Write-Host " - $b1NpcPath"
Write-Host " - $b1ItemPath"
