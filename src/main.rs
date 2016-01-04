extern crate regex;
extern crate permutohedron;
extern crate rustc_serialize;

mod file_reading;

mod day_5;
mod day_5_hard;
mod day_6;
mod day_6_hard;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

pub fn main() {
    day_13::print_answer();
}
