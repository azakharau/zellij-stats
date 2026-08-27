# zellij-stats

`zellij-stats` is a small native command that samples system CPU and memory and
prints a ready-to-render status segment. The output uses the `#[...]` formatting
understood by `zjstatus` and tmux-style status renderers.

It is deliberately a one-shot program: a status bar or wrapper script decides
how often to run it. This keeps it separate from
[`zellij-resource-status`](https://github.com/azakharau/zellij-resource-status),
which is a long-running Zellij WASM plugin with macOS-specific samplers.

## Build and run

```sh
cargo build --release
./target/release/zellij-stats
```

Example output, shown without terminal styling:

```text
CPU 12% RAM 63%
```

CPU changes from green to yellow at 40% and red at 75%. RAM changes from green
to yellow at 50% and red at 80%. If metrics cannot be collected, the command
prints an empty segment.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## License

MIT
