pub struct Lights {
	lights: Vec<u8>
}

impl Lights {
	pub fn new() -> Lights {
		Lights {
			lights: vec![0; 1000000]
		}
	}

	pub fn turned_on(&self) -> u16 {
		return 0;
	}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_1() {
    	let lights = Lights::new();
    	assert_eq!(0, lights.turned_on());
    }
}