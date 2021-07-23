# Contribution Guide

[![CI](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml/badge.svg?event=push)](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml)
[![Security check](https://github.com/brettcannon/python-launcher/actions/workflows/security-check.yml/badge.svg)](https://github.com/brettcannon/python-launcher/actions/workflows/security-check.yml)
[![codecov](https://codecov.io/gh/brettcannon/python-launcher/branch/master/graph/badge.svg?token=s2ZuXJQPPd)](https://codecov.io/gh/brettcannon/python-launcher)

The Python Launcher is _mostly_ run as a typical Rust project. The only
potential differences is the automation tool used (for convenience).

## Using doit for automation

We use [doit](https://pydoit.org/) as an automation tool. It's
[available on PyPI](https://pypi.org/project/doit/) and may be available in your
preferred package manager (e.g. `apt`).
