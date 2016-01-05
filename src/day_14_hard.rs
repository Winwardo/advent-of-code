use regex::Regex;
use std::cmp;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_14.txt");

    let mut reindeers = Vec::new();
    for line in input.iter() {
        reindeers.push(Reindeer::from_str(&line));
    }
    let mut race = Race::new(reindeers);

    let answer = race.get_winning_points_at((2503));
    println!("{:?}", answer);
}

pub type Kilometres = i32;
pub type KMs = i32;
pub type Seconds = i32;

#[derive(Debug)]
pub struct Reindeer {
    name: String,
    speed: KMs,
    duration: Seconds,
    rest: Seconds,
}

impl Reindeer {
    pub fn from_str(to_parse: &str) -> Reindeer {
        let re = Regex::new(r"(.+?) can fly (.+?) km/s for (.+?) seconds, but then must rest for (.+?) seconds.")
                     .unwrap();

        for cap in re.captures_iter(&to_parse) {
            return Reindeer {
                name: cap.at(1).unwrap().to_owned(),
                speed: (cap.at(2).unwrap().parse::<i32>().unwrap()),
                duration: (cap.at(3).unwrap().parse::<i32>().unwrap()),
                rest: (cap.at(4).unwrap().parse::<i32>().unwrap()),
            };
        }

        panic!("Unparseable line")
    }

    pub fn is_not_resting_at_time(&self, time: Seconds) -> bool {
        let trunc_sec = time % (self.duration + self.rest);
        trunc_sec < self.duration
    }
}

pub struct Race {
    reindeers: Vec<Reindeer>,
    distances: Vec<i32>,
    scores: Vec<i32>,
    current_time: i32,
}

impl Race {
    pub fn new(reindeers: Vec<Reindeer>) -> Race {
        let len = reindeers.len();
        Race {
            reindeers: reindeers,
            distances: vec![0;len],
            scores: vec![0;len],
            current_time: 0,
        }
    }

    pub fn run_race(&mut self, time: Seconds) -> Vec<i32> {
        for _ in 0..time {
            self.tick();
        }

        self.scores.clone()
    }

    fn tick(&mut self) {
        self.tick_reindeers();
        self.update_scores();

        self.current_time += 1;
    }

    fn tick_reindeers(&mut self) {
        for (i, reindeer) in self.reindeers.iter().enumerate() {
            if reindeer.is_not_resting_at_time(self.current_time) {
                self.distances[i] += reindeer.speed;
            }
        }
    }

    fn update_scores(&mut self) {
        let mut winning_reindeer_ids = vec![];
        let mut furthest_ran = 0;

        for (i, _) in self.reindeers.iter().enumerate() {
            let distance_covered = self.distances[i];

            if distance_covered > furthest_ran {
                winning_reindeer_ids = vec![i];
                furthest_ran = distance_covered;
            } else if distance_covered == furthest_ran {
                winning_reindeer_ids.push(i);
            }
        }

        for winner in winning_reindeer_ids {
            self.scores[winner] += 1;
        }
    }

    pub fn get_winning_points_at(&mut self, time: Seconds) -> i32 {
        self.run_race(time).iter().fold(0, |x, &y| cmp::max(x, y))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_race() -> Race {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        let dancer = Reindeer::from_str("Dancer can fly 16 km/s for 11 seconds, but then must \
                                         rest for 162 seconds.");

        Race::new(vec![comet, dancer])
    }


    #[test]
    fn race_comet_and_dancer_get_scores_one_second() {
        let mut race = make_race();
        let scores = race.run_race((1));

        assert_eq!(scores[0], 0);
        assert_eq!(scores[1], 1);
    }

    #[test]
    fn race_comet_and_dancer_get_scores_140_seconds() {
        let mut race = make_race();
        let scores = race.run_race((140));

        assert_eq!(scores[0], 1);
        assert_eq!(scores[1], 139);
    }

    #[test]
    fn race_comet_and_dancer_get_scores_1000_seconds() {
        let mut race = make_race();
        let scores = race.run_race((1000));

        assert_eq!(scores[0], 312);
        assert_eq!(scores[1], 689);
    }

    #[test]
    fn race_comet_and_dancer_get_winning_points() {
        let mut race = make_race();

        assert_eq!(689, race.get_winning_points_at((1000)));
    }
}
