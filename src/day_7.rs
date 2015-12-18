use regex::Regex;
use std::collections::HashMap;

pub struct Circuit {
    wires: HashMap<String, u16>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit { wires: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> u16 {
        let r = self.wires.get(key);

        match r {
            Some(x) => *x,
            None => 0,
        }
    }

    pub fn run_instruction(&mut self, instruction: &str) {
        let re = Regex::new(r"(.*) -> (.*)").unwrap();
        for cap in re.captures_iter(instruction) {
            let lhs = cap.at(1).unwrap();
            let rhs = cap.at(2).unwrap();

            self.set_val(lhs, rhs);
        }
    }

    fn get_actual_value(&self, given_value: &str) -> u16 {
        // Test if we're just a simple number assignment
        match given_value.parse::<u16>() {
            Ok(x) => return x,
            _ => {}
        };

        let mut split = given_value.split(" ");
        match split.clone().count() {
            // No space (one chunk) means it's a variable assignment
            1 => self.get(given_value),
            // One space is a unary operator, and must be NOT
            2 => !self.get_actual_value(split.skip(1).next().unwrap()),
            // Two spaces must be a binary operator
            3 => {
                let left = self.get_actual_value(split.next().unwrap());
                let operator = split.next().unwrap();
                let right = self.get_actual_value(split.next().unwrap());

                match operator {
                    "AND" => left & right,
                    "OR" => left | right,
                    "LSHIFT" => left << right,
                    "RSHIFT" => left >> right,
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    fn set_val(&mut self, lhs: &str, wire: &str) {
        let value = self.get_actual_value(lhs);
        self.wires.insert(wire.to_string(), value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_nonexistent_is_zero() {
        let circuit = Circuit::new();
        assert_eq!(0, circuit.get("x"));
    }

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
    fn set_h_using_NOT() {
        let mut circuit = Circuit::new();
        circuit.run_instruction("123 -> x");
        circuit.run_instruction("NOT x -> h");
        assert_eq!(65412, circuit.get("h"));
    }
}
