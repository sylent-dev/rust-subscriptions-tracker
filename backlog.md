# Backlog

What is noted and not being done: an evolving list, read when the next work is chosen (process.md, The backlog), not at every session. When the owner picks an item, it leaves this file and becomes a step of [roadmap.md](roadmap.md).

<!--
One entry per item, in the section of its kind: what it is in one sentence, what it depends on, and the date it was noted. For example:
- The error message of a missing file doesn't say which one. Noted on 2026-11-02.
- A Windows build. Waits on: a Windows machine to test the clipboard. Noted on 2026-11-02.
-->

## Polish

<!-- Small touches, one small pull request each. -->

## Edge cases

<!-- Cases at the margin that need thought before a pull request. -->

## Features

<!-- New commands, options, outputs or screens for a later cycle. -->

- Windows notifications under an id of the application's own (an AppUserModelID registered for the user), so that they no longer show « Windows PowerShell » as their sender. Waits on: a Windows tester. Noted on 2026-11-19 (step 1a).

- A click on a notification opens the window on the subscription. Waits on: the first release. Noted on 2026-11-19 (step 1a).

- The data file: the history of the prices (to see the rises), a period of its own (every 6 months, every 2 years), the share the user pays of a shared subscription. Waits on: the first release, used for a while. Noted on 2026-11-19 (step 1a, set aside from the draft of the data format).

- Free trials as such: a status and the day the trial turns paid, rather than a period that starts on its first paid day. Waits on: users who ask for it, once the application is shared. Noted on 2026-11-19 (step 1a2; the owner doesn't use trials).

- How a subscription is paid (a card, an account), to find those paid by a card that expires; the notes hold it meanwhile. Waits on: the first release. Noted on 2026-11-19 (step 1a2).

- macOS support: its own scheduler (launchd) and desktop notifications, and its archives in the Release workflow (its targets in `release.yml`, `deny.toml` and `about.toml`, as in rust-desktop-template; its Clippy job already runs in the CI). Waits on: a Mac to test the reminders. Noted on 2026-11-14.

## Large works

<!-- Works of several cycles, or that change the structure. -->

- Exercise material for learners: a guide to rebuild the application by hand, module by module, in the order to write them, with what each one teaches (a topic that speaks to everyone: their own money). Waits on: the first release. Noted on 2026-11-14.

- At the retrospective of the first cycle, discuss with the owner whether another template comes out of this project (a desktop application with a headless mode run daily by the system scheduler), the owner making templates after the fact. Waits on: the retrospective of cycle 1. Noted on 2026-11-14.

## Waiting on others

<!-- What depends on someone else than the owner and Claude Code; say on whom and on what. -->

## Explorations kept

<!-- Exploration branches kept unmerged, and why. -->
