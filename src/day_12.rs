use regex::Regex;

pub fn print_answer() {
    let answer = "";
    println!("{:?}", answer);
}

pub fn get_sum(input: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"(-?\d+)*").unwrap();
    for cap in re.captures_iter(input) {
        sum += match cap.at(0).unwrap().parse::<i32>() {
            Ok(q) => q,
            Err(_) => 0,
        };
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_array() {
        assert_eq!(6, get_sum("[1,2,3]"));
    }

    #[test]
    fn count_nested_array() {
        assert_eq!(3, get_sum("[[[3]]]"));
    }

    #[test]
    fn count_object() {
        assert_eq!(6, get_sum("{\"a\":2,\"b\":4}"));
    }

    #[test]
    fn count_object_with_negative() {
        assert_eq!(3, get_sum("{\"a\":{\"b\":4},\"c\":-1}"));
    }
}
