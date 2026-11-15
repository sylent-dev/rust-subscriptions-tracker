# Code quality

How the code of the application is written. `CLAUDE.md` holds the conventions of the project (files, language, commits, checks); this file holds the rules of the code itself.

Read it at the start of each step that changes code; `code-reviewer` reviews against it. A rule is followed, or the pull request says why not. Clippy (pedantic) and `rustfmt` already enforce their own rules: they aren't repeated here. The examples come from the projects this file was learned on; replace them with the project's own as they come.

## Names

The name comes first: a reader who sees only the name should know what the thing is for.

- A name says what a thing is or does, in the words of the domain (the field of the application, the services it reads), not how it is computed: `affix_limits`, not `counts`; `copied_lines`, not `list`.
- A module or a type is named by what it gives, not by its place next to another one: no module `other_…`, `misc`, `utils` nor `common` (the module that tells the lines in doubt is `doubts`, not `other_readings`). `other` stays the word for the second of a pair (`other_fit`, `other_room`).
- A name that needs its doc comment to be understood is renamed.
- Whole words. No abbreviation but those of the domain and of Rust: `id`, `ui`, `ctx`, `i18n`. A single letter only for an index or a closure parameter of a line or two.
- One notion, one word, across the whole code. Two notions never share a word: when one word means two things, one of them is renamed.
- Types are nouns (`ModifierTiers`). A function that computes a value is named by what it returns (`trade_tiers`, `possible`); a function that acts, by a verb (`save`, `send_copy`).
- A predicate reads as a question: `is_`, `has_`, `can_`, `may_`, or a verb in the third person (`rolls_on`, `holds`). A boolean field or variable too: `is_fractured`, not `flag`.
- Conversions follow the Rust API guidelines: `as_` borrows for free, `to_` costs, `into_` consumes, `from` builds. Constructors: `new`, `with_…`, `of(…)`.
- The path of a module is part of the name: `price::estimate`, not `price::price_estimate`.
- A number with a unit says it: a `Duration`, or a suffix (`_ms`, `_pixels`, `_points`).
- A test is named by the behavior it pins, as a sentence: `a_sanctified_item_gets_no_tier_from_its_values`.
- A name the code has outgrown is renamed in the change that outgrew it.

## Functions

- One function does one thing, at one level of detail, and reads from top to bottom. Past about 50 lines or three levels of nesting, look for the part that deserves a name.
- The case that stops returns early (`let … else`, `?`), so the main path isn't nested.
- At most four parameters, `self` included: Clippy counts them (`clippy.toml`).
- Two parameters of the same type side by side become a struct or an enum when a call can swap them unseen: literals (`f(true, false)`, `f(3, 7)`), or values without a name of their own at the call. Two values named at the call (`english`, `french`) may stay.
- No boolean parameter that switches what a function does: two functions, or an enum named by its cases.
- The decision apart from the wiring: the rule in a pure function, without egui, threads or I/O, and tested; the code that calls it stays thin.

## Types

- Invalid states can't be written: an enum, not two fields always set together or always opposite, not flags that cross.
- `match` on the enums of the project are exhaustive: no `_ =>` that would swallow a new variant. Adding a variant must point to every place to update.
- A rule on the meaning of a value is decided where its domain type is still known, with an exhaustive `match` that names each variant, never on the strings or the ids it was flattened into: a predicate that reads strings a type knew lets through the very case it was meant to catch.
- A newtype when two values of one type can be mixed up (the index of a line and the index of a candidate; pixels and points), or when a value carries an invariant checked once, where it is built.
- `Option` and `Result`, never a sentinel (`-1`, `""`, `NaN`).
- The smallest visibility: private, then `pub(super)`, then `pub`. In a binary crate, `pub` reaches the crate only, as `pub(crate)` would: one spelling, `pub`. A `pub` field only on a plain data type.

## Ownership and borrowing

- Borrow by default: `&str`, `&[T]` and `&T` parameters, `impl Iterator` returns. Own (`String`, `Vec`) only what outlives its source or crosses a thread.
- A `clone()` is a choice: to cross a thread, to keep a value past its source, or a cheap copy (`Copy`, `Arc`). Never to quiet the borrow checker: then the structure is wrong, and a struct split in two lets its fields be borrowed apart.
- `Cow` when a value is mostly borrowed and sometimes built.
- Data between threads: owned by one thread, sent through a channel. `Arc<Mutex<…>>` only where a channel can't do, with the reason in a comment.

## Threads

