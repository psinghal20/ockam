#!/usr/bin/env bash

if [ -z "$OCKAM_HOME" ]
then
  echo "Please set the OCKAM_HOME environment variable to the ockam repository root directory."
  exit 0
fi

OCKAM_RUST="$OCKAM_HOME/implementations/rust/ockam/"
SCRIPT_DIR="$OCKAM_HOME/tools/scripts/release"
export OCKAM_RUST
export SCRIPT_DIR

function change_dir {
  pushd "$1" >/dev/null || exit 1
}

function pop_dir {
  popd >/dev/null || exit 1
}

function crate_version {
  perl -ne '/^version = "([^"]+)"/ and print "$1\n"' < "$OCKAM_RUST/$1/Cargo.toml"
}

function all_crates {
    change_dir "$OCKAM_RUST"
    for CRATE in *
    do
      change_dir "$CRATE"
      cargo -q $* 1>/dev/null || exit 1
      pop_dir
    done
    pop_dir
}

