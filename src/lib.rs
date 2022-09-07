//! # Non-fatal, strongly typed errors
//!
//! By default, errors in Rust are handled using [`Result<T, E>`], which contains either a value
//! or an error. But sometimes, you need to return a value alongside with one or many errors. In this
//! case, you may need to use `wurm`.
//!
//! Think of non-fatal errors as compiler warnings. The compiler will give you the result (i.e. the
//! compiled binary) even if there are tons of warnings. You also receive the warnings and can inspect
//! them to decide what to do.
//!
//! As an alternative, you may just use a logger to yield such non-fatal errors, but you lose flexibility,
//! because your errors will be just strings sent into a logger, and inspecting them from code can be
//! problematic.
//!
//! # Basic usage
//!
//! Suppose you have a function, to which you want to add non-fatal error support:
//!
//! ```no_test
//! fn foo(first: u32, second: u32) -> u32;
//! ```
//!
//! Then, just add one more argument `warn: &mut impl Warn<Error>` where `Error` is the desired error type.
//! By convention, it is recommended to add the extra arg to the end of the argument list:
//!
//! ```no_test
//! fn foo(first: u32, second: u32, warn: &mut impl Warn<Error>) -> u32;
//! ```
//!
//! Then, you can yield non-fatal errors via [`Warn::warn`] or convert them from regular ones via [`OptionExt`]
//! or [`ResultExt`]. See below for details.
//!
//! # Motivating example
//!
//! ```
//! use thiserror::Error;
//! use wurm::prelude::*;
//! use wurm::CollectAll;
//!
//! // First error type
//! #[derive(Debug, Error, PartialEq, Eq)]
//! #[error("first error")]
//! struct FooError;
//!
//! // Second error type, which is converible from `FooError`
//! #[derive(Debug, Error, PartialEq, Eq)]
//! #[error("second error: {0}")]
//! struct BarError(#[from] FooError);
//!
//! // Ordinary function, which can return a simple `Result`
//! fn simple_func() -> Result<u32, FooError> {
//!     Err(FooError)
//! }
//!
//! // Function which can yield non-fatal errors of type `FooError`
//! fn foo(arg: u32, warn: &mut impl Warn<FooError>) -> u32 {
//!     // Just yield a non-fatal error via `Warn::warn`
//!     warn.warn(FooError);
//!     // You can also use `ResultExt::or_warn()` to push the error from `Result`
//!     // into `warn`.
//!     //
//!     // Explicit type hint is added for extra clarity and is not needed actually.
//!     let opt: Option<u32> = simple_func().or_warn(warn);
//!     arg + opt.unwrap_or(1)
//! }
//!
//! // Function which can yield non-fatal errors of type `BarError`. It calls `foo()`
//! // internally and converts the errors from `FooError` to `BarError` via the adapter.
//! fn bar(first: u32, second: u32, warn: &mut impl Warn<BarError>) -> u32 {
//!     // We pass `&mut warn.adapt()` to convert between different error types.
//!     let x = foo(first, &mut warn.adapt());
//!     let y = foo(second, &mut warn.adapt());
//!     x * y
//! }
//!
//! // Create a sink for non-fatal errors.
//! //
//! // Explicit type hint is added for extra clarity and is not needed actually.
//! let mut warn: CollectAll<BarError> = CollectAll::default();
//!
//! // Call `bar()` with the created sink. It must yield four errors: two from each
//! // `foo()` subcall.
//! let value = bar(2, 3, &mut warn);
//! assert_eq!(value, 12);
//! assert_eq!(warn.0.len(), 4);
//! ```
mod base;
mod ext;

pub mod sink;

pub use base::{Adapt, AdaptMap, Warn, WarnExt};
pub use ext::{OptionExt, ResultExt};
#[allow(deprecated)]
pub use sink::{All, CollectAll, Ignore, Stderr};

/// The most important types to use
///
/// This prelude re-exports common, most frequently used types from the crate. It is intended to
/// reduce the amount of imports, glob-importing the prelude instead:
///
/// ```
/// use wurm::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{OptionExt, ResultExt, Warn, WarnExt};
}

#[cfg(test)]
mod tests {
    use super::*;

    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    #[error("first: {value}")]
    struct ErrFirst {
        value: usize,
    }

    #[derive(Debug, Error, Eq, PartialEq)]
    #[error("second: {0}")]
    struct ErrSecond(#[from] ErrFirst);

    fn recursive(n: usize, warn: &mut impl Warn<ErrFirst>) {
        if n == 0 {
            return;
        }
        recursive(n - 1, warn);
        warn.warn(ErrFirst { value: n });
        recursive(n - 1, warn);
    }

    #[test]
    fn test_recursive() {
        let mut warn = CollectAll::default();
        recursive(3, &mut warn);
        let res = vec![
            ErrFirst { value: 1 },
            ErrFirst { value: 2 },
            ErrFirst { value: 1 },
            ErrFirst { value: 3 },
            ErrFirst { value: 1 },
            ErrFirst { value: 2 },
            ErrFirst { value: 1 },
        ];
        assert_eq!(warn.0, res);
    }

    fn inner(warn: &mut impl Warn<ErrFirst>) {
        warn.warn(ErrFirst { value: 1 });
    }

    fn outer(warn: &mut impl Warn<ErrSecond>) {
        inner(&mut warn.adapt());
        warn.warn(ErrSecond(ErrFirst { value: 2 }));
    }

    #[test]
    fn test_adapt() {
        let mut warn = CollectAll::default();
        outer(&mut warn);
        let res = vec![
            ErrSecond(ErrFirst { value: 1 }),
            ErrSecond(ErrFirst { value: 2 }),
        ];
        assert_eq!(warn.0, res);
    }

    #[test]
    fn test_exts() {
        let mut warn = CollectAll::default();
        let value = Some(42);
        assert_eq!(
            value.or_warn_with(ErrSecond(ErrFirst { value: 1 }), &mut warn),
            Some(42)
        );
        assert_eq!(warn.0.len(), 0);

        let value: Option<isize> = None;
        assert_eq!(
            value.or_warn_with(ErrSecond(ErrFirst { value: 1 }), &mut warn),
            None
        );
        assert_eq!(warn.0.len(), 1);

        let value: Result<_, ErrSecond> = Ok(42);
        assert_eq!(value.or_warn(&mut warn), Some(42));
        assert_eq!(warn.0.len(), 1);

        let value: Result<usize, _> = Err(ErrFirst { value: 2 });
        assert_eq!(value.or_warn(&mut warn), None);
        assert_eq!(warn.0.len(), 2);
    }
}
