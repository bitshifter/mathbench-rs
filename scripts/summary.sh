#!/usr/bin/env bash
# Summarize the criterion results in target/criterion by running summary.py in a
# local virtual environment, creating that environment on first use.
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
venv_dir="$root_dir/.venv"

if ! "$venv_dir/bin/python" -c 'import prettytable' >/dev/null 2>&1; then
    echo "Setting up $venv_dir with prettytable..." >&2
    python3 -m venv "$venv_dir"
    "$venv_dir/bin/pip" install --quiet --upgrade prettytable
fi

exec "$venv_dir/bin/python" "$root_dir/scripts/summary.py" "$@"
