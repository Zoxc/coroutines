#![feature(never_type, fundamental)]

pub enum State<Y, R> {
    Yielded(Y),
    Complete(R),
}

pub trait Generator<Args> {
    type Yield;
    type Return;
	fn resume(&mut self, args: Args) -> State<Self::Yield, Self::Return>;
}

pub trait AsGenerator<Args> {
    type Yield;
    type Return;
    fn resume(&mut self, args: Args) -> State<Self::Yield, Self::Return>;
}
