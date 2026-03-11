Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\sprites\biome\biome2"
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
# BIOME 2 — THE ASHENFALL REACHES — TILE PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    a = ash grey        d = dark ash        m = ash mid
#   r = ruin stone     s = ruin stone lt   b = broken brick    w = weathered wood
#   k = dark rubble    c = charcoal        e = ember glow      p = flagstone path
#   f = fire scar      g = grey ground     n = iron nail/dark  t = timber beam
#   h = ash highlight  y = yellow dim
$b2TilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "a" = [System.Drawing.Color]::FromArgb(255, 142, 134, 122)
    "d" = [System.Drawing.Color]::FromArgb(255, 98, 92, 84)
    "m" = [System.Drawing.Color]::FromArgb(255, 118, 112, 102)
    "r" = [System.Drawing.Color]::FromArgb(255, 128, 118, 104)
    "s" = [System.Drawing.Color]::FromArgb(255, 156, 148, 132)
    "b" = [System.Drawing.Color]::FromArgb(255, 108, 82, 68)
    "w" = [System.Drawing.Color]::FromArgb(255, 96, 78, 58)
    "k" = [System.Drawing.Color]::FromArgb(255, 62, 58, 52)
    "c" = [System.Drawing.Color]::FromArgb(255, 44, 42, 38)
    "e" = [System.Drawing.Color]::FromArgb(255, 186, 96, 42)
    "p" = [System.Drawing.Color]::FromArgb(255, 148, 140, 118)
    "f" = [System.Drawing.Color]::FromArgb(255, 72, 56, 44)
    "g" = [System.Drawing.Color]::FromArgb(255, 132, 126, 114)
    "n" = [System.Drawing.Color]::FromArgb(255, 38, 36, 34)
    "t" = [System.Drawing.Color]::FromArgb(255, 82, 66, 48)
    "h" = [System.Drawing.Color]::FromArgb(255, 168, 162, 148)
    "y" = [System.Drawing.Color]::FromArgb(255, 178, 162, 98)
}

# ---- ASH PLAIN: Open grey-brown ash expanse, wind-scattered ----
$ashPlain = @(
    "aamaadaamaadaaaa",
    "admaaaaaadaaaaaa",
    "aaaamaadaaaamaaa",
    "aadaaaaaamaaadaa",
    "aaaaaadaaaaaaamm",
    "amaaaaamaadaaaaa",
    "aadaaaaaaaaaaada",
    "aaaamadaadaaaaaa",
    "aaaaadaaaaaamada",
    "amaadaaaamaaadaa",
    "aadaadaaaaaaamaa",
    "aaaaaaadaaadaaaa",
    "amaaaaaaadaamaaa",
    "aaaadaaaaaaaaaaa",
    "aadaaaamaaadaaaa",
    "aaaamaaaaaaamaam"
)

# ---- RUIN WALL: Collapsed stone wall with broken sections ----
$ruinWall = @(
    "rrssrrssrrssrrss",
    "r..rr..rr..rr..",
    "rrssrrssrrssrrss",
    "rr..rr..rr..rrr",
    "rrssrrss..ssrrss",
    "r..rr..r...rr..",
    "rrssrrss..ssrrss",
    "rr..rr..rr..rrr",
    "rrssrrssrrssrrss",
    "r..rr..rr..rr..",
    "rrss..ssrrssrrss",
    "rr....rrr..rrr.",
    "rrss..ssrrssrrss",
    "r..rr..rr..rr..",
    "rrssrrssrrssrrss",
    "rrssrrssrrssrrss"
)

# ---- DEEP ASH: Waist-high ash pit, movement-halving ----
$deepAsh = @(
    "ddmdddmddddmddm",
    "dmdddddddmddddm",
    "ddddmddmddddddd",
    "dmdddddddddmddd",
    "ddddddmddddddmd",
    "dddmdddddmddddm",
    "dmddddddddddddm",
    "dddddmdddddmddd",
    "ddmddddddmddddm",
    "dddddddmddddddd",
    "dmddmdddddddmdd",
    "ddddddddmdddddm",
    "dddmdddddddmddd",
    "dmddddmdddddddd",
    "dddddddddmdddmd",
    "ddmdddddddddddm"
)

