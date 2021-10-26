# Contribution Guide

[![CI](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml/badge.svg?event=push)](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml)
[![Security check](https://github.com/brettcannon/python-launcher/actions/workflows/security-check.yml/badge.svg)](https://github.com/brettcannon/python-launcher/actions/workflows/security-check.yml)
[![codecov](https://codecov.io/gh/brettcannon/python-launcher/branch/master/graph/badge.svg?token=s2ZuXJQPPd)](https://codecov.io/gh/brettcannon/python-launcher)

The Python Launcher is _mostly_ run as a typical Rust project. The only
potential differences is the automation tool used (for convenience).

## Using just for automation

We use [just](https://github.com/casey/just) as an automation tool. It is similar to [make](https://www.gnu.org/software/make/)
but with a few nice features and fewer quirks.

It is available from a variety of package managers and other sources, see the [install](https://github.com/casey/just#installation) docs
for how to get it for your system.

## Changelog

The tool used to maintain the changelog is
[scriv](https://scriv.readthedocs.io). See the
[`changelog.d` directory](https://github.com/brettcannon/python-launcher/tree/main/changelog.d)
for details.

## Releasing

1. Adjust the version number in [`Cargo.toml`](https://github.com/brettcannon/python-launcher/blob/main/Cargo.toml) (previous [releases](https://github.com/brettcannon/python-launcher/releases)).
1. Check that the relevant [action workflows](https://github.com/brettcannon/python-launcher/actions) are passing.
1. Run the [`release` pipeline](https://github.com/brettcannon/python-launcher/actions/workflows/release.yml).
1. [Build for Apple Silicon](https://github.com/brettcannon/python-launcher/issues/106).
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
