#![feature(never_type)]
#![feature(fundamental)]
#![feature(conservative_impl_trait)]

extern crate coroutine;
extern crate future_traits;
use coroutine::*;
use future_traits::*;
use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::Cell;
use std::marker::Sized;

pub struct Map<A, F> {
    future: A,
    f: Option<F>,
}

pub fn map<E: Executor, U, A, F>(future: A, f: F) -> impl Generator<E>
    where A: Generator<E, Yield=!>,
          F: FnOnce(A::Return) -> U 
{
    Map {
        future: future,
        f: Some(f),
    }
}

impl<E: Executor, U, A, F> Generator<E> for Map<A, F>
    where A: Generator<E, Yield=!>,
          F: FnOnce(A::Return) -> U 
{
    type Return = U;
    type Yield = !;

    fn resume(&mut self, executor: &mut E) -> State<!, Self::Return, E::Blocked> {
        match self.future.resume(executor) {
            State::Blocked(b) => State::Blocked(b),
            State::Complete(r) => State::Complete((self.f.take().expect("cannot poll Map twice"))(r)),
            State::Yielded(..) => unreachable!(),
        }
    }
}


/*
// A Future is a Generator<Yield = !> where the executor is passed by &mut E
pub trait Future2<E: Executor>: Stream<E, Yield = !> {
    fn future_only(&self) {}
}

impl<E: Executor, T: Stream<E, Yield = !>> Future2<E> for T {
}
*/
// Cannot do Stream only operations given the above def. Is Stream only operations useful. Probably since Future streams return ()
// Can Stream just be trait StreamOnly<E>: Stream<E, Return = ()>?
// Stream only operations are probably very useful to remove from futures, since all of them would operate on the ! values
// What error do you get if you use such a value?

//struct HH(Box<Future<(), Return=()>>);

type Task = Rc<RefCell<Generator<EventLoop, Return=(), Yield=!>>>;

pub struct EventLoop {
    current: Option<Task>,
    timers: Vec<Rc<Timer>>,
}

pub struct Timer {
    remaining: Cell<u64>,
    task: Task,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            current: None,
            timers: Vec::new(),
        }
    }

    fn timer(&mut self, task: Task, delta: u64) -> Rc<Timer> {
        let timer = Rc::new(Timer {
            remaining: Cell::new(delta),
            task: task,
        });
        self.timers.push(timer.clone());
        timer
    }

    fn run_task(&mut self, task: Task) {
        self.current = Some(task.clone());
        task.borrow_mut().resume(self);
        self.current = None;
    }

    pub fn run<F: Generator<Self, Return=(), Yield=!> + 'static>(&mut self, future: F) {
        let task = Rc::new(RefCell::new(future));

        if self.current.is_some() {
            // We are currently inside the event loop, add the task to the list of tasks to run
            self.timer(task, 0);
            return;
        }

        self.run_task(task);

        while !self.timers.is_empty() {
            let mut i = 0;

            while i < self.timers.len() {
                if self.timers[i].remaining.get() == 0 {
                    let task = self.timers[i].task.clone();
                    self.run_task(task);
                    self.timers.remove(i);
                } else {
                    let remaining = self.timers[i].remaining.get();
                    self.timers[i].remaining.set(remaining - 1);
                    i += 1;
                }
            }

            thread::sleep(Duration::from_millis(1));
        }
    }
}

impl Executor for EventLoop {
    type Blocked = ();
}

pub struct RPC;

impl<E: Executor> Generator<E> for RPC {
    type Return = usize;
    type Yield = !;

    fn resume(&mut self, executor: &mut E) -> State<!, Self::Return, E::Blocked> {
        State::Complete(1)
    }
}

/*
pub struct Pong<T>(pub Option<T>);

impl<T, E: Executor> Future<E> for Pong<T> {
    type Return = T;

    fn poll(&mut self, executor: &mut E) -> State<!, Self::Return, E::Blocked> {
        State::Complete(self.0.take().unwrap())
    }
}
*/
pub trait SleepExecutor: Executor where Self:Sized {
    type Sleep: Future<Self, Return=()>;
    fn sleep(delta: u64) -> Self::Sleep;
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
    type Return = ();

    fn poll(&mut self, executor: FutureExecutor<FutureExecutor<E>>) -> State<!, Self::Return, E::Blocked> {
        //self.0.poll();
        panic!()
    }
}
*/
impl SleepExecutor for () {
    type Sleep = SyncSleep;

    fn sleep(delta: u64) -> Self::Sleep {
        SyncSleep(delta)
    }
}

pub struct SyncSleep(u64);

impl Generator<()> for SyncSleep {
    type Return = ();
    type Yield = !;

    fn resume(&mut self, executor: &mut ()) -> State<!, Self::Return, !> {
        thread::sleep(Duration::from_millis(self.0));
        State::Complete(())
    }
}

impl SleepExecutor for EventLoop {
    type Sleep = AsyncSleep;

    fn sleep(delta: u64) -> Self::Sleep {
        AsyncSleep(SleepState::Pending(delta))
    }
}

enum SleepState {
    Pending(u64),
    Started(Rc<Timer>)
}

pub struct AsyncSleep(SleepState);

impl Generator<EventLoop> for AsyncSleep {
    type Return = ();
    type Yield = !;

    fn resume(&mut self, executor: &mut EventLoop) -> State<!, Self::Return, ()> {
        match self.0 {
            SleepState::Pending(delta) => {
                let task = executor.current.as_ref().unwrap().clone();
                self.0 = SleepState::Started(executor.timer(task, delta));
                State::Blocked(())
            }
            SleepState::Started(ref timer) => if timer.remaining.get() == 0 {
                State::Complete(())
            } else {
                State::Blocked(())
            }
        }
    }
}

// can do await E::sleep(343)

pub struct ArgsExecutor<T>(Option<T>);

impl<T> ArgsExecutor<T> {
    pub fn new(args: T) -> Self {
        ArgsExecutor(Some(args))
    }
}

impl<T> Executor for ArgsExecutor<T> {
    type Blocked = !;
}

pub struct ArgsExtractor;

impl<T> Generator<ArgsExecutor<T>> for ArgsExtractor {
    type Yield = !;
    type Return = T;

    fn resume(&mut self, executor: &mut ArgsExecutor<T>) -> State<!, Self::Return, !> {
        State::Complete(executor.0.take().expect("arguments already extracted"))
    }
}