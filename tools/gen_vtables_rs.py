"""Generate src/sexy/vtables_gen.rs from port/vtables.csv and the port tags in src/.

Every widget-derived class gets a `static <Class>_vftable: VTable` whose slots point at the
Rust port of exactly the function the original vftable slot points at. A slot whose target
is not ported yet points at a generated stub that panics with the original address, so a
missing translation is loud, never silent. Re-run after porting functions:

  python tools/gen_vtables_rs.py && python tools/gen_manifest.py

Families: `w` (Widget slots 1..70, always), `go` (GameObject slots 71..81), `fish` (Fish
slots 82..91), `btn`
(ButtonWidget slots 71..73), `bl` (the class's separate ButtonListener vftable, 7 slots).
"""
import csv, collections, os, re, sqlite3, sys

HERE = os.path.dirname(os.path.abspath(__file__))
CRATE = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from vt_slots import (WIDGET, GAME_OBJECT, FISH, BUTTON, BUTTON_LISTENER, DIALOG, MONEY_DIALOG, EDIT_WIDGET, EDIT_LISTENER,  # noqa: E402
                      SLIDER, CHECKBOX, SLIDER_LISTENER, CHECKBOX_LISTENER,
                      SCROLLBAR, LIST_WIDGET, SCROLL_LISTENER, LIST_LISTENER)
from gen_manifest import scan_tags  # noqa: E402

DB = os.path.join(CRATE, '..', 'gamedb_index', 'winfish.sqlite')
OUT = os.path.join(CRATE, 'src', 'sexy', 'vtables_gen.rs')
# (field, struct name, slots, optional)
FAMILIES = [('w', 'WidgetVt', WIDGET, False), ('go', 'GameObjectVt', GAME_OBJECT, True),
            ('fish', 'FishVt', FISH, True),
            ('btn', 'ButtonVt', BUTTON, True), ('dlg', 'DialogVt', DIALOG, True),
            ('money', 'MoneyDialogVt', MONEY_DIALOG, True), ('edit', 'EditWidgetVt', EDIT_WIDGET, True),
            ('slider', 'SliderVt', SLIDER, True), ('checkbox', 'CheckboxVt', CHECKBOX, True),
            ('bl', 'ButtonListenerVt', BUTTON_LISTENER, True), ('el', 'EditListenerVt', EDIT_LISTENER, True),
            ('sl', 'SliderListenerVt', SLIDER_LISTENER, True), ('cl', 'CheckboxListenerVt', CHECKBOX_LISTENER, True),
            ('scroll', 'ScrollbarVt', SCROLLBAR, True), ('list', 'ListWidgetVt', LIST_WIDGET, True),
            ('scl', 'ScrollListenerVt', SCROLL_LISTENER, True), ('ll', 'ListListenerVt', LIST_LISTENER, True)]
# Secondary vftables only numbered in the symbols whose interface is known from the class's
# constructor (the SliderListener / CheckboxListener parts).
NUMBERED_LISTENERS = {('Sexy::OptionsDialog', 'vftable_2'): 'sl', ('Sexy::OptionsDialog', 'vftable_3'): 'cl',
                     ('Sexy::ScreenSaverDialog', 'vftable_2'): 'cl', ('Sexy::UserDialog', 'vftable_2'): 'el',
                     ('Sexy::UserDialog', 'vftable_3'): 'll'}


LISTENER_FAMILIES = ('bl', 'el', 'sl', 'cl', 'scl', 'll')


def fill_missing_slots(table, n):
    """A secondary vftable the database sized short (its last slots dropped): the missing
    slots are read from the binary right after the known ones."""
    if not table or len(table) >= n:
        return
    import struct
    from pe_read import read, EXE
    data = open(EXE, 'rb').read()
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace(chr(92), '/') + '?mode=ro', uri=True)
    names = dict(con.execute('SELECT address, qualified_name FROM port_functions'))
    any_row = next(iter(table.values()))
    base = int(any_row['vftable_address'], 16)
    for slot in range(1, n + 1):
        if slot not in table:
            tgt = struct.unpack('<I', read(data, base + 4 * (slot - 1), 4))[0]
            table[slot] = {'target': f'{tgt:08x}', 'target_name': names.get(tgt, '?'), 'vftable_address': any_row['vftable_address']}


def rust_ident(cls):
    return re.sub(r'[^0-9A-Za-z_]', '_', cls)


