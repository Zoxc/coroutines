#![feature(conservative_impl_trait)]
#![feature(never_type)]

extern crate future;
extern crate coroutine;

use coroutine::*;
use future::*;

struct ReturnTest<F> {
	next: F,
}

impl<E: Executor, F: Fn(E) -> State<!, usize, E::Blocked>> Generator<E> for ReturnTest<F> {
	type Yield = !;
	type Return = usize;
	fn resume(&mut self, executor: E) -> State<Self::Yield, Self::Return, E::Blocked> {
		(self.next)(executor)
	}
}

fn return_test<E: Executor>() -> impl Future<E> {
	ReturnTest {
		next: |executor: FutureExecutor<_>| -> State<!, usize, E::Blocked> {
			State::Complete(3)
		}
	}
}

struct SleepTest;
// Await1Arg   + Await<Await1Arg, Return=R>
impl<E: SleepExecutor + Await<Sleep>> Generator<E> for SleepTest {
	type Yield = !;
	type Return = ();
	fn resume(&mut self, mut executor: E) -> State<Self::Yield, Self::Return, E::Blocked> {
		executor.await(&mut Sleep);
		State::Complete(())
	}
}

fn sleep_test<E: SleepExecutor>() -> impl Future<E> {
	SleepTest
}

fn nested_sleep_test<E: SleepExecutor>() -> impl Future<E> {
	sleep_test()
}

/*
struct FutureTest<P>(Pong<P>);
// Await1Arg   + Await<Await1Arg, Return=R>
impl<P, H: Executor> Generator<H> for FutureTest<P> {
	type Yield = !;
	type Return = P;
	fn resume(&mut self, executor: H) -> State<Self::Yield, Self::Return, H::Blocked> {
		//let arg: &mut Await1Arg = &mut self.0;
		executor.await(&mut self.0)
	}
}

fn hm<E: Executor>() -> impl Future<E> {
	FutureTest(Pong(Some(4)))
}
*/
/*

struct FutureTest<P>(Pong<P>);

impl<R, P, Await1Arg, Await1Ret, H: Executor + Await<Await1Arg, Return=Await1Ret>> Generator<H> for FutureTest<P> {
	type Yield = !;
	type Return = R;
	fn resume(&mut self, executor: H) -> State<Self::Yield, Self::Return, H::Blocked> {
		let arg: &mut Await1Arg = &mut self.0;
		let ret: Await1Ret = executor.await(arg);
		ret
	}
}

fn hm<E: Executor>() -> impl Future<E> {
	FutureTest(Pong(Some(4)))
}
*/

struct IteratorTest;

impl Generator<()> for IteratorTest {
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
}