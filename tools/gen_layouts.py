"""Generate src/layouts/generated.rs from winfish.sqlite (opened read-only).

Every struct, union, enum, typedef and function prototype in port_types becomes a
Rust item at a module path mirroring its Ghidra category, with the original field
names and byte offsets of the 32-bit x86 build (pointers are u32 there). Each
struct/union carries compile-time size and offset assertions, so the Rust mirror
can never silently drift from the database.

Usage: python tools/gen_layouts.py [path/to/winfish.sqlite]
"""
import os, re, sqlite3, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
CRATE = os.path.dirname(HERE)
DB = sys.argv[1] if len(sys.argv) > 1 else os.path.join(CRATE, '..', 'gamedb_index', 'winfish.sqlite')
OUT = os.path.join(CRATE, 'src', 'layouts', 'generated.rs')

PRIM = {
    'char': 'i8', 'schar': 'i8', 'sbyte': 'i8', 'CHAR': 'i8',
    'uchar': 'u8', 'byte': 'u8', 'undefined1': 'u8', 'undefined': 'u8', 'BYTE': 'u8', 'bool': 'u8', 'UCHAR': 'u8', 'BOOLEAN': 'u8',
    'short': 'i16', 'SHORT': 'i16',
    'ushort': 'u16', 'word': 'u16', 'WORD': 'u16', 'wchar_t': 'u16', 'wchar16': 'u16', 'undefined2': 'u16', 'USHORT': 'u16', 'WCHAR': 'u16',
    'int': 'i32', 'long': 'i32', 'INT': 'i32', 'LONG': 'i32', 'BOOL': 'i32',
    'uint': 'u32', 'ulong': 'u32', 'dword': 'u32', 'DWORD': 'u32', 'undefined4': 'u32', 'UINT': 'u32', 'ULONG': 'u32', 'pointer': 'u32', 'pointer32': 'u32', 'ImageBaseOffset32': 'u32',
    'float': 'f32', 'FLOAT': 'f32',
    'double': 'f64',
    'longlong': 'i64', 'LONGLONG': 'i64', '__int64': 'i64',
    'ulonglong': 'u64', 'qword': 'u64', 'undefined8': 'u64', 'ULONGLONG': 'u64', 'DWORD64': 'u64',
}
PRIM_SIZE = {'i8': 1, 'u8': 1, 'i16': 2, 'u16': 2, 'i32': 4, 'u32': 4, 'f32': 4, 'f64': 8, 'i64': 8, 'u64': 8}
RUST_KW = set('as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try gen union'.split())


def ident(s):
    s = re.sub(r'[^0-9A-Za-z_]', '_', s or '')
    s = re.sub(r'_+', '_', s).strip('_') or 'unnamed'
    if s[0].isdigit():
        s = '_' + s
    if s in RUST_KW:
        s = s + '_'
    return s


