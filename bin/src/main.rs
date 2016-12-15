#![feature(conservative_impl_trait)]

extern crate future;
extern crate coroutine;

use coroutine::*;
use future::*;

struct FutureTest;

impl Coroutine for FutureTest {
	type Yield = ();
	type Arg = Handle;
	type Return = Result<usize, ()>;
	fn resume(&mut self, arg: Self::Arg) -> CoroutineResult<Self::Yield, Self::Return> {
		CoroutineResult::Completed
	}
}

fn hm() -> impl Future {
	FutureTest
}

struct IteratorTest;

impl Coroutine for IteratorTest {
	type Yield = usize;
	type Arg = ();
	type Return = ();
	fn resume(&mut self, arg: Self::Arg) -> CoroutineResult<Self::Yield, Self::Return> {
		CoroutineResult::Completed
	}
}

fn hm2() -> impl Iterator {
	IteratorTest
}


fn main() {
    println!("Hello, world!");
}
