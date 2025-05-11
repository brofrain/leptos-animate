_default: help

# Prints available recipes
@help:
    echo "\nRun a task using \`just [RECIPE]\`."
    just --list

# Formats Rust files using rustfmt
_fmt-rustfmt *args:
    cargo +nightly fmt --all {{ args }}

# Formats Leptos components using leptosfmt
_fmt-leptosfmt *args:
    leptosfmt examples {{ args }}

# Formats Rust files including Leptos component syntax
fmt-rs:
    just _fmt-rustfmt
    just _fmt-leptosfmt

_prettier +args:
    bun prettier "**/*.toml" {{ args }}

# Formats supported files with Biome
_fmt-biome:
    bun biome check --write --linter-enabled=false

# Formats TOML files with Prettier
_fmt-prettier: (_prettier "-w")

# Formats justfile
_fmt-justfile:
    just --unstable --fmt

# Formats all non-Rust files using Biome and Prettier
fmt-non-rs:
    just _fmt-biome
    just _fmt-prettier
    just _fmt-justfile

# Formats all files in the project
fmt:
    just fmt-rs
    just fmt-non-rs

_fmt-check-rustfmt:
    just _fmt-rustfmt --check

_fmt-check-leptosfmt:
    just _fmt-leptosfmt --check

_fmt-check-biome:
    npx biome check --linter-enabled=false --error-on-warnings

_fmt-check-prettier:
    just _prettier --check

_fmt-check-justfile:
    just --unstable --fmt --check

# Checks formatting
fmt-check:
    just _fmt-check-rustfmt
    just _fmt-check-leptosfmt
    just _fmt-check-biome
    just _fmt-check-prettier
    just _fmt-check-justfile

# Lints Rust codebase with Clippy
lint-rs message_format='human':
    cargo clippy --workspace --message-format={{ message_format }}
    cargo clippy --workspace --exclude ssr --target=wasm32-unknown-unknown --message-format={{ message_format }}

# Checks for TypeScript, Biome and Typos errors
lint-non-rs:
    bun tsc
    bun biome lint --error-on-warnings
    typos

# Lints the project
lint: lint-rs lint-non-rs

_rust-analyzer-check:
    just lint-rs json
