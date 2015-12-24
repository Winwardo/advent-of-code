pub fn print_answer() {}

pub fn does_not_contain_dangerous_letters(password: &str) -> bool {
    for c in password.chars() {
        match c {
            'i' | 'o' | 'l' => {
                return false;
            }
            _ => {}
        }
    }

    true
}

pub fn contains_straight(password: &str) -> bool {
    for triple in password.as_bytes().windows(3) {
        if is_straight((triple[0] as char, triple[1] as char, triple[2] as char)) {
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
