mod handle;

fn main() {
    println!("Hello, world! main");
    handle::handle();
}

#[cfg(test)]
mod tests {
    #[test]
    fn demo_test() {
        assert_eq!(4, 2 + 2);
    }
}
