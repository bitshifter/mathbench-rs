#!/usr/bin/env bash
# Summarize the criterion results in target/criterion by running summary.py in a
# local virtual environment, creating that environment on first use.
set -euo pipefail

prettytable_version="3.18.0"

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
venv_dir="$root_dir/.venv"
venv_python="$venv_dir/bin/python"

# Reinstall only when the pinned version is not already importable.
if ! "$venv_python" -c "import prettytable, sys; sys.exit(prettytable.__version__ != '$prettytable_version')" >/dev/null 2>&1; then
    echo "Setting up $venv_dir with prettytable $prettytable_version..." >&2
    if ! command -v python3 >/dev/null 2>&1; then
        echo "error: python3 not found; install Python 3 to run the summary." >&2
        exit 1
    fi
    if ! python3 -m venv "$venv_dir" 2>/dev/null; then
        echo "error: could not create a virtual environment in '$venv_dir'." >&2
        echo "Install the Python venv module (Fedora: 'sudo dnf install python3-venv'," >&2
        echo "Debian/Ubuntu: 'sudo apt install python3-venv') and retry." >&2
        exit 1
    fi
    "$venv_dir/bin/pip" install --quiet --upgrade "prettytable==$prettytable_version"
fi

exec "$venv_python" "$root_dir/scripts/summary.py" "$@"
