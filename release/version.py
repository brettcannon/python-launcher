import pathlib

import toml

CARGO_FILE = pathlib.Path(__file__).parent.parent / "Cargo.toml"


def get_version():
    """Read the version from Cargo.toml."""
    with CARGO_FILE.open("r", encoding="utf-8") as file:
        cargo_toml = toml.load(file)

    return cargo_toml["package"]["version"]


def main(args):
    """Print the version of the project."""
    version = get_version()

    if args.tag:
        print("v", end="")
    print(version)
