pub fn fun(a: Foo) -> usize {
    a.0 * 2
}

#[derive(Clone, Debug)]
pub struct Foo(usize);

#[cfg(any(test, feature="test"))]
pub mod qc {
    use super::{Foo, fun};
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use rand::Rng;

    pub const testdata: &[u8] = include_bytes!("testdata.bin");

    impl Arbitrary for Foo {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Self(g.gen())
        }
    }

    quickcheck!{

        fn fun_check(x: Foo) -> bool {
            true
        }
    }

    #[test]
    fn fun_test() {
        assert_eq!(fun(Foo(3)), 6);
    }
}

