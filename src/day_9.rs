use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use permutohedron::*;

pub fn print_answer() {
    use file_reading::*;
    let input = read_file_as_lines("res\\day_9.txt");

    let mut lines: Vec<String> = Vec::new();
    let mut distances = Distances::new();

    for line in input {
        // lines.push(line.clone());
        distances.add(&line);
    }
    // for line in lines {
    // for x in 0..lines.len() {
    // distances.add(*(lines.get(x).unwrap()));
    // }

    let distance_graph = DistanceGraph { distances: distances.get_all().clone() };
    let answer = distance_graph.shortest_distance();
    println!("{:?}", answer);
}


#[derive(Clone, Debug, Eq, Hash)]
pub struct Location {
    name: String,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Clone)]
pub struct Distance {
    from: Location,
    to: Location,
    distance: u32,
}

pub struct Distances {
    strings: Vec<&'static str>,
    distances: Vec<Distance>,
}

impl Distances {
    pub fn new() -> Distances {
        Distances {
            strings: vec![],
            distances: vec![],
        }
    }

    pub fn add(&mut self, to_parse: &str) {
        let re = Regex::new(r"(.*) to (.*) = (.*)").unwrap();
        for cap in re.captures_iter(&to_parse) {
            let from = cap.at(1).unwrap();
            let to = cap.at(2).unwrap();
            let distance = cap.at(3).unwrap().parse::<u32>().unwrap();

            self.distances.push(Distance {
                from: Location { name: from.to_string() },
                to: Location { name: to.to_string() },
                distance: distance,
            });
        }
    }

    pub fn get_all(&self) -> &Vec<Distance> {
        &self.distances
    }
}

pub struct DistanceGraph {
    distances: Vec<Distance>,
}

impl DistanceGraph {
    pub fn shortest_distance(&self) -> u32 {
        if self.distances.len() == 0 {
            return 0;
        }

        let mut shortest = 0; // Infinity

        let n = self.permutations();

        for perm in n {
            let mut i1 = perm.iter();
            let mut i2 = perm.iter().skip(1);

            let mut distance = 0;

            for to in i2 {
                let from = i1.next().unwrap();
                match self.get_distance(&from, &to) {
                    Some(x) => distance += x,
                    None => {} 
                }
            }
            shortest = cmp::max(shortest, distance);
        }

        shortest
    }

    fn permutations(&self) -> Vec<Vec<&Location>> {
        let mut perms = Vec::new();

        let mut data: Vec<&Location> = self.all_locations().into_iter().collect();
        let mut permutations = Heap::new(&mut data[..]);

        while let Some(x) = permutations.next_permutation() {
            perms.push(x.to_owned());
        }
        perms
    }

    pub fn all_locations(&self) -> HashSet<&Location> {
        let mut set: HashSet<&Location> = HashSet::new();

        for distance in self.distances.iter() {
            set.insert(&distance.from);
            set.insert(&distance.to);
        }

        set
    }

    pub fn connected_locations(&self, location: &Location) -> Vec<&Distance> {
        let mut connections: Vec<&Distance> = Vec::new();
        for distance in self.distances.iter() {
            if (distance.from == *location) || (distance.to == *location) {
                connections.push(distance);
            }
        }
        connections
    }

    pub fn get_distance(&self, from: &Location, to: &Location) -> Option<u32> {
        for distance in self.connected_locations(&from) {
            if (distance.from == *to) || (distance.to == *to) {
                return Some(distance.distance);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn empty_case() {
        let d = DistanceGraph { distances: vec![] };

        assert_eq!(0, d.shortest_distance());
    }

    #[test]
    fn trivial_case() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A".to_string() },
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            }],
        };

        assert_eq!(10, d.shortest_distance());
    }

    #[test]
    fn lines_as_in_example() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "London".to_string() },
                                to: Location { name: "Dublin".to_string() },
                                distance: 464,
                            },
                            Distance {
                                from: Location { name: "London".to_string() },
                                to: Location { name: "Belfast".to_string() },
                                distance: 518,
                            },
                            Distance {
                                from: Location { name: "Dublin".to_string() },
                                to: Location { name: "Belfast".to_string() },
                                distance: 141,
                            }],
        };

        assert_eq!(605, d.shortest_distance());
    }

    #[test]
    fn one_place_two_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A".to_string() },
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "A".to_string() },
                                to: Location { name: "C".to_string() },
                                distance: 7,
                            }],
        };

        assert_eq!(7, d.shortest_distance());
    }

    #[test]
    fn no_connected_locations() {
        let d = DistanceGraph { distances: vec![] };
        let test_location = Location { name: "B".to_string() };

        assert_eq!(0, d.connected_locations(&test_location).len());
    }

    #[test]
    fn one_connected_location() {
        let test_location = Location { name: "A".to_string() };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: test_location.clone(),
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            }],
        };

        assert_eq!(1, d.connected_locations(&test_location).len());
    }

    #[test]
    fn two_connected_locations_one_dud() {
        let test_location = Location { name: "A".to_string() };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: test_location.clone(),
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            },
                            Distance {
                                from: test_location.clone(),
                                to: Location { name: "C".to_string() },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "Q".to_string() },
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            }],
        };

        assert_eq!(2, d.connected_locations(&test_location).len());
    }

    #[test]
    fn one_reverse_connected_location() {
        let test_location = Location { name: "A".to_string() };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "B".to_string() },
                                to: test_location.clone(),
                                distance: 10,
                            }],
        };

        assert_eq!(1, d.connected_locations(&test_location).len());
    }

    #[test]
    fn get_distance_between_locations() {
        let dublin = Location { name: "Dublin".to_string() };
        let belfast = Location { name: "Belfast".to_string() };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "London".to_string() },
                                to: Location { name: "Dublin".to_string() },
                                distance: 464,
                            },
                            Distance {
                                from: Location { name: "London".to_string() },
                                to: Location { name: "Belfast".to_string() },
                                distance: 518,
                            },
                            Distance {
                                from: dublin.clone(),
                                to: belfast.clone(),
                                distance: 141,
                            }],
        };

        assert_eq!(Some(141), d.get_distance(&belfast, &dublin));
    }

    #[test]
    fn all_locations() {
        let dublin = Location { name: "Dublin".to_string() };
        let belfast = Location { name: "Belfast".to_string() };
        let london = Location { name: "London".to_string() };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: london.clone(),
                                to: dublin.clone(),
                                distance: 464,
                            },
                            Distance {
                                from: london.clone(),
                                to: belfast.clone(),
                                distance: 518,
                            },
                            Distance {
                                from: dublin.clone(),
                                to: belfast.clone(),
                                distance: 141,
                            }],
        };

        let expected: HashSet<&Location> = vec![&dublin, &belfast, &london]
                                               .into_iter()
                                               .collect();
        assert_eq!(expected, d.all_locations());
    }


    #[test]
    fn lines_as_in_example_using_text_input() {
        let mut distances = Distances::new();
        distances.add("London to Dublin = 464");
        distances.add("London to Belfast = 518");
        distances.add("Dublin to Belfast = 141");

        let d = DistanceGraph { distances: (*distances.get_all()).clone() };

        assert_eq!(605, d.shortest_distance());
    }
}
