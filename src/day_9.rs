#[derive(Copy, Clone)]
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
        for distance in self.distances.iter() {
            if distance.distance < shortest {
                shortest = distance.distance;
            }
        }
        shortest
    }

    pub fn connected_locations(&self, location: &'a Location) -> Vec<&'a Distance> {
        let mut connections: Vec<&'a Distance> = Vec::new();
        for distance in self.distances.iter() {
            if distance.from == *location {
                connections.push(distance);
            }
        }
        connections
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn two_connected_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A" },
                                to: Location { name: "B" },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "B" },
                                to: Location { name: "C" },
                                distance: 20,
                            }],
        };

        assert_eq!(30, d.shortest_distance());
    }

    #[test]
    fn three_connected_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A" },
                                to: Location { name: "B" },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "B" },
                                to: Location { name: "C" },
                                distance: 20,
                            },
                            Distance {
                                from: Location { name: "C" },
                                to: Location { name: "D" },
                                distance: 5,
                            }],
        };

        assert_eq!(35, d.shortest_distance());
    }

    #[test]
    fn three_disjoint_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A" },
                                to: Location { name: "B" },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "C" },
                                to: Location { name: "D" },
                                distance: 20,
                            },
                            Distance {
                                from: Location { name: "E" },
                                to: Location { name: "F" },
                                distance: 5,
                            }],
        };

        assert_eq!(5, d.shortest_distance());
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
}
