# Technical choices

The technical choices of the [roadmap](../roadmap.md), with their reasons.

| Topic | Choice | Reason |
|---|---|---|
| Releases | GitHub Actions workflow triggered by version tags | Builds every system on its own runner and publishes the archives with `gh release`, without extra tooling. |
| Mock-up server | `python3 docs/mockups/serve.py <port>`, port `8753` | Serves the mock-up of the terminal interface without cache, on a port of its own (8753, unused by the other projects of the owner), so that two projects never take each other's server over. |
| Terminal interface | `ratatui` with `crossterm` | Chosen by the owner on 2026-11-14, like rust-repositories-cleaner: the reference TUI library in Rust (lists, tables, colors), and the same structure in both projects for the learners. |
| Translations | [Fluent](https://projectfluent.org/) files in `i18n/<language>/messages.ftl`, read with `i18n-embed` and its `fl!` macro | Same choice as rust-repositories-cleaner: the texts are separated from the code and plurals follow each language's rules. `fl!` checks at compile time that each message and its arguments exist in English, and a test checks that every language has the same messages. |
