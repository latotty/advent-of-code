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

build-release DAY:
    cargo build --release --bin day{{trim_start_match(lowercase(DAY), 'day')}}

watch-test DAY:
    cargo watch -w src/bin/day{{trim_start_match(lowercase(DAY), 'day')}}.rs -x "nextest run --bin day{{trim_start_match(lowercase(DAY), 'day')}}"

hyperfine DAY="all":
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v hyperfine >/dev/null 2>&1; then
        echo "Please install hyperfine."
        exit 1
    fi

    execute() {
        hyperfine \
        --warmup 10 \
        --min-runs 100 \
        --max-runs 1000 \
        --setup "just build-release $1" \
        --prepare 'sync' \
        --shell=none \
        --command-name $1 \
        "{{justfile_directory()}}/target/release/$1"
    }

    if [ "{{DAY}}" == "all" ]; then
        for file in src/bin/day*.rs; do
            execute $(echo $file | cut -d. -f1 | cut -d/ -f3)
        done
    else
        execute day{{trim_start_match(lowercase(DAY), 'day')}}
    fi