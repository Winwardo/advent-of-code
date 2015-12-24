pub fn print_answer() {}

pub fn contains_straight(password: &str) -> bool {
    let mut iter_a = password.chars();
    let mut iter_b = password.chars().skip(1);
    let mut iter_c = password.chars().skip(2);

    for c in iter_c {
        let a = iter_a.next().unwrap();
        let b = iter_b.next().unwrap();
        if is_straight((a, b, c)) {
            return true;
        }
    }
    false
}

pub fn is_straight(triple: (char, char, char)) -> bool {
    let (a, b, c) = triple;
    (b as i8 - a as i8) == 1 && (c as i8 - b as i8) == 1
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
