# rust-subscriptions-tracker

A small desktop application, with a window like [Exile Market](https://github.com/sylent-dev/rust-exile-market), that keeps the list of your subscriptions in a local file and sends a desktop notification before each cancellation deadline, so that no subscription renews at full price because the date was missed.

- **Local only**: the data stays in a file on your disk, with no server and no online account.
- **Reminders without thinking about it**: the system runs the check every day (a systemd user timer on Linux, the Task Scheduler on Windows), and catches up at the next boot if the computer was off.
- **Linux and Windows**.
- **English and French** interface.

What the tool does is described in [specs.md](specs.md), and its progress in [roadmap.md](roadmap.md). The project follows [process.md](process.md), from [rust-cli-template](https://github.com/sylent-dev/rust-cli-template).

## Requirements

- Linux with systemd, or Windows, and a desktop that shows notifications
- Rust 1.93 or later (stable toolchain, see `rust-toolchain.toml`), to build it from source

## Build

```sh
cargo build --release
```

## Development

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Release

1. Update `version` in `Cargo.toml` (then run `cargo build` to update `Cargo.lock`) and merge it into `main`.
2. Tag the merged commit with the same version and push the tag:

   ```sh
   git switch main && git pull
   git tag v0.1.0
   git push origin v0.1.0
   ```

The [Release workflow](.github/workflows/release.yml) builds the archives and publishes them as a GitHub release. It fails if the tag does not match the version of `Cargo.toml`.

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT)).
