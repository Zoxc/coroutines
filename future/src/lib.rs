#![feature(never_type)]

extern crate coroutine;
use coroutine::*;

pub struct Ref<'e, E: 'e>(&'e mut E);

impl<'e, E: Executor> Executor for Ref<'e, E> {
    type Blocked = E::Blocked;
} 

pub trait Future<E: Executor> {
    type Result;

    fn poll(&mut self, executor: Ref<E>) -> ComputationState<Self::Result, E::Blocked>;
}

impl<E: Executor, R, T: for<'e> Coroutine<Ref<'e, E>, Yield = !, Return = R>> Future<E> for T {
    type Result = R;

    fn poll(&mut self, executor: Ref<E>) -> ComputationState<Self::Result, E::Blocked> {
        match self.resume(executor) {
            State::Blocked(b) => ComputationState::Blocked(b),
            State::Complete(r) => ComputationState::Ready(r),
            State::Yielded(..) => unreachable!(),
        }
    }
}

impl<'e, T: Future<E>, E: Executor> Await<T> for Ref<'e, E> {
    type Return = T::Result;

    fn await(self, obj: &mut T) -> ComputationState<Self::Return, E::Blocked> {
        obj.poll(self)
    }
}

pub trait Stream<E: Executor> {
    type Item;
    type Error;

    fn poll(&mut self, executor: Ref<E>) -> ComputationState<Result<Option<Self::Item>, Self::Error>, E::Blocked>;
}

impl<E: Executor, Y, Err, T: for<'a> Coroutine<Ref<'a, E>, Yield = Y, Return = Result<(), Err>>> Stream<E> for T {
    type Item = Y;
    type Error = Err;

    fn poll(&mut self, executor: Ref<E>) -> ComputationState<Result<Option<Self::Item>, Self::Error>, E::Blocked> {
        match self.resume(executor) {
            State::Blocked(b) => ComputationState::Blocked(b),
            State::Complete(r) => ComputationState::Ready(r.map(|_| None)),
            State::Yielded(y) => ComputationState::Ready(Ok(Some(y))),
        }
    }
}

impl<'e, E: Executor, T: Stream<E>> AwaitElement<T> for Ref<'e, E> {
    type Item = T::Item;
    type Error = T::Error;

    fn await(self, obj: &mut T) -> ComputationState<Result<Option<Self::Item>, Self::Error>, E::Blocked> {
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
            State::Complete(..) => None,
            State::Yielded(v) => Some(v),
            State::Blocked(..) => unreachable!(),
        }
    }
}
