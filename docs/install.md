# Installation

<!-- [[[cog

import cog

download_template = """
1. [Download `python_launcher-{version}-{platform}.tar.xz`](https://github.com/brettcannon/python-launcher/releases/download/{tag}/python_launcher-{version}-{platform}.tar.xz):

```
curl --location --remote-name https://github.com/brettcannon/python-launcher/releases/download/{tag}/python_launcher-{version}-{platform}.tar.xz
```

2. Install into, e.g. `/usr/local`:

```
tar --extract --strip-components 1 --directory /usr/local --file python_launcher-{version}-{platform}.tar.xz
```
"""

def platform_download(platform):
    instructions = download_template.format(tag=TAG, version=VERSION,
                                            platform=platform)
    cog.outl(instructions)

]]] -->

<!-- [[[end]]] -->

There are three ways to install the Python Launcher for Unix:

1. A supported package manager
2. A pre-built binary available from the project's [releases page](https://github.com/brettcannon/python-launcher/releases)
3. From source

Which option is available and best for you will depend on your operating system and your own personal preferences.

## Linux

### [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux)

```console
brew install python-launcher
```

- https://formulae.brew.sh/formula/python-launcher

### [Arch](https://archlinux.org/)

```console
yay -S python-launcher
```

- https://aur.archlinux.org/packages/python-launcher

### [Fedora](https://getfedora.org/)

```console
sudo dnf install python-launcher
```

Requires Fedora 34 or higher.

- https://src.fedoraproject.org/rpms/rust-python-launcher/

### Pre-built binaries

#### RISC-V
<!-- [[[cog
platform_download("riscv64gc-unknown-linux-gnu")
]]] -->
1. [Download `python_launcher-1.0.0-riscv64gc-unknown-linux-gnu.tar.xz`](https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-riscv64gc-unknown-linux-gnu.tar.xz):

```console
curl --location --remote-name https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-riscv64gc-unknown-linux-gnu.tar.xz
```

2. Install into, e.g. `/usr/local`:

```console
tar --extract --strip-components 1 --directory /usr/local --file python_launcher-1.0.0-riscv64gc-unknown-linux-gnu.tar.xz
```
<!-- [[[end]]] -->

#### AArch64
<!-- [[[cog
platform_download("aarch64-unknown-linux-gnu")
]]] -->
1. [Download `python_launcher-1.0.0-aarch64-unknown-linux-gnu.tar.xz`](https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-aarch64-unknown-linux-gnu.tar.xz):

```console
curl --location --remote-name https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-aarch64-unknown-linux-gnu.tar.xz
```

2. Install into, e.g. `/usr/local`:

```console
tar --extract --strip-components 1 --directory /usr/local --file python_launcher-1.0.0-aarch64-unknown-linux-gnu.tar.xz
```
<!-- [[[end]]] -->

#### x86-64
<!-- [[[cog
platform_download("x86_64-unknown-linux-gnu")
]]] -->
1. [Download `python_launcher-1.0.0-x86_64-unknown-linux-gnu.tar.xz`](https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-x86_64-unknown-linux-gnu.tar.xz):

```console
curl --location --remote-name https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-x86_64-unknown-linux-gnu.tar.xz
```

2. Install into, e.g. `/usr/local`:

```console
tar --extract --strip-components 1 --directory /usr/local --file python_launcher-1.0.0-x86_64-unknown-linux-gnu.tar.xz
```
<!-- [[[end]]] -->

## macOS

### [Homebrew](https://brew.sh/)

```console
brew install python-launcher
```

- https://formulae.brew.sh/formula/python-launcher

!!! note

    If you have multiple installs of Python via [Homebrew](https://brew.sh/) but
    they are not all being found (as verified via `py --list`), chances are Homebrew
    didn't symlink an installation due to the `python` symlink already being
    defined. For each installation you are missing you will need to tell Homebrew to
    ignore the conflict so that the version-specific `python` symlink gets created.

    For instance, if your Python 3.10 installation isn't being found (due to
    `python3.10` not existing), try running:

    ```console
    brew link --overwrite python@3.10
    ```

    That will symlink the `python3.10` command. It will also overwrite
    what `python3` points at, so you may want to run the `brew link` command for the
    newest version of Python you have installed last.


### Pre-built binaries

#### Apple Silicon
<!-- [[[cog
platform_download("aarch64-apple-darwin")
]]] -->
1. [Download `python_launcher-1.0.0-aarch64-apple-darwin.tar.xz`](https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-aarch64-apple-darwin.tar.xz):

```
curl --location --remote-name https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-aarch64-apple-darwin.tar.xz
```

2. Install into, e.g. `/usr/local`:

```
tar --extract --strip-components 1 --directory /usr/local --file python_launcher-1.0.0-aarch64-apple-darwin.tar.xz
```
<!-- [[[end]]] -->

#### x86-64
<!-- [[[cog
platform_download("x86_64-apple-darwin")
]]] -->
1. [Download `python_launcher-1.0.0-x86_64-apple-darwin.tar.xz`](https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-x86_64-apple-darwin.tar.xz):

```
curl --location --remote-name https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-x86_64-apple-darwin.tar.xz
```

2. Install into, e.g. `/usr/local`:

```
tar --extract --strip-components 1 --directory /usr/local --file python_launcher-1.0.0-x86_64-apple-darwin.tar.xz
```
<!-- [[[end]]] -->

## NetBSD (x86-64 binary)
<!-- [[[cog
platform_download("x86_64-unknown-netbsd")
]]] -->
1. [Download `python_launcher-1.0.0-x86_64-unknown-netbsd.tar.xz`](https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-x86_64-unknown-netbsd.tar.xz):

```
curl --location --remote-name https://github.com/brettcannon/python-launcher/releases/download/v1.0.0/python_launcher-1.0.0-x86_64-unknown-netbsd.tar.xz
```

2. Install into, e.g. `/usr/local`:

```
tar --extract --strip-components 1 --directory /usr/local --file python_launcher-1.0.0-x86_64-unknown-netbsd.tar.xz
```
<!-- [[[end]]] -->

## From source

### [Crates.io](https://crates.io)

```console
cargo install python-launcher
```

- https://crates.io/crates/python-launcher

### Repository checkout

```console
cargo install --path .
```

- https://github.com/brettcannon/python-launcher.git
