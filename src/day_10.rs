use regex::Regex;

pub fn print_answer() {
    let mut l = LookAndSay::new("3113322113");
    let answer = l.skip(49).next().unwrap();
    println!("{:?}", answer.len());
}

pub struct LookAndSay {
    current_val: String,
}

impl LookAndSay {
    pub fn new<S: Into<String>>(current_val: S) -> LookAndSay {
        LookAndSay { current_val: current_val.into() }
    }
}

impl Iterator for LookAndSay {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        fn try_print_character(new_string: &mut Vec<char>, count: u32, last_digit: char) {
            if count > 0 {
                new_string.push(match count {
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    9 => '9',
                    _ => 'a',
                });
                new_string.push(last_digit);
            }
        }

        let mut last_digit = '0';
        let mut count = 0;
        let mut new_string = vec![];

        for c in self.current_val.chars() {
            if c != last_digit {
                try_print_character(&mut new_string, count, last_digit);
                last_digit = c;
                count = 1;
            } else {
                count += 1;
            }
        }

        try_print_character(&mut new_string, count, last_digit);

        self.current_val = new_string.into_iter().collect();
        Some(self.current_val.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_1() {
        let mut l = LookAndSay::new("1");
        assert_eq!(Some("11".to_string()), l.next());
    }

    #[test]
    fn try_11() {
        let mut l = LookAndSay::new("11");
        assert_eq!(Some("21".to_string()), l.next());
    }

    #[test]
    fn try_1_twice() {
        let mut l = LookAndSay::new("1");
        assert_eq!(Some("11".to_string()), l.next());
        assert_eq!(Some("21".to_string()), l.next());
    }

    #[test]
    fn try_21() {
        let mut l = LookAndSay::new("21");
        assert_eq!(Some("1211".to_string()), l.next());
    }

    #[test]
    fn try_1211() {
        let mut l = LookAndSay::new("1211");
        assert_eq!(Some("111221".to_string()), l.next());
    }

    #[test]
    fn try_111221() {
        let mut l = LookAndSay::new("111221");
        assert_eq!(Some("312211".to_string()), l.next());
    }

}
