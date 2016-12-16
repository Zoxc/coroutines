pub enum CoroutineResult<Y, R> {
    Awaiting,
    Yield(Y),
    Return(R),
    Completed,
}

pub enum Async<T> {
    Ready(T),
    NotReady,
}

pub trait Await<Object> {
    type Return;
    fn await(self, obj: &mut Object) -> Async<Self::Return>;
}

pub trait AwaitElement<Stream> {
    type Item;
    type Error;
    fn await(self, obj: &mut Stream) -> Async<Result<Option<Self::Item>, Self::Error>>;
}

pub trait Coroutine<Args> {
    type Yield;
    type Return;
	fn resume(&mut self, args: Args) -> CoroutineResult<Self::Yield, Self::Return>;
}