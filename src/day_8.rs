use std::str;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_8.txt");

    let mut outside = 0;
    let mut inside = 0;
    let mut encoded = 0;

    for line in input {
        let cc = CharacterCount::new(&line);
        let encoded_s = encode_str(&line);
        let cc_encoded = CharacterCount::new(&encoded_s);

        encoded += cc_encoded.overall();
        outside += cc.overall();
        inside += cc.inside();
    }

    println!("{} - {} = {}", encoded, outside, encoded - outside);
}

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
        let mut escape = 0;

        for c in self.characters.chars() {
            if escape > 0 {
                escape -= 1;
                if escape == 0 && c == 'x' {
                    escape += 2;
                }
            } else {
                if c == '\\' {
                    escape = 1;
                }

                count += 1;
            }
        }
        count - 2
    }
}

pub fn encode_str(input: &str) -> String {
    let mut output: Vec<u8> = vec![];
    output.push('\"' as u8);

    for c in input.chars() {
        match c {
            '\"' => {
                output.push('\\' as u8);
                output.push('\"' as u8);
            }
            '\\' => {
                output.push('\\' as u8);
                output.push('\\' as u8);
            }
            _ => {
                output.push(c as u8);
            }
        }
    }

    output.push('\"' as u8);

    match str::from_utf8(&output[..]) {
        Ok(x) => x.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence."),
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

    #[test]
    fn overall_count_encoded_empty_string_is_6() {
        let encoded_string = encode_str(&"\"\"");
        let mut count = CharacterCount::new(&encoded_string);
        assert_eq!(6, count.overall());
    }

    #[test]
    fn overall_count_encoded_with_hex_is_6() {
        let encoded_string = encode_str(&"\"\\x27\"");
        let mut count = CharacterCount::new(&encoded_string);
        assert_eq!(11, count.overall());
    }
}
