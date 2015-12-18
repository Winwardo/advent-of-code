use regex::Regex;
use std::cmp;

pub fn print_answer() {
    use file_reading::*;

    let input = read_file_as_lines("res\\day_6.txt");

    let mut lights = Lights::new();

    for line in input {
        lights.give_instruction(&line);
    }

    let answer = lights.turned_on();
    println!("{:?}", answer);
}

pub struct Lights {
    lights: Vec<u32>,
}

impl Lights {
    pub fn new() -> Lights {
        Lights { lights: vec![0; 1000000] }
    }

    pub fn give_instruction(&mut self, instruction: &str) {
        let re = Regex::new(r"(turn off|turn on|toggle) (\d*),(\d*) through (\d*),(\d*)").unwrap();
        for cap in re.captures_iter(instruction) {
            let action = cap.at(1).unwrap();
            let x1 = cap.at(2).unwrap().parse::<u32>().unwrap();
            let y1 = cap.at(3).unwrap().parse::<u32>().unwrap();
            let x2 = cap.at(4).unwrap().parse::<u32>().unwrap();
            let y2 = cap.at(5).unwrap().parse::<u32>().unwrap();

            let start = (x1, y1);
            let end = (x2, y2);

            match action {
                "turn off" => {
                    self.turn_off(start, end);
                }
                "turn on" => {
                    self.turn_on(start, end);
                }
                "toggle" => {
                    self.toggle(start, end);
                }
                _ => panic!("Invalid action."),
            }
        }
    }

    pub fn turn_on(&mut self, start: (u32, u32), end: (u32, u32)) {
        self.add_at_group(start, end, 1);
    }

    pub fn turn_off(&mut self, start: (u32, u32), end: (u32, u32)) {
        self.add_at_group(start, end, -1);
    }

    fn toggle(&mut self, start: (u32, u32), end: (u32, u32)) {
        self.add_at_group(start, end, 2);
    }

    fn add_at_group(&mut self, start: (u32, u32), end: (u32, u32), value: i32) {
        let (x1, y1) = start;
        let (x2, y2) = end;

        for y in y1..(y2 + 1) {
            for x in x1..(x2 + 1) {
                self.add_at(x, y, value);
            }
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * 1000) as usize
    }

    fn add_at(&mut self, x: u32, y: u32, value: i32) {
        let position = self.get_index(x, y);
        let currentValue = self.lights[position];
        self.lights[position] = cmp::max(currentValue as i32 + value, 0) as u32;
    }

    pub fn turned_on(&self) -> usize {
        return self.lights.iter().fold(0u32, |sum, &val| sum + val) as usize;
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
    fn count_all_lights_on_twice() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0), (999, 999));
        lights.turn_on((0, 0), (999, 999));
        assert_eq!(2000000, lights.turned_on());
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
        assert_eq!(1500, lights.turned_on());

        lights.toggle((0, 0), (749, 0));
        assert_eq!(3000, lights.turned_on());
    }

    #[test]
    fn count_all_lights_minus_middle() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0), (999, 999));
        lights.turn_off((499, 499), (500, 500));
        assert_eq!(1000000 - 4, lights.turned_on());
    }

    #[test]
    fn give_instruction_turn_on_all_lights() {
        let mut lights = Lights::new();
        lights.give_instruction("turn on 0,0 through 999,999");
        assert_eq!(1000000, lights.turned_on());
    }

    #[test]
    fn give_instruction_turn_on_all_lights_turn_off_middle() {
        let mut lights = Lights::new();
        lights.give_instruction("turn on 0,0 through 999,999");
        lights.give_instruction("turn off 499,499 through 500,500");
        assert_eq!(1000000 - 4, lights.turned_on());
    }

    #[test]
    fn give_instruction_toggle_on_all_lights_toggle_middle() {
        let mut lights = Lights::new();
        lights.give_instruction("toggle 0,0 through 999,999");
        lights.give_instruction("toggle 499,499 through 500,500");
        assert_eq!(2000000 + 8, lights.turned_on());
    }
}
