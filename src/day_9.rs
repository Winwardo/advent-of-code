pub struct Location {
    name: String,
}

pub struct Distance {
    from: Location,
    to: Location,
    distance: u32,
}

pub struct DistanceGraph {
    distances: Vec<Distance>,
}

impl DistanceGraph {
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

    pub fn connected_locations(&self, location: &Location) -> Vec<Distance> {
        vec![]
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
                                from: Location { name: "A".to_string() },
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            }],
        };

        assert_eq!(10, d.shortest_distance());
    }

    #[test]
    fn two_connected_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A".to_string() },
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "B".to_string() },
                                to: Location { name: "C".to_string() },
                                distance: 20,
                            }],
        };

        assert_eq!(30, d.shortest_distance());
    }

    #[test]
    fn three_connected_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A".to_string() },
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "B".to_string() },
                                to: Location { name: "C".to_string() },
                                distance: 20,
                            },
                            Distance {
                                from: Location { name: "C".to_string() },
                                to: Location { name: "D".to_string() },
                                distance: 5,
                            }],
        };

        assert_eq!(35, d.shortest_distance());
    }

    #[test]
    fn three_disjoint_lines() {
        let d = DistanceGraph {
            distances: vec![Distance {
                                from: Location { name: "A".to_string() },
                                to: Location { name: "B".to_string() },
                                distance: 10,
                            },
                            Distance {
                                from: Location { name: "C".to_string() },
                                to: Location { name: "D".to_string() },
                                distance: 20,
                            },
                            Distance {
                                from: Location { name: "E".to_string() },
                                to: Location { name: "F".to_string() },
                                distance: 5,
                            }],
        };

        assert_eq!(5, d.shortest_distance());
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
}
