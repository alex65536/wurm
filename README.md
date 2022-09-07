# Würm: non-fatal, strongly typed errors

[![Crates.io: wurm](https://img.shields.io/crates/v/wurm.svg)](https://crates.io/crates/wurm)
[![Documentation](https://img.shields.io/docsrs/wurm/latest)](https://docs.rs/wurm)
[![Build](https://github.com/alex65536/wurm/actions/workflows/build.yml/badge.svg)](https://github.com/alex65536/wurm/actions/workflows/build.yml)

## Motivation

By default, errors in Rust are handled using `Result<T, E>`, which contains either a value
or an error. Bu sometimes, you need to return a value alongside with one or many errors. In this
case, you may need to use `wurm`.

Think of non-fatal errors as compiler warnings. The compiler will give you the result (i.e. the
compiled binary) even if there are tons of warnings. You also receive the warnings and can inspect
them to decide what to do.

As an alternative, you may just use a logger to yield such non-fatal errors, but you lose flexibility,
because your errors will be just strings sent into a logger, and inspecting them from code can be
problematic.

For motivating example and API documentation, go to [the docs](https://docs.rs/wurm).

## Why such name?

It's just modified word "warn" (as non-fatal errors can be also called warnings). So, "warn" → "würm" sound
pretty similar.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more details.
