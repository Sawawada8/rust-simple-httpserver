
pub fn handle() {
    println!("handle");
}

pub fn demo() -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_test() {
        assert_eq!(2, demo());
    }
}
