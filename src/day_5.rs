pub fn is_nice(input: &str) -> bool {
    return false;
}

fn contains_three_vowels(input: &str) -> bool {
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

    #[test]
    fn contains_three_vowels_true_1() {
        assert_eq!(true, is_nice(&"aei"));
    }

    #[test]
    fn contains_three_vowels_true_2() {
        assert_eq!(true, is_nice(&"xazegov"));
    }

    #[test]
    fn contains_three_vowels_true_3() {
        assert_eq!(true, is_nice(&"aeiouaeiouaeiou"));
    }

    #[test]
    fn contains_three_vowels_false_1() {
        assert_eq!(false, is_nice(&"ae"));
    }

    #[test]
    fn contains_three_vowels_false_2() {
        assert_eq!(false, is_nice(&"xazegv"));
    }
}
