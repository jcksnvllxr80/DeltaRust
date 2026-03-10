Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "assets\\sprites"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

function New-Sheet {
    param(
        [int]$Width,
        [int]$Height
    )

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
                $targetX -ge 0 -and
                $targetY -ge 0 -and
                $targetX -lt $Bitmap.Width -and
                $targetY -lt $Bitmap.Height
            ) {
                $Bitmap.SetPixel($targetX, $targetY, $Palette[$ch])
            }
        }
    }
}

$heroPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 30, 44, 58)
    "h" = [System.Drawing.Color]::FromArgb(255, 110, 67, 34)
    "m" = [System.Drawing.Color]::FromArgb(255, 182, 122, 58)
    "s" = [System.Drawing.Color]::FromArgb(255, 246, 214, 180)
    "w" = [System.Drawing.Color]::FromArgb(255, 250, 248, 238)
    "g" = [System.Drawing.Color]::FromArgb(255, 78, 170, 84)
    "d" = [System.Drawing.Color]::FromArgb(255, 42, 112, 58)
    "c" = [System.Drawing.Color]::FromArgb(255, 56, 96, 148)
    "y" = [System.Drawing.Color]::FromArgb(255, 222, 190, 92)
    "r" = [System.Drawing.Color]::FromArgb(255, 154, 56, 48)
    "b" = [System.Drawing.Color]::FromArgb(255, 122, 78, 46)
}

$heroDown1 = @(
    "................",
    ".....ohhhho.....",
    "....ohmmmmho....",
    "....hmssssmh....",
    "...ohsswwssho...",
    "...ohssssssho...",
    "...ogggyygggo...",
    "..ogggyyyygggo..",
    "..ogddggggddgo..",
    "..ogddrrrrddgo..",
    "...odggggggdo...",
    "...oddddddddo...",
    "...osdb..bdso...",
    "...osd....dso...",
    "....bb....bb....",
    "...bb......bb..."
)

$heroDown2 = @(
    "................",
    ".....ohhhho.....",
    "....ohmmmmho....",
    "....hmssssmh....",
    "...ohsswwssho...",
    "...ohssssssho...",
    "...ogggyygggo...",
    "..ogggyyyygggo..",
    "..ogddggggddgo..",
    "..ogddrrrrddgo..",
    "...odggggggdo...",
    "...oddddddddo...",
    "...osdb..bdso...",
    "...osd....dso...",
    ".....bb..bb.....",
    "....bb....bb...."
)

$heroUp1 = @(
    "................",
    ".....ohhhho.....",
    "....ohyyyyho....",
    "....ohhhhhho....",
    "...ohhccccsho...",
    "...ogggccccgo...",
    "...ogggyygggo...",
    "..ogggyyyygggo..",
    "..ogddggggddgo..",
    "..ogddrrrrddgo..",
    "...odggggggdo...",
    "...odggccggdo...",
    "...oddddddddo...",
    "...osdb..bdso...",
    "...osd....dso...",
    "....bb....bb....",
    "...bb......bb..."
)

$heroUp2 = @(
    "................",
    ".....ohhhho.....",
    "....ohyyyyho....",
    "....ohhhhhho....",
    "...ohhccccsho...",
    "...ogggccccgo...",
    "...ogggyygggo...",
    "..ogggyyyygggo..",
    "..ogddggggddgo..",
    "..ogddrrrrddgo..",
    "...odggggggdo...",
    "...odggccggdo...",
    "...oddddddddo...",
    "...osdb..bdso...",
    "...osd....dso...",
    ".....bb..bb.....",
    "....bb....bb...."
)

$heroLeft1 = @(
    "................",
    ".....ohhhho.....",
    "....ohmmmmho....",
    "....hsssssso....",
    "...ohsswwssho...",
    "...ohssssssdo...",
    "..oggggyyygdo...",
    "..ogggyyyyddo...",
    ".ogddggggggdo...",
    ".ogddrrrrggdo...",
    ".ogdggggggddo...",
    "..odggccdddo....",
    "..osdb.ddso.....",
    "...sdd..dso.....",
    "...bb...bb......",
    "..bb.....bb....."
)

