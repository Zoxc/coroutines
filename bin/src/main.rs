#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(never_type)]

use std::marker::PhantomData;

extern crate future;
extern crate coroutine;

use coroutine::*;
use future::*;

fn type_name<T>(_: &T) -> &'static str where T: ?Sized {
	unsafe { std::intrinsics::type_name::<T>() }
}


struct FutureTest<H>(H);

impl<H> Coroutine<H> for FutureTest<H> {
	type Yield = !;
	type Return = Result<usize, ()>;
	fn resume(&mut self, arg: H) -> CoroutineResult<Self::Yield, Self::Return> {
		CoroutineResult::Completed
	}
}

fn hm() -> impl Future {
	let a = unsafe { std::mem::uninitialized() };
	println!("type of a {}", type_name(&a));
	FutureTest(a)
}

struct IteratorTest;

impl Coroutine<()> for IteratorTest {
	type Yield = usize;
	type Return = ();
	fn resume(&mut self, arg: ()) -> CoroutineResult<Self::Yield, Self::Return> {
		CoroutineResult::Completed
	}
}

fn hm2() -> impl Iterator {
	IteratorTest
}

fn main() {
	hm();
}