use std::error::Error;

use crate::base::Warn;

mod sealed {
    pub trait Option {}
    pub trait Result<T, E> {}

    impl<T> Option for std::option::Option<T> {}
    impl<T, E> Result<T, E> for std::result::Result<T, E> {}
}

/// Integration between [`Option`] and [`Warn`]
///
/// Note that this trait is sealed, so it is implemented for [`Option<T>`] and cannot
/// be implemented for anything else.
pub trait OptionExt: sealed::Option {
    /// Yield error `error` into sink `warn` if the option contains `None`. The option
    /// is returned unchanged.
    fn or_warn_with<E: Error>(self, error: E, warn: &mut impl Warn<E>) -> Self;
}

/// Integration between [`Result`] and [`Warn`]
///
/// Note that this trait is sealed, so it is implemented for [`Result<T, E>`] and cannot
/// be implemented for anything else.
pub trait ResultExt<T, E: Error>: sealed::Result<T, E> {
    /// Pass the error from result to the sink
    ///
    /// If the result contains a value, then this value is returned. Otherwise, the error
    /// is passed to `warn`, and `None` is returned. The error also can be converted to a
    /// corresponding type via [`From`] trait before passing into `warn`.
    fn or_warn<D: From<E> + Error>(self, warn: &mut impl Warn<D>) -> Option<T>;

    /// Same as [`ResultExt::or_warn()`], but `func` is applied to error before passing
    /// into `warn`
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