# ---- BELLTOWER: Tall cracked stone tower, bell visible at top ----
$belltower = @(
    "....rryyrrr.....",
    "...rryyyyyrrr...",
    "...rrssssrr.....",
    "....rssssr......",
    "....rssssr......",
    "....rssssr......",
    "...rrssssrr.....",
    "...rsssssssr....",
    "...rsssssssr....",
    "...rssssssrr....",
    "..rrssssssrr....",
    "..rrss..ssrr....",
    "..rrss..ssrr....",
    "..rrssssssrr....",
    "..rrrrrrrrrr....",
    "..rrrrrrrrrr...."
)

# ---- SALVAGER CAMP: Fireproof stone building, intact ----
$salvagerCamp = @(
    "....ssssssss....",
    "...ssssssssss...",
    "..ssrrrrrrrrss..",
    "..srr......rrs..",
    "..srr.eeee.rrs..",
    "..srr.eeee.rrs..",
    "..srr......rrs..",
    "..srrrrrrrrrrs..",
    "..srrr.rrrrrrs..",
    "..srrr.rrrrrrs..",
    "..srrr.rrrrrrs..",
    "..srrrrrrrrrrs..",
    "..ssrrrrrrrrss..",
    "...ssssssssss...",
    "....ssssssss....",
    "................"
)

# ---- RUIN FLOOR: Interior floor tile of collapsed buildings ----
$ruinFloor = @(
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss",
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss",
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss",
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss",
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss",
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss",
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss",
    "rrsrrsrrsrrsrrsr",
    "ssssssssssssssss"
)

# ---- ASH RUBBLE: Collapsed building debris mixed with ash ----
$ashRubble = @(
    "aabbrraarrbbaaaa",
    "abbrrbbaarrbbaam",
    "aabbrraabbrraaaa",
    "aarrbbaarraaabba",
    "abbaabbrrbbaaaaa",
    "aarraaaarraabbaa",
    "aabbrrbbaarraaaa",
    "aaarraabbrrbbaam",
    "abbaabbrraaaaaaa",
    "aarrbbaarrbbaaaa",
    "aabbaarraarrbbam",
    "aarrbbaarraabaaa",
    "abbaabbrrbbaaaaa",
    "aarraaaarraabbaa",
    "aabbrrbbaarrbbaa",
    "aaaarraabbrraaaa"
)

# ---- RUIN DOORWAY: Intact doorframe in collapsed building ----
$ruinDoorway = @(
    "rrsrrrrrrrrrsrrr",
    "rrsrrrrrrrrrsrrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrr......srrr",
    "rrsrrccccccsrrr",
    "rrsrrrrrrrrrsrrr",
    "rrsrrrrrrrrrsrrr"
)

# ---- CRACKED FLAGSTONE: Ancient pre-fire road surface ----
$crackedFlagstone = @(
    "pppppppppppppppp",
    "pppp.pppppppp.pp",
    "ppp.pppppppppppp",
    "pppppppp.ppppppp",
    "pppppppppppp.ppp",
    "pppp.pppppppppppp",
    "pppppppppppppppp",
    "pp.pppppppppppp.",
    "pppppppp.ppppppp",
    "pppppppppppppppp",
    "pppppppppppp.ppp",
    "pppp.pppppppppppp",
    "pppppppppppppppp",
    "ppp.pppppppppppp",
    "pppppppp.ppppppp",
    "pppppppppppppppp"
)

# ---- ASH DRIFT: Subtle wind-blown ash pattern (navigation aid) ----
$ashDrift = @(
    "aaaaaahaaaamaaaa",
    "aamaaahhaadaaaaa",
    "aaaaaaahhaaaaaaa",
    "aadaaaaahhaadaam",
    "aaaamaaaahhaaaaa",
    "aaaaaaaaaaahhaam",
    "aamaadaaaaaahhaa",
    "aaaaaaaamaaaahhh",
    "aadaaaaaaaaaamaa",
    "aaaamaadaaaaaaaa",
    "aaaaaaaaaaaadaam",
    "aamaaaaamaadaaaa",
    "aaaaadaaaaaaaaaa",
    "aadaaaaaamaaadaa",
    "aaaaaadaaaaaaamm",
    "aamaaaaaaaadaaaa"
)

