use std::collections::HashMap;
use regex::Regex;
use permutohedron::*;
use std::collections::HashSet;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_13_hard.txt");

    let mut sa = SeatingArrangement::new();
    for line in input.iter() {
        sa.insert_line(line);
    }

    let answer = sa.find_optimal();
    println!("{:?}", answer);
}

pub type Happiness = i32;
pub struct SeatingTriple {
    left: String,
    right: String,
    happiness: Happiness,
}

pub struct SeatingArrangement {
    people: HashSet<String>,
    scores: Vec<SeatingTriple>,
}

impl SeatingArrangement {
    pub fn new() -> SeatingArrangement {
        SeatingArrangement { people: HashSet::new(),scores: Vec::new() }
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
        self.people.insert(triple.left.clone());
        self.people.insert(triple.right.clone());
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

    fn permutations(&self) -> Vec<Vec<&str>> {
        let mut perms = Vec::new();

        let mut data:Vec<&str>  = Vec::new();
        for person in self.people.iter() {
            data.push(&person);
        }

        let mut permutations = Heap::new(&mut data[..]); 

        while let Some(x) = permutations.next_permutation() {
            perms.push(x.to_owned());
        }
        perms
    }

    pub fn find_optimal(&self) -> i32 {
        let mut best_score = 0;
        for permutation in self.permutations() {
            let test_score = self.test_circle(&permutation);
            if test_score > best_score {
                best_score = test_score;
            }
        }
        best_score
    }

    pub fn test_circle(&self, people: &[&str]) -> i32 {
        let mut circular = people.to_owned();
        circular.push(people[0]);

        let mut total_score = 0;
        for pair in circular.windows(2) {
            let left = pair[0];
            let right = pair[1];

            total_score += match self.find(left, right) {
                Some(x) => x,
                _ => 0,
            }
        }
        total_score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_seating_arrangement() -> SeatingArrangement {
        let mut sa = SeatingArrangement::new();
        sa.insert_line("Alice would gain 54 happiness units by sitting next to Bob.");
        sa.insert_line("Alice would lose 79 happiness units by sitting next to Carol.");
        sa.insert_line("Alice would lose 2 happiness units by sitting next to David.");
        sa.insert_line("Bob would gain 83 happiness units by sitting next to Alice.");
        sa.insert_line("Bob would lose 7 happiness units by sitting next to Carol.");
        sa.insert_line("Bob would lose 63 happiness units by sitting next to David.");
        sa.insert_line("Carol would lose 62 happiness units by sitting next to Alice.");
        sa.insert_line("Carol would gain 60 happiness units by sitting next to Bob.");
        sa.insert_line("Carol would gain 55 happiness units by sitting next to David.");
        sa.insert_line("David would gain 46 happiness units by sitting next to Alice.");
        sa.insert_line("David would lose 7 happiness units by sitting next to Bob.");
        sa.insert_line("David would gain 41 happiness units by sitting next to Carol.");
        sa
    }


    #[test]
    fn fn_alice_david() {
        let sa = get_seating_arrangement();
        assert_eq!(Some(44), sa.find("Alice", "David"));
    }

    #[test]
    fn optimal_small_circle() {
        let sa = get_seating_arrangement();
        assert_eq!(330, sa.test_circle(&["David", "Alice", "Bob", "Carol"]));
    }

    #[test]
    fn find_optimal_small() {
        let sa = get_seating_arrangement();
        assert_eq!(330, sa.find_optimal());
    }
}
