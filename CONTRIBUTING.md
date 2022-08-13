# Contribution Guide

[![CI](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml/badge.svg?event=push)](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml)

The Python Launcher is _mostly_ run as a typical Rust project. The only
potential differences is the automation tool used (for convenience).

## Using `just` for automation

We use [just](https://github.com/casey/just) as an automation tool. It is similar to [make](https://www.gnu.org/software/make/)
but with a few nice features and fewer quirks.

It is available from a variety of package managers and other sources, see the [installation docs](https://github.com/casey/just#installation)
for how to get it for your system.

## README

[Cog](https://pypi.org/project/cogapp/) is used to help maintain the README.

# Releasing

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
      [version upgrade](https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md#to-submit-a-version-upgrade-for-the-foo-formula) pull request
      with the
      [appropriate commit message](https://docs.brew.sh/Formula-Cookbook#commit)
