"""Print decompiled functions from winfish.sqlite (and optionally their disassembly).

  python tools/show.py 004d62f0 Sexy::Food::vfunction23 ...   decompiled text
  python tools/show.py -a 004d62f0                            text + disassembly
  python tools/show.py -d 004d62f0                            disassembly only
  python tools/show.py -c 004d62f0                            callees with resolved names
  python tools/show.py -f src/Sexy/Sexy__Food.c               every function of a file

Long "member function inherited by" comment blocks are collapsed.
"""
import os, re, sqlite3, sys

HERE = os.path.dirname(os.path.abspath(__file__))
DB = os.path.join(HERE, '..', '..', 'gamedb_index', 'winfish.sqlite')
DISASM = os.path.join(HERE, '..', 'reference', 'disasm.txt')
_dis = None


def disasm(addr):
    global _dis
    if _dis is None:
        _dis = {}
        cur = None
        for line in open(DISASM, encoding='utf-8'):
            if line.startswith('==== '):
                cur = int(line.split()[1], 16)
                _dis[cur] = []
            elif cur is not None:
                _dis[cur].append(line.rstrip('\n'))
    return _dis.get(addr, [])


def clean(text):
    text = re.sub(r'/\*(\s*[\w:]+ member function inherited by [\w:]+)+\s*\*/\n*', '/* (inherited) */\n', text)
    return re.sub(r'\n{3,}', '\n\n', text)


def main():
    args = sys.argv[1:]
    mode = 't'
    if args and args[0] in ('-a', '-d', '-c', '-f'):
        mode, args = args[0][1], args[1:]
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace('\\', '/') + '?mode=ro', uri=True)
    rows = []
    if mode == 'f':
        for fp in args:
            rows += con.execute('SELECT address, text, recovered_text FROM port_functions WHERE file_path=? ORDER BY address', (fp,)).fetchall()
        mode = 't'
    else:
        for a in args:
            if re.fullmatch(r'[0-9a-fA-F]{6,8}', a):
                rows += con.execute('SELECT address, text, recovered_text FROM port_functions WHERE address=?', (int(a, 16),)).fetchall()
            else:
                rows += con.execute('SELECT address, text, recovered_text FROM port_functions WHERE qualified_name=? ORDER BY address', (a,)).fetchall()
    for addr, text, rtext in rows:
        if mode in 't' or mode == 'a':
            print(clean(rtext or text))
        if mode in 'ad':
            print(f'---- disassembly {addr:08x}')
            print('\n'.join(disasm(addr)))
            print()
        if mode == 'c':
            for name, ca, res, hits in con.execute(
                    'SELECT c.callee_name, c.callee_address, c.resolution, c.hits FROM port_calls c WHERE c.caller_address=? ORDER BY c.line', (addr,)):
                q = con.execute('SELECT qualified_name, size_bytes FROM port_functions WHERE address=?', (ca,)).fetchone() if ca else None
                print(f'{addr:08x} -> {name:40s} {ca and f"{ca:08x}" or "-":10s} {res:16s} x{hits} {q or ""}')


if __name__ == '__main__':
    main()
