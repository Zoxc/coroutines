#![feature(never_type, fundamental)]

pub enum State<Y, R, B> {
    Yielded(Y),
    Complete(R),
    Blocked(B),
}

pub trait Executor {
    type Blocked;
}

pub trait Generator<E: Executor> {
    type Yield;
    type Return;
	fn resume(&mut self, executor: &mut E) -> State<Self::Yield, Self::Return, E::Blocked>;
}

// Let () be the syncronious executor.
// That is an executor that requires no input and never blocks.
impl Executor for () {
    type Blocked = !;
}

// Generator<E, Blocked = !, Yield = !> is equivalent to FnMut<E>.

// Runs a generator which doesn't yield in the syncronious executor () and gives the result
// We can implement FnMut for these!!
pub fn run<T: Generator<(), Yield = !>>(mut generator: T) -> T::Return {
    match generator.resume(&mut ()) {
        State::Complete(r) => r,
        _ => panic!(),
    }
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// An Iterator is a Generator<(), Return = ()>
impl<T: Generator<(), Return = ()>> Iterator for T {
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.resume(&mut ()) {
            State::Complete(..) => None,
            State::Yielded(v) => Some(v),
            State::Blocked(..) => unreachable!(),
        }
    }
}

// A Future is a Generator<Yield = !> where the executor is passed by &mut E
pub trait Future<E: Executor> {
    type Result;

    fn poll(&mut self, executor: &mut E) -> State<!, Self::Result, E::Blocked>;
}

impl<E: Executor, R, T: Generator<E, Yield = !, Return = R>> Future<E> for T {
    type Result = R;

    fn poll(&mut self, executor: &mut E) -> State<!, Self::Result, E::Blocked> {
        self.resume(executor)
    }
}

// A Stream is a Generator where the executor is passed by &mut E
pub trait Stream<E: Executor> {
    type Yield;
    type Return;

    fn poll(&mut self, executor: &mut E) -> State<Self::Yield, Self::Return, E::Blocked>;
}

impl<E: Executor, Y, R, T: Generator<E, Yield = Y, Return = R>> Stream<E> for T {
    type Yield = Y;
    type Return = R;

    fn poll(&mut self, executor: &mut E) -> State<Self::Yield, Self::Return, E::Blocked> {
        self.resume(executor)
    }
}