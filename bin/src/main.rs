#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(never_type)]

extern crate future;
extern crate coroutine;

use coroutine::*;
use future::*;

struct FutureTest;

impl<H> Coroutine<H> for FutureTest {
	type Yield = !;
	type Return = usize;
	fn resume(&mut self, executor: H) -> CoroutineResult<Self::Yield, Self::Return> {
		CoroutineResult::Completed
	}
}

fn hm() -> impl Future {
	FutureTest
}

struct IteratorTest;

impl Coroutine<()> for IteratorTest {
	type Yield = usize;
	type Return = ();
	fn resume(&mut self, executor: ()) -> CoroutineResult<Self::Yield, Self::Return> {
		CoroutineResult::Completed
	}
}

fn hm2() -> impl Iterator {
	IteratorTest
}

fn main() {
	hm();
}