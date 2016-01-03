use std::collections::HashMap;
use regex::Regex;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file("res\\day_13.txt");
    // let answer = get_sum(&input);
    // println!("{:?}", answer);
}

pub type Happiness = i32;
pub struct SeatingTriple {
    left: String,
    right: String,
    happiness: Happiness,
}

pub struct SeatingArrangement {
    scores: Vec<SeatingTriple>,
}

impl SeatingArrangement {
    pub fn new() -> SeatingArrangement {
        SeatingArrangement { scores: Vec::new() }
    }

    pub fn insert_line(&mut self, line: &str) {
        let re = Regex::new(r"(.+?) would (.+?) (\d+?) happiness units by sitting next to (.+?)\.")
                     .unwrap();
        for cap in re.captures_iter(&line) {
            let left = cap.at(1).unwrap();
            let right = cap.at(4).unwrap();

            let modifier = match cap.at(2) {
                Some("gain") => 1,
                Some("lose") => -1,
                _ => {
                    panic!("Bad modifier.");
                }
            };
            let value = cap.at(3).unwrap();

            let score = value.parse::<Happiness>().ok().expect("Should be a number") * modifier;

            self.insert(SeatingTriple {
                left: left.to_owned(),
                right: right.to_owned(),
                happiness: score,
            });
        }
    }

    fn insert(&mut self, triple: SeatingTriple) {
        self.scores.push(triple);
    }

    pub fn find(&self, f_left: &str, f_right: &str) -> Option<Happiness> {
        let mut score: Happiness = 0;
        let mut exists = false;

        for &SeatingTriple {ref left, ref right, happiness} in self.scores.iter() {
            if left == f_left && right == f_right {
                exists = true;
                score += happiness;
            } else if left == f_right && right == f_left {
                exists = true;
                score += happiness;
            }
        }

        if exists {
            Some(score)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fn_alice_david() {
        let mut sa = SeatingArrangement::new();
        sa.insert_line("Alice would lose 2 happiness units by sitting next to David.");
        sa.insert_line("David would gain 46 happiness units by sitting next to Alice.");
        assert_eq!(Some(44), sa.find("Alice", "David"));
    }
}
