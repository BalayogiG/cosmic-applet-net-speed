#!/bin/bash
set -euo pipefail

# Generates cargo-sources.json, which flatpak-builder uses to vendor crates
# for an offline build. Run this from the repo root whenever Cargo.lock changes.

python3 -m pip install --quiet --user aiohttp toml
python3 flatpak/flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
