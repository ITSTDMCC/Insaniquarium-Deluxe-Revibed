"""Disassembly of a function with `[reg + off]` operands annotated by field name.

  python tools/ann.py 004f8aa0              class taken from the function's qualified name
  python tools/ann.py 00547610 Board        force the class
  python tools/ann.py 00547610 Board EDI    annotate only operands based on EDI

Annotations assume the base register holds `this`; for other registers they are only a hint.
Calls are shown with the callee's name; DAT_ globals that are resource globals get their id.
"""
import os, re, sqlite3, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from show import disasm, DB  # noqa: E402


def class_fields(con, cls):
    row = con.execute("SELECT id, size FROM port_types WHERE path=?", (f'/ClassDataTypes/Sexy/{cls}/{cls}',)).fetchone()
    if not row:
        return None
    tid, size = row
    parts = con.execute('SELECT name, offset, size, type_path FROM port_type_members WHERE type_id=? ORDER BY offset', (tid,)).fetchall()
    sub = {}
    for name, poff, psize, ppath in parts:
        sub[name] = {m[1]: (m[0], m[3]) for m in con.execute(
            'SELECT m.name, m.offset, m.size, m.type FROM port_type_members m JOIN port_types t ON t.id=m.type_id WHERE t.path=?', (ppath,))}

    def name(off):
        if off >= size:
            return f'ext_0x{off:x}'
        for pname, poff, psize, _ in parts:
            if poff <= off < poff + psize:
                if pname == 'vftablePtr':
                    return 'vftablePtr'
                rel = off - poff
                m = sub.get(pname, {}).get(rel)
                short = pname.replace('_data', '')
                return f'{short}.{m[0]}' if m else f'{short}.field_0x{rel:x}'
        return '?'
    return name


def main():
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace('\\', '/') + '?mode=ro', uri=True)
    addr = int(sys.argv[1], 16)
    q = con.execute('SELECT qualified_name FROM port_functions WHERE address=?', (addr,)).fetchone()[0]
    cls = sys.argv[2] if len(sys.argv) > 2 else (q.split('::')[1] if q.startswith('Sexy::') else None)
    only = sys.argv[3] if len(sys.argv) > 3 else None
    fname = class_fields(con, cls) if cls else None
    res = {}
    rg = os.path.join(HERE, '..', 'src', 'sexy', 'res_gen.rs')
    if os.path.exists(rg):
        for m in re.finditer(r'\(0x([0-9a-f]{8}), "([A-Z0-9_]+)"', open(rg, encoding='utf-8').read()):
            res[int(m.group(1), 16)] = m.group(2)
    print(f'---- {addr:08x} {q} (fields of {cls})')
    for line in disasm(addr):
        notes = []
        if fname:
            for reg, off in re.findall(r'\[(E[A-Z]{2}) \+ 0x([0-9a-f]+)\]', line):
                if only and reg != only:
                    continue
                if reg in ('ESP', 'EBP'):
                    continue
                notes.append(f'{reg}+0x{off}={fname(int(off, 16))}')
        for d in re.findall(r'DAT_([0-9a-f]{8})', line):
            if int(d, 16) in res:
                notes.append(res[int(d, 16)])
        print(line + ('    // ' + ', '.join(notes) if notes else ''))


if __name__ == '__main__':
    main()
