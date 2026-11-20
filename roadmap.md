# Roadmap

The steps chosen for the application described in [specs.md](specs.md), following [process.md](process.md). Each step is delivered in its own pull request, which checks its box. What is noted and not chosen yet lives in [backlog.md](backlog.md).

## Size of the project

A desktop application with a graphical window, like rust-exile-market (`egui`): a list of the subscriptions and their screens, plus the daily check run by the system scheduler, which shows no window. Every tool, in order, for the first cycle: specifications, structure, wireframes, mock-up, design system, build, release, retrospective. Linux and Windows (macOS in the backlog), so that the tool can be downloaded from the owner's portfolio; English and French interface. Chosen on 2026-11-14; the graphical window replaced the terminal interface (`ratatui`) the same day. A small application, like rust-exile-market in its window, not in its size: a PWA was weighed and set aside (owner, 2026-11-18), since its reminders would need a server.

Windows is built and released without being tried by hand: the owner has no Windows machine and no tester yet (2026-11-18). Until one is found, late in the cycle, Windows is checked by the CI only, its reminders (the Task Scheduler, the notifications) included.

## Reference

Read only by the steps that need them:

- [Technical choices](docs/technical-choices.md): the libraries, the services and the main designs, with their reasons, and the commit of the template the project was last updated from.
- [Code quality](docs/code-quality.md): the rules of the code, read at the start of each step that changes code.
- [Releases](docs/releases.md): how to publish a version, and what its archives hold.
- [Lessons](docs/lessons.md): what working on the project taught, kept for the retrospectives.
- `docs/roadmap-archive.md`: the detail of the steps done, moved there once their pull request is merged.
- [Backlog](backlog.md): what is noted and not chosen yet, read when the next work is chosen.

## Models

