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
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trivial_case() {
        let d = DistanceGraph { distances: vec![] };

        assert_eq!(0, d.shortest_distance());
    }
}