# Write the biome 2 tile sheet — 10 tiles
$b2TileSheet = New-Sheet -Width 144 -Height 32
Paint-Sprite $b2TileSheet $ashPlain          0   0 $b2TilePalette
Paint-Sprite $b2TileSheet $ruinWall          16  0 $b2TilePalette
Paint-Sprite $b2TileSheet $deepAsh           32  0 $b2TilePalette
Paint-Sprite $b2TileSheet $belltower         48  0 $b2TilePalette
Paint-Sprite $b2TileSheet $salvagerCamp      64  0 $b2TilePalette
Paint-Sprite $b2TileSheet $ruinFloor         80  0 $b2TilePalette
Paint-Sprite $b2TileSheet $ashRubble         96  0 $b2TilePalette
Paint-Sprite $b2TileSheet $ruinDoorway       112 0 $b2TilePalette
Paint-Sprite $b2TileSheet $crackedFlagstone  128 0 $b2TilePalette
Paint-Sprite $b2TileSheet $ashDrift          0   16 $b2TilePalette
$b2TilePath = Join-Path $outDir "tiles.png"
$b2TileSheet.Save($b2TilePath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 2 — THE ASHENFALL REACHES — ENEMY PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         w = wraith white    g = wraith grey
#   i = wraith glow    r = red eye         c = charcoal golem  s = stone golem
#   b = brown rubble   k = dark crack      t = tan ashkite     n = dark wing
#   p = purple aura    a = amber eye       e = spider body     d = spider dark
#   y = yellow eye     f = feather
$b2EnemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "w" = [System.Drawing.Color]::FromArgb(255, 198, 194, 186)
    "g" = [System.Drawing.Color]::FromArgb(255, 148, 142, 134)
    "i" = [System.Drawing.Color]::FromArgb(255, 218, 228, 242)
    "r" = [System.Drawing.Color]::FromArgb(255, 186, 52, 48)
    "c" = [System.Drawing.Color]::FromArgb(255, 78, 72, 64)
    "s" = [System.Drawing.Color]::FromArgb(255, 118, 112, 98)
    "b" = [System.Drawing.Color]::FromArgb(255, 98, 78, 58)
    "k" = [System.Drawing.Color]::FromArgb(255, 48, 44, 38)
    "t" = [System.Drawing.Color]::FromArgb(255, 148, 128, 96)
    "n" = [System.Drawing.Color]::FromArgb(255, 88, 78, 62)
    "p" = [System.Drawing.Color]::FromArgb(255, 142, 108, 168)
    "a" = [System.Drawing.Color]::FromArgb(255, 218, 168, 48)
    "e" = [System.Drawing.Color]::FromArgb(255, 66, 56, 42)
    "d" = [System.Drawing.Color]::FromArgb(255, 42, 36, 28)
    "y" = [System.Drawing.Color]::FromArgb(255, 212, 196, 52)
    "f" = [System.Drawing.Color]::FromArgb(255, 168, 148, 112)
}

# ---- ASH WRAITH: Ghostly shape emerging from ash, glowing white-grey ----
$ashWraith1 = @(
    "................",
    "......iwi.......",
    ".....iwwwi......",
    "....iwwwwwi.....",
    "...igwrwrwgi....",
    "...igwwwwwgi....",
    "..igwwwwwwwgi...",
    "..igwgwwwgwgi...",
    "...igwwwwwgi....",
    "....igwwwgi.....",
    ".....igwgi......",
    "......igi.......",
    ".......i........",
    "................",
    "................",
    "................"
)

$ashWraith2 = @(
    "................",
    ".......iwi......",
    "......iwwi......",
    ".....iwwwwi.....",
    "....igwrwrgi....",
    "...igwwwwwwgi...",
    "...igwwwwwwgi...",
    "..igwgwwgwwgi...",
    "...igwwwwwgi....",
    "...igwwwwgi.....",
    "....igwgi.......",
    "......igi.......",
    "................",
    "................",
    "................",
    "................"
)

# ---- RUIN GOLEM: Heavy rubble construct, slow, cracked stone body ----
$ruinGolem1 = @(
    "................",
    ".....ossso......",
    "....oscccsso....",
    "...osccrccso....",
    "...oscccccsoo...",
    "..ossscccsssoo..",
    "..ossscccssso...",
    ".osscccccccssoo.",
    ".osscccccccssso.",
    ".osscccccccssso.",
    "..osscccccssoo..",
    "..osscccccssso..",
    "...ossbobssoo...",
    "...ossb.bsso....",
    "....bbb.bbb.....",
    "................"
)

