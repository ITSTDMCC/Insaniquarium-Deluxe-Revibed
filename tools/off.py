"""Map an absolute object offset to the field name the port uses.

  python tools/off.py Food 0x194 0x88      ->  Food_data.offset_0x40 (int), GameObject_data.offset_0x0 (int)

Naming rule used throughout src/: a field is named as the decompiler names it inside the
class part (`<Part>_data`) that contains it: `offset_0xNN` when the database types that
member, `field_0xNN` for untyped bytes. Offsets past the database's class size are named
`ext_0xNNN` by absolute object offset (the database undersizes a few classes).
"""
import os, sqlite3, sys

HERE = os.path.dirname(os.path.abspath(__file__))
DB = os.path.join(HERE, '..', '..', 'gamedb_index', 'winfish.sqlite')


def main():
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace('\\', '/') + '?mode=ro', uri=True)
    cls = sys.argv[1]
    row = con.execute("SELECT id, size FROM port_types WHERE path=?", (f'/ClassDataTypes/Sexy/{cls}/{cls}',)).fetchone()
    if not row:
        raise SystemExit(f'no class {cls}')
    tid, size = row
    parts = con.execute('SELECT name, offset, size, type, type_path FROM port_type_members WHERE type_id=? ORDER BY offset', (tid,)).fetchall()
    for a in sys.argv[2:]:
        off = int(a, 16)
        if off >= size:
            print(f'{a}: ext_0x{off:x} (past {cls} size 0x{size:x})')
            continue
        for name, poff, psize, ptype, ppath in parts:
            if poff <= off < poff + psize:
                rel = off - poff
                if name == 'vftablePtr':
                    print(f'{a}: vftablePtr @0x{poff:x}')
                    break
                sub = con.execute('SELECT m.name, m.offset, m.size, m.type FROM port_type_members m JOIN port_types t ON t.id=m.type_id '
                                  'WHERE t.path=? AND m.offset<=? AND m.offset+m.size>? ', (ppath, rel, rel)).fetchone()
                if sub and sub[1] == rel:
                    print(f'{a}: {name}.{sub[0]} ({sub[3]}, {sub[2]} bytes)')
                elif sub:
                    print(f'{a}: {name}.{sub[0]}+{rel - sub[1]} ({sub[3]})')
                else:
                    print(f'{a}: {name}.field_0x{rel:x} (untyped)')
                break


if __name__ == '__main__':
    main()
