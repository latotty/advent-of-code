install-deps:
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    rustup component add rustfmt clippy
    cargo binstall cargo-nextest cargo-watch

test:
    cargo nextest run

run DAY:
    cargo run --bin day{{trim_start_match(lowercase(DAY), "day")}}

watch-test DAY:
    cargo watch -w src/bin/day{{trim_start_match(lowercase(DAY), "day")}}.rs -x "nextest run --bin day{{trim_start_match(lowercase(DAY), "day")}}"

hyperfine DAY="all":
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v hyperfine >/dev/null 2>&1; then
        echo "Please install hyperfine."
        exit 1
    fi

    execute() {
        hyperfine \
        --warmup 5 \
        --min-runs 10 \
        --max-runs 300 \
        --command-name $1 \
        "just run $1"
    }

    if [ "{{DAY}}" == "all" ]; then
        for file in src/bin/day*.rs; do
            execute $(echo $file | cut -d. -f1 | cut -d/ -f3)
        done
    else
        execute day{{trim_start_match(lowercase(DAY), "day")}}
    fi