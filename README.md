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

For instructions on how to use the Python Launcher, see the top section of
`py --help`.

## Search order

Please note that while searching, the search for a Python version can become
more specific. This leads to a switch in the search algorithm to the one most
appropriate to the specificity of the version.

You can always run the Python Launcher with `PYLAUNCH_DEBUG` set to some value
to have it output logging details of how it is performing its search.

### `py -3.6` (specific version)

1. Search `PATH` for `python3.6`

### `py -3` (loose/major version)

1. Check for the `PY_PYTHON3` environment variable, and if defined
   and not the empty string then use it as the specific version
   (e.g. `PY_PYTHON3=3.6`)
1. Search `PATH` for all instances of `python3.*`
1. Find the executable with the newest version number that comes earliest on
   `PATH`

### `py` (any version)

1. Use an activated virtual environment immediately via `${VIRTUAL_ENV}/bin/python`
1. Use `.venv/bin/python` if available in the current working directory or any
   of its parent directories
1. If the first argument is a file path ...
   1. Check for a shebang
   1. If shebang path starts with `/usr/bin/python`, `/usr/local/bin/python`,
      `/usr/bin/env python` or `python`, proceed based on the version found
      on that path
      (bare `python` is considered the equivalent of not specifying a
      Python version)
1. Check for the `PY_PYTHON` environment variable, and if defined then use it
   as the loose or specific version (e.g. `PY_PYTHON=3` or `PY_PYTHON=3.6`)
1. Search `PATH` for all instances of `python*.*`
1. Find the executable with the newest version that is earliest on `PATH`

## FAQ

### How do I install the Launcher?

You can either install from [crates.io](https://crates.io/) or from source.
Both approaches require you install the Rust toolchain. You can use
[rustup](https://rustup.rs/) to accomplish this or whatever your OS suggests.
Do note that if the compliation fails then your version of Rust is probably too
old; while the project is always compatible with the stable version of Rust, it
can update as quickly as the day of a new stable version of Rust.

#### From crates.io

If you want to
[install from crates.io](https://crates.io/crates/python-launcher), run:

```shell
cargo install python-launcher
```

#### From source

If you want to install from source, run:

```shell
cargo install --path .
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
pip character (`|`), but U+2502/"Box Drawings Light Vertical" (`│`).

## TODO

[Issues to finish to reach MVP](https://github.com/brettcannon/python-launcher/milestone/1)

## Appendix

- [PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/)
- [PEP 486: Make the Python Launcher aware of virtual environments](https://www.python.org/dev/peps/pep-0486/)
- Windows Launcher
  - [Documentation](https://docs.python.org/3/using/windows.html#launcher)
  - [Source](https://github.com/python/cpython/blob/master/PC/launcher.c)
