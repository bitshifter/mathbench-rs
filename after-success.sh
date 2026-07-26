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
# For PRs, GITHUB_SHA is a merge commit that doesn't exist in a fresh clone.
# Use GITHUB_HEAD_REF (the branch name) for PRs, fall back to GITHUB_SHA for pushes.
CHECKOUT="${GITHUB_HEAD_REF:-$GITHUB_SHA}"
git checkout "$CHECKOUT" &&
cargo bench --bench eulerbench -- --noplot --save-baseline after &&

# Install https://github.com/BurntSushi/critcmp
cargo install critcmp --force &&

# Compare the two generated benches
critcmp before after;

