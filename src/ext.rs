use std::error::Error;

use crate::base::Warn;

mod sealed {
    pub trait Option {}
    pub trait Result<T, E> {}

    impl<T> Option for std::option::Option<T> {}
    impl<T, E> Result<T, E> for std::result::Result<T, E> {}
}

pub trait OptionExt: sealed::Option {
    fn or_warn_with<E: Error>(self, error: E, warn: &mut impl Warn<E>) -> Self;
}

pub trait ResultExt<T, E: Error>: sealed::Result<T, E> {
    fn or_warn<D: From<E> + Error>(self, warn: &mut impl Warn<D>) -> Option<T>;
    fn or_warn_map<D: Error>(self, func: impl FnOnce(E) -> D, warn: &mut impl Warn<D>)
        -> Option<T>;
}

impl<T> OptionExt for Option<T> {
    #[inline]
    fn or_warn_with<E: Error>(self, error: E, warn: &mut impl Warn<E>) -> Self {
        if self.is_none() {
            warn.warn(error);
        }
        self
    }
}

impl<T, E: Error> ResultExt<T, E> for Result<T, E> {
    #[inline]
    fn or_warn<D: From<E> + Error>(self, warn: &mut impl Warn<D>) -> Option<T> {
        self.or_warn_map(From::from, warn)
    }

    #[inline]
    fn or_warn_map<D: Error>(
        self,
        func: impl FnOnce(E) -> D,
        warn: &mut impl Warn<D>,
    ) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(err) => {
                warn.warn(func(err));
                None
            }
        }
    }
}
