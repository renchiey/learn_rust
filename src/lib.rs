pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn equal_value(value: i32) -> bool {
    if value == 1 {
        true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        assert!(equal_value(2), "ghay");
    }

    #[test]
    #[should_panic]
    fn what_is_dis() {
        panic!("This will panic");
    }
}
