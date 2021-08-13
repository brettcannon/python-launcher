# The Python Launcher for Unix

Launch your Python interpreter the lazy/smart way!

This project is an implementation of the `py` command for Unix-based platforms
(with some potential experimentation for good measure 😉).

The goal is to have `py` become the cross-platform command that Python users
typically use to launch an interpreter while doing development. By having a
command that is version-agnostic when it comes to Python, it side-steps
the "what should the `python` command point to?" debate by clearly specifying
that upfront (i.e. the newest version of Python that can be found). This also
unifies the suggested command to document for launching Python on both Windows
as Unix as `py` has existed as the preferred
[command on Windows](https://docs.python.org/3/using/windows.html#launcher)
since 2012 with the release of Python 3.3.

Typical usage would be:

```
py -m venv .venv
py ...  # Normal `python` usage.
```

This creates a virtual environment in a `.venv` directory using the latest
version of Python installed. Subsequent uses of `py` will then use that virtual
environment as long as it is in the current (or higher) directory; no
environment activation required (although the Python Launcher supports activated
environments as well)!

A non-goal of this project is to become the way to launch the Python
interpreter _all the time_. If you know the exact interpreter you want to launch
then you should launch it directly; same goes for when you have
requirements on the type of interpreter you want (e.g. 32-bit, framework build
on macOS, etc.). The Python Launcher should be viewed as a tool of convenience,
not necessity.

## Installation

### Linux

#### [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux)

```
brew install python-launcher
```

https://formulae.brew.sh/formula/python-launcher

#### [Arch](https://archlinux.org/)

```
yay -S python-launcher
```

https://aur.archlinux.org/packages/python-launcher

#### RISC-V

1. Go to the
   [releases page](https://github.com/brettcannon/python-launcher/releases).
2. Download the `riscv64gc-unknown-linux-gnu.tar.xz` tarball.
3. Run the following command to install into, e.g.
   `/usr/local`, substituting `<tarball>` with the path to the downloaded file:

```
tar --extract --strip-components 1 --directory /usr/local --file <tarball>
```

#### AArch64

1. Go to the
   [releases page](https://github.com/brettcannon/python-launcher/releases).
2. Download the `aarch64-unknown-linux-gnu.tar.xz` tarball.
3. Run the following command to install into, e.g.
   `/usr/local`, substituting `<tarball>` with the path to the downloaded file:

```
tar --extract --strip-components 1 --directory /usr/local --file <tarball>
```

#### x86-64

1. Go to the
   [releases page](https://github.com/brettcannon/python-launcher/releases).
2. Download the `x86_64-unknown-linux-gnu.tar.xz` tarball.
3. Run the following command to install into, e.g.
   `/usr/local`, substituting `<tarball>` with the path to the downloaded file:

```
tar --extract --strip-components 1 --directory /usr/local --file <tarball>
```

### macOS

#### [Homebrew](https://brew.sh/)

```
brew install python-launcher
```

https://formulae.brew.sh/formula/python-launcher

#### Apple Silicon

1. Go to the
   [releases page](https://github.com/brettcannon/python-launcher/releases).
2. Download the `aarch64-apple-darwin.tar.xz` tarball.
3. Run the following command to install into, e.g.
   `/usr/local`, substituting `<tarball>` with the path to the downloaded file:

```
tar --extract --strip-components 1 --directory /usr/local --file <tarball>
```

#### x86-64

1. Go to the
   [releases page](https://github.com/brettcannon/python-launcher/releases).
2. Download the `x86_64-apple-darwin.tar.xz` tarball.
3. Run the following command to install into, e.g.
   `/usr/local`, substituting `<tarball>` with the path to the downloaded file:

```
tar --extract --strip-components 1 --directory /usr/local --file <tarball>
```

### NetBSD

#### x86-64

1. Go to the
   [releases page](https://github.com/brettcannon/python-launcher/releases).
2. Download the `x86_64-unknown-netbsd.tar.xz` tarball.
3. Run the following command to install into, e.g.
   `/usr/local`, substituting `<tarball>` with the path to the downloaded file:

```
tar --extract --strip-components 1 --directory /usr/local --file <tarball>
```

### Any OS supporting Rust

#### [crates.io](https://crates.io/)

```
cargo install python-launcher
```

https://crates.io/crates/python-launcher

#### [Source](https://github.com/brettcannon/python-launcher.git)

```
cargo install --path .
```

https://github.com/brettcannon/python-launcher

## Documentation

The general control flow for finding the appropriate Python executable is the
following (with Python 3.6, Python 3, and the newest version of Python installed
as examples):

<img src="https://raw.githubusercontent.com/brettcannon/python-launcher/main/docs/control-flow/control_flow.svg">

See the
[man page](https://github.com/brettcannon/python-launcher/blob/main/docs/man-page/py.1.md)
or the top section of `py --help` for more details.

See the [API docs](https://docs.rs/python-launcher/) for using this project to
build your own custom Launcher.

## FAQ

### How do I have [Starship](https://starship.rs/) use the Python Launcher to display the Python version?

Add the following to your [Starship configuration file](https://starship.rs/config/):

```TOML
[python]
python_binary = ["py"]
# The following isn't necessary, but convenient.
detect_folders = [".venv"]
```

<img width="630" alt="starship_prompt" src="https://user-images.githubusercontent.com/54418/113020490-807f7e80-9137-11eb-8cf6-69a953017e39.png">

By using the Launcher with Starship, your prompt will tell you which Python
version will be used if you run `py`. Since the Launcher supports virtual
environments, the prompt will properly reflect both what global install of
Python will be used, but also the local virtual environment.

### How do I get a table of Python executables in [Nushell](https://www.nushell.sh/)?

```sh
py --list | lines | split column "│" version path | str trim
```

Do note that the character that is being split on is **not** the traditional
[U+007C/"Vertical Line"/pipe character](https://www.compart.com/en/unicode/U+007C) (`|`),
but [U+2502/"Box Drawings Light Vertical"](https://www.compart.com/en/unicode/U+2502) (`│`).

### How can I make the Python Launcher use my default Python version from pyenv?

If you're using [pyenv](https://github.com/pyenv/pyenv) to manage your Python
versions, you'll want to grab the major and minor version from the first Python
version listed in your default
[pyenv version file](https://github.com/pyenv/pyenv#choosing-the-python-version).

You can add this line to your `.zshrc` or `bashrc` file:

```sh
export PY_PYTHON=$(head -n 1 $(pyenv root)/version | cut -d "." -f 1,2)
```

Or this line in your `~/.config/fish/config.fish` file:

```sh
set -gx PY_PYTHON (head -n 1 (pyenv root)/version | cut -d "." -f 1,2)
```

## Appendix

- [PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/)
- [PEP 486: Make the Python Launcher aware of virtual environments](https://www.python.org/dev/peps/pep-0486/)
- Python Launcher for Windows
  - [Documentation](https://docs.python.org/3/using/windows.html#launcher)
  - [Source](https://github.com/python/cpython/blob/master/PC/launcher.c)
