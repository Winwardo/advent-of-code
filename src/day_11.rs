pub fn print_answer() {}

pub fn contains_straight(password: &str) -> bool {
    false
}

pub fn is_straight(triple: (char, char, char)) -> bool {
    let (a, b, c) = triple;
    (b as u8 - a as u8) == 1 && (c as u8 - b as u8) == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_straight_true() {
        assert_eq!(true, is_straight(('a', 'b', 'c')));
        assert_eq!(true, is_straight(('x', 'y', 'z')));
    }

    #[test]
    fn is_not_straight() {
        assert_eq!(false, is_straight(('a', 'c', 'b')));
        assert_eq!(false, is_straight(('a', 'b', 'd')));
    }

    #[test]
    fn contains_straight_positive_1() {
        assert_eq!(true, contains_straight("helloabcworld"));
    }

    #[test]
    fn contains_straight_positive_2() {
        assert_eq!(true, contains_straight("xyz"));
    }

    #[test]
    fn contains_straight_negative_1() {
        assert_eq!(false, contains_straight("helloworld"));
    }

    #[test]
    fn contains_straight_negative_2() {
        assert_eq!(false, contains_straight("axbycz"));
    }
}
