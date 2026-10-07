# Insaniquarium Deluxe Revibed — macOS setup

This gets the game running on your Mac with its original art. You will download
the game data from your own copy on Steam, then build the Mac version of the port.
No Windows installation, Wine or CrossOver needed.

## Requirements

- Your own copy of Insaniquarium Deluxe (the full Steam version works).
- A Mac running a version of macOS supported by
  [Homebrew](https://docs.brew.sh/Installation#macos-requirements).
- Apple's Command Line Tools, Homebrew, Rust stable and libopenmpt (the music
  library). The steps below walk through installing them.
- Several GB of free disk space for the tools and the first build.

The release build has been checked on Apple Silicon. Intel Macs have not been
tested yet; if you try one, use the Intel versions of Rust and Homebrew.

## Step by step (from a fresh Mac install)

### 1. Buy the game

1. Buy [Insaniquarium Deluxe on Steam](https://store.steampowered.com/app/3320/Insaniquarium_Deluxe/)
   if you do not already own it. Get the full game, not the free demo.
2. Install [Steam for Mac](https://store.steampowered.com/about/), open it, and sign
   in to the account that owns the game.

Steam lists the original game as Windows-only. That's expected! You only need
its graphics, sounds and other data. The port itself runs natively on your Mac.

### 2. Download the game data

These steps use the regular Steam app, with no separate SteamCMD installation.
Pause other Steam downloads while the Windows platform override is active.

1. Open **Terminal** (press Command–Space, type `Terminal`, then press Return).
   Paste this command and press Return:

   ```sh
   open "steam://open/console"
   ```

2. Steam should display a **Console** tab. In the input field at the bottom of
   that tab, enter the following command and press Return. This command belongs
   in **Steam's console**, not Terminal:

   ```text
   @sSteamCmdForcePlatformType windows
   ```

3. Go back to Steam's **Library**, select **Insaniquarium Deluxe**, and click
   **Install**. If it is hidden, turn off the Library's Mac-only filter. If the
   Install button is still unavailable, run this in **Terminal** to open the
   game's install dialog:

   ```sh
   open "steam://install/3320"
   ```

4. Choose the default Steam library for the paths used below, and wait for the
   download to finish. The Steam **Play** button targets the Windows executable;
   you will launch the Mac port in step 6 instead.
5. After the download finishes, return to **Steam's console** and restore the
   Mac platform before resuming other downloads:

   ```text
   @sSteamCmdForcePlatformType macos
   ```

You can now close Steam. This downloads the Windows files from the game you own;
it does not make the original Windows executable run on macOS.

### 3. Make a separate copy of the game

The rest of this guide uses `~/Games/insaniquarium` (`~` means your home folder).
Use a fresh folder for the initial setup. If you already have a port installation
there, keep its `Insaniquarium Deluxe/userdata` folder safe and skip copying over
it: that is where the port stores your progress and settings.

In **Terminal**, run:

```sh
mkdir -p "$HOME/Games/insaniquarium"
cp -R "$HOME/Library/Application Support/Steam/steamapps/common/Insaniquarium Deluxe" \
  "$HOME/Games/insaniquarium/"
ls "$HOME/Games/insaniquarium/Insaniquarium Deluxe"
```

You should see folders including `images`, `sounds`, `music`, `data`,
`properties`, and `fishsongs`. Keep the complete game folder, including the
Windows files; there is no need to run or remove any `.exe` files.

If `cp` says **No such file or directory**, Steam may have installed the game in a
different library. In Steam, right-click the game and choose **Manage → Browse
local files**. In Finder, copy that entire **Insaniquarium Deluxe** folder into
`~/Games/insaniquarium`. Do not accidentally nest one `Insaniquarium Deluxe`
folder inside another.

### 4. Install the build tools

Run the following in **Terminal**, one block at a time. Wait for each installer
to finish before continuing. If a command fails, resolve the error before moving
on. These tools are only needed once; skip installers you have already completed.

**Apple's Command Line Tools**

```sh
xcode-select --install
```

Accept the installation in the dialog. If Terminal says the tools are already
installed, continue. You do not need the full Xcode application for this guide.

**Homebrew and the music library**

Install [Homebrew](https://brew.sh/) using its official command:

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Follow the installer's **Next steps**, including the commands that add Homebrew
to your shell. If it asks for your Mac login password, Terminal does not show
characters while you type it. Then run:

```sh
brew --version
brew install libopenmpt
```

[`libopenmpt`](https://formulae.brew.sh/formula/libopenmpt) plays the game's music.
The Windows DLLs included with the project do not provide this library on macOS.
Keep the Homebrew library installed after building: the game also needs it when
it runs.

**Rust**

Install the latest stable Rust using the official
[rustup installer](https://rust-lang.org/tools/install/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose the default installation when prompted. Then load Rust into this Terminal
session and check it:

```sh
. "$HOME/.cargo/env"
rustc --version
cargo --version
```

If Rust was already installed through rustup, run `rustup update stable` to update
it. Use the latest stable version so it can build the project's dependencies.
On Apple Silicon, use a normal Terminal
session, rather than one configured to open using Rosetta, so Rust and Homebrew
libraries use the same architecture.

### 5. Get this repository and build the game

In **Terminal**, run:

```sh
cd "$HOME/Games/insaniquarium"
git clone https://github.com/ITSTDMCC/Insaniquarium-Deluxe-Revibed.git
cd Insaniquarium-Deluxe-Revibed
export LIBRARY_PATH="$(brew --prefix libopenmpt)/lib${LIBRARY_PATH:+:$LIBRARY_PATH}"
cargo build --release --locked
```

The `LIBRARY_PATH` line tells Rust where to find the music library you installed
with Homebrew. It handles both the Apple Silicon and Intel Homebrew locations.
Run that line again before building in a new Terminal session.

The first build downloads and compiles dependencies and can take several minutes.
Warnings alone do not mean it failed. Wait for Cargo's **Finished** message. The
result is `target/release/winfish_rs` inside the port folder, with no `.exe` suffix.

If Git says the destination already exists, use the existing checkout by running
the second `cd` command and continuing with the build. Do not clone inside it.

Your folders should now look like this:

```text
~/Games/insaniquarium/
├── Insaniquarium Deluxe/
│   ├── images/
│   ├── music/
│   ├── properties/
│   └── ...
└── Insaniquarium-Deluxe-Revibed/
    ├── Cargo.toml
    ├── src/
    └── target/release/winfish_rs
```

### 6. Play

In **Terminal**, run:

```sh
cd "$HOME/Games/insaniquarium"
./Insaniquarium-Deluxe-Revibed/target/release/winfish_rs "./Insaniquarium Deluxe"
```

The game should open in a window titled **Insaniquarium Deluxe (Rust port)**.
Create or select a player and start Adventure. Check that you can see the tank,
hear music and sound effects, and feed the fish. Quit normally with the window's
close button, then launch again to check your profile was saved.

Saves and settings are in `~/Games/insaniquarium/Insaniquarium Deluxe/userdata`.
Back up that folder to keep your progress. This copy is outside Steam's game
folder; do not assume Steam Cloud backs it up.

## Additional steps for a double-click launcher (optional)

Paste this entire block into **Terminal**:

```sh
cat > "$HOME/Games/insaniquarium/Play Insaniquarium.command" <<'EOF'
#!/bin/zsh
cd "$(dirname "$0")" || exit 1
exec ./Insaniquarium-Deluxe-Revibed/target/release/winfish_rs "./Insaniquarium Deluxe" "$@"
EOF
chmod +x "$HOME/Games/insaniquarium/Play Insaniquarium.command"
open "$HOME/Games/insaniquarium"
```

Double-click **Play Insaniquarium.command** in Finder to play. It opens Terminal
and then the game. Keep the launcher beside the two folders above; to place a
shortcut on your Desktop, create a Finder alias to it. You do not need to rebuild
every time you play.

## HD art (optional)

The steps above use the original art. The repository's `Play HD.bat` and
`upscale_art.bat` files are Windows scripts; they do not run on macOS. Generating
HD art on a Mac has not been tested for this guide.

If you already generated HD assets from your own copy using the
[HD instructions](../README.md#additional-steps-if-using-the-hd-art), copy the
resulting `hd` folder inside your working `Insaniquarium Deluxe` folder. Enable
them by adding `--hd` to the launch command:

```sh
cd "$HOME/Games/insaniquarium"
./Insaniquarium-Deluxe-Revibed/target/release/winfish_rs "./Insaniquarium Deluxe" --hd
```

Without `--hd`, the game uses the original art even when the `hd` folder exists.

## Troubleshooting

| Problem | What to do |
| --- | --- |
| Steam says “Invalid platform” or will not install | Enter the Windows override in **Steam's console**, then retry the install link in step 2. Restore `macos` after the download. |
| Steam says you do not own the game | Sign in to the account that bought the full game (app `3320`), rather than downloading the demo. |
| `brew: command not found` | Complete Homebrew's **Next steps** to add it to your shell, then open a new Terminal window. |
| `cargo: command not found` | For the rustup installation above, run `. "$HOME/.cargo/env"`, or open a new Terminal window. If Rust came from Homebrew, follow that package's PATH instructions instead. |
| Cargo says the Rust version is unsupported | Run `rustup update stable`, then retry the build. |
| `library 'openmpt' not found` or `library not found for -lopenmpt` | Run `brew install libopenmpt` and the `export LIBRARY_PATH=...` line from step 5 in the same Terminal session as the build. |
| Linker reports incompatible `arm64` / `x86_64` architectures | Use Rust and Homebrew for the same architecture. On Apple Silicon, use native ARM tools in a Terminal session without Rosetta. |
| Launch fails with `Library not loaded` referring to `libopenmpt` | Make sure `brew install libopenmpt` succeeds. Rebuild with step 5 if Homebrew's location or libraries changed. |
| Cargo cannot find `Cargo.toml` | Run the build from `~/Games/insaniquarium/Insaniquarium-Deluxe-Revibed`. |
| Game cannot find its data or shows missing graphics | Check the directory layout in step 5 and pass the quoted game-data path from step 6. It must contain `images` and `properties` directly. |
| Launcher says `No such file or directory` | Complete the release build and keep the launcher beside both folders. |
| F1 changes screen brightness | Use Fn–F1 (or the Globe key with F1) to open the debug menu. |
