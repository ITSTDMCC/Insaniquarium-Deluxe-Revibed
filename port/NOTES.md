# Porting notes (conventions for whoever continues the port)

Source of truth: `..\gamedb_index\winfish.sqlite` (`port_functions`, `port_types`, ...).
Never edit `..\Decomp\` or the database.

## Workflow per function
1. `python tools/show.py <addr>` (decompiled C), `python tools/calls.py <addr>` (what ECX/EAX/EDX
   and pushes were at each CALL: recovers the `this` and args Ghidra drops), `python tools/ann.py
   <addr> [Class]` (disassembly with `[reg+off]` annotated by field name), `python tools/off.py
   <Class> 0x194` (field name for an object offset), `python tools/pe_read.py <addr> 8 f64`
   (constants), `grep "^Sexy::Food,vftable," port/vtables.csv` (vtable slots).
2. Write the Rust fn with the database's exact name (`mangle()` in tools/gen_manifest.py; `~X` ->
   `dtor_X`; overloads may add `__<address>`), preceded by `/// port: <addr> <qualified name>`.
3. `python tools/gen_vtables_rs.py && python tools/gen_manifest.py && cargo test`.

## Decompiler traps seen so far (always confirm with the disassembly)
- `__thiscall` helpers called as `FUN_x(args)` with the object lost (it was in ECX).
- Register-argument helpers (WidgetManager: `this` in EDI/EAX/EDX, x/y/clicks in EAX/EBX).
- Values left on the x87 stack are lost: e.g. Food::Update stores the constant 410.0 to y after
  `FUN_004d62d0()`, shown as `y = FUN_004d62d0()`.
- Vftable struct member names are ambiguous (several slots named `vfunction14`): bind by offset,
  slot N = offset (N-1)*4.
- Identical-code folding: one body under an unrelated name (`CRect::CRect` = `Color(r,g,b,a)`,
  `MemoryImage::~MemoryImage`@00489a20 = `Is3DAccelerated`, `Larva::vfunction76` = shared
  GameObject `Remove()`, `DataReaderException::DataReaderException`@00500290 = `ReadBytes`).
- STL checked-iterator failure calls (`FUN_00564335`) are dropped: they only fire on bugs.

## Object model
- `G` (Bevy Resource) holds every heap object in an arena; `Ptr` = u32 id, 0 = null, never reused.
- Field names: as the decompiler names them inside the class part (`<Part>_data`) that contains
  the offset (`offset_0xNN` if typed in the DB, else `field_0xNN`), `ext_0xNNN` past the DB size.
- Virtual calls: `vcall!(g, p, w.vfunction23)` / `go.vfunctionN`; vtables generated from the
  binary (`src/sexy/vtables_gen.rs`). App virtuals: the dynamic type is always WinFishApp, so
  they are direct calls to the WinFishApp vtable target (comment says which slot).
- Constructors return parts (base classes) or allocate and return `Ptr` (most-derived `new`).
- C++ exceptions: `DataReaderException` -> `Result<_, ReadError>`.
- Every game-range function is now ported or listed as replaced (manifest: 0 pending). Template
  instances (std::map/set/list/vector/string code for the game's types) and compiler-generated
  copy/destroy helpers are listed in `port/stl_instances.csv`; `gen_manifest.py` marks them
  replaced (Rust std). A function that becomes reachable after all gets a port tag, which wins.

## Replaced (framework/OS), with the boundary
- Drawing: `Graphics` methods are ported; the destination image's blitter calls become
  `ImageCmd`s with the original arguments; the renderer replays them onto a persistent target
  (the original only redraws dirty widgets onto a persistent screen surface).
- Sound: `SoundInstance::Play` etc. become `SoundRequest`s.
- Time: `_time64` reads `G::now_time64`, set by the host each frame.
- Music: `BassMusicInterface`'s rules (play, stop, fade in/out per update, whole-percent
  volumes, global volume `ftol(40 * mMusicVolume)`%) are in `src/sexy/music.rs` (`G::music`);
  `src/host/music.rs` renders the MO3 modules with libopenmpt (vendored in
  `vendor/libopenmpt`, linked and its DLLs copied by `build.rs`) into one Bevy audio stream.
- Browser and network: `OpenURL` only logs (after 8 s the game's own "Open Browser" dialog shows
  the URL, copied to the clipboard); update checks always fail; the registration code check
  (`SexyApp::Validate`, RSA over MD5) is not reproduced and refuses every code (the shipped
  build is registered through partner.xml).
- Window focus: losing it clears `mActive` and calls `WinFishApp::LostFocus` (pauses a game),
  except in scripted test runs.
- Assumed FPU precision: 53-bit (MSVC default, software renderer). D3D without FPU_PRESERVE
  would have switched the x87 to 24-bit; not modelled.

## Layout facts
- Game objects are top-level widgets of the WidgetManager (siblings of the Board), added by
  `board->mWidgetManager->AddWidget(obj)`; the Board also keeps typed lists (vectors) of them.
- WinFishApp: SexyAppBase_data @0x8, SexyApp_data @0x638, WinFishApp_data @0x72c; app+0x730 =
  mBoard, +0x7b0 = the game MTRand*, +0x87c = game mode, +0x8b8 = current profile.
- Board: Board_data @0x8c; +0x8c app, +0x94 paused, +0x1a4 per-sound last tick[64],
  +0x3f0 money.
