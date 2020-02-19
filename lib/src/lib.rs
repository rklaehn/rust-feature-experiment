pub fn fun(a: Foo) -> usize {
    a.0 * 2
}

#[derive(Clone, Debug)]
pub struct Foo(pub usize);

#[cfg(any(test, feature="test"))]
pub mod quickcheck {
    use super::Foo;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;

    pub const TESTDATA: &[u8] = include_bytes!("testdata.bin");

    impl Arbitrary for Foo {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Self(g.gen())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Foo, fun};
    use quickcheck::quickcheck;

    quickcheck!{

        fn fun_check(x: Foo) -> bool {
            x.0 * 2 == fun(x)
        }
    }

    #[test]
    fn fun_test() {
        assert_eq!(fun(Foo(3)), 6);
    }
}

