"""Regenerate port/manifest.csv: one row per function in port_functions (all 9,143).

status is derived, never typed by hand:
  ported    a `/// port: <address> <qualified name>` tag in src/ sits right above the Rust fn
  replaced  the function belongs to code the Rust/Bevy build gets from elsewhere
            (see REGIONS below, and port/stl_instances.csv for the template instances
            inside the game's range; the note column says by what)
  pending   game code not translated yet

Run after adding or removing port tags:  python tools/gen_manifest.py
tests/parity.rs fails if the manifest, the tags and the database disagree.
"""
import csv, os, re, sqlite3, sys

HERE = os.path.dirname(os.path.abspath(__file__))
CRATE = os.path.dirname(HERE)
DB = sys.argv[1] if len(sys.argv) > 1 else os.path.join(CRATE, '..', 'gamedb_index', 'winfish.sqlite')
OUT = os.path.join(CRATE, 'port', 'manifest.csv')

# Address regions of WinFish.exe, from link order (see port/README.md for the evidence).
REGIONS = [
    (0x00401000, 0x004A3540, 'framework', 'replaced', 'PopCap SexyApp framework (windowing, DirectDraw/D3D, widgets, sound, resources): Bevy'),
    (0x004A3540, 0x004D3590, 'codecs', 'replaced', 'statically linked zlib/libpng/libjpeg/ogg decoders: Bevy image and audio loaders'),
    (0x004D3590, 0x004D4540, 'framework', 'replaced', 'DDInterface (DirectDraw device setup): Bevy renderer'),
    (0x004D4540, 0x0055324C, 'game', 'pending', ''),
    (0x0055324C, 0x005844BB, 'crt', 'replaced', 'MSVC 8 C runtime and Dinkumware STL: Rust std'),
    (0x005844BB, 0x7FFFFFFF, 'unwind', 'replaced', 'compiler-generated SEH unwind funclets: Rust drop order'),
]


def mangle(name):
    """Rust identifier a port of the function `name` must use (overloads may append __<address>)."""
    s = name.replace('~', 'dtor_')
    s = re.sub(r'[^0-9A-Za-z_]', '_', s)
    s = re.sub(r'_+', '_', s)
    if s.startswith('_') and not name.startswith('_'):
        s = s.lstrip('_')
    if not s or s[0].isdigit():
        s = '_' + s
    return s


# Game-range functions that are template instances (std::map/set/list/vector/string code
# instantiated for the game's types) or compiler-generated copy/destroy helpers: the Rust
# port gets these from std (BTreeMap, Vec, String, Clone, Drop). Reviewed one by one; see
# port/stl_instances.csv for the list and what each is.
STL_LIST = os.path.join(CRATE, 'port', 'stl_instances.csv')


def stl_instances():
    if not os.path.exists(STL_LIST):
        return {}
    with open(STL_LIST, encoding='utf-8') as f:
        return {int(r['address'], 16): r['what'] for r in csv.DictReader(f)}


TAG = re.compile(r'^\s*///\s*port:\s*([0-9a-fA-F]{8})\s+(\S+)\s*$')
FN = re.compile(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)')


def scan_tags(src_root):
    tags = {}
    for dp, _, files in os.walk(src_root):
        for fn in files:
            if not fn.endswith('.rs') or fn == 'generated.rs':
                continue
            path = os.path.join(dp, fn)
            lines = open(path, encoding='utf-8').read().split('\n')
            for i, line in enumerate(lines):
                m = TAG.match(line)
                if not m:
                    continue
                addr = int(m.group(1), 16)
                ident = None
                for j in range(i + 1, min(i + 40, len(lines))):
                    if TAG.match(lines[j]):
                        break
                    fm = FN.search(lines[j])
                    if fm and not lines[j].lstrip().startswith('//'):
                        ident = fm.group(1)
                        break
                rel = os.path.relpath(path, CRATE).replace('\\', '/')
                if addr in tags:
                    raise SystemExit(f'duplicate port tag for {addr:08x}: {tags[addr][1]} and {rel}:{i + 1}')
                tags[addr] = (m.group(2), f'{rel}:{i + 1}', ident)
    return tags


def main():
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace('\\', '/') + '?mode=ro', uri=True)
    rows = con.execute('SELECT address, address_hex, qualified_name, name, file_path, size_bytes, text_sha256 '
                       'FROM port_functions ORDER BY address').fetchall()
    tags = scan_tags(os.path.join(CRATE, 'src'))
    stl = stl_instances()
    known = {r[0] for r in rows}
    bad = [f'{a:08x}' for a in tags if a not in known]
    if bad:
        raise SystemExit('port tags for addresses not in the database: ' + ', '.join(bad))
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    counts = {}
    with open(OUT, 'w', encoding='utf-8', newline='') as f:
        w = csv.writer(f, lineterminator='\n')
        w.writerow(['address', 'qualified_name', 'file_path', 'size_bytes', 'text_sha256', 'region', 'status', 'rust_item', 'note'])
        for addr, ahex, qname, name, fpath, size, sha in rows:
            region, status, note = next((r[2], r[3], r[4]) for r in REGIONS if r[0] <= addr < r[1])
            if region == 'game' and (qname.startswith('Catch') or fpath.startswith('src/std/')):
                status, note = 'replaced', ('C++ catch funclet: Rust Result/panic' if qname.startswith('Catch') else 'STL template instance: Rust std collections')
            if region == 'game' and status == 'pending' and addr in stl:
                status, note = 'replaced', 'STL template instance or compiler-generated helper: Rust std (' + stl[addr] + ')'
            rust_item = ''
            if addr in tags:
                tq, loc, ident = tags[addr]
                status, rust_item, note = 'ported', loc, ''
                if tq != qname:
                    raise SystemExit(f'{loc}: tag says {tq} but {ahex} is {qname} in the database')
                want = mangle(name)
                if ident not in (want, f'{want}__{ahex}'):
                    raise SystemExit(f'{loc}: fn after the tag is {ident!r}, expected {want!r} (or {want}__{ahex})')
            counts[status] = counts.get(status, 0) + 1
            w.writerow([ahex, qname, fpath, size, sha, region, status, rust_item, note])
    print(f'wrote {OUT}: {len(rows)} functions, ' + ', '.join(f'{k} {v}' for k, v in sorted(counts.items())))


if __name__ == '__main__':
    main()
