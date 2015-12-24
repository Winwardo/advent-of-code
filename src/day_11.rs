pub fn print_answer() {}

pub fn increment_password(password: &str) -> String {
    let above_z = 'z' as u8 + 1;

    let mut bytes = password.as_bytes().to_owned();

    // First increment the final character.
    let x = bytes.len() - 1;
    bytes[x] += 1;

    // Search backwards through the password for characters that
    // have rolled over, and increment the previous character
    for x in (1..bytes.len()).rev() {
        if bytes[x] == above_z {
            bytes[x] = 'a' as u8;
            bytes[x - 1] += 1;
        }
    }


    String::from_utf8(bytes).unwrap()
}

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

pub fn contains_two_different_non_overlapping_pairs(password: &str) -> bool {
    let mut first_repeated_character: Option<u8> = None;

    for pair in password.as_bytes().windows(2) {
        if pair[0] == pair[1] {
            let character = Some(pair[0]);

            if first_repeated_character == None {
                first_repeated_character = character;
            } else {
                if first_repeated_character != character {
                    return true;
                }
            }
        }
    }

    false
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

    #[test]
    fn contains_two_pairs_true_1() {
        assert_eq!(true, contains_two_different_non_overlapping_pairs("aabb"));
    }

    #[test]
    fn contains_two_pairs_true_2() {
        assert_eq!(true,
                   contains_two_different_non_overlapping_pairs("helloworrld"));
    }

    #[test]
    fn contains_two_pairs_false_1() {
        assert_eq!(false, contains_two_different_non_overlapping_pairs("aaa"));
    }

    #[test]
    fn contains_two_pairs_false_2() {
        assert_eq!(false, contains_two_different_non_overlapping_pairs("aaaa"));
    }

    #[test]
    fn contains_two_pairs_false3() {
        assert_eq!(false,
                   contains_two_different_non_overlapping_pairs("helloworld"));
    }

    #[test]
    fn increment_password_1() {
        assert_eq!("xy", increment_password("xx"));
    }

    #[test]
    fn increment_password_2() {
        assert_eq!("xz", increment_password("xy"));
    }

    #[test]
    fn increment_password_3() {
        assert_eq!("ya", increment_password("xz"));
    }

    #[test]
    fn increment_password_4() {
        assert_eq!("yb", increment_password("ya"));
    }

    #[test]
    fn increment_password_5() {
        assert_eq!("baa", increment_password("azz"));
    }

    #[test]
    fn increment_password_middle() {
        assert_eq!("azba", increment_password("azaz"));
    }

    #[test]
    fn increment_password_long() {
        assert_eq!("baaaaa", increment_password("azzzzz"));
    }
}
