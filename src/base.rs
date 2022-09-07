use std::{error::Error, marker::PhantomData};

pub trait Warn<E: Error> {
    fn warn(&mut self, error: E);
}

pub struct Adapt<'a, E, W>(&'a mut W, PhantomData<E>);

pub struct AdaptMap<'a, D, E, F, W>(&'a mut W, F, PhantomData<D>, PhantomData<E>);

pub trait WarnExt<E: Error>: Warn<E> {
    #[inline]
    fn adapt(&mut self) -> Adapt<'_, E, Self>
    where
        Self: Sized,
    {
        Adapt(self, PhantomData)
    }

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
