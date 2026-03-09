from PIL import Image

img = Image.open(r"C:\Users\A-A-Ron\git\DeltaRust\assets\sprites\tiles.png")
print(img.size)
px = img.load()

def region(x,y):
    return [px[i,j] for j in range(y,y+16) for i in range(x,x+16)]

cave = region(16,16)
stairs = region(0,48)
print("cave unique", set(cave))
print("stairs unique", set(stairs))
