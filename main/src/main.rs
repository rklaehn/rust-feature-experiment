fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use lib::qc::testdata;

    #[test]
    fn test() {
        println!("{:x?}", testdata.iter().take(100).collect::<Vec<_>>());
    }
}