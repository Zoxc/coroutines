pub enum CoroutineResult<Y, R> {
    Await,
    Yield(Y),
    Return(R),
    Completed,
}

pub trait Awaitable {
    type Arg;
    type Return;
    fn await(&mut self, arg: Self::Arg) -> Option<Self::Return>;
}

pub trait Coroutine {
    type Yield;
    type Return;
    type Arg;
	fn resume(&mut self, arg: Self::Arg) -> CoroutineResult<Self::Yield, Self::Return>;
}

// In await!() An coroutine's must be equal to all the awaits's Awaitable::Arg
// Now how do we implement Awaitable for all futures?