$ruinGolem2 = @(
    "................",
    ".....ossso......",
    "....oscccsso....",
    "...osccrccso....",
    "...oscccccsoo...",
    "..ossscccsssoo..",
    "..ossscccssso...",
    ".osscccccccssoo.",
    ".osscccccccssso.",
    ".osscccccccssso.",
    "..osscccccssoo..",
    "..osscccccssso..",
    "...ossbo.bssoo..",
    "...osb..bsso....",
    "....bbb..bbb....",
    "................"
)

# ---- ASHKITE: Swooping bird of prey, grey-brown, dive attack ----
$ashkite1 = @(
    "................",
    "..nn........nn..",
    ".nnff......ffnn.",
    "nnffff....ffffnn",
    ".nffffffffffffffn",
    "...nffffffff n...",
    "....nfftffn.....",
    ".....ntttn......",
    "......ntn.......",
    ".....naaan......",
    "......ntn.......",
    ".......n........",
    "................",
    "................",
    "................",
    "................"
)

$ashkite2 = @(
    "................",
    "................",
    "..nnff....ffnn..",
    ".nnffff..ffffnn.",
    "nnffffffffffffff",
    "..nnffffffffnn..",
    "....nfftffn.....",
    ".....ntttn......",
    "......ntn.......",
    ".....naaan......",
    "......ntn.......",
    ".......n........",
    "................",
    "................",
    "................",
    "................"
)

