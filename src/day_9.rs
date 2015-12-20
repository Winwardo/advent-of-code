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
}
