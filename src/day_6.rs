pub struct Lights {
    lights: Vec<bool>,
}

impl Lights {
    pub fn new() -> Lights {
        Lights { lights: vec![false; 1000000] }
    }

    pub fn turn_on(&mut self, start: (u32, u32), end: (u32, u32)) {
        self.set_at_group(start, end, true);
    }

    pub fn turn_off(&mut self, start: (u32, u32), end: (u32, u32)) {
        self.set_at_group(start, end, false);
    }

    fn set_at_group(&mut self, start: (u32, u32), end: (u32, u32), value: bool) {
        let (x1, y1) = start;
        let (x2, y2) = end;

        for y in y1..(y2 + 1) {
            for x in x1..(x2 + 1) {
                self.set_at(x, y, value);
            }
        }
    }

    fn toggle(&mut self, start: (u32, u32), end: (u32, u32)) {
        let (x1, y1) = start;
        let (x2, y2) = end;

        for y in y1..(y2 + 1) {
            for x in x1..(x2 + 1) {
                self.toggle_at(x, y);
            }
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * 1000) as usize
    }

    fn set_at(&mut self, x: u32, y: u32, value: bool) {
        let position = self.get_index(x, y);
        self.lights[position] = value;
    }

    fn toggle_at(&mut self, x: u32, y: u32) {
        let position = self.get_index(x, y);
        self.lights[position] = !self.lights[position];
    }

    pub fn turned_on(&self) -> usize {
        return self.lights.iter().filter(|x| **x).count();
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

    #[test]
    fn count_all_lights_on() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0), (999, 999));
        assert_eq!(1000000, lights.turned_on());
    }

    #[test]
    fn count_first_horizontal_line() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0), (999, 0));
        assert_eq!(1000, lights.turned_on());
    }

    #[test]
    fn count_first_vertical_line() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0), (0, 999));
        assert_eq!(1000, lights.turned_on());
    }

    #[test]
    fn count_first_line_toggle_half() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0), (499, 0));
        assert_eq!(500, lights.turned_on());

        lights.toggle((250, 0), (749, 0));
        assert_eq!(500, lights.turned_on());

        lights.toggle((0, 0), (749, 0));
        assert_eq!(250, lights.turned_on());
    }

    #[test]
    fn count_all_lights_minus_middle() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0), (999, 999));
        lights.turn_off((499, 499), (500, 500));
        assert_eq!(1000000 - 4, lights.turned_on());
    }
}
