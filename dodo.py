import datetime
import glob
import os
import pathlib
import re
import shutil
import venv


DOIT_CONFIG = {"backend": "sqlite3", "default_tasks": ["venv", "lint", "test", "man"]}

VENV_DIR = pathlib.Path(".venv")
VENV_EXECUTABLE = VENV_DIR / "bin" / "python"

RUST_FILES = glob.glob("**/*.rs", recursive=True)
DEBUG_BINARY = pathlib.Path("target") / "debug" / "py"


def task_man():
    """Generate the man page"""
    man_dir = pathlib.Path("man")
    md_file = man_dir / "py.1.md"
    man_file = man_dir / "py.1"

    def update_man(md_file, man_file, cargo_file):
        with open(cargo_file, "r", encoding="UTF-8") as file:
            cargo_lines = file.readlines()
        for line in cargo_lines:
            if version_match := re.match(r'version\s*=\s*"(?P<version>[\d.]+)"', line):
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
            f"pandoc {os.fspath(md_file)} --standalone -t man -o {os.fspath(man_file)}",
            (update_man, (md_file, man_file, pathlib.Path("Cargo.toml")), {}),
        ],
        "file_dep": [md_file],
        "targets": [man_file],
    }


def task_venv():
    """Create a virtual environment for tests"""

    return {
        "actions": [
            (venv.create, (VENV_DIR,), {"with_pip": True}),
            f"{os.fspath(VENV_EXECUTABLE)} -m pip --quiet --disable-pip-version-check install -r dev-requirements.txt",
        ],
        "file_dep": ["dev-requirements.txt"],
        "targets": [".venv"],
        "clean": [(shutil.rmtree, (VENV_DIR,))],
    }


def task_lint_python():
    """Lint Python code"""
    return {
        "actions": [f"{os.fspath(VENV_EXECUTABLE)} -m black --quiet --check ."],
        "file_dep": glob.glob("**/*.py", recursive=True),
        "task_dep": ["venv"],
    }


def task_lint_rust():
    """Lint Rust code"""
    return {
        "actions": ["cargo fmt --quiet --all -- --check"],
        "file_dep": RUST_FILES,
    }


def task_lint():
    """Lint code"""
    return {"actions": None, "task_dep": ["lint_rust", "lint_python"]}


def task_tests_rust():
    """Test code using Rust"""
    return {
        "actions": ["cargo --quiet test"],
        "file_dep": ["Cargo.lock"] + RUST_FILES,
        "targets": [DEBUG_BINARY],
    }


def task_tests_python():
    """Test code using Python"""
    return {
        "actions": [f"{os.fspath(VENV_EXECUTABLE)} -m pytest --quiet tests"],
        "file_dep": [DEBUG_BINARY] + glob.glob("tests/**/*.py", recursive=True),
        "task_dep": ["venv"],
    }


def task_test():
    """Run all tests"""
    return {"actions": None, "task_dep": ["tests_rust", "tests_python"]}


def task_install():
    """Install from source"""
    return {
        "actions": ["cargo install --quiet --path ."],
        "file_dep": ["Cargo.lock"] + glob.glob("src/**/**.rs", recursive=True),
        "targets": [pathlib.Path.home() / ".cargo" / "bin" / "py"],
    }
