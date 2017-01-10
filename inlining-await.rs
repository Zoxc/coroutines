#![crate_type="rlib"]
#![feature(core_intrinsics)]

pub enum Res<T> {
    Blocked,
    Val(T)
}

pub struct GenA(usize);

extern {
    pub fn hello() -> ();
    pub fn panic() -> !;
}

pub fn gen_a() -> GenA {
    GenA(0)
}

pub fn gen_a_resume(g: &mut GenA) -> Res<usize> {
    match g.0 {
        0 => {
            unsafe { hello() };
            g.0 = 1;
            Res::Blocked
        }
        1 => {
            unsafe { hello() };
            g.0 = 2;
            Res::Blocked
        }
        2 => {
            unsafe { hello() };
            g.0 = 3;
            Res::Blocked
        }
        3 => {
            unsafe { hello() };
            g.0 = 4;
            Res::Blocked
        }
        4 => {
            unsafe { hello() };
            g.0 = 5;
            Res::Blocked
        }
        _ => unsafe { std::intrinsics::unreachable() },
    }
}

pub struct GenB {
    state: usize,
    count: usize,
    a: GenA,
}

pub fn gen_b() -> GenB {
    GenB {
        state: 0,
        count: 0,
        a: gen_a(),
    }
}

pub fn gen_b_resume(g: &mut GenB) -> Res<usize> {
    match g.state {
        0 => {
            unsafe { hello() };
            g.count += 1;
            g.state = 1;
            Res::Blocked
        }
        1 => {
            match gen_a_resume(&mut g.a) {
                Res::Blocked => {
                    g.state = 1;
                    Res::Blocked
                }
                Res::Val(v) => {
                    g.state = 2;
                    g.count += v;
                    Res::Blocked
                }
            }
        }
        2 => {
            unsafe { hello() };
            Res::Val(g.count)
        }
        _ => unsafe { std::intrinsics::unreachable() },
    }
}

extern {
    fn block();
}

pub fn gen_a_test() {
    let g = gen_a();

    loop {
        match gen_b_resume(&mut g) {
            Res::Blocked => {
                unsafe { block() }
            }
            Res::Val(v) => {
                return v
            }
        }
    }
}