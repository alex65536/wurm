use std::error::Error;
use std::marker::PhantomData;

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

    fn recursive(n: usize, warn: &mut impl Sink<ErrFirst>) {
        if n == 0 {
            return;
        }
        recursive(n - 1, warn);
        warn.warn(ErrFirst { value: n });
        recursive(n - 1, warn);
    }

    #[test]
    fn test_recursive() {
        let mut sink = All::default();
        recursive(3, &mut sink);
        let res = vec![
            ErrFirst { value: 1 },
            ErrFirst { value: 2 },
            ErrFirst { value: 1 },
            ErrFirst { value: 3 },
            ErrFirst { value: 1 },
            ErrFirst { value: 2 },
            ErrFirst { value: 1 },
        ];
        assert_eq!(sink.0, res);
    }

    fn inner(warn: &mut impl Sink<ErrFirst>) {
        warn.warn(ErrFirst { value: 1 });
    }

    fn outer(warn: &mut impl Sink<ErrSecond>) {
        inner(&mut warn.adapt());
        warn.warn(ErrSecond(ErrFirst { value: 2 }));
    }

    #[test]
    fn test_adapt() {
        let mut sink = All::default();
        outer(&mut sink);
        let res = vec![
            ErrSecond(ErrFirst { value: 1 }),
            ErrSecond(ErrFirst { value: 2 }),
        ];
        assert_eq!(sink.0, res);
    }
}
