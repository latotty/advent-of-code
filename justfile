default:
  @just --list

install-deps:
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    rustup component add rustfmt clippy
    cargo binstall cargo-nextest cargo-watch

lint:
    cargo fmt
    cargo clippy

test:
    cargo nextest run

run DAY:
    cargo run --bin day{{trim_start_match(lowercase(DAY), 'day')}}

watch-test DAY:
    cargo watch -w src/bin/day{{trim_start_match(lowercase(DAY), 'day')}}.rs -x "nextest run --bin day{{trim_start_match(lowercase(DAY), 'day')}}"

hyperfine:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v hyperfine >/dev/null 2>&1; then
        >&2 echo "Please install hyperfine."
        exit 1
    fi

    cargo build --release
    hyperfine \
        --warmup 10 \
        --min-runs 100 \
        --max-runs 1000 \
        --prepare 'sync' \
        --shell=none \
        $(ls target/release/day??)