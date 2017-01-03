use std::marker::PhantomData;
use std::mem::drop;
use std::mem::size_of;

unsafe trait PtrStorage {
	unsafe fn new<T>(val: &mut T) -> Self;
	unsafe fn clone(&self) -> Self;
	unsafe fn as_raw(&mut self) -> *mut ();
}

struct Sized(*mut ());
struct Zero;

unsafe impl PtrStorage for Zero {
	unsafe fn new<T>(val: &mut T) -> Self {
		assert!(size_of::<T>() == 0);
		Zero
	}

	unsafe fn clone(&self) -> Self {
		Zero
	}

	unsafe fn as_raw(&mut self) -> *mut () {
		1 as *mut ()
	}
}

unsafe impl PtrStorage for Sized {
	unsafe fn new<T>(val: &mut T) -> Self {
		Sized(val as *mut T as *mut ())
	}

	unsafe fn clone(&self) -> Self {
		Sized(self.0)
	}

	unsafe fn as_raw(&mut self) -> *mut () {
		self.0
	}
}

struct PtrMut<'h, T: 'h, D: PtrStorage> {
	data: D,
	marker: PhantomData<&'h mut T>,
}

impl<'h, T: 'h, D: PtrStorage> PtrMut<'h, T, D> {
	fn new(ptr: &'h mut T) -> Self {
		PtrMut {
			data: unsafe { D::new(ptr) },
			marker: PhantomData,
		}
	}

	fn reborrow<'s>(&'s mut self) -> PtrMut<'s, T, D> {
		PtrMut {
			data: unsafe { self.data.clone() },
			marker: PhantomData,
		}
	}

	fn as_mut<'s>(&'s mut self) -> &'h mut T {
		unsafe { &mut *(self.data.as_raw() as *mut T) }
	}
}

fn main() {}