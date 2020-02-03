#!/usr/bin/env bash
set -xe
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
(cd $SCRIPTPATH && cargo build --release)
# use raw mode instead of canvas mode (so that there's no black foreground)
export CACA_DRIVER=raw
# replace libcaca functions
export LD_PRELOAD="${SCRIPTPATH}/"target/release/libcaca_blockish.so
# pass video height width to libcaca_blockish
export BLOCKISH_WIDTH=$(( `tput cols` * 8 ))
export BLOCKISH_HEIGHT=$(( `tput lines` * 16 - 1 ))
# start video player
