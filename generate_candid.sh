#!/bin/bash
# https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid

# This script generates the candid file for the windoge backend canister
# first compile the rust to the updated wasm file
cargo build --release --target wasm32-unknown-unknown --package playing_cards_backend

# then generate the candid file from the wasm file
candid-extractor target/wasm32-unknown-unknown/release/playing_cards_backend.wasm > src/playing_cards_backend/playing_cards_backend.did