# ---- RUBBLE SPIDER: Small dark spider, clusters of 3-4, drops from above ----
$rubbleSpider1 = @(
    "................",
    "................",
    "................",
    "....d.dd.d......",
    "...ddeeeed......",
    "...deeyeed......",
    "...ddeeeed......",
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

$rubbleSpider2 = @(
    "................",
    "................",
    "................",
    "...d..dd..d.....",
    "....deeeedd.....",
    "...deeyeeed.....",
    "....deeeedd.....",
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

# Enemy sheet: 4 enemies x 2 frames = 32 x 64
$b2EnemySheet = New-Sheet -Width 32 -Height 64
Paint-Sprite $b2EnemySheet $ashWraith1      0  0  $b2EnemyPalette
Paint-Sprite $b2EnemySheet $ashWraith2      16 0  $b2EnemyPalette
Paint-Sprite $b2EnemySheet $ruinGolem1      0  16 $b2EnemyPalette
Paint-Sprite $b2EnemySheet $ruinGolem2      16 16 $b2EnemyPalette
Paint-Sprite $b2EnemySheet $ashkite1        0  32 $b2EnemyPalette
Paint-Sprite $b2EnemySheet $ashkite2        16 32 $b2EnemyPalette
Paint-Sprite $b2EnemySheet $rubbleSpider1   0  48 $b2EnemyPalette
Paint-Sprite $b2EnemySheet $rubbleSpider2   16 48 $b2EnemyPalette
$b2EnemyPath = Join-Path $outDir "enemies.png"
$b2EnemySheet.Save($b2EnemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 2 — THE ASHENFALL REACHES — NPC PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         s = skin            a = skin shadow
#   h = hair dark      m = hair mid        e = eyes            b = boots
#   r = rust vest      d = dark vest       g = grey shirt      w = white (oswin)
#   c = coat brown     f = coat dark       k = spectacles rim  n = notebook tan
#   p = pants grey     t = trim
$b2NpcPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "s" = [System.Drawing.Color]::FromArgb(255, 216, 182, 148)
    "a" = [System.Drawing.Color]::FromArgb(255, 186, 152, 118)
    "h" = [System.Drawing.Color]::FromArgb(255, 56, 42, 32)
    "m" = [System.Drawing.Color]::FromArgb(255, 86, 62, 44)
    "e" = [System.Drawing.Color]::FromArgb(255, 42, 42, 48)
    "b" = [System.Drawing.Color]::FromArgb(255, 64, 48, 34)
    "r" = [System.Drawing.Color]::FromArgb(255, 148, 82, 52)
    "d" = [System.Drawing.Color]::FromArgb(255, 112, 62, 38)
    "g" = [System.Drawing.Color]::FromArgb(255, 142, 136, 124)
    "w" = [System.Drawing.Color]::FromArgb(255, 218, 214, 204)
    "c" = [System.Drawing.Color]::FromArgb(255, 132, 106, 72)
    "f" = [System.Drawing.Color]::FromArgb(255, 98, 78, 52)
    "k" = [System.Drawing.Color]::FromArgb(255, 168, 156, 128)
    "n" = [System.Drawing.Color]::FromArgb(255, 194, 174, 128)
    "p" = [System.Drawing.Color]::FromArgb(255, 104, 98, 86)
    "t" = [System.Drawing.Color]::FromArgb(255, 172, 118, 72)
}

# ---- MAREN, THE SALVAGER: Practical woman, dark hair, rust-coloured work vest ----
$maren1 = @(
    "................",
    ".....ohhho......",
    "....ohmmmho.....",
    "....ohmmmho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...orddddrro....",
    "..orddddddrro...",
    "..ordggggddrro..",
    "..orddddddrro...",
    "...orddddrro....",
    "...orpppprro....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$maren2 = @(
    "................",
    ".....ohhho......",
    "....ohmmmho.....",
    "....ohmmmho.....",
    "...oaseeaso.....",
    "...oassssao.....",
    "...orddddrro....",
    "..orddddddrro...",
    "..ordggggddrro..",
    "..orddddddrro...",
    "...orddddrro....",
    "...orpppprro....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# ---- OSWIN, THE HISTORIAN: Elderly man, white hair, brown coat, spectacles ----
$oswin1 = @(
    "................",
    ".....owwwo......",
    "....owwwwwo.....",
    "....owwwwwo.....",
    "...oakeekas.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffwwffcco...",
    "..ocffffffcco...",
    "...ocffffcco....",
    "...ocppppco.....",
    "....obbobbo.....",
    "....obb.bbo.....",
    "................",
    "................"
)

$oswin2 = @(
    "................",
    ".....owwwo......",
    "....owwwwwo.....",
    "....owwwwwo.....",
    "...oakeekas.....",
    "...oassssao.....",
    "...ocfffcco.....",
    "..ocffwwffcco...",
    "..ocffwwffcco...",
    "..ocffffffcco...",
    "...ocffffcco....",
    "...ocppppco.....",
    "....obbo.bbo....",
    "....bbo..bbo....",
    "................",
    "................"
)

# NPC sheet: 2 NPCs x 2 frames = 32 x 32
$b2NpcSheet = New-Sheet -Width 32 -Height 32
Paint-Sprite $b2NpcSheet $maren1   0  0  $b2NpcPalette
Paint-Sprite $b2NpcSheet $maren2   16 0  $b2NpcPalette
Paint-Sprite $b2NpcSheet $oswin1   0  16 $b2NpcPalette
Paint-Sprite $b2NpcSheet $oswin2   16 16 $b2NpcPalette
$b2NpcPath = Join-Path $outDir "npcs.png"
$b2NpcSheet.Save($b2NpcPath, [System.Drawing.Imaging.ImageFormat]::Png)

# =============================================================================
# BIOME 2 — THE ASHENFALL REACHES — ITEM PALETTE
# =============================================================================
# Key mapping (case-insensitive safe):
#   . = transparent    o = outline         r = rust metal      d = dark metal
#   g = gold           n = gold dark       s = stone grey      q = stone dark
#   b = leather brown  u = leather dark    w = paper white     p = parchment
#   a = parchment dark c = charcoal mark   k = chain link      e = ember glow
#   t = rope tan       f = rope dark       h = highlight
$b2ItemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 30, 38)
    "r" = [System.Drawing.Color]::FromArgb(255, 156, 92, 54)
    "d" = [System.Drawing.Color]::FromArgb(255, 112, 68, 38)
    "g" = [System.Drawing.Color]::FromArgb(255, 218, 188, 68)
    "n" = [System.Drawing.Color]::FromArgb(255, 178, 148, 48)
    "s" = [System.Drawing.Color]::FromArgb(255, 136, 128, 114)
    "q" = [System.Drawing.Color]::FromArgb(255, 96, 90, 78)
    "b" = [System.Drawing.Color]::FromArgb(255, 116, 82, 48)
    "u" = [System.Drawing.Color]::FromArgb(255, 82, 58, 34)
    "w" = [System.Drawing.Color]::FromArgb(255, 224, 218, 198)
    "p" = [System.Drawing.Color]::FromArgb(255, 188, 172, 136)
    "a" = [System.Drawing.Color]::FromArgb(255, 148, 132, 98)
    "c" = [System.Drawing.Color]::FromArgb(255, 54, 48, 42)
    "k" = [System.Drawing.Color]::FromArgb(255, 142, 136, 122)
    "e" = [System.Drawing.Color]::FromArgb(255, 198, 112, 46)
    "t" = [System.Drawing.Color]::FromArgb(255, 168, 142, 96)
    "f" = [System.Drawing.Color]::FromArgb(255, 128, 106, 68)
    "h" = [System.Drawing.Color]::FromArgb(255, 198, 192, 178)
}

# ---- SALVAGE TOKEN: Rusted metal disc with faded emblem ----
$salvageToken = @(
    "................",
    ".....oddo.......",
    "....odrrdo......",
    "...odrrrrrdo....",
    "..odrrrrrrrdo...",
    "..odrrrdrrredo..",
    "..odrrrrrrrdo...",
    "..odrrrdrrredo..",
    "..odrrrrrrrdo...",
    "...odrrrrrdo....",
    "....odrrdo......",
    ".....oddo.......",
    "................",
    "................",
    "................",
    "................"
)

# ---- ASHENFALL LOCKET: Small ornate locket, tarnished gold ----
$ashenfallLocket = @(
    "................",
    "......kk........",
    ".....knngk......",
    "....ongggno.....",
    "...onggggno.....",
    "...ongnggnoo....",
    "...onggggno.....",
    "....ongggno.....",
    ".....onnno......",
    "......oo........",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- BURNED PAGE: Partially legible journal page, charred edges ----
$burnedPage = @(
    "................",
    "...occwwwco.....",
    "..ocwwwwwwco....",
    "..owwcwwwwwo....",
    "..owwwwwcwwo....",
    "..owwwwwwwwco...",
    "..owwcwwwwwo....",
    "..owwwwwcwwo....",
    "..owwwwwwwwco...",
    "..ocwwcwwwco....",
    "...occwwwco.....",
    "....oocco.......",
    "................",
    "................",
    "................",
    "................"
)

# ---- WORN SOLDIER'S BADGE: Metal badge, scratched, name on back ----
$soldierBadge = @(
    "................",
    "......oo........",
    ".....osso.......",
    "....ossssso.....",
    "...ossqqssso....",
    "...osqssqsso....",
    "...ossqqssso....",
    "...ossssssso....",
    "....ossssso.....",
    ".....osso.......",
    "......oo........",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# ---- ROPE: Coiled rope, trade item ----
$rope = @(
    "................",
    ".....tttt.......",
    "....tfttftt.....",
    "...tftttttft....",
    "..tftttttttft...",
    "..tftt....tft...",
    "..tft......tft..",
    "..tft......tft..",
    "..tftt....tft...",
    "..tftttttttft...",
    "...tftttttft....",
    "....tfttftt.....",
    ".....tttt.......",
    "................",
    "................",
    "................"
)

# ---- GOLD POUCH (ash-themed): Darker leather pouch with gold ----
$ashGoldPouch = @(
    "................",
    "......aa........",
    ".....aaaa.......",
    "....ouuuuo......",
    "...oubbbuuo.....",
    "...oubggbuo.....",
    "...oubngnbuo....",
    "...oubbbbuoo....",
    "...ouuuuuuoo....",
    "....ouuuuoo.....",
    ".....oooo.......",
    "................",
    "................",
    "................",
    "................",
    "................"
)

# Item sheet: 6 items at 16x16 = 48 x 32
$b2ItemSheet = New-Sheet -Width 48 -Height 32
Paint-Sprite $b2ItemSheet $salvageToken     0  0  $b2ItemPalette
Paint-Sprite $b2ItemSheet $ashenfallLocket  16 0  $b2ItemPalette
Paint-Sprite $b2ItemSheet $burnedPage       32 0  $b2ItemPalette
Paint-Sprite $b2ItemSheet $soldierBadge     0  16 $b2ItemPalette
Paint-Sprite $b2ItemSheet $rope             16 16 $b2ItemPalette
Paint-Sprite $b2ItemSheet $ashGoldPouch     32 16 $b2ItemPalette
$b2ItemPath = Join-Path $outDir "items.png"
$b2ItemSheet.Save($b2ItemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated Biome 2 (Ashenfall Reaches) sprite sheets:"
Write-Host " - $b2TilePath"
Write-Host " - $b2EnemyPath"
Write-Host " - $b2NpcPath"
Write-Host " - $b2ItemPath"
