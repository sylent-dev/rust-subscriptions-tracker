---
name: egui-screenshots
description: Take a reliable screenshot of the egui window of the application itself (not of the mock-up), under X11 or Wayland, with a temporary patch that asks egui for an image of its own window, then look at it and delete it. Use it to check a visual change of the application, to compare it with the mock-up, or when the owner asks for a screenshot of the application.
---

# Screenshots of the egui application

Goal: see the real window after a visual change, without capturing another window. The screenshot is Claude Code's own check: kept in the scratchpad, looked at, then deleted (`CLAUDE.md`, Interface design). The owner looks at the application themselves. For the mock-up, use the `html-mockups` skill instead.

The tools of the system are not reliable here: the `__screenshot` feature of eframe (`EFRAME_SCREENSHOT_TO`) gave a blank image, and a screenshot tool of the desktop (Spectacle `-a`) once captured the owner's browser instead. The application asks egui instead (`ViewportCommand::Screenshot`), which renders its own window, whatever the desktop.

## 1. Patch

1. Copy `src/app.rs` to the scratchpad, to restore it after.
2. Paste the method of `assets/screenshot-patch.rs` into `impl eframe::App for App`. With `SHOT_TO` set, it waits a moment, takes the screenshot, writes it as a PPM and exits.
3. To show a state other than the first one (a view opened, sample data, an error), add its code where the patch says, run once.

## 2. Shoot

Run it with the folders of the application in the scratchpad, so that the owner's settings, cache, data and log are untouched, and a fresh folder shows the state of a first launch:

```sh
S=<scratchpad>/shot && mkdir -p $S
SHOT_TO=$S/window.ppm SHOT_AFTER_MS=1500 \
  XDG_CONFIG_HOME=$S/config XDG_CACHE_HOME=$S/cache XDG_DATA_HOME=$S/data XDG_STATE_HOME=$S/state \
  timeout 30 cargo run
magick $S/window.ppm $S/window.png
```

- Raise `SHOT_AFTER_MS` when the window waits for the network (an answer of a service shown in it).
- Offline states: prefix the command with `unshare -rn` (no network, the display still works); it fails inside a sandbox that forbids it, where it runs on the host.
- When Claude Code runs in a sandbox without a display (VS Code installed as a Flatpak on some setups), build there and run on the host through `flatpak-spawn --host`, the output in a folder the host sees (a folder of `outputs/`, deleted after), since the host can't see the scratchpad.

## 3. Look, then clean

1. Read the PNG and compare it with the mock-up.
2. To measure a pixel alignment, crop one row (`magick window.png -crop WxH+X+Y -colorspace gray -compress none row.pgm`) and list, with a short script, the columns that differ from the background for each element.
3. Restore `src/app.rs` from the copy, check that `git diff src/app.rs` shows nothing, and delete the screenshots and the folders of the run.
