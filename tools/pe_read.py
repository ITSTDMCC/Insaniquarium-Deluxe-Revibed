"""Read initialized data from Decomp/binaries/WinFish.exe by virtual address.

The decompiled source refers to constants as DAT_xxxxxxxx; this prints their bytes
so the port can carry the exact values.

  python tools/pe_read.py 005e80ac 8            hex dump
  python tools/pe_read.py 005e80ac 8 u32        as little-endian u32s (also i32 f32 f64 str)
"""
import struct, sys, os

EXE = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'Decomp', 'binaries', 'WinFish.exe')


def sections(data):
    pe = struct.unpack_from('<I', data, 0x3C)[0]
    nsec = struct.unpack_from('<H', data, pe + 6)[0]
    opt = struct.unpack_from('<H', data, pe + 20)[0]
    base = struct.unpack_from('<I', data, pe + 24 + 28)[0]
    out = []
    for i in range(nsec):
        o = pe + 24 + opt + i * 40
        name = data[o:o + 8].rstrip(b'\0').decode()
        vsize, va, rawsize, rawptr = struct.unpack_from('<IIII', data, o + 8)
        out.append((name, base + va, vsize, rawptr, rawsize))
    return out


def read(data, va, n):
    for name, sva, vsize, rawptr, rawsize in sections(data):
        if sva <= va < sva + max(vsize, rawsize):
            off = va - sva
            chunk = data[rawptr + off: rawptr + min(off + n, rawsize)]
            return chunk + b'\0' * (n - len(chunk))  # .bss tail reads as zero
    raise SystemExit(f'{va:08x} is not in any section')


def main():
    data = open(EXE, 'rb').read()
    va, n = int(sys.argv[1], 16), int(sys.argv[2])
    kind = sys.argv[3] if len(sys.argv) > 3 else 'hex'
    b = read(data, va, n)
    if kind == 'hex':
        for i in range(0, n, 16):
            print(f'{va + i:08x}  ' + ' '.join(f'{x:02x}' for x in b[i:i + 16]))
    elif kind == 'str':
        print(repr(b.split(b'\0')[0].decode('latin-1')))
    else:
        fmt, size = {'u32': ('<I', 4), 'i32': ('<i', 4), 'f32': ('<f', 4), 'f64': ('<d', 8), 'u16': ('<H', 2)}[kind]
        print([struct.unpack_from(fmt, b, i)[0] for i in range(0, n - size + 1, size)])


if __name__ == '__main__':
    main()