$heroLeft2 = @(
    "................",
    ".....ohhhho.....",
    "....ohmmmmho....",
    "....hsssssso....",
    "...ohsswwssho...",
    "...ohssssssdo...",
    "..oggggyyygdo...",
    "..ogggyyyyddo...",
    ".ogddggggggdo...",
    ".ogddrrrrggdo...",
    ".ogdggggggddo...",
    "..odggccdddo....",
    "..osdd.bdso.....",
    "...sd..ddso.....",
    "....bb..bb......",
    "...bb....bb....."
)

$heroRight1 = @(
    "................",
    ".....ohhhho.....",
    "....ohmmmmho....",
    "....ossssssh....",
    "...ohsswwssho...",
    "...odssssssho...",
    "...odgyyyggggo..",
    "...oddyyyyggggo.",
    "...odggggggddgo.",
    "...odggrrrrddgo.",
    "...oddggggggdgo.",
    "....odddccggdo..",
    ".....osdd.bdso..",
    ".....osd..dds...",
    "......bb...bb...",
    ".....bb.....bb.."
)

$heroRight2 = @(
    "................",
    ".....ohhhho.....",
    "....ohmmmmho....",
    "....ossssssh....",
    "...ohsswwssho...",
    "...odssssssho...",
    "...odgyyyggggo..",
    "...oddyyyyggggo.",
    "...odggggggddgo.",
    "...odggrrrrddgo.",
    "...oddggggggdgo.",
    "....odddccggdo..",
    ".....osdb.ddso..",
    ".....osdd..ds...",
    "......bb..bb....",
    ".....bb....bb..."
)

$heroSheet = New-Sheet -Width 32 -Height 64
Paint-Sprite $heroSheet $heroDown1 0 0 $heroPalette
Paint-Sprite $heroSheet $heroDown2 16 0 $heroPalette
Paint-Sprite $heroSheet $heroUp1 0 16 $heroPalette
Paint-Sprite $heroSheet $heroUp2 16 16 $heroPalette
Paint-Sprite $heroSheet $heroLeft1 0 32 $heroPalette
Paint-Sprite $heroSheet $heroLeft2 16 32 $heroPalette
Paint-Sprite $heroSheet $heroRight1 0 48 $heroPalette
Paint-Sprite $heroSheet $heroRight2 16 48 $heroPalette
$heroPath = Join-Path $outDir "hero.png"
$heroSheet.Save($heroPath, [System.Drawing.Imaging.ImageFormat]::Png)

$enemyPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "o" = [System.Drawing.Color]::FromArgb(255, 64, 88, 132)
    "g" = [System.Drawing.Color]::FromArgb(255, 92, 220, 104)
    "l" = [System.Drawing.Color]::FromArgb(255, 132, 245, 140)
    "r" = [System.Drawing.Color]::FromArgb(255, 206, 76, 66)
    "2" = [System.Drawing.Color]::FromArgb(255, 168, 44, 38)
    "w" = [System.Drawing.Color]::FromArgb(255, 240, 240, 240)
    "p" = [System.Drawing.Color]::FromArgb(255, 132, 92, 188)
    "u" = [System.Drawing.Color]::FromArgb(255, 74, 86, 154)
    "m" = [System.Drawing.Color]::FromArgb(255, 170, 56, 48)
    "y" = [System.Drawing.Color]::FromArgb(255, 232, 214, 120)
    "e" = [System.Drawing.Color]::FromArgb(255, 250, 250, 255)
    "b" = [System.Drawing.Color]::FromArgb(255, 110, 74, 164)
}

