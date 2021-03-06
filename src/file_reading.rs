use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(location: &str) -> String {

    // Create a path to the desired file
    let path = Path::new(location);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_data = String::new();
    match file.read_to_string(&mut file_data) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    }

    file_data
}

pub fn read_file_as_lines(location: &str) -> Vec<String> {
    let mut file = read_file(location);
    let mut result: Vec<String> = Vec::new();

    for x in file.split("\n") {
        result.push(x.to_owned());
    }

    result
}
