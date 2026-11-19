# Specifications

<!--
Describe what the application does, before writing any code (process.md, Specifications).
Write behavior, not implementation: technical choices go in docs/technical-choices.md.
Remove or add sections as needed.
-->

## Purpose

Subscriptions Tracker is a small desktop application that keeps the list of the user's subscriptions in a local file, and warns them by a desktop notification before the last day they can cancel each one, so that no subscription renews because the date was missed.

- Users: people who pay for a few dozen subscriptions or fewer (streaming, software, insurance, phone plan), on Linux or Windows, in English or French. Most of them never open a terminal: they download the application from the owner's portfolio.
- Used alone, in one small window, opened now and then to add, change or cancel a subscription. The reminders don't need the window: the system runs a check once a day, which shows no window (see [Daily check](#daily-check)).
- Local only: the data stays in files on the user's disk, with no server, no online account and no network access.
- Volume: up to about a hundred subscriptions; one check a day.
- Version 1: the list of the subscriptions, their deadlines and monthly costs, the daily check and its notifications, an export of the deadlines to a calendar (`.ics`), and an About panel with the notice of the GPL.

## Success criteria

Checked one by one at the retrospective of the first cycle:

- The owner keeps every subscription of theirs in it, and no other list.
- During the first three months of use, the owner gets a notification before each deadline of their subscriptions, also after a day when the computer stayed off, and no subscription renews because its deadline was missed.
- An archive downloaded from the release opens its window and installs the daily check on Linux; on Windows, the same once a tester has tried it (until then, the CI only: see the roadmap).

## Requirements

