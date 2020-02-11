#!/usr/bin/env bash
set -xe
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
(cd $SCRIPTPATH && cargo build --release)
# use raw mode instead of canvas mode (so that there's no black foreground)
export CACA_DRIVER=raw
# replace libcaca functions
export LD_PRELOAD="${SCRIPTPATH}/"target/release/libcaca_blockish.so
