#![feature(never_type)]

extern crate coroutine;
use coroutine::*;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<T: Generator<(), Return = ()>> Iterator for T {
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.resume(()) {
            State::Complete(..) => None,
            State::Yielded(v) => Some(v),
        }
    }
}
/*
impl<T: Iterator> AsGenerator<()> for T {
    type Yield = T::Item;
    type Return = ();

    fn resume(&mut self, _: ()) -> State<Self::Yield, ()> {
        if let Some(item) = self.next() {
            State::Yielded(item)
        } else {
            State::Complete(())
        }
    }
}*/

pub trait Executor {
	type Blocked;
}

pub trait Future<E: Executor + ?Sized> {
    type Result;

    fn poll(&mut self, executor: &mut E) -> State<E::Blocked, Self::Result>;
}

impl<E: Executor, R, T: for<'e> Generator<&'e mut E, Yield = !, Return = R>> Future<E> for T {
    type Result = R;

    fn poll(&mut self, executor: &mut E) -> State<E::Blocked, Self::Result> {
    	panic!()
    }
}