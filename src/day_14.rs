use regex::Regex;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_14.txt");

    let mut reindeers = Vec::new();
    for line in input.iter() {
        reindeers.push(Reindeer::from_str(&line));
    }
    let race = Race { reindeers: reindeers };

    let answer = race.get_winning_distance_at(Seconds(2503.0));
    println!("{:?}", answer);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kilometres(pub f32);
pub struct KMs(pub f32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Seconds(pub f32);

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
                speed: KMs(cap.at(2).unwrap().parse::<f32>().unwrap()),
                duration: Seconds(cap.at(3).unwrap().parse::<f32>().unwrap()),
                rest: Seconds(cap.at(4).unwrap().parse::<f32>().unwrap()),
            };
        }

        panic!("Unparseable line")
    }

    pub fn distance_after(&self, time_limit: Seconds) -> Kilometres {
        let KMs(kms) = self.speed;
        let Seconds(duration) = self.duration;
        let Seconds(rest) = self.rest;
        let Seconds(time_limit_) = time_limit;

        let time = duration + rest;

        let complete_runs = (time_limit_ / time).floor();
        let leftover_runs = (time_limit_ % time);

        let l = if (leftover_runs > duration) {
            duration
        } else {
            leftover_runs
        };

        let answer = (l + complete_runs * duration) * kms;

        Kilometres(answer)
    }
}

pub struct Race {
    reindeers: Vec<Reindeer>,
}

impl Race {
    pub fn get_winning_distance_at(&self, time: Seconds) -> Kilometres {
        let mut max_distance = 0.0;
        for reindeer in self.reindeers.iter() {
            let Kilometres(distance) = reindeer.distance_after(time);
            if distance > max_distance {
                max_distance = distance;
            }
        }

        Kilometres(max_distance)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comet_distance_one_second() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        assert_eq!(Kilometres(14.0), comet.distance_after(Seconds(1.0)));
    }

    #[test]
    fn comet_distance_ten_seconds() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        assert_eq!(Kilometres(140.0), comet.distance_after(Seconds(10.0)));
    }

    #[test]
    fn comet_distance_eleven_seconds() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        assert_eq!(Kilometres(140.0), comet.distance_after(Seconds(11.0)));
    }

    #[test]
    fn comet_distance_137_seconds() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        assert_eq!(Kilometres(140.0), comet.distance_after(Seconds(137.0)));
    }

    #[test]
    fn comet_distance_138_seconds() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        assert_eq!(Kilometres(154.0), comet.distance_after(Seconds(138.0)));
    }

    #[test]
    fn comet_distance_1000_seconds() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        assert_eq!(Kilometres(1120.0), comet.distance_after(Seconds(1000.0)));
    }

    #[test]
    fn race_comet_and_dancer() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest \
                                        for 127 seconds.");
        let dancer = Reindeer::from_str("Dancer can fly 16 km/s for 11 seconds, but then must \
                                         rest for 162 seconds.");

        let race = Race { reindeers: vec![comet, dancer] };

        assert_eq!(Kilometres(1120.0),
                   race.get_winning_distance_at(Seconds(1000.0)));
    }
}
