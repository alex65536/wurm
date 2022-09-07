//! Various sinks for errors
//!
//! Of course, all of these sinks implement [`Warn`].

use std::{error::Error, marker::PhantomData};

use crate::base::Warn;

/// Ignores all the incoming errors
#[derive(Debug, Clone)]
pub struct Ignore;

impl<E: Error> Warn<E> for Ignore {
    #[inline]
    fn warn(&mut self, _error: E) {}
}

/// Writes all the incoming errors to standard error stream
#[derive(Debug, Clone)]
pub struct Stderr;

impl<E: Error> Warn<E> for Stderr {
    #[inline]
    fn warn(&mut self, error: E) {
        eprintln!("error: {}", error);
    }
}

/// Collects all the incoming errors into a [`Vec`]
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

/// Keeps the first error which arrived into this sink
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

/// Keeps the last error which arrived into this sink
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

/// Wrapper which allows to create a sink from arbitrary function
#[derive(Debug, Clone)]
pub struct FromFn<E: Error, F: FnMut(E)>(F, PhantomData<E>);

/// Creates a sink from function `func`
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