def main():
    con = sqlite3.connect('file:' + os.path.abspath(DB).replace('\\', '/') + '?mode=ro', uri=True)
    parts = collections.defaultdict(set)
    for cname, mname in con.execute("SELECT t.name, m.name FROM port_types t JOIN port_type_members m ON m.type_id=t.id "
                                    "WHERE t.kind='struct' AND t.path LIKE '/ClassDataTypes/Sexy/%' "
                                    "AND t.path = '/ClassDataTypes/Sexy/' || t.name || '/' || t.name"):
        parts['Sexy::' + cname].add(mname)
    # Classes outside the Sexy namespace (e.g. GameSelectorOverlay).
    for cname, mname in con.execute("SELECT t.name, m.name FROM port_types t JOIN port_type_members m ON m.type_id=t.id "
                                    "WHERE t.kind='struct' AND t.path = '/ClassDataTypes/' || t.name || '/' || t.name"):
        parts[cname].add(mname)
    tags = scan_tags(os.path.join(CRATE, 'src'))
    ported = {}
    for addr, (q, loc, ident) in tags.items():
        path = loc.split(':')[0]
        mod = path[len('src/'):-len('.rs')].replace('/', '::')
        mod = re.sub(r'::mod$', '', mod)
        ported[addr] = f'crate::{mod}::{ident}'

    widget_tables = collections.defaultdict(list)
    listener_tables = collections.defaultdict(list)
    edit_listener_tables = collections.defaultdict(list)
    slider_listener_tables = collections.defaultdict(list)
    checkbox_listener_tables = collections.defaultdict(list)
    scroll_listener_tables = collections.defaultdict(list)
    list_listener_tables = collections.defaultdict(list)
    numbered = collections.defaultdict(list)
    for r in csv.DictReader(open(os.path.join(CRATE, 'port', 'vtables.csv'), encoding='utf-8')):
        # A class's widget vftable is `vftable`, or `vftable_for_Widget` under multiple inheritance.
        if r['vftable'] in ('vftable', 'vftable_for_Widget'):
            widget_tables[(r['class'], int(r['vftable_address'], 16))].append(r)
        elif r['vftable'] == 'vftable_for_ButtonListener':
            listener_tables[r['class']].append(r)
        elif r['vftable'] == 'vftable_for_EditListener':
            edit_listener_tables[r['class']].append(r)
        elif r['vftable'] == 'vftable_for_SliderListener':
            slider_listener_tables[r['class']].append(r)
        elif r['vftable'] == 'vftable_for_CheckboxListener':
            checkbox_listener_tables[r['class']].append(r)
        elif r['vftable'] == 'vftable_for_ScrollListener':
            scroll_listener_tables[r['class']].append(r)
        elif (r['class'], r['vftable']) in NUMBERED_LISTENERS:
            kind = NUMBERED_LISTENERS[(r['class'], r['vftable'])]
            {'sl': slider_listener_tables, 'cl': checkbox_listener_tables, 'el': edit_listener_tables,
             'll': list_listener_tables}[kind][r['class']].append(r)
        elif re.fullmatch(r'vftable_[0-9]+', r['vftable']):
            numbered[(r['class'], r['vftable'])].append(r)
    # Secondary vftables whose symbol is only numbered (`vftable_1`, `vftable_2`): a 7-slot table
    # is the class's ButtonListener part, a 4-slot table its EditListener part.
    for (cls, _), rows in numbered.items():
        if len(rows) == len(BUTTON_LISTENER) and cls not in listener_tables:
            listener_tables[cls] = rows
        elif (len(rows) == len(EDIT_LISTENER) and cls not in edit_listener_tables
              and any(r['target_name'].startswith('Sexy::EditListener::') for r in rows)):
            edit_listener_tables[cls] = rows

    out = ['// @generated by tools/gen_vtables_rs.py from port/vtables.csv. Do not edit by hand.\n',
           '#![allow(clippy::all, unused_variables)]\n',
           'use crate::sexy::prelude::*;\n\n']
    for field, sname, slots, _ in FAMILIES:
        out.append(f'/// `{field}` slots of a vftable (slot N sits at offset (N-1)*4).\n')
        out.append(f'#[derive(Clone, Copy)]\npub struct {sname} {{\n')
        for n, meaning, params, ret in slots:
            ps = ', '.join(['&mut G', 'Ptr'] + [p.split(':', 1)[1].strip() for p in params.split(', ') if p])
            out.append(f'    /// #{n} `{meaning}`\n    pub vfunction{n}: fn({ps}) -> {ret},\n')
        out.append('}\n\n')

    stubs = {}
    statics = []
    count = 0

    def slot_target(fam, n, params, ret, r):
        if r is None:
            name = f'noslot_{fam}{n}'
            if ('none', fam, n) not in stubs:
                sig_params = ', '.join(['g: &mut G', 'this: Ptr'] + [x for x in params.split(', ') if x])
                stubs[('none', fam, n)] = (f'fn {name}({sig_params}) -> {ret} {{\n'
                                           f'    panic!("{{this}} has no virtual slot {n}")\n}}\n')
            return name
        tgt = int(r['target'], 16)
        if tgt in ported and ported[tgt].startswith('crate::sexy::trivial::'):
            # An empty body shared by folding: call it from a wrapper with this slot's signature.
            name = f'trivial_{tgt:08x}_{fam}{n}'
            if (tgt, fam, n) not in stubs:
                sig_params = ', '.join(['g: &mut G', 'this: Ptr'] + [x for x in params.split(', ') if x])
                stubs[(tgt, fam, n)] = f'fn {name}({sig_params}) -> {ret} {{\n    {ported[tgt]}()\n}}\n'
            return name
        elif tgt in ported:
            return ported[tgt]
        key = (tgt, fam, n)
        name = f'stub_{tgt:08x}_{fam}{n}'
        if key not in stubs:
            sig_params = ', '.join(['g: &mut G', 'this: Ptr'] + [x for x in params.split(', ') if x])
            what = r['target_name'] if r['target_name'] != '?' else 'function missing from the database'
            stubs[key] = (f'fn {name}({sig_params}) -> {ret} {{\n'
                          f'    crate::sexy::pending(0x{tgt:08x}, "{what}")\n}}\n')
        return name

    for (cls, vaddr), rows in sorted(widget_tables.items(), key=lambda x: x[0][1]):
        p = parts.get(cls, set())
        if 'WidgetContainer_data' not in p:
            continue
        container_only = 'Widget_data' not in p  # WidgetManager: only the container slots exist
        present = {'w': True, 'go': 'GameObject_data' in p, 'fish': 'Fish_data' in p, 'btn': 'ButtonWidget_data' in p,
                   'dlg': 'Dialog_data' in p, 'money': 'MoneyDialog_data' in p, 'edit': 'EditWidget_data' in p,
                   'slider': 'Slider_data' in p, 'checkbox': 'Checkbox_data' in p,
                   'bl': cls in listener_tables, 'el': cls in edit_listener_tables,
                   'sl': cls in slider_listener_tables, 'cl': cls in checkbox_listener_tables,
                   'scroll': 'ScrollbarWidget_data' in p, 'list': 'ListWidget_data' in p,
                   'scl': cls in scroll_listener_tables, 'll': cls in list_listener_tables}
        needed = max([s[0] for f, _, sl, _ in FAMILIES if present[f] and f not in LISTENER_FAMILIES for s in sl])
        if not container_only and len(rows) < needed:
            continue
        by_slot = {int(r['slot']) + 1: r for r in rows}
        bl_slot = {int(r['slot']) + 1: r for r in listener_tables.get(cls, [])}
        el_slot = {int(r['slot']) + 1: r for r in edit_listener_tables.get(cls, [])}
        sl_slot = {int(r['slot']) + 1: r for r in slider_listener_tables.get(cls, [])}
        cl_slot = {int(r['slot']) + 1: r for r in checkbox_listener_tables.get(cls, [])}
        scl_slot = {int(r['slot']) + 1: r for r in scroll_listener_tables.get(cls, [])}
        ll_slot = {int(r['slot']) + 1: r for r in list_listener_tables.get(cls, [])}
        for table, fam_slots in ((el_slot, EDIT_LISTENER), (ll_slot, LIST_LISTENER)):
            fill_missing_slots(table, len(fam_slots))
        for table, fam_slots in ((ll_slot, LIST_LISTENER),):
            for k in [k for k in table if k > len(fam_slots)]:
                del table[k]
        body = [f'/// `{cls}::vftable` @ {vaddr:08x}\npub static {rust_ident(cls)}_vftable: VTable = VTable {{\n',
                f'    class: "{cls}",\n    address: 0x{vaddr:08x},\n']
        for field, sname, slots, optional in FAMILIES:
            if not present[field]:
                body.append(f'    {field}: None,\n')
                continue
            src = {'bl': bl_slot, 'el': el_slot, 'sl': sl_slot, 'cl': cl_slot, 'scl': scl_slot, 'll': ll_slot}.get(field, by_slot)
            body.append(f'    {field}: {"Some(" if optional else ""}{sname} {{\n')
            for n, meaning, params, ret in slots:
                body.append(f'        vfunction{n}: {slot_target(field, n, params, ret, src.get(n))},\n')
            body.append('    }' + (')' if optional else '') + ',\n')
        body.append('};\n\n')
        statics.append(''.join(body))
        count += 1

    out.append('/// One C++ vftable of the original, slot for slot.\npub struct VTable {\n'
               '    pub class: &\'static str,\n    /// Address of the vftable in WinFish.exe.\n    pub address: u32,\n')
    for field, sname, slots, optional in FAMILIES:
        out.append(f'    pub {field}: {"Option<" + sname + ">" if optional else sname},\n')
    out.append('}\n\n')
    out.extend(statics)
    out.append('// ---- slots whose target is not ported yet ----\n')
    out.extend(stubs[k] for k in sorted(stubs, key=str))
    with open(OUT, 'w', encoding='utf-8', newline='\n') as f:
        f.write(''.join(out))
    print(f'wrote {OUT}: {count} vtables, {len(stubs)} pending slot targets')


if __name__ == '__main__':
    main()
