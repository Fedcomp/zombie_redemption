#!/bin/bash
set -e
set -x

SCRIPTDIR=$(dirname "$0")
SRC_DIR="resources"
DST_DIR="game/assets"

cargo run --bin bundlebox -- "$SCRIPTDIR/$SRC_DIR" "$SCRIPTDIR/$DST_DIR"
