# Insaniquarium Deluxe Revibed — A Rust/Bevy enhancement port

A Claude Opus 5.5 powered port of Popcap's Insaniquarium Deluxe to Rust on the Bevy
engine. This was made as a personal project for self-education purposes. **As this was built primarily by AI, please run all code at your own risk**. 

This repository contains no game data. You are required to provide your own copy of Insaniquarium Deluxe. Please support the official release!
(the Steam version works). (Seriously, it's $5, or $1 on sale)

https://store.steampowered.com/app/3320/Insaniquarium_Deluxe/

## Software Requirements

- Windows 10/11 x64
- Rust stable (1.85+, MSVC toolchain) via [rustup](https://rustup.rs), plus Visual Studio
  C++ Build Tools
- A GPU with Vulkan, DirectX 12 or OpenGL support
- Your own copy of Insaniquarium Deluxe
- Python 3 (Optional, if you want to use the "HD art"). 

## Hardware Requirements

**Running with Original art**
- Any graphics card with DirectX 12 or Vulkan support
- 4 GB RAM

**Running with HD art** 
- 8 GB RAM (the game uses about 2.2 GB)
- A 4-core CPU or better
- About 450 MB of free disk space (430 MB HD art, plus the 45 MB upscaler)
- A Vulkan-capable graphics card, to run the upscaler once

## Step by step (from a fresh Windows install)

This gets the game running with its original art.

1. **Install the game** (for example from Steam) and note its folder, e.g.
   `C:\Program Files (x86)\Steam\steamapps\common\Insaniquarium Deluxe`.
2. **Install the Visual Studio C++ Build Tools** (Rust needs them on Windows): download them
   from https://visualstudio.microsoft.com/visual-cpp-build-tools/, run the installer, tick
   **"Desktop development with C++"** and install.
3. **Install Rust**: download `rustup-init.exe` from https://rustup.rs, run it and accept the
   default options.
4. **Get this repository**: on its GitHub page, click **Code > Download ZIP** and unzip it
   (or `git clone` it). The folder you get is the one these steps call "this folder".
5. **Build the game**: open a new Command Prompt in this folder and run:
   ```
   cargo build --release
   ```
   The first build takes a few minutes. The game is built as `target\release\winfish_rs.exe`.
   The libopenmpt DLLs (music) are copied next to it and must stay beside it.
6. **Play**: start the game with the path to your game folder:
   ```
   target\release\winfish_rs.exe "C:\Program Files (x86)\Steam\steamapps\common\Insaniquarium Deluxe"
   ```
   To launch it later with a double-click, make a desktop shortcut to `winfish_rs.exe` and
   add the game path (in quotes) to the end of its **Target**. If this folder sits next to
   the game folder and the game folder is named `Insaniquarium Deluxe`, no path is needed.

That's it: the game runs with its original art. Profiles, saves and high scores are written
into the game folder's `userdata`, like the original. Point the port at a copy of the game
folder if you want to keep the original untouched.

## Additional steps if using the HD Art

Do the steps above first. Then:

1. **Install Python 3** from https://www.python.org/downloads/. In the installer, tick
   **"Add python.exe to PATH"**.
2. **Install Pillow** (the image library the upscale tool uses). Open a new Command Prompt
   and run:
   ```
   pip install pillow
   ```
3. **Download Real-ESRGAN**: from https://github.com/xinntao/Real-ESRGAN/releases/tag/v0.2.5.0
   download `realesrgan-ncnn-vulkan-20220424-windows.zip` (about 45 MB) and unzip it to a
   folder of your choice, e.g. `C:\Tools\realesrgan`. That folder should contain
   `realesrgan-ncnn-vulkan.exe` and a `models` folder.
4. **Make the HD art.** In a Command Prompt in this folder, run (with your own paths):
   ```
   python tools\upscale_art.py --game "C:\Program Files (x86)\Steam\steamapps\common\Insaniquarium Deluxe" --esrgan "C:\Tools\realesrgan"
   ```
   It takes a few minutes and prints one line per image. When it ends with `wrote ...\hd`,
   the game folder has a new `hd` folder (about 430 MB). If the game is under
   `Program Files`, run the Command Prompt as administrator so the tool can write there.
5. **Turn the HD art on**: start the game with `--hd` at the end:
   ```
   target\release\winfish_rs.exe "C:\Program Files (x86)\Steam\steamapps\common\Insaniquarium Deluxe" --hd
   ```
   Or double-click `Play HD.bat` in this folder (first edit its `GAME=` line if your game
   isn't in an `Insaniquarium Deluxe` folder next to this one). For a desktop shortcut, add
   the game path and `--hd` to the end of its **Target**.

Without `--hd` the game always uses the original art, whether or not the `hd` folder exists.
To remove the HD art, delete the `hd` folder. Adding `--model realesrgan-x4plus` to step 4
(the general model) gives a softer result that stays closer to the original painting than the
default cartoon model.

## Building with an AI agent

An AI coding agent (for example Claude Code) can do the setup for you. It will ask before it
installs anything. Replace the path with your own game folder and give it one of these
prompts.

**Without HD art:**

> Clone https://github.com/ITSTDMCC/Insaniquarium-Deluxe-Revibed and read the README. The
> game files are at `C:\path\to\Insaniquarium Deluxe`. Follow "Step by step (from a fresh
> Windows install)" to build the game, but don't launch it.

**With HD art:**

> Clone https://github.com/ITSTDMCC/Insaniquarium-Deluxe-Revibed and read the README. The
> game files are at `C:\path\to\Insaniquarium Deluxe`. Follow "Step by step (from a fresh
> Windows install)" and then "Additional steps if using the HD Art" to build the game and
> make the HD art, but don't launch it.

When the agent is done, start the game yourself:
```
target\release\winfish_rs.exe "C:\path\to\Insaniquarium Deluxe"
```
Add `--hd` at the end (or double-click `Play HD.bat`) to play with the HD art.

## HD art (optional, Powered by https://github.com/xinntao/real-esrgan)

The game uses its original art unless you start it with `--hd`. HD art is made once, on your
PC, from your own copy of the game. Every image and font is enlarged 4x by Real-ESRGAN (an
AI upscaler) and saved in a new `hd` folder inside the game folder. Your original game files
are never changed, and no game art is stored in this repository.

Needs: about 475 MB of free disk space, a graphics card with Vulkan support (for the
upscaler), and 8 GB of RAM to play with HD art (the game then uses about 2.2 GB).

## Layout

- `src/game/` — the game's classes (board, fish, aliens, pets, screens, dialogs, ...)
- `src/sexy/` — the parts of the PopCap framework the game relies on
- `src/host/` — the Bevy side: window, input, rendering, sound, music (libopenmpt), saves,
  and the test hooks above
- `port/` — `NOTES.md` (conventions and replaced pieces), `manifest.csv` (coverage),
  `stl_instances.csv`, screenshots
- `tools/` — scripts that generate the vtables and manifest from the reference database
- `vendor/libopenmpt/` — libopenmpt 0.8.9 (BSD-3-Clause), the MO3 music player

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


## Known differences from the original

- Links (registration page, Options web link) are logged, not opened in a browser.
- Update checks always fail (no network access).
- Registration codes are not verified (the Steam version starts registered).

## Tests

`cargo test` runs unit tests and the parity tests, which check the manifest and port tags
against the reference database (`..\gamedb_index\winfish.sqlite`, not in this repository).
