#!/usr/bin/env just --justfile
# Written for https://github.com/casey/just/tree/0.10.2 .

ROOT := justfile_directory()
DOCS := join(ROOT, "docs")
MAN_DIR := join(DOCS, "man-page")
MAN_MD := join(MAN_DIR, "py.1.md")
MAN_FILE := join(MAN_DIR, "py.1")
CARGO_TOML := join(ROOT, "Cargo.toml")
DOT_DIR := join(DOCS, "control-flow")
DOT_FILE := join(DOT_DIR, "control_flow.dot")
DOT_FILE_NO_STEM := without_extension(DOT_FILE)
DOT_SVG := DOT_FILE_NO_STEM + ".svg"
DOT_PNG := DOT_FILE_NO_STEM + ".png"

# TODO: `just` release after 0.10.2 will make `join` accept variadic parameters;
# would clean up the variables quite a bit.

# Set default recipes
_default: lint test man dot

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

# Build the man page
man: _man-md
    #!/usr/bin/env python3

    import datetime
    import pathlib
    import re

    VERSION_REGEX = re.compile(r'version\s*=\s*"(?P<version>[\d.]+)"')

    with open("{{ CARGO_TOML }}", "r", encoding="utf-8") as file:
        cargo_lines = file.readlines()

    for line in cargo_lines:
        if version_match := VERSION_REGEX.match(line):
            version = version_match.group("version")
            break
    else:
        raise ValueError("'version' not found in {{ CARGO_TOML }}")

    with open("{{ MAN_FILE }}", "r", encoding="utf-8") as file:
        man_text = file.read()

    man_text_with_version = man_text.replace("LAUNCHER_VERSION", version)
    new_man_text = man_text_with_version.replace(
        "CURRENT_DATE", datetime.date.today().isoformat()
    )

    with open("{{ MAN_FILE }}", "w", encoding="utf-8") as file:
        file.write(new_man_text)

# Build the control flow diagram as a SVG
dot_svg:
    dot -T "svg" -o {{ DOT_SVG }} {{ DOT_FILE }}

# Build the control flow diagram as a PNG
dot_png:
    dot -T "png" -o {{ DOT_PNG }} {{ DOT_FILE }}

# Build the control flow diagram
dot: dot_svg dot_png