- Before coding concurrent code (the threads of a hotkey, of the clipboard and of the interface), design who decides what: one judge per decision, and the instant of an event carried with it. Timings patched after the fact make the reviews find the races one by one.
- When two threads watch the same outside state (the active window, the focus), the one that acts publishes its intent first (a counter, read after the state), and a test replays the events in their real order.

## Systems

- A stand-in type for the other systems (`#[cfg(not(target_os = "linux"))]`) is built the same way on every system (`new()`), never with `default()`: Clippy refuses `default()` on a unit struct, an error only the lint of that system shows.
- An item compiled for one system and for the tests (`#[cfg(any(target_os = "windows", test))]`) also compiles for the tests of every other system, where what it uses may not exist: bound it to the systems whose tests use it (`all(test, target_os = "linux")`). `scripts/clippy-windows.sh` and `scripts/clippy-macos.sh` catch it before the push.

## Modules

- One file, one responsibility, named by it. A file that holds two subjects that change for different reasons is split: the data kept by hand apart from the code that reads it, the rules apart from the drawing.
- Past about 800 lines of code (tests aside), a file is looked at for its parts, and so is a struct that gathers unrelated state. The length is a sign; the responsibilities decide.
- The tests of a module past a few hundred lines go to its `tests.rs` (`CLAUDE.md`).
- The dependencies go one way: the domain knows neither egui nor the interface nor the application; the interface reads the domain.
- Reviewing one rule takes reading its file and its tests, not a whole area.

## Rules of the domain

- One rule in one place: a rule of the domain or of an external service (a threshold, a bound, a list of words) is written once and called by every use. Each copy is a place where the application can drift.
- The words of the domain read from outside data, in each language, live in one file of their own; the texts of the interface in `locales/`, never in the code.
- A number of the rules gets a named constant, with a comment that says where it comes from (a measure, the external service, the mock-up).
- A rule that reads the data (a heuristic, a threshold) is measured before it is decided (`CLAUDE.md`, Code review).

## Dependencies

- Before turning off the default features of a crate (`default-features = false`), read what each one does, and keep the ones the application needs, with a check of the CI when nothing else would fail: on eframe, it silently dropped `clipboard`, so that "Copied" showed while nothing reached the system (`.github/scripts/system-clipboard.sh` checks it now). A copy is tested by pasting it into another application.

## Errors

- A `Result` carries its context: what was being done, on what.
- No `unwrap` outside the tests (Clippy `unwrap_used`); an `expect` only for an invariant, which its message states.
- An error is caught where its information still is (the status and the message of a response), not flattened into one variant.
- The world is expected to be wrong at times (a file, an input, the network): it is handled gently. A broken invariant is a bug: it shows.

## Comments

- Each module, type, function and constant has a doc comment that says what it is for, in one sentence, before how; but `main`, the tests (their name is their sentence) and the methods of a trait `impl`, which their module and their trait document. A field or a variant has one when its name and its type don't say it all.
- A comment in a body says why, never what the code already says.
- A comment that no longer matches the code is a bug, fixed in the same change.
- The code says the rule of today. The source of a fact stays next to it (what was measured, where and when), since it tells when to check again; the story of the decisions (what was tried, who chose, what changed) lives in `docs/`.

## Patterns

- Simple first (KISS): a function before a trait, an enum before a trait object, a struct literal before a builder.
- A pattern only once its value is diagnosed, and that value written in the pull request:
  - a trait at a seam a test must replace (the network, the windowing system, the clock);
  - an enum for a closed set of cases;
  - a newtype for an invariant;
  - a builder for a value of many optional parts (the data of the tests);
  - a typestate where an order matters and a mistake would be costly.
- Never a layer that adds without giving: no trait with one implementation and no test double, no generic used with one type.
- SOLID, read for Rust: one responsibility per module and per type; a new case added by a new variant or a new `impl`, the exhaustive `match` naming every place to update; small traits; a trait at a seam, not everywhere.
- When a second fix lands on the same case, look for the rule that makes the case impossible rather than a third condition.

## Tests

