#![feature(never_type)]

extern crate coroutine;
use coroutine::*;

pub trait Future<E: Executor>: Generator<E, Yield=!> {}

impl<E: Executor, T: Generator<E, Yield=!>> Future<E> for T {}

pub trait Stream<E: Executor>: Generator<E> {}

impl<E: Executor, T: Generator<E>> Stream<E> for T {}
