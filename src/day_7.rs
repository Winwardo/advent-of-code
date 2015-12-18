use std::collections::HashMap;

pub struct Circuit {
	wires: HashMap<&'static str, u32>,
}

impl Circuit {
	pub fn new() -> Circuit {
		Circuit {
			wires: HashMap::new(),
		}
	}

	pub fn get(&self, key: &str) -> u32 {
		let r = self.wires.get(key);

		match r {
			Some(x) => *x,
			None => 0
		}
	}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_nonexistent_is_zero() {
        let circuit = Circuit::new();
        assert_eq!(0, circuit.get("ab"));
    }
}