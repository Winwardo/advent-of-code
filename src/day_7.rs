use regex::Regex;
use std::collections::HashMap;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_7.txt");

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
    wires2: HashMap<String, String>,
    wire_vals: HashMap<String, u16>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            wires2: HashMap::new(),
            wire_vals: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> u16 {
        {
            // Check memoization list
            match self.wire_vals.get_mut(key) {
                Some(x) => return *x,
                _ => {}
            }


            let to_reduce = self.wires2.get(key).unwrap();
            println!("key: {:?}, to_reduce: {:?}", key, to_reduce);
            match self.attempt_reduce(to_reduce) {
                Some(x) => x,
                _ => panic!("Poo"),
            }

// 3
        }
    }

    pub fn run_instruction(&mut self, instruction: &str) {
        let re = Regex::new(r"(.*) -> (.*)").unwrap();
        for cap in re.captures_iter(instruction) {
            let lhs = cap.at(1).unwrap();
            let rhs = cap.at(2).unwrap();

            // self.set_val(lhs, rhs);


            self.wires2.insert(rhs.to_string(), lhs.to_string());

            // Try to memoize
            match self.attempt_reduce(lhs) {
                Some(x) => {
                    self.wire_vals.insert(rhs.to_string(), x);
                }
                _ => {}
            };
        }
    }

    fn attempt_reduce(&self, to_reduce: &str) -> Option<u16> {
        // println!("{:?}", to_reduce);
        // Test if we're just a simple number assignment
        match to_reduce.parse::<u16>() {
            Ok(x) => {
                return Some(x);
            }
            _ => {}
        };

        // Split up and try operators
        let mut split = to_reduce.split(" ");
        match split.clone().count() {
            // No space (one chunk) means it's a variable assignment
            1 => {
                match self.wire_vals.get(to_reduce) {
                    Some(x) => Some(*x),
                    _ => {
                        let next_level = self.wires2.get(to_reduce);
                        match (next_level) {
                            Some(x) => { return self.attempt_reduce(x); },
                            _ => None,
                        }
                    },
                }
            }
            // One space is a unary operator, and must be NOT
            2 => {
                let num = split.skip(1).next().unwrap();
                match self.attempt_reduce(num) {
                    Some(x) => Some(!x),
                    _ => None,
                }
            }
            // Two spaces must be a binary operator
            3 => {
                let left_ = self.attempt_reduce(split.next().unwrap());
                let operator = split.next().unwrap();
                let right_ = self.attempt_reduce(split.next().unwrap());

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
                    _ => return None,
                };

                Some(result)
            }
            _ => None,
        }
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