- **Systems**: Linux with a systemd user session (the owner's desktop: Bazzite, KDE Plasma) and a desktop that shows the notifications of the freedesktop standard; Windows 10 and 11, with the Task Scheduler and its notifications. macOS waits in the backlog.
- **The daily check** runs in under a second for a hundred subscriptions, shows no window and, on Windows, no console window either.
- **Code that can be rewritten by hand**: the owner will rebuild the application later as a Rust exercise, and show it to learners. The code favors what can be followed from top to bottom over the most compact form: few dependencies, short modules with one clear role each, and comments that explain why rather than what.

## Subscriptions

A subscription is one entry of the data file (see [Local files](#local-files)); [docs/subscriptions-example.toml](docs/subscriptions-example.toml) shows one of each kind. Its fields, all optional except the first six:

| Field | What it holds |
|---|---|
| `id` | A short name of its own, unique in the file, that never changes (`netflix`): a rename of the subscription keeps its reminders already sent. |
| `name` | The name shown. |
| `price`, `currency` | The price of one period, as an exact amount with two decimals at most (`"13.49"`, never rounded), and its currency: `EUR`, `USD`, `GBP`, `CHF` or `CAD` (more when users ask). |
| `period` | `weekly`, `monthly`, `quarterly` or `yearly`. |
| `periods` | The periods the user was subscribed, one or more (see below). |
| `category` | `streaming`, `music`, `software`, `cloud`, `news`, `gaming`, `insurance`, `telecom` (phone and internet), `utilities` (energy, water), `fitness`, `education` or `other`, the default. |
| `notice_days` | The notice: the subscription must be cancelled at least this many days before a renewal. 0 by default. |
| `reminders_days` | When to be warned, in days before the deadline (`[30, 14, 3]`), in place of the default of the settings (`[7, 1]`). |
| `cancel_url` | The page where it is cancelled: an address that starts with `https://` or `http://`, without spaces. Another address (`file:`, `javascript:`) is refused, since opening it would start something on the computer. |
| `account` | The account used, to know where to log in: an email address or a user name, as the user writes it. Never a password. |
| `notes` | Free notes (« Shared with the family », « Cancelled by registered letter »). |

The values of `category`, `currency` and `period` are written in English in the file, whatever the language of the interface, which shows them translated (« Assurance » for `insurance` in French). A value outside its list (`insurrance`) makes this subscription unreadable, with the field and the value said (see [Error handling](#error-handling)); the others stay readable.

### Periods

Each period is one stretch the user was subscribed: `start`, the first day paid, and `end`, the last day paid, which the user writes when they cancel; a period without `end` is the current one, renewed until it's cancelled. A period may also have a `commitment_end`, the end of a commitment (a phone plan for 12 months), before which no cancellation takes effect. Someone who subscribes to a streaming service for a month at each new series, then stops, keeps one period for each month.

- The periods are in the order of their `start`, each one starts after the end of the previous one, and only the last may have no `end`; a `start` is never after its `end`. A list that breaks one of these rules makes the subscription unreadable.
- The subscription is **active** while its last period has no `end`; **ending** while its last `end` is today or later (cancelled, but paid until that day); **ended** once it's past. This is computed, never written in the file, so that the dates and a status can't contradict each other.
- A free trial is entered as a subscription whose period starts on its first paid day: its first deadline comes before that day. Trials have no field of their own (owner, 2026-11-19; backlog).
- A subscription whose start isn't known any more takes any past renewal as its `start`.

## Deadlines

The deadline of a subscription is the last day it can be cancelled without paying the next period. Every date is a calendar day of the computer's time zone, without hours: « today » is the date on the computer's clock. Only an active subscription has a deadline: one that is ending or ended renews no more.

- **Renewals**: counted from the `start` of the current period, by whole periods: 7 days for weekly; 1, 3 or 12 months for monthly, quarterly and yearly. The `start` itself is the first payment. A month counted from a day it doesn't have gives its last day, each renewal counted from the `start` itself, never from the previous one: a monthly subscription from 31 January renews on 28 February (29 in a leap year), then on 31 March. A yearly one from 29 February renews on 28 February of the years without it.
- **Next renewal**: the first renewal that is today or later; the `start` itself while it is still to come.
- **Deadline**: the next renewal minus `notice_days`. When that day is already past (today is inside the notice), this renewal can no longer be avoided: the window says so, and the deadline is the one of the following renewal.
- **Commitment**: until the `commitment_end` of the current period, the deadline is the one of the first renewal on or after it.

The monthly cost of a subscription is its price brought to a month: weekly × 52 ÷ 12, monthly as it is, quarterly ÷ 3, yearly ÷ 12, rounded to the cent, half up, only when shown. The totals add the active subscriptions of each currency apart, without converting one currency into another.

```text
Monthly from 2026-01-31, today 2026-02-10, notice 0          → next renewal 2026-02-28, deadline 2026-02-28
Monthly from 2026-01-31, today 2026-03-01, notice 0          → next renewal 2026-03-31
Yearly from 2024-02-29, today 2025-01-10                     → next renewal 2025-02-28
Yearly from 2026-03-16, notice 30, today 2026-11-19          → next renewal 2027-03-16, deadline 2027-02-14
Monthly from 2026-10-03, notice 7, today 2026-11-28          → 2026-12-03 can't be avoided; deadline 2026-12-27
Monthly from 2026-11-28 (a trial until then), notice 1,
  today 2026-11-19                                           → next renewal 2026-11-28, deadline 2026-11-27
Two periods: 2025-03-10 to 2025-04-09, then monthly from
  2026-11-01, today 2026-11-19                               → renewals from 2026-11-01: next 2026-12-01
Commitment until 2027-06-30, monthly from 2026-07-15         → deadline 2027-07-15 minus the notice
Last period ends 2026-12-02, today 2026-11-19                → ending: no deadline, « Ends on 2 December »
Last period ended 2026-04-09                                 → ended: no deadline
Two periods that overlap                                     → unreadable: « Its periods overlap »
category = "insurrance"                                      → unreadable: « insurrance isn't a category »
Yearly at 186.00 EUR                                         → 15.50 EUR a month
Weekly at 4.99 USD                                           → 21.62 USD a month
```

## Daily check

The reminders come from a check the system runs every day, without a window: the user gets them without opening the application, nor leaving it open.

### Installation

- The window installs the check at its first launch, on by default; a setting turns it off, which removes it from the system, and on again (see [Settings](#settings)). The window says once, in a short notice, that the reminders are on, and how to turn them off.
- On Linux, a systemd user timer, which starts the check every day at 9:00 and, when the computer was off or the session closed at that hour, at the next login. On Windows, a task of the Task Scheduler, with the same hours and the same catching up.
- The scheduler starts the executable itself with the argument `--check`: there is a single executable, which opens its window when it's started without it.
- At each launch, the window checks that the installed check starts this very executable, and installs it again otherwise: an archive unpacked in another folder, a new version that replaced the old one, then keep their reminders without a step from the user.
- Removing the application without turning the check off first leaves the timer or the task behind, starting a file that no longer exists: it fails without a message. The README says how to remove it by hand.

### What a check does

1. Reads the settings and the subscriptions. It never writes the data file: only the window does.
2. Computes the deadline of each subscription (see [Deadlines](#deadlines)), and the reminders due: the day of a reminder is the deadline minus one of its `reminders_days`.
3. Sends a notification for each reminder due today, and for each reminder missed since the last check (the computer off that day) while its deadline isn't past. Each reminder is sent once only; when several reminders of the same deadline were missed, only the latest is sent.
4. Writes what it did to its log, then exits.

A notification names the subscription and its deadline, in the language of the settings, for example « Netflix: cancel by Wednesday 3 December (in 7 days) », or « Home insurance: today is the last day to cancel ». More than three notifications at once give way to a single one: « 5 subscriptions to cancel soon: open Subscriptions Tracker ». A click on a notification does nothing in version 1 (opening the window from it waits in the backlog).

```text
Deadline 2026-12-03, reminders [7, 1], check on 2026-11-26   → « in 7 days », remembered
Same, second check on 2026-11-26 (a second login)            → nothing
Same, the computer off from 2026-11-25 to 2026-12-02         → on 2026-12-02, « in 1 day » only
Same, the computer off from 2026-11-25 to 2026-11-28         → on 2026-11-28, « in 5 days »
Same, the computer off until 2026-12-04                      → nothing: the deadline is past
Reminder 0 in the list, check on the deadline                → « today is the last day »
The subscription renamed after a reminder                    → the reminder isn't sent again (same id)
Four reminders due on the same day                           → one notification for the four
A subscription ending or ended                               → nothing
The data file can't be read                                  → one notification a day at most: « The subscriptions can't be read: open Subscriptions Tracker »
One subscription can't be read                               → the others checked; one notification a day at most: « 1 subscription can't be read: open Subscriptions Tracker »
The check turned off in the settings                         → the timer or the task is gone; no check runs
```

The window and a check can run at the same time: they never write the same file (see [Local files](#local-files)).

## Design principles

<!-- Look and feel, tone of the messages in each language, formal or informal address, use of colors. -->

## Desktop conventions

<!-- The answers to process.md, Desktop conventions: the window (sizes, resizable, position kept), the title bar (the system's or its own), the keyboard and any global hotkey, the language, the theme, the feedback of a wait and of an error, the files, a second instance. Only what departs from the usual, or a choice to make. -->

## Window layout

<!-- Written once the wireframes are agreed (excalidraw-wireframes skill): the blocks of each window, their order, what each shows, the sizes at which the layout changes. Once the mock-up is approved, it is the reference of the interface: say so here, with its link. -->

## Windows

<!-- One subsection per window or panel: what it shows, what each control does, the edge cases, and its acceptance scenarios, each of which becomes a test of the state it draws, or a step of a guide of the test by hand when it needs the system. For example:

### Main window

Shows the last item copied and its price.

```text
First launch, no settings file     → the window opens at 420 × 640 px, in the language of the system
An item copied in the game         → the window shows its name and its price within a second
The price service unreachable      → « Prices unavailable » with the age of the cached prices, and a retry button
```
-->

## Settings

<!-- Each setting: what it changes, its default, where it is shown, and what happens to a value this version can't read (it takes its default). -->

## Visual style

<!-- Written at the end of the mock-up (process.md, Design system; html-mockups skill): the tokens of colors, sizes, typography, icons and images, shapes and motion, with the decisions and the options set aside. -->

## Local files

None of them is tracked by Git: they belong to the user. Every file the application writes is written atomically (a new file synced to the disk, then renamed), never left half-written.

| File | What it holds | Written by | Linux | Windows |
|---|---|---|---|---|
| `settings.toml` | The settings | The window | `~/.config/subscriptions-tracker/` | `%APPDATA%\subscriptions-tracker\config\` |
| `window.ron` | The size and position of the window (eframe) | The window | Same | Same |
| `subscriptions.toml` | The subscriptions | The window, or the user by hand | `~/.local/share/subscriptions-tracker/` | `%APPDATA%\subscriptions-tracker\data\` |
| `reminders.toml` | The reminders already sent, by `id`, deadline and day; a reminder of a past deadline is dropped | The check | `~/.local/share/subscriptions-tracker/` | `%LOCALAPPDATA%\subscriptions-tracker\data\` |
| `log.txt` | The log of the window, written again at each launch | The window | `~/.local/share/subscriptions-tracker/` | `%LOCALAPPDATA%\subscriptions-tracker\data\` |
| `check-log.txt` | The log of the checks, added to at each check and cut to its last 500 lines, so that it never grows and the window never erases it | The check | Same as `log.txt` | Same as `log.txt` |
| `subscriptions-tracker-check.service`, `.timer` | The daily check | The window | `~/.config/systemd/user/` | A task « Subscriptions Tracker check » of the Task Scheduler, at its root |

The data file can be edited by hand, like [its example](docs/subscriptions-example.toml): a change made in the window keeps its comments, its order and the fields this version doesn't know (written by a later version). The window never writes over what it can't read: while the file, or one of its subscriptions, can't be read, the window shows which one and why, and saves nothing until it's fixed by hand (see [Error handling](#error-handling)). `%APPDATA%` holds what follows the user from one Windows computer to another (the settings and the subscriptions), `%LOCALAPPDATA%` what belongs to this computer.

## Error handling

| Situation | Behavior |
|---|---|