Each step to come names the Claude model recommended for its session, so that the owner switches with `/model` before starting it; a new step or sub-step gets its model when it is written here. A judgment of what a quiet mistake would cost, not of the size of the step: Fable 5.2 for critical code (the rules of an external service the application must not break, the matching of the data a result depends on, what installs code on a user's machine); Opus 5.5 for every other step (specifications, mock-up, interface, refactoring, release, documentation). A step that decides what the user is shown as true (a price, a match, an estimate) is critical, whatever its size: rate it so when it is written. In this project, the reading of the data, the computation of the renewals and of the reminders, the daily check, its installation in the system scheduler and the writing of the data file by the window are critical.

The subagents (`code-reviewer`, `security-auditor`, `dependency-reviewer`) name no model and run on the model of the session, but a code review, whose model follows what it reads: Opus 5.5 for a pull request of tests alone or of moves alone, also in a session on Fable 5.2, a miss there leaving a weaker guard or a move to look at again, not a bug; Fable 5.2 for critical code; the model of the session otherwise. Claude Code gives `code-reviewer` its model when it launches it, and says it with the cost of the review; the notes of a review name at their top the model that wrote them, and the line of the step says which model reviewed it. If a step proves harder than its model (a review that keeps finding medium recommendations, a design that doesn't settle), say so and propose to switch.

## Steps

<!--
One line per step, small enough to be reviewed in one pull request, naming its model and the tools it needs (process.md, Size a change), for example:
- [ ] **1. Settings and local files** (model: Opus 5.5; specifications, build; 1 pull request expected): read and write `settings.toml`.
A later cycle adds its steps under a heading of its own (« Cycle 2: a history window and Windows support »), numbered after the last one.
-->

### Cycle 1: the first release

The steps may be split into sub-steps when a pull request grows too large. When no step is left, Claude Code proposes the next items of the backlog to the owner.

- [x] **0. Project setup** (#1)
- [x] **0b. Update from the template** (#2)
- [x] **1. Specifications** (#4, #5, #6)
- [x] **2. Structure** (#7)
- [ ] **3. Wireframes** (model: Opus 5.5; wireframes; 1 pull request, two sub-steps with a stop between), in `docs/wireframes/main-window.excalidraw`, the title bar of the system:
  - [ ] **3a, the main frames at three sizes**: commits: (1) the archive of step 2 and this split; (2) the list and the form at 480 × 400, 720 × 560 and 1200 × 760, the form beside the table from 960 px wide; (3) the owner's version, then its rearrangement. Leaves out the states and the screens over the window (3b). No hand test.
  - [ ] **3b, the rest under them**: commits: (1) the states (empty, unreadable, reminders off, Ending and Ended, the form with its errors and its buttons per status, the confirmations, the settings, About and the export at the smallest size); (2) French at 480 px, where a text may not fit; (3) the owner's version, then its rearrangement; (4) the window layout in `specs.md`. No hand test.
- [ ] **4. Mock-up and design system** (model: Opus 5.5; mock-up, design system; 1 or 2 pull requests expected): 3 or 4 styles compared, then the design tokens in `specs.md`.
- The build steps, after the mock-up (owner, 2026-11-19), each split into sub-steps at its start; the translations, English and French, in each step:
- [ ] **5. The data** (model: Fable 5.2; build; 1 or 2 pull requests expected): `subscriptions.toml` read into the types of the application, the amounts in cents, the rules of the periods, an error per field.
- [ ] **6. The renewals and the costs** (model: Fable 5.2; build; 1 pull request expected): the renewals, the last day to cancel (the notice and the commitment) and the days of the reminders with `jiff`, the monthly and yearly costs; the examples of `specs.md` as tests.
- [ ] **7. The daily check** (model: Fable 5.2; build; 2 pull requests expected): `--check`, the reminders due and missed, `reminders.toml`, the notifications, its log.
- [ ] **8. The settings and the installation of the check** (model: Fable 5.2; build, a guide of the test by hand; 2 pull requests expected): `settings.toml`, the systemd timer, the task of the Task Scheduler, installed again when it starts another executable.
- [ ] **9. The list** (model: Opus 5.5; build; 1 or 2 pull requests expected): the totals, the groups, the empty and unreadable states.
- [ ] **10. The form of a subscription** (model: Fable 5.2; build; 2 pull requests expected): the fields and their errors, the cancellation, the deletion, the file changed with `toml_edit` without losing its comments, the changes made outside the window.
- [ ] **11. The settings and About** (model: Opus 5.5; build; 1 pull request expected): the screen of the settings, the panel About with the notice of the GPL.
- [ ] **12. The calendar export** (model: Opus 5.5; build; 1 pull request expected): the `.ics` file of the renewals of the next year.
- [ ] **Quality review** (model: Opus 5.5, Fable 5.2 for the critical areas): before the first release, the whole repository reviewed area by area (process.md, Release): `code-reviewer` in its area mode, one pass per area at once, each told what the others read and given the results of Clippy and of the tests run once before them, against SOLID, DRY and KISS, a design pattern only where it removes a real difficulty. Its findings split into pull requests, in this order: the fixes of behavior, the tests, what needs a mock-up, the moves last.
- [ ] **First release** (model: Opus 5.5): the security audit, the guides of `docs/tests/` run again, the notes of the version, a build by hand of Linux and Windows, then the tag (`docs/releases.md`).
- [ ] **Donation method** (model: Opus 5.5): if the owner wants one, a donation link in the README, on the site and, discreetly, in the application (mock-up first). Before opening a new donation page, check what the payment service shows the donors (the legal name, the email, the statement descriptor), with one page per application (process.md, Release).
- [ ] **First retrospective** (model: Opus 5.5): right after the first release, while the lessons are fresh and the users' reports take time to come: the lessons of `docs/lessons.md` sorted one by one with the owner (process.md, Retrospective), into this project, the templates, or dropped. With the backlog entry of a template for a desktop application with a daily check.

### After the first release

- [ ] **Polish** (model: Opus 5.5): the last touches, picked with the owner from `backlog.md` (Polish, Edge cases), released together as a patch version.
- [ ] **Second retrospective** (model: Opus 5.5): right after the polish, the same sorting on the lessons written since the first one, with the users' reports as well.