$slime1 = @(
    "................",
    "................",
    ".....llllll.....",
    "...llggggggll...",
    "..lggggggggggl..",
    "..lgggg..ggggl..",
    ".lggggggggggggl.",
    ".lggggggggggggl.",
    ".oggggggggggggo.",
    "..oggggggggggo..",
    "...ooggggggoo...",
    "....oooooooo....",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$slime2 = @(
    "................",
    "................",
    "................",
    ".....llllll.....",
    "...llggggggll...",
    "..lggggggggggl..",
    ".lggggg..gggggl.",
    ".lggggggggggggl.",
    "..oggggggggggo..",
    "...oggggggggo...",
    "...ooggggggoo...",
    "....oooooooo....",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$octorok1 = @(
    "................",
    ".....orrrro.....",
    "...orrrrrrrro...",
    "..orrrwwwwrrro..",
    "..orrrwrrwrrro..",
    ".orrrrrrrrrrrro.",
    ".orrrr2222rrrro.",
    ".orrrrrrrrrrrro.",
    "..orrr....rrro..",
    "..or.r....r.ro..",
    "...r.r....r.r...",
    "....o......o....",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$octorok2 = @(
    "................",
    ".....orrrro.....",
    "...orrrrrrrro...",
    "..orrrwwwwrrro..",
    "..orrrwrrwrrro..",
    ".orrrrrrrrrrrro.",
    ".orrrr2222rrrro.",
    ".orrrrrrrrrrrro.",
    "..orr..rr..rro..",
    "...r.r....r.r...",
    "...or......ro...",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$bat1 = @(
    "................",
    "...pp......pp...",
    "..pppp....pppp..",
    ".pppppp..pppppp.",
    "pppppppppppppppp",
    "..pppppppppppp..",
    "....pppppppp....",
    "...pp.peep.pp...",
    "..pp........pp..",
    ".pp..........pp.",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$bat2 = @(
    "................",
    "................",
    "..pppp....pppp..",
    ".pppppp..pppppp.",
    "pppppppppppppppp",
    "..pppppppppppp..",
    "....pppppppp....",
    "...pp.peep.pp...",
    "..pp........pp..",
    "...p........p...",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................",
    "................"
)

$darknut1 = @(
    "................",
    ".....ouuuuo.....",
    "....ou2222uo....",
    "...ou22ee22uo...",
    "...ou222222uo...",
    "..ouu222222uuo..",
    "..ouu222222uuo..",
    "..ouu22yy22uuo..",
    "..ouu222222uuo..",
    "...ouu2222uuo...",
    "...oou....uoo...",
    "...obo....obo...",
    "....bb....bb....",
    "................",
    "................",
    "................",
    "................"
)

$darknut2 = @(
    "................",
    ".....ouuuuo.....",
    "....ou2222uo....",
    "...ou22ee22uo...",
    "...ou222222uo...",
    "..ouu222222uuo..",
    "..ouu22yy22uuo..",
    "..ouu222222uuo..",
    "..ouu222222uuo..",
    "...ouu2222uuo...",
    "...ouo....ouo...",
    "...ob......bo...",
    "....bb....bb....",
    "................",
    "................",
    "................",
    "................"
)

$boss1 = @(
    "........................",
    ".......yy......yy.......",
    ".....yymmy....ymmy......",
    "...ym22222222222222my...",
    "..ym2222m222222m2222my..",
    "..m222222222222222222m..",
    ".m22222ww222222ww2222m.",
    ".m22222e22222222e2222m.",
    ".m222222222yy22222222m.",
    ".m2222222222222222222m.",
    ".m22222m22222222m2222m.",
    ".m2222m2222222222m222m.",
    "..m222m2222222222m22m..",
    "..mm22m2222222222m2mm..",
    "...mmmmmmmmmmmmmmmm....",
    "....mmb......bbmm......",
    "...bbb........bbb......",
    "........................",
    "........................",
    "........................",
    "........................",
    "........................",
    "........................",
    "........................"
)

$boss2 = @(
    "........................",
    "......yy........yy......",
    "....yymmy......ymmy.....",
    "...ym22222222222222my...",
    "..ym2222m222222m2222my..",
    "..m222222222222222222m..",
    ".m22222ww222222ww2222m.",
    ".m22222e22222222e2222m.",
    ".m222222222yy22222222m.",
    ".m22222m22222222m2222m.",
    ".m2222222222222222222m.",
    ".m2222m2222222222m222m.",
    "..m222m2222222222m22m..",
    "..mm22m2222222222m2mm..",
    "...mmmmmmmmmmmmmmmm....",
    "....bbm........mbb.....",
    "...bb..............bb..",
    "........................",
    "........................",
    "........................",
    "........................",
    "........................",
    "........................",
    "........................"
)

$enemySheet = New-Sheet -Width 48 -Height 88
Paint-Sprite $enemySheet $slime1 0 0 $enemyPalette
Paint-Sprite $enemySheet $slime2 16 0 $enemyPalette
Paint-Sprite $enemySheet $octorok1 0 16 $enemyPalette
Paint-Sprite $enemySheet $octorok2 16 16 $enemyPalette
Paint-Sprite $enemySheet $bat1 0 32 $enemyPalette
Paint-Sprite $enemySheet $bat2 16 32 $enemyPalette
Paint-Sprite $enemySheet $darknut1 0 48 $enemyPalette
Paint-Sprite $enemySheet $darknut2 16 48 $enemyPalette
Paint-Sprite $enemySheet $boss1 0 64 $enemyPalette
Paint-Sprite $enemySheet $boss2 24 64 $enemyPalette
$enemyPath = Join-Path $outDir "enemies.png"
$enemySheet.Save($enemyPath, [System.Drawing.Imaging.ImageFormat]::Png)

$tilePalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "g" = [System.Drawing.Color]::FromArgb(255, 74, 184, 90)
    "d" = [System.Drawing.Color]::FromArgb(255, 48, 132, 64)
    "t" = [System.Drawing.Color]::FromArgb(255, 100, 74, 44)
    "w" = [System.Drawing.Color]::FromArgb(255, 56, 132, 214)
    "r" = [System.Drawing.Color]::FromArgb(255, 126, 126, 116)
    "s" = [System.Drawing.Color]::FromArgb(255, 220, 190, 118)
    "p" = [System.Drawing.Color]::FromArgb(255, 174, 138, 88)
    "c" = [System.Drawing.Color]::FromArgb(255, 34, 34, 42)
    "m" = [System.Drawing.Color]::FromArgb(255, 124, 90, 118)
    "k" = [System.Drawing.Color]::FromArgb(255, 186, 128, 48)
    "u" = [System.Drawing.Color]::FromArgb(255, 84, 84, 122)
    "x" = [System.Drawing.Color]::FromArgb(255, 170, 52, 68)
    "b" = [System.Drawing.Color]::FromArgb(255, 116, 88, 54)
    "e" = [System.Drawing.Color]::FromArgb(255, 230, 222, 164)
}

$grass = @(
    "gggggggggggggggg","gggdggggggdggggg","gggggggggggggggg","ggggggdggggggggg",
    "ggggggggggggdggg","gggggggggggggggg","ggdggggggggggggg","gggggggggggggggg",
    "gggggggggdgggggg","gggggggggggggggg","ggggdggggggggggg","gggggggggggggggg",
    "ggggggggggggggdg","gggggggggggggggg","ggdggggggggggggg","gggggggggggggggg"
)
$tree = @(
    "ggggggttttgggggg","gggggtggggtggggg","ggggtggggggtgggg","gggtggggggggtggg",
    "gggtggggggggtggg","ggtggggggggggtgg","ggtgggdggdgggtgg","ggtggggggggggtgg",
    "gggtggggggggtggg","ggggttttttttgggg","ggggggttttgggggg","ggggggttttgggggg",
    "ggggggttttgggggg","ggggggttttgggggg","gggggggggggggggg","gggggggggggggggg"
)
$water = @(
    "wwwwwwwwwwwwwwww","w..www..www..www","wwwwwwwwwwwwwwww","ww..www..www..ww",
    "wwwwwwwwwwwwwwww","w..www..www..www","wwwwwwwwwwwwwwww","ww..www..www..ww",
    "wwwwwwwwwwwwwwww","w..www..www..www","wwwwwwwwwwwwwwww","ww..www..www..ww",
    "wwwwwwwwwwwwwwww","w..www..www..www","wwwwwwwwwwwwwwww","wwwwwwwwwwwwwwww"
)
$rock = @(
    "................","....rrrrrrrr....","..rrrrrrrrrrrr..","..rrrrrrrrrrrr..",
    ".rrrrrrr.rrrrrr."," .rrrrrrrrrrrrr..","..rrrrrrrrrrrr..","..rrrrr.rrrrrr..",
    "..rrrrrrrrrrrr..","...rrrrrrrrrr...","....rrrrrrrr....","................",
    "................","................","................","................"
)
$sand = @(
    "ssssssssssssssss","ss.sssssss.sssss","ssssssssssssssss","ssssss.sssssssss",
    "ssssssssssss.sss","ssssssssssssssss","ss.sssssssssssss","ssssssssssssssss",
    "ssssssss.sssssss","ssssssssssssssss","sssssssssssss.ss","ssssssssssssssss",
    "ssssssssssssssss","ssssssssss.sssss","ssssssssssssssss","ssssssssssssssss"
)
$path = @(
    "pppppppppppppppp","pppppppppppppppp","pppppppppppppppp","pppppppppppppppp",
    "pppppppppppppppp","pppppppppppppppp","pppppppppppppppp","pppppppppppppppp",
    "pppppppppppppppp","pppppppppppppppp","pppppppppppppppp","pppppppppppppppp",
    "pppppppppppppppp","pppppppppppppppp","pppppppppppppppp","pppppppppppppppp"
)
$cave = @(
    # diagonal wedge: dark on left increases each row, grey on right
    "rrrrrrrrrrrrrrrr",
    "crrrrrrrrrrrrrrr",
    "ccrrrrrrrrrrrrrr",
    "cccrrrrrrrrrrrrr",
    "ccccrrrrrrrrrrrr",
    "cccccrrrrrrrrrrr",
    "ccccccrrrrrrrrrr",
    "cccccccrrrrrrrrr",
    "ccccccccrrrrrrrr",
    "cccccccccrrrrrrr",
    "ccccccccccrrrrrr",
    "cccccccccccrrrrr",
    "ccccccccccccrrrr",
    "cccccccccccccrrr",
    "ccccccccccccccrr",
    "cccccccccccccccr"
)
$dungeon = @(
    "mmmmmmmmmmmmmmmm","muuumuuumuuumuuu","muuumuuumuuumuuu","muuumuuumuuumuuu",
    "muuumuuumuuumuuu","muuumuuumuuumuuu","muuumuuumuuumuuu","muuumuuumuuumuuu",
    "muuumuuumuuumuuu","muuumuuumuuumuuu","muuumuuumuuumuuu","muuumuuumuuumuuu",
    "muuumuuumuuumuuu","muuumuuumuuumuuu","muuumuuumuuumuuu","mmmmmmmmmmmmmmmm"
)
$cracked = @(
    "rrrrrrrrrrrrrrrr","rrrrrr.rrrrrrrrr","rrr.rrrrrrrrrrrr","rrrrrrrr.rrrrrrr",
    "rrrrrrrrrrrr.rrr","rrrr.rrrrrrrrrrr","rrrrrrrrrrrrrrrr","rr.rrrrrrrrrrrrr",
    "rrrrrrrrr.rrrrrr","rrrrrrrrrrrrrrrr","rrrrrrrrrrrr.rrr","rrrr.rrrrrrrrrrr",
    "rrrrrrrrrrrrrrrr","rrr.rrrrrrrrrrrr","rrrrrrrrrrrrrrrr","rrrrrrrrrrrrrrrr"
)
$bush = @(
    "................","....gggggggg....","...gggggggggg...","..gggdggdggdgg..",
    "..gggggggggggg..","..gggdggdggdgg..","..gggggggggggg..","...gggggggggg...",
    "....gggggggg....","......tttt......","......tttt......","................",
    "................","................","................","................"
)
$bridge = @(
    "bbbbbbbbbbbbbbbb","....b......b....","bbbbbbbbbbbbbbbb","....b......b....",
    "bbbbbbbbbbbbbbbb","....b......b....","bbbbbbbbbbbbbbbb","....b......b....",
    "bbbbbbbbbbbbbbbb","....b......b....","bbbbbbbbbbbbbbbb","....b......b....",
    "bbbbbbbbbbbbbbbb","....b......b....","bbbbbbbbbbbbbbbb","....b......b...."
)
$wall = @(
    "uuuuuuuuuuuuuuuu","u..uu..uu..uu..u","uuuuuuuuuuuuuuuu","uu..uu..uu..uuuu",
    "uuuuuuuuuuuuuuuu","u..uu..uu..uu..u","uuuuuuuuuuuuuuuu","uu..uu..uu..uuuu",
    "uuuuuuuuuuuuuuuu","u..uu..uu..uu..u","uuuuuuuuuuuuuuuu","uu..uu..uu..uuuu",
    "uuuuuuuuuuuuuuuu","u..uu..uu..uu..u","uuuuuuuuuuuuuuuu","uuuuuuuuuuuuuuuu"
)
$floor = @(
    "u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu","u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu",
    "u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu","u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu",
    "u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu","u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu",
    "u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu","u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu"
)
$doorLocked = @(
    "uuuuukkkkkuuuuuu","uuuukkkkkkkkuuuu","uuukkkkkkkkkkuuu","uuukkkkkkkkkkuuu",
    "uuukkkkkkkkkkuuu","uuukkkkkkkkkkuuu","uuukkkkkkkkkkuuu","uuukkkkkkkkkkuuu",
    "uuukkkkkxkkkkuuu","uuukkkkkkkkkkuuu","uuukkkkkkkkkkuuu","uuukkkkkkkkkkuuu",
    "uuukkkkkkkkkkuuu","uuukkkkkkkkkkuuu","uuuukkkkkkkkuuuu","uuuuukkkkkuuuuuu"
)
$door = @(
    "uuuuucccccuuuuuu","uuuuccccccccuuuu","uuucc....cccuuuu","uuucc......ccuuu",
    "uuucc......ccuuu","uuucc...e..ccuuu","uuucc......ccuuu","uuucc......ccuuu",
    "uuucc......ccuuu","uuucc......ccuuu","uuucc......ccuuu","uuucc......ccuuu",
    "uuucc....cccuuuu","uuucccccccccuuuu","uuuuccccccccuuuu","uuuuucccccuuuuuu"
)
$stairs = @(
    "uuuuuuuuuuuuuuuu","uuuuuuuuuuuuuuuu","u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu",
    "u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu","u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu",
    "u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu","u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu",
    "u.u.u.u.u.u.u.u.","uuuuuuuuuuuuuuuu","uuuuuuuuuuuuuuuu","uuuuuuuuuuuuuuuu"
)
$chest = @(
    "................","....kkkkkkkk....","...kkkkkkkkkk...","..kkkkkkkkkkkk..",
    "..kkkkkkkkkkkk..","..kkkkkxxkkkkk..","..kkkkkkkkkkkk..","..kkkkkkkkkkkk..",
    "..kttttttttttk..","..kttttttttttk..","..kttttttttttk..","..kkkkkkkkkkkk..",
    "................","................","................","................"
)
$goal = @(
    "......xx........",".....xxxx.......","....xxkkxx......","...xxkkkkxx.....",
    "..xxkkkkkkxx....","...xxkkkkxx.....","....xxkkxx......",".....xxxx.......",
    "......xx........",".....xxxx.......","....xxkkxx......","...xxkkkkxx.....",
    "..xxkkkkkkxx....","...xxkkkkxx.....","....xxkkxx......",".....xxxx......."
)
$bossDoor = @(
    "xxxxxxkkkkxxxxxx","xxxkkkkkkkkkxxxx","xxkkxxxxxxxxkkxxx","xxkxxxxxxxxxxkxxx",
    "xxkxxx....xxxkxx","xxkxx.xxxx.xxkxx","xxkxx.xxxx.xxkxx","xxkxxxxxxxxxxkxx",
    "xxkxxxxxxxxxxkxx","xxkxxxxxxxxxxkxx","xxkxx.xxxx.xxkxx","xxkxx.xxxx.xxkxx",
    "xxkxxx....xxxkxx","xxkxxxxxxxxxxkxx","xxxkkkkkkkkkxxxx","xxxxxxkkkkxxxxxx"
)
$floorAlt = @(
    "uuuuuuuuuuuuuuuu","uu.u.uuuu.u.uuuu","uuuuuuuuuuuuuuuu","u.uuuu.u.uuuu.uu",
    "uuuuuuuuuuuuuuuu","uu.u.uuuu.u.uuuu","uuuuuuuuuuuuuuuu","u.uuuu.u.uuuu.uu",
    "uuuuuuuuuuuuuuuu","uu.u.uuuu.u.uuuu","uuuuuuuuuuuuuuuu","u.uuuu.u.uuuu.uu",
    "uuuuuuuuuuuuuuuu","uu.u.uuuu.u.uuuu","uuuuuuuuuuuuuuuu","u.uuuu.u.uuuu.uu"
)

$tileSheet = New-Sheet -Width 80 -Height 64
Paint-Sprite $tileSheet $grass 0 0 $tilePalette
Paint-Sprite $tileSheet $tree 16 0 $tilePalette
Paint-Sprite $tileSheet $water 32 0 $tilePalette
Paint-Sprite $tileSheet $rock 48 0 $tilePalette
Paint-Sprite $tileSheet $sand 64 0 $tilePalette
Paint-Sprite $tileSheet $path 0 16 $tilePalette
Paint-Sprite $tileSheet $cave 16 16 $tilePalette
Paint-Sprite $tileSheet $dungeon 32 16 $tilePalette
Paint-Sprite $tileSheet $cracked 48 16 $tilePalette
Paint-Sprite $tileSheet $bush 64 16 $tilePalette
Paint-Sprite $tileSheet $bridge 0 32 $tilePalette
Paint-Sprite $tileSheet $wall 16 32 $tilePalette
Paint-Sprite $tileSheet $floor 32 32 $tilePalette
Paint-Sprite $tileSheet $doorLocked 48 32 $tilePalette
Paint-Sprite $tileSheet $door 64 32 $tilePalette
Paint-Sprite $tileSheet $stairs 0 48 $tilePalette
Paint-Sprite $tileSheet $chest 16 48 $tilePalette
Paint-Sprite $tileSheet $goal 32 48 $tilePalette
Paint-Sprite $tileSheet $bossDoor 48 48 $tilePalette
Paint-Sprite $tileSheet $floorAlt 64 48 $tilePalette
$tilePath = Join-Path $outDir "tiles.png"
$tileSheet.Save($tilePath, [System.Drawing.Imaging.ImageFormat]::Png)

$itemPalette = @{
    "." = [System.Drawing.Color]::FromArgb(0, 0, 0, 0)
    "r" = [System.Drawing.Color]::FromArgb(255, 214, 64, 78)
    "k" = [System.Drawing.Color]::FromArgb(255, 230, 194, 74)
    "b" = [System.Drawing.Color]::FromArgb(255, 66, 74, 88)
    "w" = [System.Drawing.Color]::FromArgb(255, 242, 242, 248)
    "o" = [System.Drawing.Color]::FromArgb(255, 255, 168, 58)
    "u" = [System.Drawing.Color]::FromArgb(255, 96, 196, 255)
}

$heart = @(
    "................","....rr....rr....","...rrrr..rrrr...","..rrrrrrrrrrrr..",
    "..rrrrrrrrrrrr..","...rrrrrrrrrr...","....rrrrrrrr....",".....rrrrrr.....",
    "......rrrr......",".......rr.......","................","................",
    "................","................","................","................"
)
$heartContainer = @(
    "................","....rr....rr....","...r..r..r..r...","..r....rr....r..",
    "..r..........r..","...r........r...","....r......r....",".....r....r.....",
    "......r..r......",".......rr.......","................","................",
    "................","................","................","................"
)
$key = @(
    "................","......kk........",".....kkkk.......","......kk........",
    "......kk........","......kk........","...kkkkkkkkk....","...kkkkkkkkkk....",
    "...kkkkkkkkk....","................","................","................",
    "................","................","................","................"
)
$bossKey = @(
    "................","......oo........",".....okko.......","......oo........",
    "......oo........","......oo........","...ooookoooo....","...okkookoko....",
    "...ooookoooo....","................","................","................",
    "................","................","................","................"
)
$bombAmmo = @(
    "................","......bb........","....bbbbbb......","...bbbbbbbb.....",
    "...bbbbbbbb.....","...bbbbbbbb.....","....bbbbbb......","......ww........",
    ".....w..w.......","................","................","................",
    "................","................","................","................"
)
$bombs = $bombAmmo
$bomb = @(
    "................","......bb........","....bbbbbb......","...bbbbbbbb.....",
    "...bbbbbbbb.....","...bbbbbbbb.....","....bbbbbb......","......ww........",
    ".....woow.......","................","................","................",
    "................","................","................","................"
)
$enemyShot = @(
    "................","................","......oo........",".....oooo.......",
    "....oooooo......","....ookkoo......","....oooooo......",".....oooo.......",
    "......oo........","................","................","................",
    "................","................","................","................"
)
$playerShot = @(
    "................","................","......uu........",".....uuuu.......",
    "....uuuuuu......","....uuwwuu......","....uuuuuu......",".....uuuu.......",
    "......uu........","................","................","................",
    "................","................","................","................"
)
$heartFull = $heart
$heartHalf = @(
    "................","....rr....rr....","...rrrr..r..r...","..rrrrrrr....r..",
    "..rrrrrrr.....r.","...rrrr......r..","....rr.......r..",".....r......r...",
    "......r.....r...","...........r....","................","................",
    "................","................","................","................"
)
$heartEmpty = $heartContainer
$hudBomb = $bomb
$hudKey = $key
$hudBossKey = $bossKey

$itemSheet = New-Sheet -Width 64 -Height 64
Paint-Sprite $itemSheet $heart 0 0 $itemPalette
Paint-Sprite $itemSheet $heartContainer 16 0 $itemPalette
Paint-Sprite $itemSheet $key 32 0 $itemPalette
Paint-Sprite $itemSheet $bossKey 48 0 $itemPalette
Paint-Sprite $itemSheet $bombAmmo 0 16 $itemPalette
Paint-Sprite $itemSheet $bombs 16 16 $itemPalette
Paint-Sprite $itemSheet $bomb 32 16 $itemPalette
Paint-Sprite $itemSheet $enemyShot 48 16 $itemPalette
Paint-Sprite $itemSheet $playerShot 0 32 $itemPalette
Paint-Sprite $itemSheet $heartFull 16 32 $itemPalette
Paint-Sprite $itemSheet $heartHalf 32 32 $itemPalette
Paint-Sprite $itemSheet $heartEmpty 48 32 $itemPalette
Paint-Sprite $itemSheet $hudBomb 0 48 $itemPalette
Paint-Sprite $itemSheet $hudKey 16 48 $itemPalette
Paint-Sprite $itemSheet $hudBossKey 32 48 $itemPalette
$itemPath = Join-Path $outDir "items.png"
$itemSheet.Save($itemPath, [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Generated sprite sheets:"
Write-Host " - $heroPath"
Write-Host " - $enemyPath"
Write-Host " - $tilePath"
Write-Host " - $itemPath"
