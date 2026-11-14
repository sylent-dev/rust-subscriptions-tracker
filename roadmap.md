# Roadmap

The steps chosen for the application described in [specs.md](specs.md), following [process.md](process.md). Each step is delivered in its own pull request, which checks its box. What is noted and not chosen yet lives in [backlog.md](backlog.md).

## Size of the project

A terminal interface, like rust-repositories-cleaner: menus and screens with `ratatui`, plus the daily check run by a systemd timer, which shows no screen. Every tool, in order, for the first cycle: specifications, structure, wireframes, mock-up, design system, build, release, retrospective. Linux and Windows (macOS in the backlog), so that the tool can be downloaded from the owner's portfolio; English and French interface. Chosen on 2026-11-14.

## Reference

Read only by the steps that need them:

- [Technical choices](docs/technical-choices.md): the libraries, the services and the main designs, with their reasons.
- `docs/roadmap-archive.md`: the detail of the steps done, moved there once their pull request is merged.
- [Backlog](backlog.md): what is noted and not chosen yet, read when the next work is chosen.

## Steps

<!--
One line per step, small enough to be reviewed in one pull request, naming the tools it needs (process.md, Size a change), for example:
- [ ] **1. Config and local files** (specifications, build; 1 pull request expected): read and write `config.toml`.
A later cycle adds its steps under a heading of its own (« Cycle 2: JSON output and Windows support »), numbered after the last one.
-->

### Cycle 1: the first release

The steps may be split into sub-steps when a pull request grows too large. When no step is left, Claude Code proposes the next items of the backlog to the owner.

- [x] **0. Project setup** (start a project; 1 pull request): package renamed, MIT license, README, size of the project, `main` protected.
- [ ] **1. Specifications** (specifications; 1 pull request expected): purpose, success criteria, screens and the daily check, data file, reminders, acceptance examples; the technical choices in `docs/technical-choices.md`.
- [ ] **2. Structure** (structure; 1 pull request expected): the screens, the menus and the commands in Excalidraw, the chosen lead in `specs.md`.
- [ ] **3. Wireframes** (wireframes; 1 pull request expected): every screen at the minimum terminal size and a wide one, with its empty and error states, in `docs/wireframes/`.
- [ ] **4. Mock-up and design system** (mock-up, design system; 1 or 2 pull requests expected): 3 or 4 styles compared, then the design tokens in `specs.md`.
- The build steps are written once the specifications and the structure are merged, followed by the release (Linux archive only) and the retrospective.
