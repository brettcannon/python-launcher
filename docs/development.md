# Development

## Code

The Python Launcher is _mostly_ run as a typical Rust project. The only
potential differences is the automation tool used (for convenience).


We use [just](https://github.com/casey/just) as a task runner. Some rules require Python >= 3.11 to be installed. Some rules will also use `py` itself via `cargo run`, so the source code needs to be working.

## Website

The website is built using [MkDocs](https://www.mkdocs.org/) and [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/).

While developing the website, you can run `just docs-dev` to start a local server that will automatically reload when you make changes. This will create a virtual environment in `.venv` and install the necessary dependencies.

To build the docs, use `just docs`. This will create a virtual environment like `just docs-dev`.

## Releasing

1. Adjust the version number in [`Cargo.toml`](https://github.com/brettcannon/python-launcher/blob/main/Cargo.toml) (previous [releases](https://github.com/brettcannon/python-launcher/releases)).
1. Check that the relevant [action workflows](https://github.com/brettcannon/python-launcher/actions) are passing.
1. Run the [`release` pipeline](https://github.com/brettcannon/python-launcher/actions/workflows/release.yml).
1. Publish the [release](https://github.com/brettcannon/python-launcher/releases).
1. Update the
   [Homebrew formula](https://github.com/Homebrew/homebrew-core/blob/master/Formula/python-launcher.rb)
   1. Get the URL to the
      [release](https://github.com/brettcannon/python-launcher/releases) tarball
   1. `curl --location <URL to tarball> | shasum --algorithm 256`
   1. Create a
      [version upgrade](https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md#to-submit-a-version-upgrade-for-the-foo-formula) pull request with the [appropriate commit message](https://docs.brew.sh/Formula-Cookbook#commit)

## Useful links

- [Repository](https://github.com/brettcannon/mousebender/)
- [crates.io page](https://crates.io/crates/python-launcher)
- [API docs](https://docs.rs/python-launcher/)

## Appendix

### PEPs

- [PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/)
- [PEP 486: Make the Python Launcher aware of virtual environments](https://www.python.org/dev/peps/pep-0486/)

### Python Launcher for Windows

- [Documentation](https://docs.python.org/3/using/windows.html#launcher)
- [Source](https://github.com/python/cpython/blob/master/PC/launcher.c)
- [Experimental source](https://github.com/python/cpython/blob/main/PC/launcher2.c)
