"""Make HD art for the port from your own copy of the game (nothing is downloaded or shipped).

Each image is merged with its alpha mask the way the game loads it (`_name`, `name_`, or an
alphagrid mask repeated across the cels), transparent pixels get their colour from nearby
pixels (so no dark fringe bleeds in), and Real-ESRGAN enlarges it 4x. The results go to
`<game>\\hd\\images\\<name>.png`; the original files are never touched. The port uses an HD
image only when it is exactly 4x the original's size (see src/sexy/hd.rs).

Real-ESRGAN: realesrgan-ncnn-vulkan (BSD-3-Clause), from
https://github.com/xinntao/Real-ESRGAN/releases (v0.2.5.0, the Windows zip). Unzip it and
pass its folder with --esrgan.

Usage:
  python tools/upscale_art.py --game "C:\\path\\to\\Insaniquarium Deluxe" --esrgan "C:\\path\\to\\realesrgan"
  [--model realesrgan-x4plus-anime] [names...]     (default names: the main menu's images)

Needs Pillow (`pip install pillow`).
"""
import argparse, os, re, shutil, subprocess, sys, tempfile
from PIL import Image, ImageFilter

# The main menu (GameSelector) images: name -> alphagrid mask (or None: look for _name / name_).
MAIN_MENU = {
    'selectorback': None, 'tailflop': 'tailflop_', 'merylblink': None, 'gsspeechbubble': None,
    'welcomeback': None, 'mainbutton': 'mainbutton_', 'leftbutton': 'leftbutton_',
    'centerbutton': 'centerbutton_', 'rightbutton': 'rightbutton_', 'middlebutton': None,
    'middlebuttond': None,
}
SCALE = 4


def load(files, folder, name):
    """Loads `name` (.png/.jpg/.gif) as RGBA; GIF transparent pixels become 0, like the game."""
    for ext in ('.png', '.jpg', '.gif'):
        f = files.get(name.lower() + ext)
        if f:
            with Image.open(os.path.join(folder, f)) as raw:
                gif = raw.format == 'GIF'
                im = raw.convert('RGBA')
            if gif:
                px = im.load()
                for y in range(im.height):
                    for x in range(im.width):
                        if px[x, y][3] == 0:
                            px[x, y] = (0, 0, 0, 0)
            return im
    return None


def merged(files, folder, name, grid):
    c = load(files, folder, name)
    if c is None:
        return None
    a = load(files, folder, grid) if grid else (load(files, folder, '_' + name) or load(files, folder, name + '_'))
    if a is not None:
        alpha = a.getchannel('B')
        if alpha.size != c.size:  # alphagrid: one cel's mask, repeated across the cels
            tiled = Image.new('L', c.size)
            for x in range(0, c.width, alpha.width):
                tiled.paste(alpha, (x, 0))
            alpha = tiled
        c.putalpha(alpha)
    fill_transparent(c)
    return c


def fill_transparent(c):
    """Gives fully transparent pixels the alpha-weighted average colour around them."""
    w, h = c.size
    px = c.load()
    for radius in (1, 2, 4, 8, 16, 32):
        pm = Image.new('RGBA', c.size)
        pp = pm.load()
        for y in range(h):
            for x in range(w):
                r, g, b, a = px[x, y]
                k = 1 if a > 0 else 0
                pp[x, y] = (r * k, g * k, b * k, 255 * k)
        bl = pm.filter(ImageFilter.BoxBlur(radius)).load()
        done = True
        for y in range(h):
            for x in range(w):
                if px[x, y][3] == 0:
                    br, bg, bb, ba = bl[x, y]
                    if ba > 0:
                        s = 255 / ba
                        px[x, y] = (min(255, int(br * s)), min(255, int(bg * s)), min(255, int(bb * s)), 0)
                    else:
                        done = False
        if done:
            break


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--game', required=True)
    ap.add_argument('--esrgan', required=True, help='folder holding realesrgan-ncnn-vulkan.exe and models\\')
    ap.add_argument('--model', default='realesrgan-x4plus-anime')
    ap.add_argument('names', nargs='*')
    args = ap.parse_args()
    folder = os.path.join(args.game, 'images')
    files = {f.lower(): f for f in os.listdir(folder)}
    names = {n.lower(): None for n in args.names} if args.names else MAIN_MENU
    exe = os.path.join(args.esrgan, 'realesrgan-ncnn-vulkan.exe')
    out = os.path.join(args.game, 'hd', 'images')
    os.makedirs(out, exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        src, dst = os.path.join(tmp, 'src'), os.path.join(tmp, 'dst')
        os.makedirs(src)
        os.makedirs(dst)
        sizes = {}
        for name, grid in names.items():
            im = merged(files, folder, name, grid)
            if im is None:
                print(f'skipped {name}: not found')
                continue
            sizes[name] = im.size
            im.save(os.path.join(src, name + '.png'))
        subprocess.run([exe, '-i', src, '-o', dst, '-n', args.model, '-s', str(SCALE), '-f', 'png'], check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        for name, (w, h) in sizes.items():
            with Image.open(os.path.join(dst, name + '.png')) as im:
                size = im.size
            if size != (w * SCALE, h * SCALE):
                print(f'skipped {name}: upscaled to {size}, expected {(w * SCALE, h * SCALE)}')
                continue
            shutil.copyfile(os.path.join(dst, name + '.png'), os.path.join(out, name + '.png'))
            print(f'{name}: {w}x{h} -> {w * SCALE}x{h * SCALE}')
    print(f'wrote {out}')


if __name__ == '__main__':
    main()
