pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_10.txt");

    // let mut circuit = Circuit::new();

    for line in input {
        // circuit.run_instruction(&line);
    }

    // let answer = circuit.get("a");
    // println!("{:?}", answer);
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
        None
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
