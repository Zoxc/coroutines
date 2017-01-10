
// A function pointer and some data to pass to the callback
pub struct Callback<R> {
	pointer: fn (R, *const ()),
	data: *const (),
}

// This is the trait implemented for generators
pub trait Future {
 	type Return;

	// This starts the computation. The result will be given by callback
	fn schedule<'c>(self, callback: &'c mut Callback<Self::Return>) -> impl ActiveFuture<'c> + ?Move;
}

// This represent an immovable and active computation
pub trait ActiveFuture<'c>: ?Move {
	// This cancels the computation
	fn cancel(&mut self);
}

fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Return=(A::Return, B::Return)> {
	let a_active = a.schedule();
	let b_active = b.schedule();
}

fn select<A: Future, B: Future>(a: A, b: B) -> impl Future<Return=(A::Return, B::Return)> {
	let a_active = a.schedule();
	let b_active = b.schedule();
}

pub struct Join<A: Future, B: Future> {
	a: A,
	b: B,
}

impl<A: Future, B: Future> Future for Join<A, B> {
	type Return = (A::Return, B::Return);

	fn schedule<'c>(self, callback: &'c mut Callback<Self::Return>) -> impl ActiveFuture<'c> + ?Move {

	}
}