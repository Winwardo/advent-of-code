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
    sum_from_json(data)
}

fn sum_from_json(json: Json) -> i64 {
    let mut sum = 0i64;
    match json {
        Json::Array(a) => {
            for object in a { 
                sum += sum_from_json(object);
            }
        }
        Json::Object(o) => {
            for value in o.values() {
                sum += sum_from_json(value.clone());
            }
        }
        Json::Null => {
            panic!("No nulls should be present.");
        }
        Json::I64(n) => {
            sum += n as i64;
        }
        Json::U64(n) => {
            sum += n as i64;
        }
        Json::F64(n) => {
            sum += n as i64;
        }
        _ => {
            println!("poo");
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
}
