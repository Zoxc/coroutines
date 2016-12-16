#![feature(never_type)]

extern crate coroutine;

use coroutine::*;
use std::marker::PhantomData;
/*
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

pub trait FutureA {
    type Item;

    type Error;

    fn poll(&mut self, handle: Handle) -> Poll<Self::Item, Self::Error>;
}

impl<T: Coroutine<Handle, Return = Result<R, E>, Yield = !>, R, E> FutureA for T {
    type Item = R;
    type Error = E;

    fn poll(&mut self, handle: Handle) -> Poll<Self::Item, Self::Error> {
        match self.resume(handle) {
            CoroutineResult::Awaiting => Ok(Async::NotReady),
            CoroutineResult::Return(r) => r.map(|v| Async::Ready(v)),
            CoroutineResult::Yield(..) | CoroutineResult::Completed => unreachable!(),
        }
    }
}

impl<T: FutureA> WaitFor<T> for Handle {
    type Return = Result<T::Item, T::Error>;

    fn wait_for(self, obj: &mut T) -> Option<Self::Return> {
        match obj.poll(self) {
            Ok(Async::NotReady) => None,
            Ok(Async::Ready(v)) => Some(Ok(v)),
            Err(e) => Some(Err(e)),
        }
    }
}
*/
pub struct EventLoop;

pub trait Future {
    type Result;

    fn poll(&mut self, event_loop: &mut EventLoop) -> Async<Self::Result>;
}

impl<R, T: for<'a> Coroutine<&'a mut EventLoop, Yield = !, Return = R>> Future for T {
    type Result = R;

    fn poll(&mut self, event_loop: &mut EventLoop) -> Async<Self::Result> {
        match self.resume(event_loop) {
            CoroutineResult::Awaiting => Async::NotReady,
            CoroutineResult::Return(r) => Async::Ready(r),
            CoroutineResult::Yield(..) | CoroutineResult::Completed => unreachable!(),
        }
    }
}

impl<'h, T: Future> Await<T> for &'h mut EventLoop {
    type Return = T::Result;

    fn await(self, obj: &mut T) -> Async<Self::Return> {
        obj.poll(self)
    }
}

pub trait Stream {
    type Item;
    type Error;

    fn poll(&mut self, event_loop: &mut EventLoop) -> Async<Result<Option<Self::Item>, Self::Error>>;
}

impl<Y, E, T: for<'a> Coroutine<&'a mut EventLoop, Yield = Y, Return = Result<(), E>>> Stream for T {
    type Item = Y;
    type Error = E;

    fn poll(&mut self, event_loop: &mut EventLoop) -> Async<Result<Option<Self::Item>, Self::Error>> {
        match self.resume(event_loop) {
            CoroutineResult::Awaiting => Async::NotReady,
            CoroutineResult::Return(r) => Async::Ready(r.map(|_| None)),
            CoroutineResult::Yield(y) => Async::Ready(Ok(Some(y))),
            CoroutineResult::Completed => unreachable!(),
        }
    }
}

impl<'h, T: Stream> AwaitElement<T> for &'h mut EventLoop {
    type Item = T::Item;
    type Error = T::Error;

    fn await(self, obj: &mut T) -> Async<Result<Option<Self::Item>, Self::Error>> {
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
        match self.resume(()) {
            CoroutineResult::Return(..) |
            CoroutineResult::Awaiting |
            CoroutineResult::Completed => None,
            CoroutineResult::Yield(v) => Some(v),
        }
    }
}
