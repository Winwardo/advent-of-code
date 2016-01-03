use regex::Regex;
use rustc_serialize::json::*;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file("res\\day_12.txt");
    let answer = get_sum(&input);
    println!("{:?}", answer);
}

pub fn get_sum(input: &str) -> i64 {
    let data = Json::from_str(input).unwrap();
    sum_from_json(&data)
}

fn sum_from_json(json: &Json) -> i64 {
    match json {
        &Json::Array(ref a) => sum_array(a),
        &Json::Object(ref o) => sum_object(o),
        &Json::I64(n) => n as i64,
        &Json::U64(n) => n as i64,
        &Json::F64(n) => n as i64,
        &Json::String(_) => 0,
        &Json::Boolean(_) => 0,
        &Json::Null => {
            panic!("No nulls should be present.");
        }
    }
}

fn sum_array(array: &Array) -> i64 {
    array.into_iter().map(|object| sum_from_json(&object)).fold(0, |sum, val| sum + val)
}

fn sum_object(object: &Object) -> i64 {
    let mut sum = 0;
    for json in object.values() {
        match json {
            &Json::String(ref s) if s == "red" => {
                return 0;
            }
            json @ _ => sum += sum_from_json(&json),
        }
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

    #[test]
    fn count_object_with_one_red_partially_ignored() {
        assert_eq!(4, get_sum("[1,{\"c\":\"red\",\"b\":2},3]"));
    }

    #[test]
    fn count_object_with_one_red_all_ignored() {
        assert_eq!(0, get_sum("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"));
    }

    #[test]
    fn count_object_with_one_red_in_array_so_ignored() {
        assert_eq!(6, get_sum("[1,\"red\",5]"));
    }
}
