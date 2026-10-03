"""For each CALL in a function: what ECX/EAX/EDX were last set to and the pushes before it.

  python tools/calls.py 00547610

This recovers what the decompiler drops for __thiscall helpers (`this` in ECX) and
register-argument helpers, without reading the whole listing.
"""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from show import disasm  # noqa: E402


def main():
    for a in sys.argv[1:]:
        lines = disasm(int(a, 16))
        regs, pushes = {}, []
        for line in lines:
            parts = line.split('  ', 1)
            if len(parts) < 2:
                continue
            ins = parts[1].split('  ;')[0].strip()
            note = parts[1].split('  ;', 1)[1].strip() if '  ;' in parts[1] else ''
            m = re.match(r'(MOV|LEA) (E[A-Z]{2}),(.*)', ins)
            if m:
                regs[m.group(2)] = f'{m.group(1)} {m.group(3)}' + (f' ({note})' if note else '')
            elif ins.startswith('PUSH '):
                pushes.append(ins[5:] + (f' ({note})' if note else ''))
            elif ins.startswith('CALL'):
                tgt = note or ins[5:]
                print(f'{parts[0]} CALL {tgt}')
                for r in ('ECX', 'EAX', 'EDX'):
                    if r in regs:
                        print(f'      {r} <- {regs[r]}')
                if pushes:
                    print(f'      pushes (last first): {", ".join(reversed(pushes[-8:]))}')
                pushes = []
                regs = {}
            elif re.match(r'(ADD|SUB) ESP', ins) or ins.startswith(('RET', 'JMP')):
                pushes = []
            elif re.match(r'(XOR|SUB|ADD|AND|OR|IMUL|SHL|SAR|SHR|NEG|INC|DEC|MOVZX|MOVSX|CDQ|POP) ', ins + ' '):
                r = re.match(r'\w+ (E[A-Z]{2})', ins)
                if r:
                    regs[r.group(1)] = ins


if __name__ == '__main__':
    main()
