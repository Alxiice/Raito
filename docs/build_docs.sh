#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Build the rust doc
# additional : --no-deps --offline
cargo doc --no-deps --release --manifest-path "$SCRIPT_DIR/../Cargo.toml" --target-dir "$SCRIPT_DIR/build/rust"

# Build the doc
python -m mkdocs build --config-file "$SCRIPT_DIR/mkdocs.yml" --site-dir "$SCRIPT_DIR/build/html"
