# quickdna

Quickdna is a simple, fast library for working with DNA sequences. It is up to 100x faster than Biopython for some
translation tasks, in part because it uses a native Rust module (via PyO3) for the translation. However, it exposes
an easy-to-use, type-annotated API that should still feel familiar for Biopython users.

## Development

Quickdna uses `PyO3` and `maturin` to build and upload the wheels, and `poetry` for handling dependencies. This is handled via
a `Justfile`, which requires [Just](https://github.com/casey/just), a command-runner similar to `make`.

### Poetry

You can install poetry from https://python-poetry.org, and it will handle the other python dependencies.

### Just

You can install `Just` with `cargo install just`, and then run it in the project directory to get a list of commands.

### Flamegraphs

The `just profile` command requires [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph), please see that repository for installation instructions.
