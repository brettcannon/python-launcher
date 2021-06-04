# The Python Launcher for Unix

An implementation of the `py` command for Unix-based platforms
(with some potential experimentation for good measure 😉)

The goal is to have `py` become the cross-platform command that Python users
typically use to launch an interpreter. By having a command that is
version-agnostic command when it comes to Python, it side-steps the "what should
the `python` command point to?" debate by clearly specifying that upfront (i.e.
the newest version of Python that can be found). This also unifies the suggested
command to document for launching Python on both Windows as Unix as `py` has
existed as the preferred
[command on Windows](https://docs.python.org/3/using/windows.html#launcher)
since 2012 with the release of Python 3.3.

A non-goal of this project is to become the way to launch the Python
interpreter _all the time_. If you know the exact interpreter you want to launch
then you should launch it directly; same goes for when you have
requirements on the type of interpreter you want (e.g. 32-bit, framework build
on macOS, etc.). The Python Launcher should be viewed as a tool of convenience,
not necessity.

## Installation

### Via `cargo`

If you have the latest stable
[release of Rust](https://www.rust-lang.org/tools/install) installed, then you
can install the [Python Launcher via crates.io](https://crates.io/crates/python-launcher):

```
cargo install python-launcher
```

If you get a compilation error then it's very likely you don't have the latest
stable release of Rust as there is a
[release every 6 weeks](https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md)
and this project tracks Rust's stable channel closely.

### From a `.tar.xz` file

If you go to the
[releases page](https://github.com/brettcannon/python-launcher/releases) you will
find various `.tar.xz` files for each release that target various platforms. If
one is available for your platform then you can download the tarball and install
it into e.g. `/usr/local/` via:

```
tar --extract --strip-components 1 --directory /usr/local --file <tarball>
```

You can use `tar -t -f <tarball>` to see what files are included and where they
will be installed.

If you don't want to install the tarball then you can extract the tarball
and copy the files manually as desired; the `py` binary is self-contained and is
not dependent on any other files from the tarball.


### From [source](https://github.com/brettcannon/python-launcher/)

#### Using [`cargo`](https://doc.rust-lang.org/cargo/)

```
cargo install --path .
```

#### Using [`doit`](https://pydoit.org/)

[Doit](https://pydoit.org/) will only perform an installation if source code as
changed since the last time you used the `install` command:

```
doit install
```

## Documentation

The general control flow for finding the appropriate Python executable is the
following (with Python 3.6, Python 3, and the newest version of Python installed
as examples):

<img src="https://raw.githubusercontent.com/brettcannon/python-launcher/main/docs/control-flow/control_flow.svg">

See the top section of
`py --help` or the
[man page](https://github.com/brettcannon/python-launcher/blob/main/docs/man-page/py.1.md)
for more details.

## FAQ

### How do I have [Starship](https://starship.rs/) use the Python Launcher to display the Python version?

Add the following to your [Starship configuration file](https://fishshell.com/docs/current/completions.html#where-to-put-completions):

```TOML
[python]
python_binary = ["py"]
# The following isn't necessary, but convenient.
detect_folders = [".venv"]
```

By using the Launcher with Starship, your prompt will tell you which Python
version will be used if you run `py`. Since the Launcher supports virtual
environments, the prompt will properly reflect both what global install of
Python will be used, but also the local virtual environment.

<img width="630" alt="starship_prompt" src="https://user-images.githubusercontent.com/54418/113020490-807f7e80-9137-11eb-8cf6-69a953017e39.png">

### How do I get a table of Python executables in [Nushell](https://www.nushell.sh/)?

```sh
py --list | lines | split column "│" version executable | str trim
```

Do note that the character that is being split on is **not** the traditional
[U+007C/"Vertical Line"/pipe character](https://www.compart.com/en/unicode/U+007C) (`|`),
but [U+2502/"Box Drawings Light Vertical"](https://www.compart.com/en/unicode/U+2502) (`│`).

## Appendix

- [PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/)
- [PEP 486: Make the Python Launcher aware of virtual environments](https://www.python.org/dev/peps/pep-0486/)
- Python Launcher for Windows
  - [Documentation](https://docs.python.org/3/using/windows.html#launcher)
  - [Source](https://github.com/python/cpython/blob/master/PC/launcher.c)
