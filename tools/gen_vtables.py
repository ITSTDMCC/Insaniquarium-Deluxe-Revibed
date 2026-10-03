"""Write port/vtables.csv: every C++ vftable of WinFish.exe, slot by slot.

Vftable addresses come from the disassembly symbols (`Sexy::Food::vftable` ...), their slot
counts from the database's `<Class>_vftable` struct sizes, and each slot's target is read
from the binary's .rdata and resolved to the function in port_functions. The Rust vtables
in src/ are written from this table so virtual dispatch matches the original slot for slot.
"""
import csv, os, re, sqlite3, struct, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from pe_read import read, EXE  # noqa: E402

DB = os.path.join(HERE, '..', '..', 'gamedb_index', 'winfish.sqlite')
DIS = os.path.join(HERE, '..', 'reference', 'disasm.txt')
OUT = os.path.join(HERE, '..', 'port', 'vtables.csv')


def main():
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace('\\', '/') + '?mode=ro', uri=True)
    names = dict(con.execute('SELECT address, qualified_name FROM port_functions'))
    found = {}
    for m in re.finditer(r'0x([0-9a-f]+)  ; ([A-Za-z_:<>,]*::(vftable\w*))', open(DIS, encoding='utf-8').read()):
        found.setdefault(m.group(2), set()).add(int(m.group(1), 16))
    # slot counts from the DB vftable structs: /ClassDataTypes/<ns>/<Class>/<Class>_vftable[_for_X]
    sizes = {}
    for path, size in con.execute("SELECT path, size FROM port_types WHERE kind='struct' AND name LIKE '%vftable%'"):
        parts = path.strip('/').split('/')
        cls = '::'.join(parts[1:-1])
        suffix = parts[-1].split('_vftable', 1)[1] if '_vftable' in parts[-1] else ''
        sizes[(cls, 'vftable' + suffix)] = size
    data = open(EXE, 'rb').read()

    def run_length(addr):
        n = 0
        while n < 400 and struct.unpack('<I', read(data, addr + 4 * n, 4))[0] in names:
            n += 1
        return n
    # Ghidra gives a class's primary and secondary vftables the same symbol name; the one
    # with the longest run of function pointers is the primary (`vftable`, DB `_vftable0`).
    vt = {}
    for sym, addrs in found.items():
        ranked = sorted(addrs, key=run_length, reverse=True)
        vt[sym] = ranked[0]
        for k, a in enumerate(ranked[1:], 1):
            vt[f'{sym}_{k}'] = a
    for (cls, vname) in list(sizes):
        if vname == 'vftable0':
            sizes[(cls, 'vftable')] = sizes[(cls, vname)]
        elif vname.startswith('vftable') and vname[7:].isdigit() and vname != 'vftable0':
            sizes[(cls, f'vftable_{vname[7:]}')] = sizes[(cls, vname)]
    import capstone
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)

    def resolve(tgt):
        """Name of the slot target; a `jmp` thunk missing from the database is followed."""
        if tgt in names:
            return tgt, names[tgt]
        ins = next(cs.disasm(read(data, tgt, 16), tgt), None)
        if ins is not None and ins.mnemonic == 'jmp' and ins.op_str.startswith('0x'):
            dest = int(ins.op_str, 16)
            if dest in names:
                return dest, f'thunk {tgt:08x} -> {names[dest]}'
        return tgt, '?'
    rows = []
    for sym, addr in sorted(vt.items(), key=lambda x: x[1]):
        cls, vname = sym.rsplit('::', 1)
        size = sizes.get((cls, vname))
        if size is None:
            # fall back: read until a slot no longer points at a known function
            n = 0
            while n < 200:
                tgt = struct.unpack('<I', read(data, addr + 4 * n, 4))[0]
                if tgt not in names:
                    break
                n += 1
            size = 4 * n
        for i in range(size // 4):
            tgt = struct.unpack('<I', read(data, addr + 4 * i, 4))[0]
            dest, name = resolve(tgt)
            rows.append([cls, vname, f'{addr:08x}', i, f'0x{4 * i:x}', f'vfunction{i + 1}', f'{dest:08x}', name])
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, 'w', newline='', encoding='utf-8') as f:
        w = csv.writer(f, lineterminator='\n')
        w.writerow(['class', 'vftable', 'vftable_address', 'slot', 'offset', 'slot_name', 'target', 'target_name'])
        w.writerows(rows)
    print(f'wrote {OUT}: {len(vt)} vftables, {len(rows)} slots')


if __name__ == '__main__':
    main()
