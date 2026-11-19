# Lessons

What working on the project taught: a correction of the owner, or a practice that proved useful. One line per lesson, written when it happens: its date, the lesson, and where it may go (this project, the template, a skill, an agent). Read only by the retrospective of each cycle (process.md, Retrospective), which sorts each one into the template, this project's `CLAUDE.md`, its skills or its agents, or drops it, and writes its decision on its line.

<!-- - YYYY-MM-DD: the lesson. Where it may go. -->

- 2026-11-15: to move a project to another template, take the template's files whole, then apply the project's own choices again (name, license, systems, roadmap), and record the commit of the template in `docs/technical-choices.md`, so that the next update is a delta from it. Took one session, with 3 commits. The templates' `process.md` (Start a project, or a section on updating from the template).
- 2026-11-15: the license is chosen again when the kind of application changes: a command-line tool under MIT became a window that ships assets, under the GPL. The templates' `process.md` (Start a project, the license).
- 2026-11-19: the owner found the roadmap too thin on the build: once the specifications describe the behavior without a window (the data, the computation, a headless mode), its build steps can be written at once, one line each with its model, and the steps of the screens as one line each, detailed after the mock-up; « the build steps are written once the structure is merged » hid half of the path to the release. The templates' `roadmap.md` and `process.md` (Specifications, step 5).
