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

Without a path the port looks for `..\Insaniquarium Deluxe` next to this folder. Add `--hd` to
use the optional HD art (see "HD art" below); without it the game uses its original art.
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
| `WINFISH_HD=1` | Same as `--hd`: uses the HD art (see below). |
| `WINFISH_DEBUG_MENU=1` | Starts with the debug menu open. |
| `WINFISH_DEBUG_KEYS=1100:F1;1110:1` | Presses F1 or the debug menu's keys on those frames. |
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

## HD art (optional)

The game uses its original art unless you start it with `--hd`. HD art is made once, on your
PC, from your own copy of the game: every image and font is enlarged 4x by Real-ESRGAN (an
AI upscaler) and saved in a new `hd` folder inside the game folder. Your original game files
are never changed, and no game art is stored in this repository.

Needs: about 475 MB of free disk space, a graphics card with Vulkan support (for the
upscaler), and 8 GB of RAM to play with HD art (the game then uses about 2.2 GB).

### Step by step (from a fresh Windows install)

1. **Install the game** (for example from Steam) and note its folder, e.g.
   `C:\Program Files (x86)\Steam\steamapps\common\Insaniquarium Deluxe`.
2. **Build the port** as described in "Build and run" above (install Rust from
   https://rustup.rs, then run `cargo build --release` in this folder). Check that the game
   starts with its original art.
3. **Install Python 3** from https://www.python.org/downloads/. In the installer, tick
   **"Add python.exe to PATH"**.
4. **Install Pillow** (the image library the tool uses). Open a new Command Prompt and run:
   ```
   pip install pillow
   ```
5. **Download Real-ESRGAN**: from https://github.com/xinntao/Real-ESRGAN/releases/tag/v0.2.5.0
   download `realesrgan-ncnn-vulkan-20220424-windows.zip` (about 45 MB) and unzip it to a
   folder of your choice, e.g. `C:\Tools\realesrgan`. That folder should contain
   `realesrgan-ncnn-vulkan.exe` and a `models` folder.
6. **Make the HD art.** In a Command Prompt in this folder (`winfish_rs`), run (with your own
   paths):
   ```
   python tools\upscale_art.py --game "C:\Program Files (x86)\Steam\steamapps\common\Insaniquarium Deluxe" --esrgan "C:\Tools\realesrgan"
   ```
   It takes a few minutes and prints one line per image. When it ends with `wrote ...\hd`,
   the game folder has a new `hd` folder (about 430 MB). If the game is under
   `Program Files`, Windows may need the Command Prompt to be run as administrator to write
   there.
7. **Play with HD art**: start the port with `--hd`:
   ```
   target\release\winfish_rs.exe "C:\path\to\Insaniquarium Deluxe" --hd
   ```
   or double-click `Play HD.bat` (edit the `GAME=` line in it first if your game is not in
   the `Insaniquarium Deluxe` folder next to this one). You can also make a desktop shortcut
   to `winfish_rs.exe` and add the game path and `--hd` to its Target.

Without `--hd` the game always uses the original art, whether or not the `hd` folder exists.
To remove the HD art, delete the `hd` folder. `--model realesrgan-x4plus` (the general model)
gives a softer result that stays closer to the original painting than the default cartoon
model.

## Debug menu (experimental)

Press **F1** in the game to show or hide the debug menu. It shows the frame rate, the window
size, whether HD art is on, the scaling filter, the game speed, the tank's money and the
player's shells. While it is open, the number keys do the following (they are not passed to
the game):

| Key | Action |
|---|---|
| 1 | Add $1,000 (in a tank) |
| 2 | Add 1,000 shells |
| 3 | Unlock all adventure tanks (saved to the player's profile) |
| 4 | HD art on / off (needs the `hd` folder) |
| 5 | Next scaling filter (bicubic, nearest, bilinear, xBR) |
| 6 | Game speed: 1x, 2x, 4x, paused |
| 7 | Bring the chosen alien into the tank |
| 8 | Choose the alien (Sylvester, Balrog, Gus, Destructor, Ulysses, Psychosquid, Bilaterus) |

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
