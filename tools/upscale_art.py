"""Make HD art for the port from your own copy of the game (nothing is downloaded or shipped).

Each image is assembled the way the game loads it: the colour file, its alpha mask (`_name`,
`name_`, or the resource's `alphaimage` / `alphagrid` from properties/resources.xml, a grid
mask repeated across the cels), white for mask-only images such as the fonts. Transparent
pixels get their colour from nearby pixels (so no dark fringe bleeds in), and Real-ESRGAN
enlarges the result 4x. The output goes to `<game>\\hd\\<folder>\\<name>.png`; the original files
are never touched. The port uses an HD image only when it is exactly 4x the original's size
and its transparency matches (see src/sexy/hd.rs).

Real-ESRGAN: realesrgan-ncnn-vulkan (BSD-3-Clause), from
https://github.com/xinntao/Real-ESRGAN/releases (v0.2.5.0, the Windows zip). Unzip it and
pass its folder with --esrgan.

Usage:
  python tools/upscale_art.py --game "C:\\path\\to\\Insaniquarium Deluxe" --esrgan "C:\\path\\to\\realesrgan"
  [--model realesrgan-x4plus-anime] [--set all|menu|ui] [paths...]   (paths like images\\store)

Needs Pillow (`pip install pillow`).
"""
import argparse, os, re, shutil, subprocess, tempfile
from PIL import Image, ImageFilter

SCALE = 4
SETS = {
    # The main menu (GameSelector).
    'menu': ['images/selectorback', 'images/tailflop', 'images/merylblink', 'images/gsspeechbubble',
             'images/welcomeback', 'images/mainbutton', 'images/leftbutton', 'images/centerbutton',
             'images/rightbutton', 'images/middlebutton', 'images/middlebuttond'],
    # Interface pieces: dialogs, buttons, checkboxes, sliders, bars, and every font.
    'ui': ['images/dialogue-panel', 'images/dialogbutton2', 'images/editbox', 'images/screenback',
           'images/screentitle', 'images/screentitlehole', 'images/battletankbutton',
           'images/battletankbuttond', 'images/optionsbutton', 'images/optionsbuttond', 'images/mbuttond',
           'images/mbuttonu', 'images/mbuttono', 'images/uncheckbutton', 'images/checkbutton',
           'images/slidertrack', 'images/sliderwidget', 'images/menubar', 'images/mbreflection',
           'images/waitbar', 'images/trophybar', 'images/speechbubble', 'data/*'],
    # Everything: every image and font of the game.
    'all': ['images/*', 'data/*'],
}


def resource_masks(game):
    """path (lowercase, `/`) -> (alphaimage, alphagrid) from properties/resources.xml."""
    out = {}
    xml = open(os.path.join(game, 'properties', 'resources.xml'), encoding='latin-1').read()
    folder = 'images'
    for m in re.finditer(r'<(SetDefaults|Image)\b([^>]*)>', xml):
        attrs = dict((k.lower(), v.strip('"')) for k, v in re.findall(r'(\w+)\s*=\s*("[^"]*"|\S+)', m.group(2)))
        if m.group(1) == 'SetDefaults':
            folder = attrs.get('path', folder)
            continue
        path = f"{folder}/{attrs.get('path', '')}".lower()
        alpha = attrs.get('alphaimage')
        grid = attrs.get('alphagrid')
        if path not in out:  # the first resource naming a file decides its mask
            out[path] = (alpha and f'{folder}/{alpha}'.lower(), grid and f'{folder}/{grid}'.lower())
    return out


class Files:
    def __init__(self, game):
        self.game = game
        self.index = {}
        for folder in ('images', 'data'):
            for f in os.listdir(os.path.join(game, folder)):
                self.index[f'{folder}/{f}'.lower()] = os.path.join(game, folder, f)

    def load(self, path):
        """Loads `path` (no extension) as RGBA; GIF transparent pixels become 0, like the game."""
        for ext in ('.png', '.jpg', '.gif'):
            f = self.index.get(path + ext)
            if f:
                with Image.open(f) as raw:
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


def assemble(files, masks, path):
    alpha_path, grid_path = masks.get(path, (None, None))
    folder, name = path.rsplit('/', 1)
    c = files.load(path)
    if alpha_path:
        a = files.load(alpha_path)
    elif grid_path:
        a = files.load(grid_path)
    else:
        a = files.load(f'{folder}/_{name}') or files.load(f'{path}_')
    if c is None:
        if a is None:
            return None
        c = Image.new('RGBA', a.size, (255, 255, 255, 255))
    if a is not None:
        alpha = a.getchannel('B')
        if alpha.size != c.size:  # alphagrid: one cel's mask, repeated across the cels
            if not grid_path:
                return None
            tiled = Image.new('L', c.size)
            for y in range(0, c.height, alpha.height):
                for x in range(0, c.width, alpha.width):
                    tiled.paste(alpha, (x, y))
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


def expand(files, entries):
    out = []
    for e in entries:
        e = e.replace('\\', '/').lower()
        if e.endswith('/*'):
            folder = e[:-2]
            names = set()
            for p in files.index:
                if p.startswith(folder + '/') and os.path.splitext(p)[1] in ('.png', '.jpg', '.gif'):
                    stem = os.path.splitext(p)[0]
                    base = stem.rsplit('/', 1)[1]
                    names.add(f'{folder}/{base.lstrip("_").rstrip("_")}')
            out += sorted(names)
        else:
            out.append(e)
    return list(dict.fromkeys(out))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--game', required=True)
    ap.add_argument('--esrgan', required=True, help='folder holding realesrgan-ncnn-vulkan.exe and models\\')
    ap.add_argument('--model', default='realesrgan-x4plus-anime')
    ap.add_argument('--set', default='all', help='comma-separated: ' + ', '.join(SETS))
    ap.add_argument('paths', nargs='*')
    args = ap.parse_args()
    files = Files(args.game)
    masks = resource_masks(args.game)
    entries = args.paths or [p for s in args.set.split(',') for p in SETS[s.strip()]]
    exe = os.path.join(args.esrgan, 'realesrgan-ncnn-vulkan.exe')
    with tempfile.TemporaryDirectory() as tmp:
        src, dst = os.path.join(tmp, 'src'), os.path.join(tmp, 'dst')
        os.makedirs(src)
        os.makedirs(dst)
        sizes = {}
        for path in expand(files, entries):
            im = assemble(files, masks, path)
            if im is None:
                print(f'skipped {path}: not found')
                continue
            key = path.replace('/', '__')
            sizes[key] = (path, im.size)
            im.save(os.path.join(src, key + '.png'))
        subprocess.run([exe, '-i', src, '-o', dst, '-n', args.model, '-s', str(SCALE), '-f', 'png'], check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        for key, (path, (w, h)) in sizes.items():
            with Image.open(os.path.join(dst, key + '.png')) as im:
                size = im.size
            if size != (w * SCALE, h * SCALE):
                print(f'skipped {path}: upscaled to {size}, expected {(w * SCALE, h * SCALE)}')
                continue
            out = os.path.join(args.game, 'hd', *path.split('/')) + '.png'
            os.makedirs(os.path.dirname(out), exist_ok=True)
            shutil.copyfile(os.path.join(dst, key + '.png'), out)
            print(f'{path}: {w}x{h} -> {w * SCALE}x{h * SCALE}')
    print(f"wrote {os.path.join(args.game, 'hd')}")


if __name__ == '__main__':
    main()
