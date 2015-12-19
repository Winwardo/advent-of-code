
pub struct CharacterCount<'a> {
    characters: &'a str,
}

impl<'a> CharacterCount<'a> {
    pub fn new(characters: &str) -> CharacterCount {
        CharacterCount { characters: characters }
    }

    pub fn count(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_empty_string_is_2() {
        let mut count = CharacterCount::new("\"\"");
        assert_eq!(2, count.count());
    }
}
