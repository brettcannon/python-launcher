import datetime
import glob
import os
import pathlib
import re
import shutil
import venv


DOIT_CONFIG = {
    "backend": "sqlite3",
    "default_tasks": ["lint", "test", "man_page", "control_flow"],
}

DOCS = pathlib.Path("docs")

RUST_FILES = glob.glob("**/*.rs", recursive=True)
DEBUG_BINARY = pathlib.Path("target") / "debug" / "py"


def task_man_page():
    """Generate the man page"""
    man_dir = DOCS / "man-page"
    md_file = man_dir / "py.1.md"
    man_file = man_dir / "py.1"

    def update_man(md_file, man_file, cargo_file):
        with open(cargo_file, "r", encoding="UTF-8") as file:
            cargo_lines = file.readlines()
        for line in cargo_lines:
            version_match = re.match(r'version\s*=\s*"(?P<version>[\d.]+)"', line)
            if version_match:
                version = version_match.group("version")
                break
        else:
            raise ValueError(f"'version' not found in {cargo_file}")

        with open(man_file, "r", encoding="UTF-8") as file:
            man_text = file.read()

        man_text_with_version = man_text.replace("LAUNCHER_VERSION", version)
        new_man_text = man_text_with_version.replace(
            "CURRENT_DATE", datetime.date.today().isoformat()
        )

        with open(man_file, "w", encoding="UTF-8") as file:
            file.write(new_man_text)

    return {
        "actions": [
            f"pandoc {os.fsdecode(md_file)} --standalone -t man -o {os.fsdecode(man_file)}",
            (update_man, (md_file, man_file, pathlib.Path("Cargo.toml")), {}),
        ],
        "file_dep": [md_file],
        "targets": [man_file],
    }


def task_control_flow():
    dot_file = DOCS / "control-flow" / "control_flow.dot"
    for file_type in ["svg", "png"]:
        output_file = dot_file.with_suffix("." + file_type)
        yield {
            "name": file_type,
            "actions": [
                f"dot -T {file_type} -o {os.fsdecode(output_file)} {os.fsdecode(dot_file)}"
            ],
            "file_dep": [dot_file],
            "targets": [output_file],
        }


def task_lint():
    return {
        "actions": [
            "cargo fmt --quiet --all -- --check",
            "cargo clippy --quiet --all-targets --all-features -- -D warnings",
        ],
        "file_dep": RUST_FILES,
    }


def task_test():
    return {
        "actions": ["cargo --quiet test"],
        "file_dep": ["Cargo.lock"] + RUST_FILES,
        "targets": [DEBUG_BINARY],
    }


def task_install():
    """Install from source"""
    return {
        "actions": ["cargo install --quiet --path ."],
        "file_dep": ["Cargo.lock"] + glob.glob("src/**/**.rs", recursive=True),
        "targets": [pathlib.Path.home() / ".cargo" / "bin" / "py"],
    }