- A test reads as an example of its rule: its name says the behavior, its body the case, built by a small helper rather than fifty lines of set-up.
- Each rule is tested on data written by hand, in the `tests.rs` of its file. The tests on real outside data (a file of a service, a table generated from it) are kept for what a new version of that data changes, in a file of their own next to them: the ones to look at, and to record again, when the data change, while the `tests.rs` must still pass as they are. What the tests of several files share (the builders of their data, the readers of a result) lives in the `fixtures.rs` of their module.
- The expected value of a test is written from what the function says it does (its doc comment, `specs.md`), then the test is run: a difference is a finding, the comment or the code is wrong, and the owner decides which.
- An assertion compares the whole value (a list, a section, a struct); a part (`contains`, `starts_with`) only when the rest doesn't matter. An expected exception is a named list, compared whole, never a condition that lets a case through.
- Several cases of one behavior may share a test; two behaviors are two tests.
- A rule that orders or chooses gets a case where the rule and its nearest wrong neighbour disagree: for a `find` over one of two lists, the other list; for two sort keys, each alone and both swapped. Data that give the same answer to both (values already sorted, which a stable sort hides) pin nothing.
- A rule read through a bound or a set taken from the data (the longest entry, the origins a value can have) is tested with the size of the real data and with more than two values of the set.
- What stays on the far side of a test seam is kept as small as possible and tested through a real local server (HTTP) rather than a fake that hands over values already read. A rule kept by the absence of a feature (no cookie sent back) gets a test of the behavior and a check of the feature tree (`cargo tree -e features`).
- A measure quoted in a comment or a document is committed with the code that produces it, as an ignored test with its recorded numbers. Before turning a property a review proposes into an assertion, read the notes of the feature: the property may be false on purpose.
- An egui test whose frames step past an animation (a few tenths of a second) finds a state written only while the widget is drawn, lost when a slow frame skips it: test such widgets with such frames on purpose.
- At the end of a series of test steps, read the written rules (`specs.md`, the notes of `docs/`) one sentence at a time, and ask which test fails if that sentence stops being true: a rule is pinned by one test that fails without it, not by two that surround it.
- Mutations are no part of the usual flow, which the code review and the CI guard. A series is run only on the owner's go, asked beforehand with what it would bring on this code, how many mutations and about how long: it takes time and slows the step. A mutation is one small wrong change of the code, the nearest wrong neighbour of a rule (a `>=` for a `==`, the other list), which a test of the file must catch. One that survives is a question, not yet a missing test: first ask which result it changes that anything reads (the code it touches may do nothing, and is then removed), run it on the real data (a probe in a worktree apart), then hand it to the code review without the author's opinion. A series runs in a worktree apart, never in the project folder, so that no mutation can be left in the code, and each run of the tests is bounded in time and in memory, since a mutation can make a loop endless.

## Refactoring

- At the start of a refactoring step, read the whole code of its area and propose a numbered plan of improvements, one commit each, beyond what the line of the step names (a word shared by two notions, a boolean parameter, a long function, a clone that only quiets the borrow checker): the line says the least to do; the owner accepts the plan.
- No change of behavior, but the small fixes of the last rule, each in its commit: the tests stay the same, checked after each commit, not only at the end, so that a difference names its commit. Where a refactored tool writes files (a generator), run it after each commit and compare its files and what it prints with those of `main`, byte for byte. A tool that can't be run (it reads an external service) is changed only by moves the compiler checks, and the pull request says so. A test whose assertion changes is named in the pull request, with the reason.
- Where the code has a hot path (a loop run millions of times), time it before and after each commit, not only compare what it gives: a call the compiler no longer inlines slows it unseen. A proposal of a review on a hot path is measured like any other change, and the measure of what was tried and left is written next to the code (a doc comment), so that nobody tries again.
- When a refactoring hands functions a wider value (parameters grouped into one struct), list what each one now receives, and pin by a test what it must not read: the compiler only tells a field that is missing.
- Before moving code, check it is tested; a path without a test gets one first, or, where only the network or the system reaches it, it is changed only by steps the compiler checks, and the pull request says so.
- The moves first, in commits of their own: `git diff --color-moved=dimmed-zebra --color-moved-ws=allow-indentation-change` shows them unchanged, but for paths, `use` lines, visibility and the line breaks `rustfmt` puts again, the same tokens. Then one commit per improvement.
- A move of more than a few items is done by a script, not by hand: it cuts each file into its top-level items and copies each one, as it is, to the file a plan names, and can be run again from the commit before, so that a change of the plan costs a minute; a second script checks that every line of the old files is in the new ones. Then `cargo doc --document-private-items` finds the doc links the move left without target.
- Before splitting or moving a file, search the tests for an `include_str!` of its path.
- Each small refactoring seen on the way, in the files the commit already touches, is done in the same pull request. Work that needs a commit of its own and that the line of the step doesn't name is extra work: finish the commit under way, then tell the owner what it is and its cost, and the owner decides (`CLAUDE.md`, Specifications and roadmap). A small fix of the behavior seen on the way, with its test, goes in a commit of its own, named in the pull request as its change of behavior and checked by its review; one that needs a decision or a mock-up goes to `backlog.md`.
