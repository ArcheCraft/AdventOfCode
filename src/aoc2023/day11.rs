use itertools::Itertools as _;

use crate::util::Puzzles;

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Cosmic Expansion".to_string(),
        year: 2023,
        day: 11,
        solver: Box::new(|input| {
            let grid = input.text().lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
            let empty_rows = grid.iter().enumerate().filter(|(i, l)| l.iter().all(|c| c == &'.')).map(|(i, _)| i).collect::<Vec<_>>();
            let empty_columns = (0..grid[0].len()).filter(|i| grid.iter().all(|v| v[*i] == '.')).collect::<Vec<_>>();

            let galaxies = grid.iter().enumerate().flat_map(|(i, l)| l.iter().enumerate().map(move |(j, v)| (i, j, *v))).filter(|(_, _, v)| v == &'#').collect::<Vec<_>>();
            let transformed_galaxies = galaxies.iter().map(|(i, j, _)| (empty_rows.iter().filter(|x| *x < i).count() + i, empty_columns.iter().filter(|y| *y < j).count() + j)).collect::<Vec<_>>();
            let sum = transformed_galaxies.iter().permutations(2).map(|v| v[0].0.abs_diff(v[1].0) + v[0].1.abs_diff(v[1].1)).sum::<usize>() / 2;

            Ok(sum.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 11,
        solver: Box::new(|input| {
            let grid = input.text().lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
            let empty_rows = grid.iter().enumerate().filter(|(i, l)| l.iter().all(|c| c == &'.')).map(|(i, _)| i).collect::<Vec<_>>();
            let empty_columns = (0..grid[0].len()).filter(|i| grid.iter().all(|v| v[*i] == '.')).collect::<Vec<_>>();

            let galaxies = grid.iter().enumerate().flat_map(|(i, l)| l.iter().enumerate().map(move |(j, v)| (i, j, *v))).filter(|(_, _, v)| v == &'#').collect::<Vec<_>>();
            let transformed_galaxies = galaxies.iter().map(|(i, j, _)| (empty_rows.iter().filter(|x| *x < i).count() * 999999 + i, empty_columns.iter().filter(|y| *y < j).count() * 999999 + j)).collect::<Vec<_>>();
            let sum = transformed_galaxies.iter().permutations(2).map(|v| v[0].0.abs_diff(v[1].0) + v[0].1.abs_diff(v[1].1)).sum::<usize>() / 2;

            Ok(sum.to_string())
        }),
    });
}
