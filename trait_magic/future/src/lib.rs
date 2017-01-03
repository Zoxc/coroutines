#![feature(never_type)]
#![feature(fundamental)]
#![feature(conservative_impl_trait)]

extern crate coroutine;
extern crate future_traits;
use coroutine::*;
use future_traits::*;
use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::Cell;
use std::marker::Sized;

struct A<T>(T);

impl<T> Iterator for A<T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        Some(3)
    }
}

impl<T> Generator<()> for A<T> {

}


pub struct Pong<T>(pub Option<T>);

impl<T, E: Executor> Future<E> for Pong<T> {
    type Result = T;

    fn poll(&mut self, executor: &mut E) -> State<E::Blocked, Self::Result> {
        panic!()
    }
}
