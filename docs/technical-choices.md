# Technical choices

The technical choices of the [roadmap](../roadmap.md), with their reasons.

| Topic | Choice | Reason |
|---|---|---|
| Releases | GitHub Actions workflow triggered by version tags, for Linux and Windows | Builds each system on its own runner and publishes the archives with `gh release`, without extra tooling. Linux and Windows chosen by the owner on 2026-11-14, so that the tool can be downloaded from their portfolio; macOS waits in the backlog. |
| Mock-up server | `python3 docs/mockups/serve.py <port>`, port `8753` | Serves the mock-up of the window without cache, on a port of its own (8753, unused by the other projects of the owner), so that two projects never take each other's server over. |
| Graphical interface | [`egui`](https://github.com/emilk/egui) with `eframe` | Chosen by the owner on 2026-11-14, like rust-exile-market: pure Rust, one executable per system without an embedded browser, and code redrawn from the data at every frame, which a learner can follow from top to bottom. A window reaches the public of the owner's portfolio, who don't use a terminal. Replaces `ratatui` with `crossterm`, chosen the same day: lighter (a few dozen crates against a few hundred, an executable of about 2 MB against about 10 MB), but a terminal interface suits few of the people who track their subscriptions. |
| Translations | [Fluent](https://projectfluent.org/) files in `i18n/<language>/messages.ftl`, embedded in the executable and read with `fluent-bundle` directly, through a small translation function of our own | The texts are separated from the code and plurals follow each language's rules, as in rust-repositories-cleaner. Rather than `i18n-embed` and its `fl!` macro, which generates code a reader can't see: a few lines a learner can rewrite by hand (owner, 2026-11-14). The compile-time check of `fl!` is replaced by tests: every message used exists in English, and every language has the same messages. |
