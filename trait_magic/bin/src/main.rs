#![feature(conservative_impl_trait)]
#![feature(never_type, fundamental_attribute)]

extern crate future;
extern crate coroutine;
extern crate future_traits;
use coroutine::*;
use future_traits::*;
use future::*;

pub struct Map<A, F> {
    future: A,
    f: Option<F>,
}

pub fn map<E: Executor, U, A, F>(future: A, f: F) -> impl Future<E>
    where A: Future<E>,
          F: FnOnce(A::Result) -> U 
{
    Map {
        future: future,
        f: Some(f),
    }
}

impl<E: Executor, U, A, F> Future<E> for Map<A, F>
    where A: Future<E>,
          F: FnOnce(A::Result) -> U 
{
    type Result = U;

    fn poll(&mut self, executor: &mut E) -> State<E::Blocked, Self::Result> {
    	panic!()
    }
}


struct IterGen;

impl Iterator for IterGen {
	
}