#![feature(never_type)]

extern crate coroutine;
use coroutine::*;

pub struct ExecutorRef<'e, E: 'e>(&'e mut E);

impl<'e, E: Executor> Executor for ExecutorRef<'e, E> {
    type Blocked = E::Blocked;
} 


// A Future is a Generator<Yield = !> where the executor is passed by &mut E
pub trait Future<E: Executor> {
    type Result;

    fn poll(&mut self, executor: ExecutorRef<E>) -> State<!, Self::Result, E::Blocked>;
}

impl<E: Executor, R, T: for<'e> Generator<ExecutorRef<'e, E>, Yield = !, Return = R>> Future<E> for T {
    type Result = R;

    fn poll(&mut self, executor: ExecutorRef<E>) -> State<!, Self::Result, E::Blocked> {
        self.resume(executor)
    }
}

impl<'e, T: Future<E>, E: Executor> Await<T> for ExecutorRef<'e, E> {
    type Return = T::Result;

    fn await(&mut self, obj: &mut T) -> State<!, Self::Return, E::Blocked> {
        obj.poll(ExecutorRef(self.0))
    }
}

// A Stream is a Generator where the executor is passed by &mut E
pub trait Stream<E: Executor> {
    type Yield;
    type Return;

    fn poll(&mut self, executor: ExecutorRef<E>) -> State<Self::Yield, Self::Return, E::Blocked>;
}

impl<E: Executor, Y, R, T: for<'e> Generator<ExecutorRef<'e, E>, Yield = Y, Return = R>> Stream<E> for T {
    type Yield = Y;
    type Return = R;

    fn poll(&mut self, executor: ExecutorRef<E>) -> State<Self::Yield, Self::Return, E::Blocked> {
        self.resume(executor)
    }
}

impl<'e, E: Executor, T: Stream<E>> AwaitGenerator<T> for ExecutorRef<'e, E> {
    type Yield = T::Yield;
    type Return = T::Return;

    fn await(&mut self, obj: &mut T) -> State<Self::Yield, Self::Return, E::Blocked> {
        obj.poll(ExecutorRef(self.0))
    }
}

// A Future is a Generator<Yield = !> where the executor is passed by &mut E
pub trait Future2<E: Executor>: Stream<E, Yield = !> {
    fn future_only(&self) {}
}

impl<E: Executor, T: Stream<E, Yield = !>> Future2<E> for T {
}

// Cannot do Stream only operations given the above def. Is Stream only operations useful. Probably since Future streams return ()
// Can Stream just be trait StreamOnly<E>: Stream<E, Result = ()>?
// Stream only operations are probably very useful to remove from futures, since all of them would operate on the ! values
// What error do you get if you use such a value?

struct EventLoop;

impl Executor for EventLoop {
    type Blocked = ();
}

pub struct RPC;

impl<E: Executor> Future<E> for RPC {
    type Result = usize;

    fn poll(&mut self, executor: ExecutorRef<E>) -> State<!, Self::Result, E::Blocked> {
        State::Complete(1)
    }
}

pub struct Pong<T>(pub Option<T>);

impl<T, E: Executor> Future<E> for Pong<T> {
    type Result = T;

    fn poll(&mut self, executor: ExecutorRef<E>) -> State<!, Self::Result, E::Blocked> {
        State::Complete(self.0.take().unwrap())
    }
}

pub trait SleepExecutor: Executor {
    fn sleep(&mut self) -> State<!, (), Self::Blocked>;
}

pub struct Sleep;

impl<E: SleepExecutor> Future<E> for Sleep {
    type Result = ();

    fn poll(&mut self, executor: ExecutorRef<E>) -> State<!, Self::Result, E::Blocked> {
        executor.0.sleep()
    }
}
/*
fn sleep<E: SleepExecutor>() -> impl Generator<E, Yield = !, Return = ()> {
    await executor.sleep(); // No access to executor here. Must manually implement this. 
}*/