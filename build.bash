#!/bin/bash

./clean.bash
rustup override set nightly
cargo build --release
cp target/release/vanity .
if [ ! -e prefixes.txt ]; then echo -e "MAIA\nOTHER\nTODO" > prefixes.txt; fi
