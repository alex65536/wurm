mod base;
mod ext;

pub mod sink;

pub use base::{Warn, WarnExt, Adapt, AdaptMap};
pub use ext::{OptionExt, ResultExt};
pub use sink::{Ignore, Stderr, All};

pub mod prelude {
    pub use crate::{Warn, WarnExt, OptionExt, ResultExt};
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
        let mut warn = All::default();
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
        let mut warn = All::default();
        outer(&mut warn);
        let res = vec![
            ErrSecond(ErrFirst { value: 1 }),
            ErrSecond(ErrFirst { value: 2 }),
        ];
        assert_eq!(warn.0, res);
    }

    #[test]
    fn test_exts() {
        let mut warn = All::default();
        let value = Some(42);
        assert_eq!(value.or_warn_with(ErrSecond(ErrFirst {value: 1}), &mut warn), Some(42));
        assert_eq!(warn.0.len(), 0);

        let value: Option<isize> = None;
        assert_eq!(value.or_warn_with(ErrSecond(ErrFirst {value: 1}), &mut warn), None);
        assert_eq!(warn.0.len(), 1);

        let value: Result<_, ErrSecond> = Ok(42);
        assert_eq!(value.or_warn(&mut warn), Some(42));
        assert_eq!(warn.0.len(), 1);

        let value: Result<usize, _> = Err(ErrFirst {value: 2});
        assert_eq!(value.or_warn(&mut warn), None);
        assert_eq!(warn.0.len(), 2);
    }
}
