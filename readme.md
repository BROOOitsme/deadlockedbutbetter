<div align="center">

# deadlockedbutbetter

[![Fork of avitran0/deadlocked](https://img.shields.io/badge/fork%20of-avitran0%2Fdeadlocked-blue?style=for-the-badge)](https://github.com/avitran0/deadlocked)

<br>

personal fork of [deadlocked](https://github.com/avitran0/deadlocked) — a simple CS2 aimbot and ESP for Linux.

tuned for offline / bot games. built from source, not distributed as a release.

</div>

<br>

## About this fork

This is a personal, local-use fork of `avitran0/deadlocked`. It exists to experiment with the aim and
targeting logic. There are no releases, no binaries, and no support — build it yourself if you want it.

**Upstream** is the source of truth for everything except the files listed under "Changes from upstream"
below. If something here is broken, it's almost certainly my edit, not upstream.

<br>

## Setup (required, one-time)

> [!NOTE]
> Running NixOS, Fedora Atomic, Hyprland (Legacy `.conf` config)?
>
> See [compatibility.md](compatibility.md) upstream.

The cheat drives mouse input through `uinput`, so the system needs a `uinput` group, a udev rule, and the
`uinput` kernel module loaded. The binary refuses to start until this is done.

```bash
# from the repo root
./setup.sh
```

**Restart your machine (required)** — the group membership and udev rule only take effect after a reboot.

Verify the kernel module is present:

```bash
lsmod | grep uinput
# if missing:
sudo modprobe uinput
```

<br>

## Build from source

Requires the Rust toolchain and `git`.

```bash
# rust toolchain (skip if already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# clone and build
git clone https://github.com/BROOOitsme/deadlockedbutbetter
cd deadlockedbutbetter
cargo run --release --bin deadlocked
```

The reduced-model variant is `--bin deadlocked-reduced-models` if you need a smaller binary.

<br>

## Features

Everything below is inherited from upstream unless noted in "Changes from upstream".

### Aimbot
- Hotkey
- FOV
- Smooth
- Start bullet
- Targeting mode
- Visibility check (VPK parsing)
- Head only / whole body
- Flash check
- FOV circle

### ESP
- Hotkey
- Box
- Skeleton
- Health bar
- Armor bar
- Player name
- Weapon icon
- Player tags (helmet, defuser, bomb)
- Dropped weapons
- Bomb timer

### Triggerbot
- Activation mode
- Min/max delay
- Additional duration
- Visibility check
- Flash check
- Scope check
- Velocity threshold
- Head only mode

### Standalone RCS
- Smoothing

### Per-weapon overrides
- Aimbot
- Triggerbot
- RCS

### Web radar
- Web radar
- Follow player
- Shareable

### Misc
- Sniper crosshair
- Bomb timer
- Grenade lineup alignment
- Web radar ([FAQ](radar.md))

### Unsafe
> [!WARNING]
> These features write to game memory and might get you banned.
> In a bot game there's no VAC risk in practice, but treat them as offline-only regardless.

- No flash (with max flash alpha)
- FOV changer
- No smoke
- Smoke color change

<br>

## Changes from upstream

*(keep this list updated as you edit — it's the whole point of a fork)*

- `src/...` — crosshair-angle target selection (planned / in progress)
- `src/...` — exponential smoothing step (planned / in progress)
- `.github/workflows/` — removed (no CI on this fork)

<br>

## FAQ

<details>
<summary>Where are my configs saved?</summary>

`$XDG_CONFIG_HOME`, falling back to `$HOME/.config`. If neither is set, they're saved alongside the executable.

</details>

<br>

<details>
<summary>Which desktop environments and window managers are supported?</summary>

**Best:** GNOME (Mutter), KDE (KWin)
**Good:** SwayWM, Weston
**Fair:** i3, OpenBox, XFCE, Hyprland (tweaks may be needed; see [compatibility.md](compatibility.md/#hyprland))

</details>

<br>

<details>
<summary>I'm using Hyprland and something doesn't work</summary>

Hyprland has poor X11 support for the techniques this cheat uses. Try another WM if possible.

</details>

<br>

<details>
<summary>I'm using Gamescope and the overlay is too small</summary>

The game still thinks it's running at 16:9, so the cheat reads the wrong window resolution. Run without Gamescope.

</details>

<br>

<details>
<summary>My screen/overlay is black</summary>

Your compositor doesn't support transparency, or it's disabled. On KDE: **Display and Monitor → Compositor → Enable compositor on startup**.

</details>

<br>

<details>
<summary>The overlay shows but I can't click anything</summary>

Click-through failed — window manager/compositor limitation.

</details>

<br>

<details>
<summary>The overlay doesn't show up / isn't on top</summary>

Your WM doesn't support positioning/resizing or always-on-top windows.

</details>

<br>

## Credits

All original code and design: [avitran0/deadlocked](https://github.com/avitran0/deadlocked).
This fork only modifies the aim/targeting behavior for personal use. Upstream license applies.
