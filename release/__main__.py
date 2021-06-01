import argparse

from . import version

if __name__ == "__main__":
    arg_parser = argparse.ArgumentParser()
    arg_subparsers = arg_parser.add_subparsers()
    version_parser = arg_subparsers.add_parser("version")
    version_parser.add_argument("--tag", action="store_true")
    version_parser.set_defaults(func=version.main)

    args = arg_parser.parse_args()
    args.func(args)
