#!/bin/zsh
CARGOFLAGS="-c -d 4 -q"
TESTFLAGS="--test-threads 1"

if [ "$#" -ne 1 ] ; then
    cargo watch $(echo $CARGOFLAGS) -x "test -- $(echo $TESTFLAGS)"
else
    RUSTFLAGS="-Z sanitizer=address" cargo watch $(echo $CARGOFLAGS) -x "test -- $TESTFLAGS"
    #RUST_BACKTRACE=1 RUSTFLAGS="-Z sanitizer=address" cargo watch $(echo $CARGOFLAGS) -x "test -- $TESTFLAGS"
fi
