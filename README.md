# Insaniquarium Deluxe Revibed

A port of PopCap's **Insaniquarium Deluxe** (`WinFish.exe`) to Rust and the Bevy engine,
translated function by function from the decompiled game. It plays like the original, with a
few modern extras: it scales to any window or monitor, has real full screen, can use
optional HD art, and has an experimental debug menu.

This repository contains **no game data**. You need your own copy of Insaniquarium Deluxe
(the Steam version works).

## Contents

- [What's new compared to the original](#whats-new-compared-to-the-original)
- [Requirements](#requirements)
- [Build and play](#build-and-play)
- [Controls](#controls)
- [Settings and saves](#settings-and-saves)
- [HD art (optional)](#hd-art-optional)
- [Debug menu (experimental)](#debug-menu-experimental)
- [Known differences from the original](#known-differences-from-the-original)
- [For developers](#for-developers)

## What's new compared to the original

- **Any window size:** the game scales to the window or monitor and keeps its 4:3 shape,
  with black bars at the sides.
- **Real full screen:** the Fullscreen option in Options switches to exclusive full screen
  at your monitor's own resolution. The original switched the monitor to 640×480.
- **Sharper scaling:** the enlarged picture uses bicubic filtering, which keeps every original
  pixel exact.
- **Optional HD art:** every image and font can be upscaled 4× from your own game files
  (see [HD art](#hd-art-optional)). It's off unless you ask for it.
- **Remembers how you left it:** settings, full screen, and the window's size and position
  are kept between launches.
- **Escape opens the pause menu** in a tank.
- **Hold to collect:** middle click toggles it. While it's on, holding the left mouse button
  collects any money under the cursor.
- **Debug menu:** press F1 (see [Debug menu](#debug-menu-experimental)).
- **Centred shop dialogs:** "Name Your Fish" and "Confirm Purchase" in the Fish Emporium
  open centred. The original placed them off centre.
- **Music** plays through libopenmpt (included), the same MO3 modules the original played
  through BASS.

## Requirements

**To play with the original art**
- Windows 10 or 11, 64-bit
- Any graphics card with DirectX 12 or Vulkan support
- Your own copy of Insaniquarium Deluxe
- Rust (stable), to build the game: https://rustup.rs

**To play with HD art (optional)**
- 8 GB of RAM (the game then uses about 2.2 GB)
- A 4-core CPU or better
- About 475 MB of free disk space
- A graphics card with Vulkan support, to make the art once
- Python 3 with Pillow, and Real-ESRGAN (free); see [HD art](#hd-art-optional)

Tested on Windows 10 with an AMD Radeon RX 9060 XT at 2560×1440.

## Build and play

1. Install the game and note its folder, for example
   `C:\Program Files (x86)\Steam\steamapps\common\Insaniquarium Deluxe`.
2. Install Rust from https://rustup.rs.
3. In this folder, build and start the game with the path to your game folder:
   ```
   cargo run --release -- "C:\path\to\Insaniquarium Deluxe"
   ```
   The first build takes a few minutes.

After that, start the game directly:
```
target\release\winfish_rs.exe "C:\path\to\Insaniquarium Deluxe"
```
Without a path, it looks for an `Insaniquarium Deluxe` folder next to this one. Add `--hd` to
use HD art. The libopenmpt DLLs (music) are copied next to `winfish_rs.exe` by the build and
must stay beside it.

## Controls

The game plays with the mouse, as the original does. In addition:

| Input | Action |
|---|---|
| Escape | Opens the pause menu in a tank |
| Middle click | Turns hold to collect on or off (a notice shows ON or OFF) |
| Left button held | With hold to collect on: collects money under the cursor |
| F1 | Shows or hides the debug menu |

## Settings and saves

Profiles, saves and high scores are written to the game folder's `userdata`, like the
original. The settings the original kept in the Windows registry go to
`userdata\registry.ini`: volumes, full screen, custom cursors, and the window's size and
position. Closing the window with its X button saves, as the Quit button does.

To keep your original game folder untouched, point the port at a copy of it.

## HD art (optional)

The game uses its original art unless you start it with `--hd`. HD art is made once, on your
PC, from your own copy of the game: every image and font is enlarged 4× by Real-ESRGAN (an
AI upscaler) and saved in a new `hd` folder inside the game folder. Your original game files
are never changed, and no game art is stored in this repository.

AI upscaling guesses at detail the original art doesn't have, so the look changes a little.
The default cartoon model gives bold, clean outlines. The general model
(`--model realesrgan-x4plus`) stays closer to the original painting but is softer.

### Step by step

1. **Build the game** as in [Build and play](#build-and-play), and check that it starts.
2. **Install Python 3** from https://www.python.org/downloads/. In the installer, tick
   **"Add python.exe to PATH"**.
3. **Install Pillow** (the image library the tool uses). Open a new Command Prompt and run:
   ```
   pip install pillow
   ```
4. **Download Real-ESRGAN** from
   https://github.com/xinntao/Real-ESRGAN/releases/tag/v0.2.5.0: get
   `realesrgan-ncnn-vulkan-20220424-windows.zip` (about 45 MB) and unzip it to a folder such
   as `C:\Tools\realesrgan`. That folder should contain `realesrgan-ncnn-vulkan.exe` and a
   `models` folder.
5. **Make the HD art.** In a Command Prompt in this folder, run (with your own paths):
   ```
   python tools\upscale_art.py --game "C:\path\to\Insaniquarium Deluxe" --esrgan "C:\Tools\realesrgan"
   ```
   It takes a few minutes and prints one line per image. When it ends with `wrote ...\hd`,
   the game folder has a new `hd` folder of about 430 MB. If the game is under
   `Program Files`, run the Command Prompt as administrator so it can write there.
6. **Play with HD art:**
   ```
   target\release\winfish_rs.exe "C:\path\to\Insaniquarium Deluxe" --hd
   ```
   Or double-click `Play HD.bat`; first edit its `GAME=` line if your game isn't in the
   `Insaniquarium Deluxe` folder next to this one. You can also make a desktop shortcut to
   `winfish_rs.exe` with the game path and `--hd` in its Target.

HD art loads in the background as each screen's images load, so startup isn't slower. To go
back to the original art, start without `--hd`, or delete the `hd` folder.

## Debug menu (experimental)

Press **F1** to show or hide it. It shows the frame rate, game updates per second, window
size, HD art status, scaling filter, game speed, the tank's money and your shells. While it's
open, the number keys do the following and aren't passed to the game:

| Key | Action |
|---|---|
| 1 | Add $1,000 (in a tank) |
| 2 | Add 1,000 shells |
| 3 | Unlock all adventure tanks |
| 4 | Turn HD art on or off (needs the `hd` folder) |
| 5 | Next scaling filter (bicubic, nearest, bilinear, xBR) |
| 6 | Game speed: 1×, 2×, 4×, paused |
| 7 | Bring the chosen alien into the tank |
| 8 | Choose the alien (Sylvester, Balrog, Gus, Destructor, Ulysses, Psychosquid, Bilaterus) |
| 9 | Hold to collect on or off (same as middle click) |
| 0 | Reroll the virtual tank store's items (while the store is open) |

Keys 2 and 3 save to the player's profile, so use a test profile if you want to keep your
progress. Rerolling the store also clears today's "SOLD" marks.

## Known differences from the original

- Links (the registration page, the Options web link) are logged, not opened in a browser.
- Update checks always fail (no network access).
- Registration codes aren't checked (the Steam version starts registered).
- Full screen keeps your monitor's resolution and enlarges the picture.
- The shop's naming and purchase dialogs are centred.
- The middle mouse button toggles hold to collect instead of acting as a click.

## For developers

### How the port is organized

Every function of the game is either ported, with a `/// port: <address> <name>` tag, or
listed as replaced by something else. `port/manifest.csv` tracks all 9,143.

- `src/game/`: the game's classes (board, fish, aliens, pets, screens, dialogs, ...)
- `src/sexy/`: the parts of the PopCap framework the game relies on, plus the HD screen
  (`hd.rs`)
- `src/host/`: the Bevy side: window, input, rendering and scaling, sound, music
  (libopenmpt), saves, the debug menu and the test hooks
- `port/`: `NOTES.md` (conventions and what is replaced), `manifest.csv` (coverage),
  `stl_instances.csv`, screenshots
- `tools/`: `upscale_art.py` (HD art), and the scripts that generate the vtables and manifest
  from the reference database
- `vendor/libopenmpt/`: libopenmpt 0.8.9 (BSD-3-Clause), the MO3 music player

### Tests

`cargo test` runs the unit tests and the parity tests. The parity tests check the manifest
and port tags against the reference database (`..\gamedb_index\winfish.sqlite`, not in this
repository).

### Scripted runs

An AI agent or a test can play the game through environment variables:

| Variable | Effect |
|---|---|
| `WINFISH_SCRIPT=@steps.txt` | Input at given frames (or the script inline instead of `@file`) |
| `WINFISH_SNAPSHOT="900:a.png\|1500:b.png"` | Saves the 640×480 screen at those frames (plus `<name>_hd.png` with HD art on); quits after the last one |
| `WINFISH_FIXED_STEPS=1` | One game update per rendered frame, so runs are reproducible |
| `WINFISH_AUTOPLAY=1` | The game plays itself (shoots aliens, collects coins, feeds, buys) |
| `WINFISH_NO_SAVE=1` | Saves stay in memory; nothing is written to the game folder |
| `WINFISH_NO_AUDIO=1` | No sound or music |
| `WINFISH_HD=1` | Same as `--hd` |
| `WINFISH_FILTER=<name>` | Scaling filter: `bicubic` (default), `nearest`, `bilinear`, `xbr` |
| `WINFISH_DEBUG_MENU=1` | Starts with the debug menu open |
| `WINFISH_DEBUG_KEYS=1100:F1;1110:1` | Presses F1, Esc, or the debug menu's keys on those frames |
| `WINFISH_DEBUG_HOLD=<y>` | Holds the left button with the cursor sweeping across the tank at height y |

A script is a list of `frame:action:arg` steps separated by `;`. Frames count rendered
frames from launch.

- `click:x,y`: a left click at screen coordinates (0..639, 0..479)
- `type:text`: typed characters (trailing spaces at the very end of the script are trimmed)
- `vk:13`: a key press by Windows virtual-key code (13 is Enter)
- Test setup, not game input: `shells:n` (the player's shells), `levels:n` (first n levels
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

To play step by step, run with a snapshot, look at the picture, add the next clicks to the
script and run again. With `WINFISH_FIXED_STEPS=1`, the same script gives the same game.
