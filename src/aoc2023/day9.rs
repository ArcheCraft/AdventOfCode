use rayon::prelude::*;

use crate::util::Puzzles;

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Mirage Maintenance".to_string(),
        year: 2023,
        day: 9,
        solver: Box::new(|input| {
            let sum: i64 = input
                .text()
                .lines()
                .par_bridge()
                .map(|l| {
                    let numbers = l
                        .split_ascii_whitespace()
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    let mut stack = vec![numbers.clone()];

                    let mut current = numbers;
                    while !current.iter().all(|n| n == &0) {
                        let next = current
                            .par_iter()
                            .zip(current[1..].par_iter())
                            .map(|(l, r)| r - l)
                            .collect::<Vec<_>>();
                        stack.push(next.clone());
                        current = next;
                    }

                    while stack.len() > 1 {
                        let last_idx = stack.len() - 1;
                        let last_last_idx = stack[last_idx].len() - 1;
                        let second_last_last_idx = stack[last_idx - 1].len() - 1;
                        let num = stack[last_idx - 1][second_last_last_idx]
                            + stack[last_idx][last_last_idx];
                        stack[last_idx - 1].push(num);
                        stack.pop();
                    }

                    stack[0][stack[0].len() - 1]
                })
                .sum();

            Ok(sum.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 9,
        solver: Box::new(|input| {
            let sum: i64 = input
                .text()
                .lines()
                .par_bridge()
                .map(|l| {
                    let mut numbers = l
                        .split_ascii_whitespace()
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    numbers.reverse();
                    let mut stack = vec![numbers.clone()];

                    let mut current = numbers;
                    while !current.iter().all(|n| n == &0) {
                        let next = current
                            .par_iter()
                            .zip(current[1..].par_iter())
                            .map(|(l, r)| r - l)
                            .collect::<Vec<_>>();
                        stack.push(next.clone());
                        current = next;
                    }

                    while stack.len() > 1 {
                        let last_idx = stack.len() - 1;
                        let last_last_idx = stack[last_idx].len() - 1;
                        let second_last_last_idx = stack[last_idx - 1].len() - 1;
                        let num = stack[last_idx - 1][second_last_last_idx]
                            + stack[last_idx][last_last_idx];
                        stack[last_idx - 1].push(num);
                        stack.pop();
                    }

                    stack[0][stack[0].len() - 1]
                })
                .sum();

            Ok(sum.to_string())
        }),
    });
}
