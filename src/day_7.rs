use regex::Regex;
use std::collections::HashMap;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_7_hard.txt");

    let mut circuit = Circuit::new();

    for line in input {
        println!("{:?}", line);
        circuit.run_instruction(&line);
        // println!("{:?}", circuit.get("lx"));
    }

    println!("Instructions entered");

    // let answer = 7;
    let answer = circuit.get("a");
    println!("{:?}", answer);
}

pub struct Circuit {
    gates: HashMap<String, String>,
    computed_wire_results: HashMap<String, u16>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            gates: HashMap::new(),
            computed_wire_results: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> u16 {
        // Check memoization list
        match self.resolve_and_memoize(key) {
            Some(x) => return x,
            _ => panic!("Unable to resolve"),
        }
    }

    pub fn run_instruction(&mut self, instruction: &str) {
        let re = Regex::new(r"(.*) -> (.*)").unwrap();
        for cap in re.captures_iter(instruction) {
            let lhs = cap.at(1).unwrap();
            let rhs = cap.at(2).unwrap();

            self.gates.insert(rhs.to_string(), lhs.to_string());
        }
    }

    fn resolve_and_memoize(&mut self, variable_name: &str) -> Option<u16> {
        // Try to return the memoized value
        match self.computed_wire_results.get(variable_name) {
            Some(x) => return Some(*x),
            None => {}
        }

        // Else just resolve the value and return that instead
        match self.resolve(variable_name) {
            Some(x) => {
                self.computed_wire_results.insert(variable_name.to_string(), x);
                Some(x)
            }
            None => None,
        }
    }

    fn resolve(&mut self, variable_name: &str) -> Option<u16> {
        let definition = self.resolve_definition(&variable_name);

        // Test for as a single u16 definition
        match definition.parse::<u16>() {
            Ok(x) => {
                return Some(x);
            }
            Err(e) => {}
        };

        // Split up and try operators
        return self.resolve_by_splitting(&definition);
    }

    fn resolve_definition(&self, variable_name: &str) -> String {
        match self.gates.get(variable_name) {
            Some(x) => x.clone(),
            _ => variable_name.to_string(),
        }
    }

    fn resolve_by_splitting(&mut self, definition: &str) -> Option<u16> {
        let mut split = definition.split(" ");
        match split.clone().count() {
            // No space (one chunk) means it's a variable assignment
            1 => {
                return self.resolve_and_memoize(&definition);
            }
            // One space is a unary operator, and must be NOT
            2 => {
                let rhs = split.skip(1).next().unwrap();
                return match self.resolve_and_memoize(rhs) {
                    Some(x) => Some(!x),
                    None => None,
                };

            }
            // Two spaces must be a binary operator
            3 => {
                let left_ = self.resolve_and_memoize(split.next().unwrap());
                let operator = split.next().unwrap();
                let right_ = self.resolve_and_memoize(split.next().unwrap());

                if left_.is_none() {
                    return None;
                }
                if right_.is_none() {
                    return None;
                }

                let left = left_.unwrap();
                let right = right_.unwrap();

                let result = match operator {
                    "AND" => left & right,
                    "OR" => left | right,
                    "LSHIFT" => left << right,
                    "RSHIFT" => left >> right,
                    _ => 0,
                };

                return Some(result);
            }
            _ => {}
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_x_is_123() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        assert_eq!(123, circuit.get("x"));
    }

    #[test]
    fn set_xy_is_123() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> xy");
        assert_eq!(123, circuit.get("xy"));
    }

    #[test]
    fn set_x_is_123_then_overwrite() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        circuit.run_instruction("256 -> x");
        assert_eq!(256, circuit.get("x"));
    }

    #[test]
    fn set_y_is_x() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        circuit.run_instruction("x -> y");
        assert_eq!(123, circuit.get("y"));
    }

    #[test]
    fn set_d_using_AND() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        circuit.run_instruction("456 -> y");
        circuit.run_instruction("x AND y -> d");
        assert_eq!(72, circuit.get("d"));
    }

    #[test]
    fn set_e_using_OR() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        circuit.run_instruction("456 -> y");
        circuit.run_instruction("x OR y -> e");
        assert_eq!(507, circuit.get("e"));
    }

    #[test]
    fn set_f_using_LSHIFT() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        circuit.run_instruction("x LSHIFT 2 -> f");
        assert_eq!(492, circuit.get("f"));
    }

    #[test]
    fn set_g_using_RSHIFT() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("456 -> y");
        circuit.run_instruction("y RSHIFT 2 -> g");
        assert_eq!(114, circuit.get("g"));
    }

    #[test]
    fn set_h_using_NOT_literal() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("NOT 123 -> h");
        assert_eq!(65412, circuit.get("h"));
    }

    #[test]
    fn set_h_using_NOT_variable() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        circuit.run_instruction("NOT x -> h");
        assert_eq!(65412, circuit.get("h"));
    }

    #[test]
    fn set_h_wire_in_reverse() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("y -> h");
        circuit.run_instruction("x -> y");
        circuit.run_instruction("123 -> x");
        assert_eq!(123, circuit.get("h"));
    }
}
