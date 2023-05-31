#!/usr/bin/env bash

# Clone the repository
REMOTE_URL="$(git config --get remote.origin.url)"
cd $GITHUB_WORKSPACE/.. &&
git clone $REMOTE_URL "${GITHUB_REPOSITORY}-bench" &&
cd "${GITHUB_REPOSITORY}-bench" &&

# Bench master
git checkout master &&
cargo bench --bench eulerbench -- --noplot --save-baseline before &&

# Bench current branch
git checkout $GITHUB_SHA &&
cargo bench --bench eulerbench -- --noplot --save-baseline after &&

# Install https://github.com/BurntSushi/critcmp
cargo install critcmp --force &&

# Compare the two generated benches
critcmp before after;

