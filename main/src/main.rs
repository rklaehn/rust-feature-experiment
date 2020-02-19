fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;
    use lib::{fun, Foo, quickcheck::TESTDATA};

    #[test]
    fn test() {
        println!("{:x?}", TESTDATA.iter().take(100).collect::<Vec<_>>());
    }

    quickcheck!{

        fn fun_check(x: Foo) -> bool {
            x.0 * 2 == fun(x)
        }
    }
}