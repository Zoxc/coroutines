#![feature(never_type)]

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
	fn resume(&mut self, executor: E) -> State<Self::Yield, Self::Return, E::Blocked>;
}

pub trait Await<G>: Executor {
    type Return;
    fn await(&mut self, obj: &mut G) -> State<!, Self::Return, Self::Blocked>;
}

pub trait AwaitGenerator<G>: Executor {
    type Yield;
    type Return;
    fn await(&mut self, obj: &mut G) -> State<Self::Yield, Self::Return, Self::Blocked>;
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
    match generator.resume(()) {
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
        match self.resume(()) {
            State::Complete(..) => None,
            State::Yielded(v) => Some(v),
            State::Blocked(..) => unreachable!(),
        }
    }
}
