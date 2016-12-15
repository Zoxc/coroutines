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

impl<T: Coroutine<Return = Result<R, E>, Yield = (), Arg = Handle>, R, E> Future for T
{
    type Item = R;
    type Error = E;

    fn poll(&mut self, handle: Handle) -> Poll<Self::Item, Self::Error> {
        Ok(Async::NotReady)
    }
}

impl<T: Future> Awaitable for T
{
    type Arg = Handle;
    type Return = Poll<T::Item, T::Error>;

    fn await(&mut self, arg: Self::Arg) -> Option<Self::Return> {
        None
    }
}

/*
struct Task;

pub trait Future2 {
    type Result;

    fn poll(&mut self, task: &mut Task) -> Option<Self::Result>;
}

impl<'a, T: Coroutine<Yield = (), Arg = &'a mut Task>> Future2 for T
{
    type Result = T::Return;

    fn poll(&mut self, task: &mut Task) -> Option<Self::Result> {
        self.resume(task);
        None
    }
}

impl<'a, T: Future2<>> Awaitable for T
{
    type Arg = &mut Task;
    type Result = T::Return;

    fn poll(&mut self, task: &mut Task) -> Option<Self::Result> {
        self.resume(task);
        None
    }
}
*/
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<T: Coroutine<Return = (), Arg = ()>> Iterator for T
{
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
