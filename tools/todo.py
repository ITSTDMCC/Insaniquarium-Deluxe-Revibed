"""List the not-yet-ported callees reachable from functions, breadth first.

  python tools/todo.py 00552100 [depth]

Prints address, size, region, status, name for each unported function reached; ported
functions are not descended into, nor are CRT helpers.
"""
import csv, os, sqlite3, sys

HERE = os.path.dirname(os.path.abspath(__file__))
DB = os.path.join(HERE, '..', '..', 'gamedb_index', 'winfish.sqlite')
man = {int(r['address'], 16): r for r in csv.DictReader(open(os.path.join(HERE, '..', 'port', 'manifest.csv'), encoding='utf-8'))}
con = sqlite3.connect('file:' + os.path.abspath(DB).replace(os.sep, '/') + '?mode=ro', uri=True)
roots = [int(a, 16) for a in sys.argv[1:] if len(a) > 3]
depth = int(next((a for a in sys.argv[1:] if len(a) <= 3), '3'))
seen = set(roots)
frontier = list(roots)
for d in range(depth):
    nxt = []
    for a in frontier:
        for (c,) in con.execute('SELECT DISTINCT callee_address FROM port_calls WHERE caller_address=? AND callee_address IS NOT NULL', (a,)):
            if c in seen or c not in man:
                continue
            seen.add(c)
            r = man[c]
            if r['status'] == 'ported':
                continue
            print(f"{'  ' * d}{c:08x} {r['size_bytes']:>5} {r['region']:<9} {r['status']:<8} {r['qualified_name']}")
            if r['region'] not in ('crt',):
                nxt.append(c)
    frontier = nxt
