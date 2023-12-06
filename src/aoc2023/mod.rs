use crate::util::Puzzles;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn register_all(puzzles: &mut Puzzles) {
    day1::register_all(puzzles);
    day2::register_all(puzzles);
    day3::register_all(puzzles);
    day4::register_all(puzzles);
    day5::register_all(puzzles);
    day6::register_all(puzzles);
}
