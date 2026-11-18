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

Each step to come names the Claude model recommended for its session, so that the owner switches with `/model` before starting it; a new step or sub-step gets its model when it is written here. A judgment of what a quiet mistake would cost, not of the size of the step: Fable 5.2 for critical code (the rules of an external service the application must not break, the matching of the data a result depends on, what installs code on a user's machine); Opus 5.5 for every other step (specifications, mock-up, interface, refactoring, release, documentation). A step that decides what the user is shown as true (a price, a match, an estimate) is critical, whatever its size: rate it so when it is written. In this project, the computation of the deadlines and the installation of the daily check in the system scheduler are critical.

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
- [ ] **1. Specifications** (model: Opus 5.5; specifications; 2 pull requests expected):
  - [ ] **1a, the data and the daily check**: commits: (1) this archive and this split; (2) purpose, success criteria, requirements, the data file and the computation of the deadlines; (3) the daily check without a window, its reminders, the local files and a log that the check and the window don't erase for each other; (4) the technical choices. Leaves out the windows, the settings shown, the desktop conventions. No hand test.
  - [ ] **1b, the windows and the conventions**: the desktop conventions, what each window shows (the list, the `.ics` export, the About panel with the notice of the GPL), the settings, the error handling and the acceptance scenarios. No hand test.
- [ ] **2. Structure** (model: Opus 5.5; structure; 1 pull request expected): the windows, their screens and the settings in Excalidraw, the chosen lead in `specs.md`.
- [ ] **3. Wireframes** (model: Opus 5.5; wireframes; 1 pull request expected): every window at its minimum size and a wide one, with its empty and error states, and the title bar decided, in `docs/wireframes/`.
- [ ] **4. Mock-up and design system** (model: Opus 5.5; mock-up, design system; 1 or 2 pull requests expected): 3 or 4 styles compared, then the design tokens in `specs.md`.
- The build steps are written once the specifications and the structure are merged.
- [ ] **Quality review** (model: Opus 5.5, Fable 5.2 for the critical areas): before the first release, the whole repository reviewed area by area (process.md, Release): `code-reviewer` in its area mode, one pass per area at once, each told what the others read and given the results of Clippy and of the tests run once before them, against SOLID, DRY and KISS, a design pattern only where it removes a real difficulty. Its findings split into pull requests, in this order: the fixes of behavior, the tests, what needs a mock-up, the moves last.
- [ ] **First release** (model: Opus 5.5): the security audit, the guides of `docs/tests/` run again, the notes of the version, a build by hand of Linux and Windows, then the tag (`docs/releases.md`).
- [ ] **Donation method** (model: Opus 5.5): if the owner wants one, a donation link in the README, on the site and, discreetly, in the application (mock-up first). Before opening a new donation page, check what the payment service shows the donors (the legal name, the email, the statement descriptor), with one page per application (process.md, Release).
- [ ] **First retrospective** (model: Opus 5.5): right after the first release, while the lessons are fresh and the users' reports take time to come: the lessons of `docs/lessons.md` sorted one by one with the owner (process.md, Retrospective), into this project, the templates, or dropped. With the backlog entry of a template for a desktop application with a daily check.

### After the first release

- [ ] **Polish** (model: Opus 5.5): the last touches, picked with the owner from `backlog.md` (Polish, Edge cases), released together as a patch version.
- [ ] **Second retrospective** (model: Opus 5.5): right after the polish, the same sorting on the lessons written since the first one, with the users' reports as well.
