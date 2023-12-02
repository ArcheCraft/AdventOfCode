use crate::util::Puzzles;

mod day1;
mod day2;

pub fn register_all(puzzles: &mut Puzzles) {
    day1::register_all(puzzles);
    day2::register_all(puzzles);
}
