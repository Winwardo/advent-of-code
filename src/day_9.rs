use std::cmp;
use std::collections::HashSet;
use permutohedron::*;

#[derive(Copy, Clone, Debug, Eq, Hash)]
pub struct Location<'a> {
    name: &'a str,
}

impl<'a> PartialEq for Location<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub struct Distance<'a> {
    from: Location<'a>,
    to: Location<'a>,
    distance: u32,
}

pub struct DistanceGraph<'a> {
    distances: Vec<Distance<'a>>,
}

impl<'a> DistanceGraph<'a> {
    pub fn shortest_distance(&self) -> u32 {
        if self.distances.len() == 0 {
            return 0;
        }

        let mut shortest = !0; // Infinity

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
            shortest = cmp::min(shortest, distance);
        }

        shortest
    }

    fn permutations(&self) -> Vec<Vec<&'a Location>> {
        let mut perms = Vec::new();

        let mut data: Vec<&Location> = self.all_locations().into_iter().collect();
        let mut permutations = Heap::new(&mut data[..]);

        while let Some(x) = permutations.next_permutation() {
            perms.push(x.to_owned());
        }
        perms
    }

    pub fn all_locations(&self) -> HashSet<&'a Location> {
        let mut set: HashSet<&'a Location> = HashSet::new();

        for distance in self.distances.iter() {
            set.insert(&distance.from);
            set.insert(&distance.to);
        }

        set
    }

    pub fn connected_locations(&self, location: &'a Location) -> Vec<&'a Distance> {
        let mut connections: Vec<&'a Distance> = Vec::new();
        for distance in self.distances.iter() {
            if (distance.from == *location) || (distance.to == *location) {
                connections.push(distance);
            }
        }
        connections
    }

    pub fn get_distance(&self, from: &'a Location, to: &'a Location) -> Option<u32> {
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
                                from: Location { name: "A" },
                                to: Location { name: "B" },
                                distance: 10,
                            }],
        };

        assert_eq!(10, d.shortest_distance());
    }

    #[test]
    fn lines_as_in_example() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "London" },
                                to: Location { name: "Dublin" },
                                distance: 464,
                            },
                            Distance {
                                from: Location { name: "London" },
                                to: Location { name: "Belfast" },
                                distance: 518,
                            },
                            Distance {
                                from: Location { name: "Dublin" },
                                to: Location { name: "Belfast" },
                                distance: 141,
                            }],
        };

        assert_eq!(605, d.shortest_distance());
    }

    #[test]
    fn one_place_two_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A" },
                                to: Location { name: "B" },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "A" },
                                to: Location { name: "C" },
                                distance: 7,
                            }],
        };

        assert_eq!(7, d.shortest_distance());
    }

    #[test]
    fn no_connected_locations() {
        let d = DistanceGraph { distances: vec![] };
        let test_location = Location { name: "B" };

        assert_eq!(0, d.connected_locations(&test_location).len());
    }

    #[test]
    fn one_connected_location() {
        let test_location = Location { name: "A" };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: test_location,
                                to: Location { name: "B" },
                                distance: 10,
                            }],
        };

        assert_eq!(1, d.connected_locations(&test_location).len());
    }

    #[test]
    fn two_connected_locations_one_dud() {
        let test_location = Location { name: "A" };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: test_location,
                                to: Location { name: "B" },
                                distance: 10,
                            },
                            Distance {
                                from: test_location,
                                to: Location { name: "C" },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "Q" },
                                to: Location { name: "B" },
                                distance: 10,
                            }],
        };

        assert_eq!(2, d.connected_locations(&test_location).len());
    }

    #[test]
    fn one_reverse_connected_location() {
        let test_location = Location { name: "A" };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "B" },
                                to: test_location,
                                distance: 10,
                            }],
        };

        assert_eq!(1, d.connected_locations(&test_location).len());
    }

    #[test]
    fn get_distance_between_locations() {
        let dublin = Location { name: "Dublin" };
        let belfast = Location { name: "Belfast" };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "London" },
                                to: Location { name: "Dublin" },
                                distance: 464,
                            },
                            Distance {
                                from: Location { name: "London" },
                                to: Location { name: "Belfast" },
                                distance: 518,
                            },
                            Distance {
                                from: dublin,
                                to: belfast,
                                distance: 141,
                            }],
        };

        assert_eq!(Some(141), d.get_distance(&belfast, &dublin));
    }

    #[test]
    fn all_locations() {
        let dublin = Location { name: "Dublin" };
        let belfast = Location { name: "Belfast" };
        let london = Location { name: "London" };

        let d = DistanceGraph {
            distances: vec![Distance {
                                from: london,
                                to: dublin,
                                distance: 464,
                            },
                            Distance {
                                from: london,
                                to: belfast,
                                distance: 518,
                            },
                            Distance {
                                from: dublin,
                                to: belfast,
                                distance: 141,
                            }],
        };

        let expected: HashSet<&Location> = vec![&dublin, &belfast, &london]
                                               .into_iter()
                                               .collect();
        assert_eq!(expected, d.all_locations());
    }
}
