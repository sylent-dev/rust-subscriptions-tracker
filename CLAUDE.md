# CLAUDE.md

## Communication

- Always communicate with the user in French through the Claude Code harness (chat responses, explanations, questions).
- Address the user informally, with « tu » (tutoiement), never « vous », in every session.
- Ask a few numbered open questions at a time, in plain text, the main one first, rather than multiple-choice forms: the owner prefers to answer freely, often by voice dictation (interpret the misspelled names, and confirm when an answer is ambiguous). An unanswered question takes Claude Code's recommendation.
- Keep a critical eye on the owner's ideas: give a clear opinion with its reason, point out the consequence a choice hides, offer an alternative, then follow the owner's decision.
- The first time a technical term comes up that the owner may not know (design token, trait object, lifetime…), explain it in plain words with a short example.
- When the owner asks for an opinion (« qu'en penses-tu ? », « tu penses que… ? »), answer and wait: no edit, commit or push before they decide. A question about what to do (« qu'est-ce qu'on fait ? ») gets a plan, not an action.
- Give the recommendation first, as one numbered plan the owner can accept with one word (« go »), and ask a question only where a choice is really open: a recommendation hidden inside questions isn't read.
- Before calling an option costly, size it from the code and give that size (the lines, the files touched); before stating how a third-party tool or service behaves, or comparing a rule with it, read its source or try it.
- A command the owner pastes alone in the chat may be the one they are about to type in their own terminal: ask before running it.
- A date of the plan isn't a deadline: plan each step for what it needs, and don't bring the schedule up unless the owner does.
- While the owner has said to wait for their signal, the notification of a background task isn't one: one line of status at most, no command. Before a pause, say what is still running and will report.
- A practice Claude Code would add to the method (a series of checks, a habit that costs time, tokens or the machine) is proposed to the owner first, with what it brings and what it costs, and becomes a rule only once accepted: a habit is never written down as the method.

## Naming conventions

- Follow Rust conventions: Rust source files and module directories use snake_case (e.g. `src/file_parser.rs`, `src/user_settings/`).
- All other files and directories use kebab-case (e.g. `docs/getting-started.md`, `scripts/`).
- Standard names keep their conventional form (e.g. `Cargo.toml`, `README.md`, `CLAUDE.md`).
- File and directory names must be in English.

## Language

- Everything in the project (code, identifiers, comments, documentation, commit messages) must be written in English.
- Exception: the translations of the user interface, if the project has any. Only the translated strings and their comments are in the other language; the code stays in English.
- If the user interface is translated, its texts are never hard-coded: each message goes into the translation file of every language (English is the reference), read with the library chosen in `docs/technical-choices.md`.
- Exception: the code review notes in `outputs/` are written in French.
- Exception: the test guides in `docs/tests/` may be written in French, the language of the testers.

## Specifications and roadmap

- Before writing code, describe the expected behavior of the application in `specs.md`, then split the work into steps in `roadmap.md`, and write the technical choices and their reasons in `docs/technical-choices.md`.
- At the start of each session, read `roadmap.md` to know the progress; when asked to continue, resume at the first unchecked step.
- At the start of each step, before writing any code, say which tools of `process.md` the change needs and why (Size a change); when a step turns out larger than sized, stop and size it again with the owner. Judge also whether splitting it into sub-steps (2a, 2b…) helps, and how finely: the split must be an advantage, never a burden, and its granularity is Claude Code's call at each step. A sub-step is one pull request with one coherent change that can be reviewed in one pass and tested on its own, without leaving the application broken in between; a small step stays whole, since each pull request also costs a review, the checks and an archive. Propose the split to the owner, then write it in `roadmap.md`.
- Size each sub-step for about 30 minutes of work: list its commits in its line of `roadmap.md` before any code, four at most, more meaning another sub-step, and read that line as a closed list. Short steps come before fewer reviews: the split is Claude Code's to make, as fine as needed. Stop at a commit, never in the middle of one, so that the next session resumes from the log. When proposing a split, list what it leaves out next to what it covers, and say how many hand tests each part adds. Two small sub-steps may share a pull request and a review, with a stop after the first (a short account of what was written and what differed from the plan) and the second only on the owner's word: never a run without a stop.
- At the start of each step, say which file it grows: before adding to a file past about 800 lines of code (tests aside), or to a struct that gathers unrelated state, propose its split first (`docs/code-quality.md`, Modules).
- When work shows up that the line of the step doesn't name and that needs a commit of its own, finish the commit under way, then tell the owner what it is and what it would cost, and let them decide: now, in a sub-step of its own, or in `backlog.md`. A small refactoring in the files the commit already touches is done (`docs/code-quality.md`, Refactoring).
- A step whose design needs the owner's decisions and whose code depends on a file an external service reads from `main` only (an issue form, read by GitHub from the default branch) is split in two pull requests: first the specifications, the mock-up, the roadmap and that file, without code; then the code, tried against the real service.
- Before coding for a system, check that the product targets it, against the real service or program the application works with (does it run there at all?), so that its cost is weighed with the owner first.
- Each step is delivered in its own pull request, which checks the step's box in `roadmap.md`.
- Keep `roadmap.md` consistent with `specs.md` when the specifications change.
- Keep `roadmap.md` short, since it is read at every session: the steps to come in detail, one line per step done with its pull requests. When the pull request of a step (or of a sub-step) is merged, move the detail of the step to `docs/roadmap-archive.md`, in the first commit of the next step's pull request, never in a pull request of its own: each step then costs the owner one merge. Reference notes (technical choices, rules of the external services) live in `docs/`, linked from `roadmap.md`, and are read only by the steps that need them.
- When an idea, a small touch or an edge case comes up that is not part of the current step, add it to `backlog.md` (process.md, The backlog), with its kind, what it depends on and the date, rather than keeping it only in the memory. Read `backlog.md` only when the next work is chosen: when no step is left in `roadmap.md`, or at the end of a cycle, propose the next items to the owner with their cost and dependencies.
- Read `specs.md` by section: list its headings first (`grep -n '^#' specs.md`), then read only the sections the step needs.
- When the pull request of a step (or of a sub-step) is merged, tell the owner that the step is done and remind them to start a new session for the next step by typing `/clear` (give them the command): a fresh session costs far fewer tokens, and `roadmap.md` and the memory hold what is needed to resume.
- Claude Code's note of the state of the session, in its memory, holds the current state only (where the work stands, what comes next, what is still owed), replaced at each session, never added to: read whole at each resumption, a note that grows by a section a session costs more each time. The commands, the tools and the pitfalls go in a note of their own, read when needed; the history stays in `roadmap.md`, its archive and Git. The scripts written again at each session (the checks) are kept in `outputs/`.

## Connectors

- The claude.ai connectors (Claude Docs and the others) are turned off for the project in `.claude/settings.json` (`disableClaudeAiConnectors`): their instructions and tools would be sent with every message. Remove the setting only if the project needs one of them.
- The skills a project doesn't use (documents, slides, spreadsheets, browsers, charts, Claude API…) are hidden from the model in the same file (`skillOverrides`, `user-invocable-only`): their descriptions would be sent with every message. The owner can still run them with `/<name>`; to give one back to the model, remove its line or use `/skills`.
- The skills of the project live in `.claude/skills/`, copied into every project from the template, never shared from `~/.claude/skills`: each project has one source of truth. A lesson that improves a skill goes back to the template too.
- Claude Code's attribution is turned off in the same file (`attribution`: `commit` and `pr` empty, `sessionUrl` false), so that no commit or pull request carries it (Commit messages). Keep it in every project made from the template.

## Working folders

Two folders are the exchange area between the owner and Claude Code. They are ignored by Git, except their `.gitkeep`, and hold only temporary files.

- `inputs/`: what the owner gives for context: screenshots, examples, references, copied texts. Read them when the owner points to them; never commit, move or delete them unless asked.
- `outputs/`: what Claude Code produces for the owner: reports, code review notes, screenshots to compare styles. Name them `<YYYY-MM-DD>-<subject>.<ext>` when they are meant to be kept for a while. Claude Code's own checks (wireframe previews, test screenshots) stay in its scratchpad, never in `outputs/`, so that the owner doesn't take them for the deliverable. At the end of a numbered step (not of a sub-step), propose to tidy both folders (`tidy-io-folders` skill), and run it on the owner's word.
- Anything meant to last goes elsewhere and is committed: documentation in `docs/`, images used by the application in `assets/`, test data next to the tests.
- The guides of the tests by hand (step by step, a blank results table, a free test last) go in `docs/tests/linux/`, `docs/tests/windows/` or `docs/tests/macos/`, named by feature (e.g. `docs/tests/linux/settings-saved.md`) and listed in `docs/tests/README.md`, in the pull request of the step they test: they are run again before a release. They stay blank templates: their results come back in the conversation or the pull request. Write them with the `hand-test-guides` skill.

## Process

- Follow `process.md`: the first cycle runs the tools the project needs in order (specifications, structure, then wireframes, mock-up and design system for the interface, build, release, retrospective); after it, each change runs only the tools it needs, picked from the backlog into the roadmap. At the retrospective of a cycle, sort the backlog, and move to `CLAUDE.md` or `process.md` the rules of the memory that hold for the whole project.
- An urgent fix jumps the roadmap (a `fix/` branch from `main`, its test, a pull request at once and a patch release, then one line in `roadmap.md`). An idea that needs trying before deciding is an exploration: a branch `explore/<short-description>` that is never merged as it is, bounded by the question it must answer, and ended by its result written down (a decision in `docs/`, a backlog entry or a roadmap step).
- Before the wireframe, the riskiest unknown of the design (a window over another application, an API of the system) is settled by a throwaway probe, in a worktree or the scratchpad, never pushed: half an hour that tells what the design can rely on.
- Sketch any question of structure (the windows and their screens, the menus, the settings) as an editable Excalidraw drawing, with the current state and two leads side by side, before writing it in `specs.md`.
- Draw the interface before coding it: first an editable grayscale wireframe in Excalidraw (`docs/wireframes/`, one group per block; `excalidraw-wireframes` skill), which the owner edits too; then, once the layout is agreed, the interactive HTML mock-up (`docs/mockups/`, `html-mockups` skill).
- During the specifications, check the documentation, rate limits and terms of use of every external service, and record its limits and the risks the owner accepts, with their date, in a note of `docs/` linked from `roadmap.md` (e.g. `docs/<service>-usage.md`).
- For the mock-up, compare 3 or 4 styles that explore new directions for this application. `design-journal.md` records the past design experiences: use it as a source of ideas for one style at most, never as default rules. Give each open decision its own control on the mock-up page (tabs for a few distinct variants, lists that apply at once for values that combine, such as a font per role, sliders for values tuned by eye; see the `html-mockups` skill), and remove it once decided. A new section of the mock-up goes at the top of the page, so that the owner finds it at once; past ten sections, a table of contents under the header links to each.
- Each time the mock-up changes (a new mock-up, a new section, any change to its page or its files), give the owner its link at once, served: `python3 docs/mockups/serve.py <port>` (no cache), on the port of this project written in `docs/technical-choices.md`, kept running in the background; check that the page answers with the change, and give `http://127.0.0.1:<port>/docs/mockups/<name>.html`, with the section and the windows to look at. Never a `file://` path: a sandboxed browser may not open it.
- The interface never names a competitor, not even in a help text.
- Decide in the first wireframes whether the window keeps the title bar of the system (the template's default) or draws its own, and draw its buttons from the start.
- The VS Code Excalidraw extension doesn't reload a file changed on disk: before rewriting a wireframe the owner has open, ask them to save it and close its tab, and commit their version first.
- A mock-up is split into small files (the shared styles and scripts, one style sheet and one script per block, one script per section), so that a change reads only what it needs.
- Once approved, the mock-up (its page and its folder) is the single source of truth of the interface. A design change goes into the mock-up first, then into `specs.md` and into the code when that part is already coded. The wireframe only fixed the first layout and isn't kept up to date.
- Check a visual change of the application with a few screenshots, of the window concerned only (`egui-screenshots` skill), and delete them once looked at: they are for this check, not for the owner, who looks at the mock-up and the application themselves.
- A test that needs extra code (a delay, a simulated click) runs on a copy of the mock-up page, never on the file the owner has open. Stop the helper processes by their PID, never with `pkill -f` and a pattern that also matches Claude Code's own command.
- When a deliverable is handed to the owner to look at (a drawing, the mock-up, a build to try), wait for their approval before writing it into `specs.md` or opening the pull request: « let's go » is not an approval of something they haven't seen yet.

## Lessons

- When the owner corrects how Claude Code works, or a practice proves useful, add one line to `docs/lessons.md` at once (its date, the lesson, where it may go), in the pull request of the step under way: the retrospective of each cycle (process.md, Retrospective) sorts them into the template, this `CLAUDE.md`, the skills or the agents.
- Hold a retrospective often, like the refactoring, rather than one big one at the end: the first right after the first release, while the lessons are fresh and the users' reports take time to come; the next after the polish that follows, with those reports.

## Git workflow

- Never commit or push directly to `main`.
- For each task, create a dedicated branch from `main`, commit and push only on that branch.
- Every change must go through a pull request targeting `main`.
- Each time a pull request is opened or updated, give the owner its full link (`https://github.com/<owner>/<repository>/pull/<number>`), not only its number.
- Merge pull requests with **Squash and merge** only, so that each pull request becomes one commit on `main`, titled in Conventional Commits. The repository allows only squash merging and deletes the branch once merged; check these settings on every new repository (`gh repo edit --enable-squash-merge --enable-merge-commit=false --enable-rebase-merge=false --delete-branch-on-merge`).
- Before pushing to the branch of a pull request, check that it is still open (`gh pr view <branch> --json state`): a commit pushed after the merge never reaches `main`.
- Keep only one pull request open at a time: open the next one once the previous one is merged. Two open pull requests often edit the same files (`roadmap.md`, `docs/`), and the second one conflicts once the first is merged. If urgent work comes up meanwhile, prepare its branch and push it, then open its pull request once the open one is merged.
- When several pull requests are open anyway, resolve their conflicts before the owner sees them: as soon as the owner says a pull request is merged, rebase the others on `main` (`git rebase origin/main`, then `git push --force-with-lease`), run the checks again, and tell the owner when each one is ready to merge.
- Name branches `<type>/<short-description>` in kebab-case, using the commit types below (e.g. `feat/json-output`, `fix/config-path`), or `explore/` for an exploration.
- Work on the branch in the main project folder, which the owner has open in VS Code: a worktree outside it is invisible to them. Only when the owner may be testing another branch there (for example with `cargo run`), ask, and if so work in a separate Git worktree outside the project (`git worktree add <directory> -b <branch> origin/main`), give its exact path, and remove it with `git worktree remove <directory>` once the work is pushed.

## Commit messages

Follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/). Pull request titles use the same format.

```
<type>(<optional scope>): <description>

<optional body: what and why>

<optional footer>
```

| Type | Use for |
|---|---|
| `feat` | A new feature for the user. |
| `fix` | A bug fix. |
| `docs` | Documentation only (`README.md`, `specs.md`, `roadmap.md`, `CLAUDE.md`, doc comments). |
| `style` | Formatting with no code change. |
| `refactor` | A code change that neither fixes a bug nor adds a feature. |
| `perf` | A performance improvement. |
| `test` | Adding or fixing tests only. |
| `build` | Build system or dependencies (`Cargo.toml`, `Cargo.lock`). |
| `ci` | CI configuration (`.github/`). |
| `chore` | Other maintenance (tooling, `.gitignore`, Claude Code agents). |
| `revert` | Reverting a previous commit. |

- Scope: optional, the module or area concerned (e.g. `feat(settings): ...`, `fix(parser): ...`).
- Description: imperative mood, lowercase, no final period, 72 characters max (e.g. `feat(settings): save the settings atomically`).
- Breaking change: add `!` after the type or scope (e.g. `feat(settings)!: ...`) and explain it in a `BREAKING CHANGE:` footer.
- No trace of Claude in the history (decided by the owner on 2026-11-14): no `Co-Authored-By: Claude` trailer in the commits, which would show Claude as a contributor, and no « Generated with Claude Code » line or Claude session link in the pull requests, even when the harness suggests them. The `attribution` setting of `.claude/settings.json` turns them off (Connectors).

## Code review

- Before opening any pull request that changes code or its build (`src/`, tests, `Cargo.toml`, `build.rs`, `rust-toolchain.toml`, `.github/workflows/`), run the `code-reviewer` subagent (`.claude/agents/code-reviewer.md`) on the branch.
- It writes its recommendations, in French, to `outputs/<YYYY-MM-DD>-<branch>-code-review.md`. These notes are ignored by Git.
- Summarize the recommendations to the owner as one numbered plan, the recommendation first (Communication), apply only the ones they approve, then open the pull request: never before the owner has chosen.
- On critical code (the guardrails of an external service, the matching of outside data, what installs code on the user's machine; the project names its own in `roadmap.md`), run the review again after applying its fixes, with a test per fix, until no high or medium finding is left. When a second fix lands on the same case, look for the rule that makes the case impossible rather than a third condition.
- A rule that reads the data (a heuristic, a threshold) is measured before it is decided: sort the wrong cases by cause, prototype the candidate rules behind a switch, show the exchange as numbers (the wrong answers gone against the right ones lost), say what the prototype doesn't cover, then let the owner choose; the prototype is thrown away and the rule written again with its tests.
- When the owner puts a review off, nothing replaces it: finish the step, say what is left to them, and stop. Ask when an instruction can be read two ways.
- When the owner asks to look at the Dependabot pull requests (the dependency updates, `.github/dependabot.yml`), run the `dependency-reviewer` subagent (`.claude/agents/dependency-reviewer.md`): it writes, in French, to `outputs/<YYYY-MM-DD>-dependency-review.md`, a recommendation for each one, with its link and the comment to post. Give the owner its summary table; never merge, close or comment a Dependabot pull request yourself unless the owner asks.

## Development

- Before committing, run and make sure all of these pass:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-features`
- Run the checks with their exit codes written out (`cargo clippy …; echo CLIPPY=$?`), never piped into `| tail` inside an `&&` chain: the pipe hides a failure.
- Before pushing code written for Windows or macOS alone, run Clippy for that system from Linux: `scripts/clippy-windows.sh` and `scripts/clippy-macos.sh` (nothing is linked, so no compiler of that system is needed).
- The same checks run in CI (`.github/workflows/ci.yml`) on every pull request, with Clippy on Windows and for macOS too, skipped when it only changes Markdown files and `docs/` (the jobs, required on `main`, then succeed at once).
- When a bug reaches the owner that a test or a check could have caught, add it in the same pull request as the fix.
- Never remove a failing CI job before reading its log: it may be the only check of its system.
- The rules of the code are in `docs/code-quality.md`: read it at the start of each step that changes code. Keep the code clean at every change, without waiting to be asked: before each commit, reread the functions the change touches as a whole and refactor them (remove what became dead or redundant, rename what no longer says what it does, keep each rule in one place, never set a value that is always overwritten), most of all after the fixes of a code review, which pile up; say in the summary what was refactored.
- Every acceptance scenario of `specs.md` that a step touches becomes a test in the same pull request: its logic apart from the window, as unit tests; what only the window shows, a step of a guide of the tests by hand (Working folders). A step is done only when the facts of its **done when** in `process.md` (The tools) hold.
- When Claude Code runs in an editor installed as a Flatpak (no access to the owner's desktop: X11, other applications), a check that must run on the desktop (a probe of the windows, of the keyboard or of the pointer) is built in the scratchpad, copied to `~/.cache/<probe>/`, and run with `flatpak-spawn --host <path of the probe>`, outside the sandbox of Claude Code. Ask the owner to allow only that command in `.claude/settings.local.json` (`"Bash(flatpak-spawn --host <path of the probe>:*)"`, never committed), rather than giving the editor access to X11 for good. Warn the owner before each run, and say what they must do in the meantime.
- Before a probe that takes the focus or sends keys or clicks on the owner's desktop (`xdotool`, the application sending keys), tell them how long it takes and to let go of the keyboard and the mouse, wait for their go, then show a 3-2-1 countdown and an end notice as desktop notifications (`notify-send`): the output of a command isn't shown live.
- A feature that depends on the desktop (a window over another one, the focus, the pointer) is checked in each setup the users have (one screen and two, each display mode), and a small probe of the real desktop (the active window read, a grab tried) settles a cause before any fix is coded.
- `unsafe` code is forbidden and Clippy pedantic lints are enabled (see `[lints]` in `Cargo.toml`).
- `rust-version` in `Cargo.toml` is the minimum Rust version: CI also builds with it. Raise it when a dependency or a language feature needs a newer Rust.
- Once the tests of a module run past a few hundred lines, move them to a file of their own, `<module>/tests.rs` (`src/session.rs` ends with `#[cfg(test)] mod tests;`, its tests in `src/session/tests.rs`), so that reading the code doesn't read its tests too; as a child module, they still reach its private items (`use super::*;`). The paths of `include_str!` are then one `../` deeper.

## Releases

- Pushing a version tag (e.g. `v0.1.0`) that matches the version of `Cargo.toml` publishes a GitHub release with the executables for Linux and Windows (`.github/workflows/release.yml`); macOS waits in `backlog.md`, its Clippy job kept in the CI.
- Follow `docs/releases.md`: the notes of the version in `docs/release-notes/`, a build by hand of every system before the tag, and what an archive holds.
- Before every release, run the `security-auditor` subagent (`.claude/agents/security-auditor.md`) on the whole repository at `main`: it writes its findings, in French, to `outputs/<YYYY-MM-DD>-security-audit.md`. Summarize them to the owner, and fix the ones they approve in a pull request before the tag; a critical or high finding blocks the tag.
- The template is made for desktop applications released as an archive per system with the executable. Installers and automatic updates aren't in it yet: a project that needs them records its choice in `docs/technical-choices.md`.
