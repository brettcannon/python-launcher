import argparse

from . import notes
from . import readme
from . import tarball
from . import version

if __name__ == "__main__":
    arg_parser = argparse.ArgumentParser()
    arg_subparsers = arg_parser.add_subparsers()
    version_parser = arg_subparsers.add_parser("version")
    version_parser.add_argument("--tag", action="store_true")
    version_parser.set_defaults(func=version.main)

    tarball_parser = arg_subparsers.add_parser("tarball")
    tarball_parser.add_argument("--target", required=True)
    tarball_parser.set_defaults(func=tarball.main)

    notes_parser = arg_subparsers.add_parser("notes")
    notes_parser.set_defaults(func=notes.main)

    readme_parser = arg_subparsers.add_parser("readme")
    readme_parser.add_argument("--version", required=True)
    readme_parser.add_argument("--tag", required=True)
    readme_parser.set_defaults(func=readme.main)

    args = arg_parser.parse_args()
    args.func(args)
