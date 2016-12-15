#![feature(never_type)]

extern crate coroutine;

use coroutine::*;
use std::marker::PhantomData;

/// Return type of future, indicating whether a value is ready or not.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Async<T> {
    /// Represents that a value is immediately ready.
    Ready(T),

    /// Represents that a value is not ready yet, but may be so later.
    NotReady,
}

pub type Poll<T, E> = Result<Async<T>, E>;

pub struct Handle;

pub trait Future {
    type Item;

    type Error;

    fn poll(&mut self, handle: Handle) -> Poll<Self::Item, Self::Error>;
}

impl<T: Coroutine<Handle, Return = Result<R, E>, Yield = !>, R, E> Future for T {
    type Item = R;
    type Error = E;

    fn poll(&mut self, handle: Handle) -> Poll<Self::Item, Self::Error> {
        Ok(Async::NotReady)
    }
}

impl<T: Future> WaitFor<T> for Handle {
    type Return = Result<T::Item, T::Error>;

    fn wait_for(self, obj: &mut T) -> Option<Self::Return> {
        match obj.poll(self) {
            Ok(Async::NotReady) => None,
            Ok(Async::Ready(v)) => Some(Ok(v)),
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct Handle2;

pub trait Future2 {
    type Result;

    fn poll(&mut self, task: &mut Handle2) -> Option<Self::Result>;
}

impl<R, T: for<'a> Coroutine<&'a mut Handle2, Yield = !, Return = R>> Future2 for T
{
    type Result = R;

    fn poll(&mut self, task: &mut Handle2) -> Option<Self::Result> {
        self.resume(task);
        None
    }
}

impl<'h, T: Future2> WaitFor<T> for &'h mut Handle2 {
    type Return = T::Result;

    fn wait_for(self, obj: &mut T) -> Option<Self::Return> {
        obj.poll(self)
    }
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<T: Coroutine<(), Return = ()>> Iterator for T {
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
