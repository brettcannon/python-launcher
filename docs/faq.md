# Frequently Asked Questions

## How do I have [Starship](https://starship.rs/) use the Python Launcher to display the Python version?

Add the following to your [Starship configuration file](https://starship.rs/config/):

```TOML
[python]
python_binary = ["py"]
# The following isn't necessary, but convenient.
detect_folders = [".venv"]
```

This will then have your prompt list the Python version that will be used if you run `py`:

```console
/tmp/starship-demo via 🐍 v3.11.0
❯ py -3.9 -m venv .venv

/tmp/starship-demo via 🐍 v3.9.15
❯
```

## How do I get a table of Python executables in [Nushell](https://www.nushell.sh/)?

```console
py --list | lines | split column "│" version path | str trim
```

Do note that the character that is being split on is **not** the traditional [U+007C/"Vertical Line"/pipe character](https://www.compart.com/en/unicode/U+007C) (`|`), but [U+2502/"Box Drawings Light Vertical"](https://www.compart.com/en/unicode/U+2502) (`│`).


## How can I make the Python Launcher use my default Python version from [pyenv](https://github.com/pyenv/pyenv)?

If you're using [pyenv](https://github.com/pyenv/pyenv) to manage your Python versions, you'll want to set the version the Launcher uses to the pyenv [global version](https://github.com/pyenv/pyenv/blob/master/COMMANDS.md#pyenv-global).


=== "bash/zsh"

    Add this line to your `.zshrc` or `.bashrc` file:

    ```console
    export PY_PYTHON=$(pyenv exec python -c "import sys; print('.'.join(map(str, sys.version_info[:2])))")
    ```

=== "fish"

    Add this line to your `~/.config/fish/config.fish` file:

    ```console
    set -gx PY_PYTHON (pyenv exec python -c "import sys; print('.'.join(map(str, sys.version_info[:2])))")
    ```

## How do I disable the automatic search/usage of the `.venv` virtual environment?

If you look at the [diagram of how the Launcher chooses what Python interpreter/environment to use](index.md#diagram-of-how-the-python-launcher-selects-a-python-interpreter), you will notice that the `.venv` virtual environment is only selected if you **don't** specify a verison restriction, e.g. `py -3`.

The thinking behind this is that if you want a specific Python version then you aren't interested in a specific virtual environment, and so the search for `.venv` is skipped.
