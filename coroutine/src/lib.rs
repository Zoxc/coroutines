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

pub trait WaitFor<Object> {
    type Return;
    fn wait_for(self, obj: &mut Object) -> Async<Self::Return>;
}

pub trait Coroutine<Executor> {
    type Yield;
    type Return;
	fn resume(&mut self, executor: Executor) -> CoroutineResult<Self::Yield, Self::Return>;
}