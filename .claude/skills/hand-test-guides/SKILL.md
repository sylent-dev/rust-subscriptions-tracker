---
name: hand-test-guides
description: Write the step-by-step guide of a test by hand (what only the window shows, or what needs the desktop of a system: the clipboard, the focus, the keys, the screens) in docs/tests/<system>/, as a blank template run again before every release, and list it in docs/tests/README.md. Use it when a step needs the owner or a tester to try the application by hand, when a probe build must be run on a real desktop, or when the owner asks for a test guide.
---

# Guides of the tests by hand

Goal: a test the owner or a tester runs by hand is written as a guide they follow without the conversation, run again before every release, never as instructions in the chat: given in the chat, a probe that had to be launched first was missed.

## 1. Size it first

- **Size the guide by what the tester can run**, not by the list of a review: thirteen tests, one per assumption, were cut to the four that matter for the release, the rest left to `backlog.md` until a player reports. When proposing a step, say how many hand tests it adds.
- **One guide per feature**, named by it in kebab-case (`docs/tests/linux/settings-saved.md`), in `docs/tests/linux/`, `docs/tests/windows/` or `docs/tests/macos/`, listed in `docs/tests/README.md` with its step, in the pull request of the step it tests.
- **A feature that depends on the desktop is tested in each setup the players have**: one screen and two, each display mode (windowed, full screen), each desktop the project supports (X11 and Wayland, KDE and GNOME). A window checked on two screens only failed on one. When the cause of a failure isn't known, a small probe of the real desktop (the active window read, a grab tried) settles it before any fix is coded.

## 2. Write it

Start from `assets/guide-template.md`, in the language of the testers (an exception to the English of the project, see `CLAUDE.md`):

- **Why first**, in two or three sentences: what changed and what the tests check, then « run the tests in a row; a ✅ is enough when it works, detail only what goes wrong ».
- **Before**: the exact commands, from a fresh state (close the application, the branch, the build, where the log is). On Linux, launch it from a terminal of the system (Konsole, GNOME Terminal): the terminal of VS Code installed as a Flatpak has no X11 and no `~/.cargo/bin` in its `PATH`, so the window can't open there. For a Windows or macOS tester, the archive the release workflow builds on the pull request (the artifacts of its run).
- **Each test**: a bold name, the action, then the expected result after a ✅, precise enough to tell a pass from a near miss (the exact text shown, the exact log line).
- **Go to the end of the lists** (every result loaded, every line shown, the last page): the rare data lives there.
- **A free test last**: « your own scenarios ». It found a case no test asked for.
- **A blank results table**, one row per test with a Result and a Remark column. The guide stays a blank template: its results come back in the conversation or the pull request, never in the file.

## 3. Check it against the code

- **A log line or a text quoted in a guide is read where the code emits it**: its system (`cfg`), its order, how often (once per session, once per event), its exact words. Quoted from a list of the messages, three lines came out wrong: one written on Windows only, two in the wrong order, one once per session.
- **A text a code review proposes for a guide is checked against the code** before it is written: a condition copied from a review came out inverted.
- **A step must not hide what it checks**: another application that catches the same key, an earlier test that left a state behind. When a test depends on how another application behaves, read its source or its behavior of today first.
- The `code-reviewer` agent reads the guides of the branch against the code: give them to it with the code.

## 4. Hand it over

- Give the owner the path of the guide and the commands of its section Before; wait for their results.
- **A tester who isn't at hand** (often the Windows or macOS one) is never asked for results step after step: each step still writes its guide, blank, and the tester runs every guide of their system at once before the release.
- Before every release, every guide is run again (`docs/releases.md`); a guide whose feature changed is updated in the pull request of the change.
