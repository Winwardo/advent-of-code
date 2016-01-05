use regex::Regex;
use std::cmp;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_14.txt");

    let mut reindeers = Vec::new();
    for line in input.iter() {
        reindeers.push(Reindeer::from_str(&line));
    }
    let race = Race { reindeers: reindeers };

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
}

pub struct Race {
    reindeers: Vec<Reindeer>,
}

impl Race {
    pub fn run_race(&self, time: Seconds) -> Vec<i32> {
        let mut scores = vec![0; self.reindeers.len()];
        let mut distances = vec![0; self.reindeers.len()];
        let mut rest_left = vec![0; self.reindeers.len()];

        let seconds = time;
        for x in 0..seconds {
            let mut winners = vec![];
            let mut top_score = 0;
            for (i, reindeer) in self.reindeers.iter().enumerate() {
                let trunc_sec = x % (reindeer.duration + reindeer.rest);

                if trunc_sec < reindeer.duration {
                    distances[i] += reindeer.speed;
                }

                if distances[i] > top_score {
                    winners = vec![i];
                    top_score = distances[i];
                } else if distances[i] == top_score {
                    winners.push(i);
                }
            }

            for winner in winners {
                scores[winner] += 1;
            }
        }

        scores
    }

    pub fn get_winning_points_at(&self, time: Seconds) -> i32 {
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

        Race { reindeers: vec![comet, dancer] }
    }


    #[test]
    fn race_comet_and_dancer_get_scores_one_second() {
        let race = make_race();
        let scores = race.run_race((1));

        assert_eq!(scores[0], 0);
        assert_eq!(scores[1], 1);
    }

    #[test]
    fn race_comet_and_dancer_get_scores_140_seconds() {
        let race = make_race();
        let scores = race.run_race((140));

        assert_eq!(scores[0], 1);
        assert_eq!(scores[1], 139);
    }

    #[test]
    fn race_comet_and_dancer_get_scores_1000_seconds() {
        let race = make_race();
        let scores = race.run_race((1000));

        assert_eq!(scores[0], 312);
        assert_eq!(scores[1], 689);
    }

    #[test]
    fn race_comet_and_dancer_get_winning_points() {
        let race = make_race();

        assert_eq!(689, race.get_winning_points_at((1000)));
    }
}
