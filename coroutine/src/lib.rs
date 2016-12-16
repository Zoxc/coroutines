#![feature(never_type)]

pub enum State<Y, R, B> {
    Yielded(Y),
    Complete(R),
    Blocked(B),
}

pub trait Executor {
    type Blocked;
}

pub trait Coroutine<E: Executor> {
    type Yield;
    type Return;
	fn resume(&mut self, executor: E) -> State<Self::Yield, Self::Return, E::Blocked>;
}

pub enum ComputationState<R, B> {
    Ready(R),
    Blocked(B),
}

pub trait Await<Computation>: Executor {
    type Return;
    fn await(self, obj: &mut Computation) -> ComputationState<Self::Return, Self::Blocked>;
}

pub trait AwaitElement<Generator>: Executor {
    type Item;
    type Error;
    fn await(self, obj: &mut Generator) -> ComputationState<Result<Option<Self::Item>, Self::Error>, Self::Blocked>;
}

impl Executor for () {
    type Blocked = !;
}

pub fn run<C: Coroutine<(), Yield = !>>(mut coroutine: C) -> C::Return {
    match coroutine.resume(()) {
        State::Return(r) => r,
        _ => panic!(),
    }
}