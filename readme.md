# deadlockedbutbetter

personal fork of [deadlocked](https://github.com/avitran0/deadlocked) — cs2 aimbot + esp for linux.

i use it in bot games. that's it. no releases, no binaries, no support. build it yourself or don't.

**offline only.** don't take this onto real servers. seriously.

## what this is

a fork i'm using to mess with the aim and targeting code. everything else is upstream's. if something's
broken it's probably my edit, not theirs.

## build it

linux and rust. that's the whole list.

```bash
cargo build --release --bin deadlocked
cargo run --release --bin deadlocked
```

don't run upstream's `setup.sh` or any of the other `.sh` files. they're for their prebuilt releases, i've
changed things since, and i never tested them against this tree. if one fails, that's expected — not a bug.

### uinput

the binary won't start without this. one-time, then reboot.

```bash
sudo groupadd -f uinput
sudo usermod -aG uinput "$USER"
sudo modprobe uinput
```

you also need upstream's udev rule — grab it from any upstream release zip. it's system config, not code,
so it doesn't go stale with my edits. **reboot** afterwards; group membership and udev rules don't take
effect until you do.

nixos, fedora atomic, hyprland? read upstream's
[compatibility.md](https://github.com/avitran0/deadlocked/blob/main/compatibility.md). i can't help with those.

## where things live

- `src/cs2/` — all game-specific code
  - `src/cs2/entity/` — in-game entities: players, grenades, weapons
  - `src/cs2/features/` — aimbot, esp, triggerbot, rcs
- `src/ui/` — gui and overlay
- `src/parser/` — bvh implementation, for fast visibility lookups
- `src/os/` — low-level os interaction: memory reads/writes, mouse input

the aim and targeting changes will live under `src/cs2/features/`. everything else is untouched.

## what it does

inherited from upstream unless it's in the changes list.

- **aimbot:** hotkey, fov, smoothing, start bullet, targeting mode, visibility check, head-only/whole-body, flash check, fov circle
- **esp:** hotkey, box, skeleton, health/armor bars, names, weapon icons, tags, dropped weapons, bomb timer
- **triggerbot:** activation mode, min/max delay, extra duration, visibility/flash/scope checks, velocity threshold, head-only
- **rcs:** standalone, with smoothing
- **per-weapon overrides:** aimbot, triggerbot, rcs
- **web radar:** follow player, shareable ([upstream faq](https://github.com/avitran0/deadlocked/blob/main/radar.md))
- **misc:** sniper crosshair, grenade lineup alignment

### unsafe

these write to game memory. offline only, don't push it.

- no flash (max flash alpha)
- fov changer
- no smoke
- smoke color

## changes from upstream

this is my fork changelog. every time i edit a file, it goes here with one line saying what changed.
months from now i won't remember which code is mine and which is upstream's — this list is how i tell
them apart, and how i know what to revert if an upstream update conflicts.

- `src/...` — crosshair-angle target selection (in progress, maybe)
- `src/...` — exponential smoothing (in progress, maybe)
- `.github/workflows/` — deleted. no ci.

## faq

**where are my configs?**
`$XDG_CONFIG_HOME`, falling back to `$HOME/.config`, falling back to next to the executable.

**which desktops work?**
best: gnome, kde. good: sway, weston. fair: i3, openbox, xfce, hyprland (needs tweaks).

**hyprland is broken**
yeah. bad x11 support for what this does. use something else.

**gamescope overlay is tiny**
game thinks it's 16:9. don't use gamescope.

**overlay is black**
compositor has no transparency. on kde: display and monitor → compositor → enable on startup.

**overlay shows but i can't click anything**
click-through failed. wm/compositor limitation.

**overlay isn't showing or isn't on top**
your wm doesn't do positioning or always-on-top.

## a note from the ai

hi. i wrote a lot of this readme and some of the code, i don't know which parts by now, and i will not be
here when it breaks — so if the aimbot starts spinning in circles after a game update, that's a you problem
now. sorry. good luck, and commit often.

## credits

everything original is [avitran0/deadlocked](https://github.com/avitran0/deadlocked). upstream is gpl-3.0,
so this fork is too. license terms win over anything i've written here.
