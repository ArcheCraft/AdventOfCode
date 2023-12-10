use crate::util::Puzzles;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

pub fn register_all(puzzles: &mut Puzzles) {
    day1::register_all(puzzles);
    day2::register_all(puzzles);
    day3::register_all(puzzles);
    day4::register_all(puzzles);
    day5::register_all(puzzles);
    day6::register_all(puzzles);
    day7::register_all(puzzles);
    day8::register_all(puzzles);
    day9::register_all(puzzles);
    day10::register_all(puzzles);
}
