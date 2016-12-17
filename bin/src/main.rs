#![feature(conservative_impl_trait)]
#![feature(never_type)]

extern crate future;
extern crate coroutine;

use coroutine::*;
use future::*;

struct FutureTest<P>(Pong<P>);

impl<P, R, H: Executor + Await<Pong<P>, Return=R>> Coroutine<H> for FutureTest<P> {
	type Yield = !;
	type Return = R;
	fn resume(&mut self, executor: H) -> State<Self::Yield, Self::Return, H::Blocked> {
		match executor.await(&mut self.0) {
			ComputationState::Ready(r) => State::Complete(r),
			ComputationState::Blocked(b) => State::Blocked(b),
		}
	}
}

fn hm<E: Executor>() -> impl Future<E> {
	FutureTest(Pong(Some(4)))
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
	hm::<()>();
}