def main():
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace('\\', '/') + '?mode=ro', uri=True)
    types = con.execute('SELECT id, kind, name, path, category, size, base_type, prototype FROM port_types ORDER BY path').fetchall()
    members = collections.defaultdict(list)
    for row in con.execute('SELECT type_id, ordinal, name, offset, size, type, type_path, comment, value FROM port_type_members ORDER BY type_id, ordinal'):
        members[row[0]].append(row)
    by_path = {t[3]: t for t in types}

    # Assign each type a module path and a unique Rust name inside it.
    modpath = {}
    used = collections.defaultdict(set)
    # Child module names are taken first, so a type never shadows a module of the same name.
    for t in types:
        parts = [ident(p) for p in (t[4] or '/').strip('/').split('/') if p]
        for i in range(len(parts)):
            used[tuple(parts[:i])].add(parts[i])
    for t in types:
        tid, kind, name, path, cat = t[:5]
        parts = [ident(p) for p in (cat or '/').strip('/').split('/') if p]
        mod = tuple(parts)
        base = ident(name)
        n, k = base, 2
        while n in used[mod]:
            n, k = f'{base}_{k}', k + 1
        used[mod].add(n)
        modpath[path] = (mod, n)

    def rust_ref(path, from_mod):
        mod, n = modpath[path]
        return 'crate::layouts::generated::' + ''.join(m + '::' for m in mod) + n

    resolving = set()

    def prim_of(type_name, type_path, size, from_mod, depth=0):
        """Rust type for a member/typedef target of the given byte size."""
        if size is None or size < 0:
            return None
        tn = (type_name or '').strip()
        if tn.endswith('*') and size == 4:
            return 'u32'
        m = re.match(r'^(.*)\[(\d+)\]$', tn)
        if m:
            count = int(m.group(2))
            if count == 0:
                return '[u8; 0]'
            esize = size // count if count else 0
            epath = type_path[: type_path.rfind('[')] if type_path and '[' in type_path else None
            et = prim_of(m.group(1), epath, esize, from_mod, depth + 1) or f'[u8; {esize}]'
            return f'[{et}; {count}]'
        if tn in PRIM and PRIM_SIZE[PRIM[tn]] == size:
            return PRIM[tn]
        if type_path and type_path in by_path and depth < 8:
            t = by_path[type_path]
            if t[1] in ('struct', 'union') and t[5] == size and type_path not in resolving:
                return rust_ref(type_path, from_mod)
            if t[1] == 'typedef' and t[5] == size:
                return prim_of(t[6].rsplit('/', 1)[-1] if t[6] else None, t[6], size, from_mod, depth + 1)
            if t[1] == 'enum':
                return {1: 'u8', 2: 'u16', 4: 'u32', 8: 'u64'}.get(size)
        if size in (1, 2, 4, 8) and tn.startswith('undefined'):
            return {1: 'u8', 2: 'u16', 4: 'u32', 8: 'u64'}[size]
        return f'[u8; {size}]' if size >= 0 else None

    tree = collections.defaultdict(list)  # mod tuple -> list of code chunks
    nstruct = nunion = nenum = ntypedef = nfunc = 0
    for t in types:
        tid, kind, name, path, cat, size, base_type, proto = t
        mod, rname = modpath[path]
        doc = f'/// `{path}` ({kind}, {size} bytes)\n'
        if kind in ('struct', 'union'):
            resolving.add(path)
            size = max(size or 0, 0)
            fields, asserts, seen = [], [], set()
            cursor = 0
            mlist = [m for m in members[tid] if m[3] is not None and m[4] is not None and m[4] >= 0]
            for (_, ordn, mname, off, msize, mtype, mtpath, comment, _v) in mlist:
                fname = ident(mname) if mname else f'field_0x{off:x}'
                if fname in seen:
                    fname = f'{fname}_{ordn}'
                rt = prim_of(mtype, mtpath, msize, mod)
                if kind == 'struct':
                    if off < cursor or off + msize > size:
                        fields.append(f'    // overlapping/out-of-range member skipped: {mname} @0x{off:x} ({mtype})\n')
                        continue
                    if off > cursor:
                        fields.append(f'    pub _pad_0x{cursor:x}: [u8; {off - cursor}],\n')
                    cursor = off + msize
                else:
                    if msize > size:
                        continue
                seen.add(fname)
                cdoc = f'    /// {mtype}' + (f' -- {comment}' if comment else '') + '\n'
                fields.append(cdoc.replace('*/', '* /') + f'    pub {fname}: {rt},\n')
                asserts.append(f'    assert!(core::mem::offset_of!({rname}, {fname}) == {off});\n')
            if kind == 'struct' and cursor < size:
                fields.append(f'    pub _pad_0x{cursor:x}: [u8; {size - cursor}],\n')
            if kind == 'union' and not seen:
                fields.append(f'    pub _raw: [u8; {size}],\n')
            if kind == 'union':
                fields.append(f'    pub _size: [u8; {size}],\n')
            body = ''.join(fields)
            keyword = 'struct' if kind == 'struct' else 'union'
            code = (doc + f'#[repr(C, packed)]\n#[derive(Clone, Copy)]\npub {keyword} {rname} {{\n{body}}}\n'
                    + f'const _: () = {{\n    assert!(core::mem::size_of::<{rname}>() == {size});\n' + ''.join(asserts) + '};\n')
            tree[mod].append(code)
            resolving.discard(path)
            nstruct += kind == 'struct'
            nunion += kind == 'union'
        elif kind == 'enum':
            consts = []
            seenc = set()
            for (_, ordn, mname, off, msize, mtype, mtpath, comment, val) in members[tid]:
                cn = ident(mname)
                if cn in seenc:
                    cn = f'{cn}_{ordn}'
                seenc.add(cn)
                consts.append(f'    pub const {cn}: i64 = {val};\n')
            tree[mod].append(doc + f'pub mod {rname} {{\n    pub const SIZE: usize = {size};\n' + ''.join(consts) + '}\n')
            nenum += 1
        elif kind == 'typedef':
            target = prim_of(base_type.rsplit('/', 1)[-1] if base_type else None, base_type, size, mod) if size and size > 0 else None
            tree[mod].append(doc + f'/// typedef of `{base_type}`\npub type {rname} = {target or "()"};\n')
            ntypedef += 1
        elif kind == 'funcdef':
            p = (proto or '').replace('\n', ' ')
            tree[mod].append(f'/// `{path}`: `{p}`\n/// A 32-bit code address in the original binary.\npub type {rname} = u32;\n')
            nfunc += 1

    # Emit the module tree.
    out = []
    out.append('// @generated by tools/gen_layouts.py from gamedb_index/winfish.sqlite (port_types,\n'
               '// port_type_members). Do not edit by hand; re-run the generator instead.\n'
               '// Layouts are the 32-bit x86 ones of WinFish.exe: pointers are u32 addresses.\n\n')
    out.append(f'pub const STRUCT_COUNT: usize = {nstruct};\npub const UNION_COUNT: usize = {nunion};\n'
               f'pub const ENUM_COUNT: usize = {nenum};\npub const TYPEDEF_COUNT: usize = {ntypedef};\n'
               f'pub const FUNCDEF_COUNT: usize = {nfunc};\n\n')

    def emit(prefix, depth):
        ind = ''
        children = sorted({m[len(prefix)] for m in tree if len(m) > len(prefix) and m[:len(prefix)] == prefix})
        for chunk in tree.get(prefix, []):
            out.append(chunk + '\n')
        for ch in children:
            out.append(f'pub mod {ch} {{\n#![allow(unused_imports)]\n')
            emit(prefix + (ch,), depth + 1)
            out.append('}\n')

    # make sure every intermediate module exists
    for m in list(tree):
        for i in range(len(m)):
            tree.setdefault(m[:i], [])
    emit((), 0)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, 'w', encoding='utf-8', newline='\n') as f:
        f.write(''.join(out))
    print(f'wrote {OUT}: {nstruct} structs, {nunion} unions, {nenum} enums, {ntypedef} typedefs, {nfunc} funcdefs')


if __name__ == '__main__':
    main()
