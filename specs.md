# Specifications

<!--
Describe what the application does, before writing any code (process.md, Specifications).
Write behavior, not implementation: technical choices go in docs/technical-choices.md.
Remove or add sections as needed.
-->

## Purpose

<!-- What the application is for, who uses it, alone or next to which program (a game, an editor), and the expected volume of data. -->

## Success criteria

<!-- Two or three facts the owner can check after the release of this version, checked one by one at the retrospective. For example:
- It replaces my spreadsheet of prices.
- It prices a copied item in under a second.
-->

## Requirements

<!-- What isn't a feature but must hold: the target systems (checked against the program the application works with), the smallest and the default size of the window in logical pixels (e.g. 420 × 640 px), the speed on a given volume, the size of the executable. Only what matters for this application. -->

- **Code that can be rewritten by hand**: the owner will rebuild the application later as a Rust exercise, and show it to learners. The code favors what can be followed from top to bottom over the most compact form: few dependencies, short modules with one clear role each, and comments that explain why rather than what.

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
