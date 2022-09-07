use std::{error::Error, marker::PhantomData};

use crate::base::Warn;

#[derive(Debug, Clone)]
pub struct Ignore;

impl<E: Error> Warn<E> for Ignore {
    #[inline]
    fn warn(&mut self, _error: E) {}
}

#[derive(Debug, Clone)]
pub struct Stderr;

impl<E: Error> Warn<E> for Stderr {
    #[inline]
    fn warn(&mut self, error: E) {
        eprintln!("error: {}", error);
    }
}

#[derive(Debug, Clone)]
pub struct All<E: Error>(pub Vec<E>);

impl<E: Error> Default for All<E> {
    #[inline]
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<E: Error> Warn<E> for All<E> {
    #[inline]
    fn warn(&mut self, error: E) {
        self.0.push(error);
    }
}

#[derive(Debug, Clone)]
pub struct First<E: Error>(pub Option<E>);

impl<E: Error> Default for First<E> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<E: Error> Warn<E> for First<E> {
    #[inline]
    fn warn(&mut self, error: E) {
        if self.0.is_none() {
            self.0 = Some(error);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Last<E: Error>(pub Option<E>);

impl<E: Error> Default for Last<E> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<E: Error> Warn<E> for Last<E> {
    #[inline]
    fn warn(&mut self, error: E) {
        self.0 = Some(error);
    }
}

#[derive(Debug, Clone)]
pub struct FromFn<E: Error, F: FnMut(E)>(F, PhantomData<E>);

#[inline]
pub fn from_fn<E: Error, F: FnMut(E)>(func: F) -> FromFn<E, F> {
    FromFn(func, PhantomData)
}

impl<E: Error, F: FnMut(E)> Warn<E> for FromFn<E, F> {
    #[inline]
    fn warn(&mut self, error: E) {
        self.0(error)
    }
}
