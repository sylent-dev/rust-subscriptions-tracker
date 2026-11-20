# rust-subscriptions-tracker

A small desktop application, with a window like [Exile Market](https://github.com/sylent-dev/rust-exile-market), that keeps the list of your subscriptions in a local file shows when each one renews, and sends a desktop notification a few days before each renewal (before the last day to cancel when there is a notice), so that no payment comes as a surprise.

- **Local only**: the data stays in a file on your disk, with no server and no online account.
- **Reminders without thinking about it**: the system runs the check every day (a systemd user timer on Linux, the Task Scheduler on Windows), and catches up at the next boot if the computer was off.
- **Linux and Windows**.
- **English and French** interface.

What the tool does is described in [specs.md](specs.md), and its progress in [roadmap.md](roadmap.md). The project follows [process.md](process.md), from [rust-desktop-template](https://github.com/sylent-dev/rust-desktop-template).

## Requirements

- Linux with systemd, an X11 session or XWayland, and OpenGL; or Windows. A desktop that shows notifications.
- Rust 1.95 or later (the version of `rust-toolchain.toml` is installed by rustup), to build it from source

## Build

```sh
cargo build --release
cargo run
```

## Development

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
sh scripts/clippy-windows.sh
sh scripts/clippy-macos.sh
```

## Release

Follow [docs/releases.md](docs/releases.md): the version in `Cargo.toml`, the notes in `docs/release-notes/v<version>.md`, a build by hand of every system, then the tag:

```sh
git switch main && git pull
git tag v0.1.0
git push origin v0.1.0
```

The [Release workflow](.github/workflows/release.yml) builds the archives for Linux and Windows and publishes them as a GitHub release. It fails if the tag does not match the version of `Cargo.toml`, if the notes are missing, or if the tagged commit isn't on `main`.

## License

Subscriptions Tracker is free software, under the GNU General Public License, version 3 or (at your option) any later version: see [LICENSE](LICENSE). It comes with no warranty. The libraries built into the executable stay under their own licenses, gathered in `THIRD-PARTY-LICENSES` in each release archive.
