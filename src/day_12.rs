pub fn print_answer() {
    let answer = "";
    println!("{:?}", answer);
}

pub fn get_sum(input: &str) -> i32 {
    0
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
