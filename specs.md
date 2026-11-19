# Specifications

<!--
Describe what the application does, before writing any code (process.md, Specifications).
Write behavior, not implementation: technical choices go in docs/technical-choices.md.
Remove or add sections as needed.
-->

## Purpose

Subscriptions Tracker is a small desktop application that keeps the list of the user's subscriptions in a local file, and warns them by a desktop notification before the last day they can cancel each one, so that no subscription renews, and no free trial turns paid, because the date was missed.

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

A subscription is one entry of the data file (see [Local files](#local-files)). Its fields, all optional except the first five:

| Field | What it holds |
|---|---|
| `id` | A short name of its own, unique in the file, that never changes (`netflix`): a rename of the subscription keeps its reminders already sent. |
| `name` | The name shown. |
| `status` | `active`, `trial` (a free trial that turns paid at its end) or `cancelled`. |
| `price`, `currency` | The price of one period, as an exact amount with two decimals at most (`"13.49"`, never rounded), and the three-letter code of its currency (`EUR`). |
| `period` | `weekly`, `monthly`, `quarterly` or `yearly`. |
| `renews_on` | One date the subscription renewed or will renew on, usually the first one: the next renewals are counted from it (see [Deadlines](#deadlines)), so that the file doesn't need to change at each renewal. Not needed for a trial, whose renewals start at its end. |
| `notice_days` | The notice: the subscription must be cancelled at least this many days before a renewal. 0 by default. |
| `trial_end` | The day a trial turns paid. |
| `commitment_end` | The end of a commitment (a phone plan for 12 months): no cancellation takes effect before it. |
| `reminders_days` | When to be warned, in days before the deadline (`[30, 14, 3]`), in place of the default of the settings (`[7, 1]`). |
| `category`, `cancel_url`, `account`, `payment`, `started`, `notes` | Free information shown in the window: a category to group them by, the page where it is cancelled, the account used, how it is paid, the day it started, notes. |
| `cancelled`, `ends` | For a cancelled subscription: the day it was cancelled, and the last day of the service already paid. |

The data format comes from the draft accepted by the owner on 2026-11-18, refined along the cycle. Set aside for later (backlog): the history of the prices, a period of its own (every 6 months, every 2 years), the share the user pays of a shared subscription.

## Deadlines

The deadline of a subscription is the last day it can be cancelled without paying the next period. Every date is a calendar day of the computer's time zone, without hours: « today » is the date on the computer's clock.

- **Renewals**: counted from `renews_on`, by whole periods: 7 days for weekly; 1, 3 or 12 months for monthly, quarterly and yearly. A month counted from a day it doesn't have gives its last day, each renewal counted from `renews_on` itself, never from the previous one: a monthly subscription from 31 January renews on 28 February (29 in a leap year), then on 31 March. A yearly one from 29 February renews on 28 February of the years without it.
- **Next renewal**: the first renewal that is today or later.
- **Deadline**: the next renewal minus `notice_days`. When that day is already past (today is inside the notice), this renewal can no longer be avoided: the window says so, and the deadline is the one of the following renewal.
- **Trial**: the deadline is `trial_end` minus `notice_days`. Once `trial_end` is past, the trial is treated as active, renewing from `trial_end` by its period, until the user changes its status.
- **Commitment**: until `commitment_end`, the deadline is the one of the first renewal on or after `commitment_end`.
- **Cancelled**: no deadline and no reminder.

The monthly cost of a subscription is its price brought to a month: weekly × 52 ÷ 12, monthly as it is, quarterly ÷ 3, yearly ÷ 12, rounded to the cent, half up, only when shown. The totals add the active subscriptions and the trials (at their price once paid) of each currency apart, without converting one currency into another.

```text
Monthly from 2026-01-31, today 2026-02-10, notice 0   → next renewal 2026-02-28, deadline 2026-02-28
Monthly from 2026-01-31, today 2026-03-01, notice 0   → next renewal 2026-03-31
Yearly from 2024-02-29, today 2025-01-10              → next renewal 2025-02-28
Yearly on 2027-03-16, notice 30, today 2026-11-19     → deadline 2027-02-14
Monthly on the 3rd, notice 7, today 2026-11-28        → 2026-12-03 can't be avoided; deadline 2026-12-27
Trial ending 2026-11-28, notice 1, today 2026-11-19   → deadline 2026-11-27
Trial ending 2026-11-28, monthly, today 2026-12-10    → treated as active, next renewal 2026-12-28
Commitment until 2027-06-30, monthly on the 15th      → deadline 2027-07-15 minus the notice
Cancelled                                             → no deadline
Yearly at 186.00 EUR                                  → 15.50 EUR a month
Weekly at 4.99 USD                                    → 21.62 USD a month
```

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

<!-- Files read or written by the application (settings, window.ron, data, cache, log), where they live on each system, and whether they are tracked by Git. -->

## Error handling

| Situation | Behavior |
|---|---|
