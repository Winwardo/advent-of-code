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
    let mut chars = input.chars();
    let mut chars_plus_one = input.chars().skip(1);

    let limit = input.len();
    for _ in 0..(limit - 3) {
        let l1 = chars.next();
        let r1 = chars_plus_one.next();

        // Now iterate ahead to find any matches
        let mut chars2 = chars.clone().skip(1);
        let mut chars2_plus_one = chars_plus_one.clone().skip(1);

        for _ in 0..(limit - 2) {
            let l2 = chars2.next();
            let r2 = chars2_plus_one.next();

            if l1 == l2 && r1 == r2 {
                return true;
            }
        }
    }

    false
}

pub fn contains_one_triplet(input: &str) -> bool {
    // It contains at least one letter which repeats with exactly one letter between them, like xyx,
    // abcdefeghi (efe), or even aaa.

    let mut chars = input.chars();
    let mut chars_compare = input.chars().skip(2);

    let limit = input.len() - 2;
    for _ in 0..limit {
        let l = chars.next();
        let r = chars_compare.next();

        if l == r {
            return true;
        }
    }

    false
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
        assert_eq!(false, contains_one_triplet(&"abcde"));
    }

    #[test]
    fn does_contain_two_overlapping_pairs_1() {
        assert_eq!(true, contains_two_nonoverlapping_pairs(&"xyxy"));
    }

    #[test]
    fn does_contain_two_overlapping_pairs_2() {
        assert_eq!(true, contains_two_nonoverlapping_pairs(&"aabcdefgaa"));
    }

    #[test]
    fn does_not_contain_two_overlapping_pairs_1() {
        assert_eq!(false, contains_two_nonoverlapping_pairs(&"aaa"));
    }
}
