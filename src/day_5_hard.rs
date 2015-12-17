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
    return contains_two_nonoverlapping_pairs(input) && contains_one_triplet(input);
}

pub fn contains_two_nonoverlapping_pairs(input: &str) -> bool {
    // It contains a pair of any two letters that appears at least twice in the string without
    // overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    return false;
}

pub fn contains_one_triplet(input: &str) -> bool {
    // It contains at least one letter which repeats with exactly one letter between them, like xyx,
    // abcdefeghi (efe), or even aaa.
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

    #[test]
    fn does_contain_one_triplet_1() {
        assert_eq!(true, contains_one_triplet(&"xyx"));
    }

    #[test]
    fn does_contain_one_triplet_2() {
        assert_eq!(true, contains_one_triplet(&"abcdefeghi"));
    }

    #[test]
    fn does_contain_one_triplet_3() {
        assert_eq!(true, contains_one_triplet(&"aaa"));
    }

    #[test]
    fn does_not_contain_one_triplet_2() {
        assert_eq!(true, contains_one_triplet(&"abcde"));
    }
}
