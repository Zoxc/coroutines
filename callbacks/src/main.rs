#![feature(conservative_impl_trait)]
#![feature(never_type, fundamental_attribute)]

use std::marker::PhantomData;

// A function pointer and some data to pass to the callback
pub struct Callback<R> {
	pointer: fn (R, *const ()),
	data: *const (),
}

pub struct DummyActiveFuture<'c> {
dummy: &'c (),
}

// This is the trait implemented for generators
pub trait Future {
	type Return;

	fn schedule<'c>(self, callback: &'c mut Callback<Self::Return>) -> DummyActiveFuture<'c>;
}

// This represent an immovable and active computation
pub trait ActiveFuture<'c> {
	fn cancel(&mut self);
}

impl<'c> ActiveFuture<'c> for DummyActiveFuture<'c> {
	fn cancel(&mut self) {
	}
}

fn main() {
}