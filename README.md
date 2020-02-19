# Rust feature experiments

Can you provide test tooling for a crate under a feature flag?

E.g. you want to have some arbitraries in your crate for internal use but also for use by other crates. You do not want to have the quickcheck dependency when compiling the binary.

## This arbitrary will be available both in tests in lib, and in crates that use this crate and use the "test" feature flag...

```rust
#[cfg(any(test, feature="test"))]
pub mod quickcheck {
    use super::Foo;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;

    impl Arbitrary for Foo {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Self(g.gen())
        }
    }
}
```

## Dependencies need to be imported separately for the test scope and for the test feature...

```toml

[dependencies]
quickcheck = { version = "0.9.2", optional = true }
rand = { version = "0.7.3", optional = true } 

[dev-dependencies]
quickcheck = "0.9.2"
rand = "0.7.3"

[features]
test = ["quickcheck", "rand"]
```

## And this is how you use it from another crate:

```toml
[dependencies]
lib = { path = "../lib" }

[dev-dependencies]
lib = { path = "../lib", features = ["test"] }
```

## Does it work

The quickcheck dependency does not get included in the main binary, just in the test binaries...

```bash
$ ls -l target/release/main-c1e994908510bec7
-rwxr-xr-x  2 rklaehn  staff  3034688 Feb 19 22:44 target/release/main-c1e994908510bec7
$ ls -l target/release/main
-rwxr-xr-x  2 rklaehn  staff  276376 Feb 19 22:52 target/release/main
```
