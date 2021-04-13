import datetime
import os
import pathlib
import re
import venv


DOIT_CONFIG = {"backend": "sqlite3", "default_tasks": ["man", "venv"]}

# lint: lint-python; lint-rust
# test: cargo --quiet test; py -m pytest --quiet tests
# install (not a default target)


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
    venv_dir = pathlib.Path(".venv")
    venv_executable = venv_dir / "bin" / "python"

    return {
        "actions": [
            (venv.create, (venv_dir,), {"with_pip": True, "clear": True}),
            f"{os.fspath(venv_executable)} -m pip --quiet --disable-pip-version-check install -r dev-requirements.txt",
        ],
        "file_dep": ["dev-requirements.txt"],
        "targets": [".venv"],
        # "clean" not necessary thanks to `clear` argument to venv.create().
    }


def task_install():
    """Install from source"""
    return {"actions": ["cargo install --quiet --path ."]}
