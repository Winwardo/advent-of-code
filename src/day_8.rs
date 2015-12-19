
pub struct CharacterCount<'a> {
    characters: &'a str,
}

impl<'a> CharacterCount<'a> {
    pub fn new(characters: &str) -> CharacterCount {
        CharacterCount { characters: characters }
    }

    pub fn overall(&self) -> usize {
        self.characters.chars().count()
    }

    pub fn inside(&self) -> usize {
        // First and last character must be quotes
        assert!(self.characters.chars().next().unwrap() == '"');
        assert!(self.characters.chars().rev().next().unwrap() == '"');

        let mut count = 0;
        let mut escape = false;
        let mut hex = 0;

        for c in self.characters.chars() {
            if hex > 0 {
                hex -= 1;
                if hex == 0 {
                    escape = false;
                }
            }

            if escape {
                match c {
                    '\\' | '\"' => {
                        escape = false;
                        count += 1;
                    }
                    'x' => {
                        hex = 2;
                    }
                    _ => {}
                }
            } else {
                if c == '\\' {
                    escape = true;
                } else {
                    count += 1;
                }
            }
        }
        count - 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn overall_count_empty_string_is_2() {
        let mut count = CharacterCount::new("\"\"");
        assert_eq!(2, count.overall());
    }

    #[test]
    fn inside_count_empty_string_is_0() {
        let mut count = CharacterCount::new("\"\"");
        assert_eq!(0, count.inside());
    }

    #[test]
    fn overall_count_abc_is_5() {
        let mut count = CharacterCount::new("\"abc\"");
        assert_eq!(5, count.overall());
    }

    #[test]
    fn inside_count_abc_is_3() {
        let mut count = CharacterCount::new("\"abc\"");
        assert_eq!(3, count.inside());
    }

    #[test]
    fn overall_count_with_escaped_quote_is_10() {
        let mut count = CharacterCount::new("\"aaa\\\"aaa\"");
        assert_eq!(10, count.overall());
    }

    #[test]
    fn inside_count_with_escaped_quote_is_7() {
        let mut count = CharacterCount::new("\"aaa\\\"aaa\"");
        assert_eq!(7, count.inside());
    }

    #[test]
    fn overall_count_with_hex_is_6() {
        let mut count = CharacterCount::new("\"\\x27\"");
        assert_eq!(6, count.overall());
    }

    #[test]
    fn inside_count_with_hex_is_1() {
        let mut count = CharacterCount::new("\"\\x27\"");
        assert_eq!(1, count.inside());
    }
}
