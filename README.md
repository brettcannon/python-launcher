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

### How do I install the Launcher?

You can either install from [crates.io](https://crates.io/) or from source.
Both approaches require you install the Rust toolchain. You can use
[rustup](https://rustup.rs/) to accomplish this or whatever your OS suggests.
Do note that if the compilation fails then your version of Rust is probably too
old; while the project is always compatible with the stable version of Rust, it
can update as quickly as the day of a new stable release of Rust.

#### From crates.io

If you want to
[install from crates.io](https://crates.io/crates/python-launcher), run:

```shell
cargo install python-launcher
```

#### From source

If you want to install from source, you can either use `cargo` directly:

```shell
cargo install --path .
```

Or you can use `doit` to do the install only if source code as changed since
the last time you used the `install` command:

```shell
doit install
```

### How do I get shell completions for [fish](https://fishshell.com/)?

The [`completions/py.fish` file in the repository](https://github.com/brettcannon/python-launcher/blob/main/completions/py.fish)
provides completions for the Launcher. Beyond the statically-known completions
(e.g. `--list`), the completions are also system-specific by providing version
completions tied to the running shell (e.g. `-3.9` is only a completion if
Python 3.9 is installed and will list the path to the Python executable that
would be used). Completions for `python` itself are also included
(although they are generic to Python itself, so all options may not be valid
for the version of Python you will be launching).

<img width="537" alt="fish_completions" src="https://user-images.githubusercontent.com/54418/113020397-6a71be00-9137-11eb-9047-2df1022592fa.png">

See [fish's documentation on where to put completions](https://fishshell.com/docs/current/completions.html#where-to-put-completions)
to know where the file should be copied/symlinked.

### How do I have [Starship](https://starship.rs/) use the Launcher to display the Python version?

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

## TODO

[Issues to finish to reach MVP](https://github.com/brettcannon/python-launcher/milestone/1)

## Appendix

- [PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/)
- [PEP 486: Make the Python Launcher aware of virtual environments](https://www.python.org/dev/peps/pep-0486/)
- Windows Launcher
  - [Documentation](https://docs.python.org/3/using/windows.html#launcher)
  - [Source](https://github.com/python/cpython/blob/master/PC/launcher.c)
