#![feature(conservative_impl_trait)]
#![feature(never_type, fundamental_attribute)]

extern crate future;
extern crate coroutine;
extern crate future_traits;
use coroutine::*;
use future_traits::*;
use future::*;

struct ReturnTest<F> {
	next: F,
}

impl<E: Executor, F: Fn(&mut E) -> State<!, usize, E::Blocked>> Generator<E> for ReturnTest<F> {
	type Yield = !;
	type Return = usize;
	fn resume(&mut self, executor: &mut E) -> State<Self::Yield, Self::Return, E::Blocked> {
		(self.next)(executor)
	}
}

fn return_test<E: Executor>() -> impl Future<E> {
	ReturnTest {
		next: |executor: &mut E| -> State<!, usize, E::Blocked> {
			State::Complete(3)
		}
	}
}

struct SleepTest;
// Await1Arg   + Await<Await1Arg, Return=R>
impl<E: SleepExecutor> Generator<E> for SleepTest {
	type Yield = !;
	type Return = ();
	fn resume(&mut self, mut executor: &mut E) -> State<Self::Yield, Self::Return, E::Blocked> {
		let mut s = E::sleep(1000);
		s.resume(&mut executor);
		State::Complete(())
	}
}

fn sleep_test<E: SleepExecutor>() -> impl Generator<E> {
	SleepTest
}

fn nested_sleep_test<E: SleepExecutor>() -> impl Generator<E> {
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
	fn resume(&mut self, executor: &mut ()) -> State<Self::Yield, Self::Return, !> {
		panic!()
	}
}

fn hm2() -> impl Iterator {
	IteratorTest
}

fn main() {
	EventLoop::new().run(sleep_test());
}