#![crate_type="rlib"]
#![feature(conservative_impl_trait)]

trait Gen {
    type Prime: Gen;
    fn prime(self) -> Self::Prime;
}

struct Test<'a>(&'a usize);
struct TestP(usize);

impl Gen for TestP {
    type Prime = TestP;

    fn prime(self) -> Self::Prime {
        self
    }
}

impl<'a> Gen for Test<'a> {
    type Prime = TestP;

    fn prime(self) -> Self::Prime {
        TestP(*self.0)
    }
}

fn test<'a>(a: &'a usize) -> impl Gen<where Prime: 'static> + 'a {
    Test(a)
}

fn test_wrapper<'a>(a: &'a usize) -> impl Gen {
    TestP(5)
   // panic!()
    //test(a).prime()
}

static TESTA: usize = 5;

fn a() -> impl Gen + 'static {
    let a = 4;
    test_wrapper(&TESTA)
}