# Contribution Guide

## Testing and linting

See the
[main GitHub Action workflow](https://github.com/brettcannon/python-launcher/blob/main/.github/workflows/main.yml)
on how to run the various linters and tests that are used to maintain this
project (not repeated here as this is the first place that information will
go stale 😉).

### Running the Python tests

Do note that the Python tests that exercise the CLI require a debug build of the
project to already exist. This happens as a side-effect of `cargo test`, so
typically there is no need to do a debug build explicitly.

If ever get an error from pytest about stale cache files, run `cargo clean` and
then run `cargo build` to get the debug build of the `py` command built again.

## Installing a local copy

```bash
$ cargo install --path .
```

This does require that the appropriate directory Cargo is on your `$PATH` in
order to use the command (typically `~/.cargo/bin`).
