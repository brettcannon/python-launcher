import pathlib

import toml

CARGO_FILE = pathlib.Path(__file__).parent / ".." / "Cargo.toml"


def main(args):
    """Print the version of the project."""
    with CARGO_FILE.open("r", encoding="utf-8") as file:
        cargo_toml = toml.load(file)

    version = cargo_toml["package"]["version"]

    if args.tag:
        print("v", end="")
    print(cargo_toml["package"]["version"])
