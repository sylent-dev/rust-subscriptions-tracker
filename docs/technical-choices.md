# Technical choices

The technical choices of the [roadmap](../roadmap.md), with their reasons.

| Topic | Choice | Reason |
|---|---|---|
| Releases | GitHub Actions workflow triggered by version tags, for Linux and Windows | Builds each system on its own runner and publishes the archives with `gh release`, without extra tooling. Linux and Windows chosen by the owner on 2026-11-14, so that the tool can be downloaded from their portfolio; macOS waits in the backlog. |
| Mock-up server | `python3 docs/mockups/serve.py <port>`, port `8753` | Serves the mock-up of the terminal interface without cache, on a port of its own (8753, unused by the other projects of the owner), so that two projects never take each other's server over. |
| Terminal interface | `ratatui` with `crossterm` | Chosen by the owner on 2026-11-14, like rust-repositories-cleaner: the reference TUI library in Rust (lists, tables, colors), and the same structure in both projects for the learners. |
| Translations | [Fluent](https://projectfluent.org/) files in `i18n/<language>/messages.ftl`, embedded in the executable and read with `fluent-bundle` directly, through a small translation function of our own | The texts are separated from the code and plurals follow each language's rules, as in rust-repositories-cleaner. Rather than `i18n-embed` and its `fl!` macro, which generates code a reader can't see: a few lines a learner can rewrite by hand (owner, 2026-11-14). The compile-time check of `fl!` is replaced by tests: every message used exists in English, and every language has the same messages. |
