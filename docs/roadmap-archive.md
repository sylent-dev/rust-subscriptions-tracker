# Roadmap archive

The detail of the steps done, moved from the [roadmap](../roadmap.md) once their pull request is merged. The pull requests and the Git history hold the rest.

- [x] **0. Project setup** (start a project; 1 pull request, #1): package renamed, MIT license, README, size of the project, `main` protected.
- [x] **0b. Update from the template** (model: Opus 5.5; start a project; 1 pull request, #2): rust-desktop-template at `6e0625e` taken in place of rust-cli-template. Commits: the rules, the process, the agents and the skills; the egui skeleton, its CI and its release, under GPL-3.0-or-later, for Linux and Windows; this project's documents merged. Once merged, `Lint on windows-latest` and `Lint on macos-latest` added to the required checks of `main`.
- **1. Specifications**, in sub-steps:
  - **1a, the data and the daily check** (#4): commits: (1) this archive and this split; (2) purpose, success criteria, requirements, the data file and the computation of the deadlines; (3) the daily check without a window, its reminders, the local files and a log that the check and the window don't erase for each other; (4) the technical choices. Leaves out the windows, the settings shown, the desktop conventions. No hand test.
