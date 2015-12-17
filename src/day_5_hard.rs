pub fn print_basic_answer() {
    use file_reading::*;

    let input = read_file_as_lines("res\\day_5.txt");
    let answer = count_nice_strings(input);


    println!("{:?}", answer);
}

pub fn count_nice_strings(strings: Vec<String>) -> usize {
    return strings.iter().filter(|x| is_nice(&x)).count();
}

pub fn is_nice(input: &str) -> bool {
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nice_example_1() {
        assert_eq!(true, is_nice(&"qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn nice_example_2() {
        assert_eq!(true, is_nice(&"xxyxx"));
    }

    #[test]
    fn naughty_example_1() {
        assert_eq!(false, is_nice(&"uurcxstgmygtbstg"));
    }

    #[test]
    fn naughty_example_2() {
        assert_eq!(false, is_nice(&"ieodomkazucvgmuy"));
    }

    // #[test]
    
}
