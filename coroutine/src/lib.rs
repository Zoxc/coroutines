pub enum CoroutineResult<Y, R> {
    Await,
    Yield(Y),
    Return(R),
    Completed,
}

pub trait WaitFor<Object> {
    type Return;
    fn wait_for(self, obj: &mut Object) -> Option<Self::Return>;
}

pub trait Coroutine<Arg> {
    type Yield;
    type Return;
	fn resume(&mut self, arg: Arg) -> CoroutineResult<Self::Yield, Self::Return>;
}

// In await!() An coroutine's must be equal to all the awaits's Awaitable::Arg
// Now how do we implement Awaitable for all futures?