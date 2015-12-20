pub struct Location {
    name: String,
}

pub struct Distance {
    from: Location,
    to: Location,
    distance: u32,
}

struct Locations {
    distances: Vec<Distance>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trivial_case() {}
}
