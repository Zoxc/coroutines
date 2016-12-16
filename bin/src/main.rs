#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(never_type)]

extern crate future;
extern crate coroutine;

use coroutine::*;
use future::*;

struct FutureTest;

impl<H: Executor> Coroutine<H> for FutureTest {
	type Yield = !;
	type Return = usize;
	fn resume(&mut self, executor: H) -> State<Self::Yield, Self::Return, H::Blocked> {
		panic!()
	}
}

fn hm<E: Executor>() -> impl Future<E> {
	FutureTest
}

struct IteratorTest;

impl Coroutine<()> for IteratorTest {
	type Yield = usize;
	type Return = ();
	fn resume(&mut self, executor: ()) -> State<Self::Yield, Self::Return, !> {
		panic!()
	}
}

fn hm2() -> impl Iterator {
	IteratorTest
}

fn main() {
	hm();
}