#![feature(never_type)]
#![feature(fundamental)]

extern crate coroutine;
extern crate future_traits;
use coroutine::*;
use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::Cell;
use std::marker::Sized;

// A Future is a Generator<Yield = !> where the executor is passed by &mut E
pub trait Future2<E: Executor>: Stream<E, Yield = !> {
    fn future_only(&self) {}
}

impl<E: Executor, T: Stream<E, Yield = !>> Future2<E> for T {
}

// Cannot do Stream only operations given the above def. Is Stream only operations useful. Probably since Future streams return ()
// Can Stream just be trait StreamOnly<E>: Stream<E, Result = ()>?
// Stream only operations are probably very useful to remove from futures, since all of them would operate on the ! values
// What error do you get if you use such a value?

type Task = Rc<RefCell<Future<EventLoop, Result=()>>>;

pub struct EventLoop {
    current: Option<Task>,
    timers: Vec<Rc<Timer>>,
}

pub struct Timer {
    delta: Cell<u64>,
    task: Task,
}

impl EventLoop {
    pub fn timer(&mut self, delta: u64) -> Rc<Timer> {
        let timer = Rc::new(Timer {
            delta: Cell::new(delta),
            task: self.current.as_ref().unwrap().clone()
        });
        self.timers.push(timer.clone());
        timer
    }

    pub fn run<F: Future<Self, Result=()> + 'static>(&mut self, future: F) {
        assert!(self.current.is_none());

        macro_rules! run {
            ($task:expr) => {
                let task = $task;
                self.current = Some(task.clone());
                task.borrow_mut().poll(self);
                self.current = None;
            }
        }

        let task = Rc::new(RefCell::new(future));
        run!(task);

        while !self.timers.is_empty() {
            thread::sleep(Duration::from_millis(1));

            let mut len = self.timers.len();
            let mut i = 0;

            while i < len {
                if self.timers[i].delta.get() == 0 {
                    run!(self.timers[i].task.clone());
                    self.timers.remove(i);
                    len -= 1;
                } else {
                    let delta = self.timers[i].delta.get();
                    self.timers[i].delta.set(delta - 1);
                    i += 1;
                }
            }
        }
    }
}

impl Executor for EventLoop {
    type Blocked = ();
}

pub struct RPC;

impl<E: Executor> Future<E> for RPC {
    type Result = usize;

    fn poll(&mut self, executor: &mut E) -> State<!, Self::Result, E::Blocked> {
        State::Complete(1)
    }
}

impl Future<()> for RPC {
    type Result = usize;

    fn poll(&mut self, executor: &mut ()) -> State<!, Self::Result, !> {
        State::Complete(1)
    }
}
/*
pub struct Pong<T>(pub Option<T>);

impl<T, E: Executor> Future<E> for Pong<T> {
    type Result = T;

    fn poll(&mut self, executor: &mut E) -> State<!, Self::Result, E::Blocked> {
        State::Complete(self.0.take().unwrap())
    }
}
*/
pub trait SleepExecutor: Executor where Self:Sized {
    type Sleep: Future<Self, Result = ()>;
    fn sleep(&mut self, delta: u64) -> Self::Sleep;
}
/*
pub struct FutureSleep<E: SleepExecutor>(E::Sleep);

impl<'e, E: SleepExecutor> SleepExecutor for FutureExecutor<'e, E> {
    type Sleep = FutureSleep<E>;
    fn sleep(&mut self, delta: u64) -> Self::Sleep {
        FutureSleep(self.0.sleep(delta))
    }
}

impl<'e, E: SleepExecutor> Future<FutureExecutor<'e, E>> for FutureSleep<E> {
    type Result = ();

    fn poll(&mut self, executor: FutureExecutor<FutureExecutor<E>>) -> State<!, Self::Result, E::Blocked> {
        //self.0.poll();
        panic!()
    }
}
*/
impl SleepExecutor for () {
    type Sleep = SyncSleep;

    fn sleep(&mut self, delta: u64) -> Self::Sleep {
        SyncSleep(delta)
    }
}

pub struct SyncSleep(u64);

impl Future<()> for SyncSleep {
    type Result = ();

    fn poll(&mut self, executor: &mut ()) -> State<!, Self::Result, !> {
        thread::sleep(Duration::from_millis(self.0));
        State::Complete(())
    }
}

impl SleepExecutor for EventLoop {
    type Sleep = AsyncSleep;

    fn sleep(&mut self, delta: u64) -> Self::Sleep {
        AsyncSleep(SleepState::Pending(delta))
    }
}

enum SleepState {
    Pending(u64),
    Started(Rc<Timer>)
}

pub struct AsyncSleep(SleepState);

impl Future<EventLoop> for AsyncSleep {
    type Result = ();

    fn poll(&mut self, executor: &mut EventLoop) -> State<!, Self::Result, ()> {
        match self.0 {
            SleepState::Pending(delta) => {
                self.0 = SleepState::Started(executor.timer(delta));
                State::Blocked(())
            }
            SleepState::Started(ref timer) => if timer.delta.get() == 0 {
                State::Complete(())
            } else {
                State::Blocked(())
            }
        }
    }
}

/*
fn sleep<E: SleepExecutor>() -> impl Generator<E, Yield = !, Return = ()> {
    await executor.sleep(); // No access to executor here. Must manually implement this. 
}*/