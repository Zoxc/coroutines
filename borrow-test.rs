use std::marker::PhantomData;
use std::mem::drop;

struct EventLoop(usize);

struct Handle<'h>(PhantomData<&'h mut EventLoop>);

fn get_handle<'h>(_ev: &'h mut EventLoop) -> Handle<'h> {
	Handle(PhantomData)
}

fn shrink_ref<'h, 's>(h: &'s mut &'h mut EventLoop) -> &'s mut EventLoop {
    *h
}

fn shrink_handle<'h, 's>(h: &'s mut Handle<'h>) -> Handle<'s> {
	Handle(PhantomData)
}

unsafe fn get_global_handle() -> Handle<'static> {
	Handle(PhantomData)
}

fn get_loop<'h>(_: &mut Handle<'h>) -> &'h mut EventLoop {
    static mut EV: EventLoop = EventLoop(0);
    unsafe { &mut EV }
}

fn main() {
	let mut ev = EventLoop;
	let handle = unsafe { get_global_handle() };
	process(handle);
}

fn print(mut h: Handle) {
    println!("Loop value {}", get_loop(&mut h).0);
}

fn process(mut h: Handle) {
    print(shrink_handle(&mut h));
    print(shrink_handle(&mut h));
}
