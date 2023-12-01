pub mod puzzle;
use std::collections::HashMap;

pub use puzzle::*;

pub mod input;
pub use input::*;

use crate::aoc2023;

pub struct Puzzles {
    puzzles: HashMap<u32, HashMap<u8, Vec<Puzzle>>>,
    input_loader: InputLoader,
}

impl Puzzles {
    pub fn new() -> eyre::Result<Self> {
        return Ok(Self {
            puzzles: HashMap::new(),
            input_loader: InputLoader::new()?,
        });
    }

    pub fn register(&mut self, puzzle: Puzzle) {
        let year_map = self.puzzles.entry(puzzle.year).or_insert_with(HashMap::new);
        let day_vec = year_map.entry(puzzle.day).or_insert_with(Vec::new);
        day_vec.push(puzzle);
    }

    pub fn register_all(&mut self) {
        aoc2023::register_all(self)
    }

    pub fn get_years(&self) -> Vec<u32> {
        self.puzzles.keys().copied().collect()
    }

    pub fn get_days(&self, year: u32) -> Vec<u8> {
        self.puzzles.get(&year).map(|map| map.keys().copied().collect()).unwrap_or_else(Vec::new)
    }

    pub fn run(&mut self, year: u32, day: u8) -> eyre::Result<()> {
        let year_map = self
            .puzzles
            .get_mut(&year)
            .ok_or_else(|| eyre::eyre!("Couldn't find puzzles for year {}!", year))?;
        let puzzles_to_run = year_map
            .get_mut(&day)
            .ok_or_else(|| eyre::eyre!("Couldn't find puzzles for day {}/{}!", year, day))?;

        let input = self.input_loader.get_input(year, day)?;
        for puzzle in puzzles_to_run {
            let output = (puzzle.solver)(input);
            match output {
                Ok(output) => println!("{}: {}", puzzle.name, output),
                Err(e) => println!(
                    "Error running solver for puzzle {}/{} {}: {:?}",
                    year, day, puzzle.name, e
                ),
            }
        }
        Ok(())
    }
}
