pub fn is_nice(input: &str) -> bool {
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nice_example_1() {
        assert_eq!(true, is_nice(&"ugknbfddgicrmopn"));
    }

    #[test]
    fn nice_example_2() {
        assert_eq!(true, is_nice(&"aaa"));
    }

    #[test]
    fn naughty_example_1() {
        assert_eq!(false, is_nice(&"jchzalrnumimnmhp"));
    }

    #[test]
    fn naughty_example_2() {
        assert_eq!(false, is_nice(&"haegwjzuvuyypxyu"));
    }

    #[test]
    fn naughty_example_3() {
        assert_eq!(false, is_nice(&"dvszwmarrgswjxmb"));
    }
}
