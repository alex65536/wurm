use std::{error::Error, marker::PhantomData};

/// Sink to which the non-fatal errors of type `E` can be written
pub trait Warn<E: Error> {
    /// Push the error `error` to the sink
    fn warn(&mut self, error: E);
}

/// Sink adapter that is able to convert from different error types via [`From`] trait
pub struct Adapt<'a, E, W>(&'a mut W, PhantomData<E>);

/// Sink adapter that applies some function before passing the value to the wrapped sink
pub struct AdaptMap<'a, D, E, F, W>(&'a mut W, F, PhantomData<D>, PhantomData<E>);

mod sealed {
    use std::error::Error;

    pub trait WarnExt<E> {}

    impl<E: Error, W: super::Warn<E>> WarnExt<E> for W {}
}

/// Extension methods for trait [`Warn`]
///
/// This trait is implemented for all the traits which implement [`Warn`]. Note that this
/// trait is sealed, so you cannot implement it for anything else.
pub trait WarnExt<E: Error>: Warn<E> + sealed::WarnExt<E> {
    /// Wraps the sink into the adapter, which is able to convert from different error
    /// types via [`From`] trait
    ///
    /// This is especially useful when you try to pass the sink into a subfunction,
    /// which yields different errors. See the [example](#example) below.
    ///
    /// # Example
    ///
    /// ```
    /// use thiserror::Error;
    /// use wurm::prelude::*;
    /// use wurm::CollectAll;
    ///
    /// // First error type
    /// #[derive(Debug, Error, PartialEq, Eq)]
    /// #[error("first error")]
    /// struct FooError;
    ///
    /// // Second error type, which is converible from `FooError`
    /// #[derive(Debug, Error, PartialEq, Eq)]
    /// #[error("second error: {0}")]
    /// struct BarError(#[from] FooError);
    ///
    /// // This function yields errors of type `FooError`
    /// fn foo(warn: &mut impl Warn<FooError>) {
    ///     warn.warn(FooError);
    /// }
    ///
    /// // This function yields errors of type `FooError`
    /// // So, we need the adapter to call `foo()`
    /// fn bar(warn: &mut impl Warn<BarError>) {
    ///     foo(&mut warn.adapt());
    /// }
    ///
    /// let mut warn = CollectAll::default();
    /// bar(&mut warn);
    /// // We get exactly one error of type `BarError` after calling `bar()`.
    /// // Note that the error is originally coming from `foo()` and is wrapped.
    /// assert_eq!(warn.0.len(), 1);
    /// ```
    #[inline]
    fn adapt(&mut self) -> Adapt<'_, E, Self>
    where
        Self: Sized,
    {
        Adapt(self, PhantomData)
    }

    /// Like [`WarnExt::adapt()`], but applies function `func` instead of converting errors via [`From`]
    #[inline]
    fn adapt_map<D, F>(&mut self, func: F) -> AdaptMap<'_, D, E, F, Self>
    where
        Self: Sized,
        D: Error,
        F: FnMut(D) -> E,
    {
        AdaptMap(self, func, PhantomData, PhantomData)
    }
}

impl<E: Error, W: Warn<E>> WarnExt<E> for W {}

impl<'a, D, E, W> Warn<D> for Adapt<'a, E, W>
where
    D: Error,
    E: Error + From<D>,
    W: Warn<E>,
{
    #[inline]
    fn warn(&mut self, error: D) {
        self.0.warn(E::from(error))
    }
}

impl<'a, D, E, F, W> Warn<D> for AdaptMap<'a, D, E, F, W>
where
    D: Error,
    E: Error,
    F: FnMut(D) -> E,
    W: Warn<E>,
{
    #[inline]
    fn warn(&mut self, error: D) {
        self.0.warn(self.1(error))
    }
}
