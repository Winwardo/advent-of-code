pub fn is_nice(input: &str) -> bool {
    return
    	contains_three_vowels(input) &&
    	contains_a_double_character(input) &&
    	does_not_contain_offending_string(input);
}

pub fn contains_three_vowels(input: &str) -> bool {
    return vowel_count(input) >= 3;
}

pub fn contains_a_double_character(input: &str) -> bool {
	let mut last : char = '\x00';
    for c in input.chars(){
    	if c == last {
    		return true;
    	} else {
    		last = c;
    	}
    }

    return false;
}

pub fn does_not_contain_offending_string(input: &str) -> bool {
	return !(
		input.contains("ab") ||
		input.contains("cd") ||
		input.contains("pq") ||
		input.contains("xy")
	);
}

pub fn vowel_count(input: &str) -> u8 {
    let mut count = 0;
    for c in input.chars() {
        if is_vowel(&c) {
            count += 1;
        }
    }
    return count;
}

pub fn is_vowel(input: &char) -> bool {
    return match *input {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nice_example_1() {
        assert_eq!(true, is_nice(&"ugknbfddgicrmopn"));
    }

    #[test]
    fn nice_example_2() {
        assert_eq!(true, is_nice(&"aaa"));
    }

    #[test]
    fn naughty_example_1() {
        assert_eq!(false, is_nice(&"jchzalrnumimnmhp"));
    }

    #[test]
    fn naughty_example_2() {
        assert_eq!(false, is_nice(&"haegwjzuvuyypxyu"));
    }

    #[test]
    fn naughty_example_3() {
        assert_eq!(false, is_nice(&"dvszwmarrgswjxmb"));
    }

    #[test]
    fn check_is_vowel() {
        assert_eq!(true, is_vowel(&'a'));
    }

    #[test]
    fn check_is_not_vowel() {
        assert_eq!(false, is_vowel(&'b'));
    }

    #[test]
    fn vowel_count_none() {
        assert_eq!(0, vowel_count(&"qwr"));
    }

    #[test]
    fn vowel_count_one() {
        assert_eq!(1, vowel_count(&"a"));
    }

    #[test]
    fn vowel_count_two_different() {
        assert_eq!(2, vowel_count(&"ae"));
    }

    #[test]
    fn vowel_count_two_same() {
        assert_eq!(2, vowel_count(&"aa"));
    }

    #[test]
    fn vowel_count_all_vowels() {
        assert_eq!(5, vowel_count(&"aeiou"));
    }

    #[test]
    fn contains_three_vowels_true_1() {
        assert_eq!(true, contains_three_vowels(&"aei"));
    }

    #[test]
    fn contains_three_vowels_true_2() {
        assert_eq!(true, contains_three_vowels(&"xazegov"));
    }

    #[test]
    fn contains_three_vowels_true_3() {
        assert_eq!(true, contains_three_vowels(&"aeiouaeiouaeiou"));
    }

    #[test]
    fn contains_three_vowels_false_1() {
        assert_eq!(false, contains_three_vowels(&"ae"));
    }

    #[test]
    fn contains_three_vowels_false_2() {
        assert_eq!(false, contains_three_vowels(&"xazegv"));
    }

    #[test]
    fn does_contain_a_double_character_1() {
        assert_eq!(true, contains_a_double_character(&"abcdde"));
    }

    #[test]
    fn does_contain_a_double_character_2() {
        assert_eq!(true, contains_a_double_character(&"aabbccdd"));
    }

    #[test]
    fn does_not_contain_a_double_character_1() {
        assert_eq!(false, contains_a_double_character(&"abcde"));
    }

    #[test]
    fn does_contain_offending_string_all() {
    	assert_eq!(false, does_not_contain_offending_string(&("ab")));
    	assert_eq!(false, does_not_contain_offending_string(&("cd")));
    	assert_eq!(false, does_not_contain_offending_string(&("pq")));
    	assert_eq!(false, does_not_contain_offending_string(&("xy")));
    }

    #[test]
    fn does_contain_offending_string_1() {
    	assert_eq!(false, does_not_contain_offending_string(&("some_long_string_with_ab_in_it")));
    }

    #[test]
    fn does_not_contain_offending_string_1() {
    	assert_eq!(true, does_not_contain_offending_string(&("some_safe_string")));
    }
}
