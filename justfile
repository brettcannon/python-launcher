#!/usr/bin/env just --justfile

ROOT := justfile_directory()
MAN_MD := join(ROOT, "man-page", "py.1.md")
MAN_FILE := join(ROOT, "man-page", "py.1")
CARGO_TOML := join(ROOT, "Cargo.toml")
VENV := join(ROOT, ".venv")

# Set default recipes
_default: lint test man docs

# Run the unit tests
test:
    cargo --quiet test

# Run linting on source files
lint:
    cargo fmt --quiet --all -- --check
    cargo clippy --quiet --all-targets --all-features -- -D warnings

# Install from source
install:
    cargo install --quiet --path .

# Convert the markdown-formatted man page to the man file format
_man-md:
    pandoc {{ MAN_MD }} --standalone -t man -o {{ MAN_FILE }}

# Build the man page (requires Python >= 3.11)
man: _man-md
    #!/usr/bin/env python3

    import datetime
    import pathlib
    import re
    import tomllib

    VERSION_REGEX = re.compile(r'version\s*=\s*"(?P<version>[\d.]+)"')

    with open("{{ CARGO_TOML }}", "rb") as file:
        cargo_data = tomllib.load(file)

    try:
        version = cargo_data["package"]["version"]
    except KeyError as exc:
        raise ValueError("'version' not found in {{ CARGO_TOML }}") from exc

    with open("{{ MAN_FILE }}", "r", encoding="utf-8") as file:
        man_text = file.read()

    man_text_with_version = man_text.replace("LAUNCHER_VERSION", version)
    new_man_text = man_text_with_version.replace(
        "CURRENT_DATE", datetime.date.today().isoformat()
    )

    with open("{{ MAN_FILE }}", "w", encoding="utf-8") as file:
        file.write(new_man_text)

# Create a virtual environment for building the docs
docs-venv:
    cargo run -- -m venv {{ VENV }}
    cargo run -- -m pip install --quiet --disable-pip-version-check -r docs/requirements.txt

# Launch the documentation dev server
docs-dev: docs-venv
    cargo run -- -m mkdocs serve

# Build the documentation
docs: docs-venv
    cargo run -- -m mkdocs build
