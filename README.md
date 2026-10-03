# Insaniquarium Deluxe — Rust/Bevy port

A function-by-function port of `WinFish.exe` (Insaniquarium Deluxe) to Rust on the Bevy
engine, translated from the decompiled game. Every game function is either ported (with a
`/// port: <address> <name>` tag) or listed as replaced; `port/manifest.csv` tracks all 9,143.

This repository contains no game data. You need your own copy of Insaniquarium Deluxe
(the Steam version works).

## Build and run

Requirements: Windows (x64), a Rust toolchain (`rustup`, stable), and the game folder.

```
cargo run --release -- "C:\path\to\Insaniquarium Deluxe"
```

Without an argument the port looks for `..\Insaniquarium Deluxe` next to this folder.
The first build takes a few minutes. The built game is `target\release\winfish_rs.exe`;
the libopenmpt DLLs (music) are copied next to it by `build.rs` and must stay beside it.

Profiles, saves and high scores are written into the game folder's `userdata`, like the
original. Point the port at a copy if you want to keep the original folder untouched.

## Playing without a screen (scripted runs)

An AI agent or a test can play the game through environment variables:

| Variable | Effect |
|---|---|
| `WINFISH_SCRIPT=@steps.txt` | Input at given frames (or the script inline instead of `@file`). |
| `WINFISH_SNAPSHOT="900:a.png\|1500:b.png"` | Saves the 640×480 screen at those frames; quits after the last one. |
| `WINFISH_FIXED_STEPS=1` | One logic update per rendered frame, so runs are reproducible. |
| `WINFISH_AUTOPLAY=1` | The port plays by itself (shoots aliens, collects coins, feeds, buys). |
| `WINFISH_NO_SAVE=1` | Saves stay in memory (nothing written to the game folder). |
| `WINFISH_NO_AUDIO=1` | No sound or music. |
| `WINFISH_FILTER=<name>` | How the picture is enlarged: `bicubic` (default, keeps every original pixel exact), `nearest` (sharp blocks), `bilinear` (soft), `xbr` (redraws edges as smooth curves). |

A script is a list of `frame:action:arg` steps separated by `;` (frames count rendered
frames from launch):

- `click:x,y` — left click at screen coordinates (0..639, 0..479)
- `type:text` — typed characters (trailing spaces at the very end of the script are trimmed)
- `vk:13` — a key press by Windows virtual-key code (13 = Enter)
- Test setup (not game input): `shells:n` (player's shells), `levels:n` (first n levels
  done), `finished:n` (adventure beaten n times), `stories:hex` (unlocked stories),
  `ending:3,7,12` (open the ending as if those pets were lost)

Example: create a player named Tester, open the Adventure, continue past the instructions,
and take a screenshot in the first tank:

```
set WINFISH_NO_SAVE=1
set WINFISH_SCRIPT=200:click:300,430;260:type:Tester;262:vk:13;400:click:465,85;700:click:318,432
set WINFISH_SNAPSHOT=1100:tank.png
target\release\winfish_rs.exe "C:\path\to\Insaniquarium Deluxe"
```

To play step by step: run with a snapshot, look at the picture, add the next clicks to the
script, run again (with `WINFISH_FIXED_STEPS=1` the same script gives the same game).

## Layout

- `src/game/` — the game's classes (board, fish, aliens, pets, screens, dialogs, ...)
- `src/sexy/` — the parts of the PopCap framework the game relies on
- `src/host/` — the Bevy side: window, input, rendering, sound, music (libopenmpt), saves,
  and the test hooks above
- `port/` — `NOTES.md` (conventions and replaced pieces), `manifest.csv` (coverage),
  `stl_instances.csv`, screenshots
- `tools/` — scripts that generate the vtables and manifest from the reference database
- `vendor/libopenmpt/` — libopenmpt 0.8.9 (BSD-3-Clause), the MO3 music player

## Known differences from the original

- Links (registration page, Options web link) are logged, not opened in a browser.
- Update checks always fail (no network access).
- Registration codes are not verified (the Steam version starts registered).

## Tests

`cargo test` runs unit tests and the parity tests, which check the manifest and port tags
against the reference database (`..\gamedb_index\winfish.sqlite`, not in this repository).
