use std::collections::HashSet;

use crate::util::Puzzles;

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "".to_string(),
        year: 2023,
        day: 4,
        solver: Box::new(|input| {
            let mut sum = 0;
            for line in input.text().lines() {
                let (num, rest) = line.split_once(": ").unwrap();
                let game = num.split_once(" ").unwrap().1.trim().parse::<u32>()?;
                let (winning, own) = rest.split_once(" | ").unwrap();

                let mut winning_set = HashSet::new();
                for win in winning.split(" ") {
                    if win.is_empty() {
                        continue;
                    }
                    winning_set.insert(win.trim().parse::<u32>()?);
                }

                let mut points = 0;
                for num in own.split(" ") {
                    if !num.is_empty() && winning_set.contains(&num.trim().parse::<u32>()?) {
                        if points == 0 {
                            points = 1;
                        } else {
                            points *= 2;
                        }
                    }
                }

                sum += points
            }
            Ok(sum.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 4,
        solver: Box::new(|input| {
            let mut totals = Vec::<u32>::new();
            fn push(totals: &mut Vec<u32>, game: u32, n: u32) {
                if totals.len() < game as usize {
                    totals.resize(game as usize, 0);
                }
                totals[(game - 1) as usize] += n;
            }

            for line in input.text().lines() {
                let (num, rest) = line.split_once(": ").unwrap();
                let game = num.split_once(" ").unwrap().1.trim().parse::<u32>()?;
                let (winning, own) = rest.split_once(" | ").unwrap();

                let mut winning_set = HashSet::new();
                for win in winning.split(" ") {
                    if win.is_empty() {
                        continue;
                    }
                    winning_set.insert(win.trim().parse::<u32>()?);
                }

                let mut points = 0;
                for num in own.split(" ") {
                    if !num.is_empty() && winning_set.contains(&num.trim().parse::<u32>()?) {
                        points += 1;
                    }
                }

                push(&mut totals, game, 1);
                for i in 1..=points {
                    let n = totals[(game - 1) as usize].clone();
                    push(&mut totals, game + i, n);
                }
            }

            let sum = totals.iter().sum::<u32>();
            Ok(sum.to_string())
        }),
    });
}
