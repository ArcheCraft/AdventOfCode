use std::collections::HashMap;

use crate::util::Puzzles;

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Gear Ratios".to_string(),
        year: 2023,
        day: 3,
        solver: Box::new(|input| {
            let text = input.text();
            let lines = text.lines().collect::<Vec<_>>();
            let line_chars = lines
                .iter()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let number_regex = regex::Regex::new("\\d+")?;

            let mut sum = 0;
            for (i, line) in lines.iter().enumerate() {
                for number in number_regex.find_iter(line) {
                    'inner: for x in ((number.start() as isize - 1).max(0) as usize)
                        ..=number.end().min(line.len() - 1)
                    {
                        for y in ((i as isize - 1).max(0) as usize)..=(i + 1).min(lines.len() - 1) {
                            if (!number.range().contains(&x) || i != y)
                                && line_chars[y][x] != '.'
                                && !line_chars[y][x].is_ascii_digit()
                            {
                                sum += number.as_str().parse::<u32>()?;
                                break 'inner;
                            }
                        }
                    }
                }
            }
            Ok(sum.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 3,
        solver: Box::new(|input| {
            let text = input.text();
            let lines = text.lines().collect::<Vec<_>>();
            let line_chars = lines
                .iter()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let number_regex = regex::Regex::new("\\d+")?;

            let mut gears = HashMap::new();
            for (i, line) in lines.iter().enumerate() {
                for number in number_regex.find_iter(line) {
                    'inner: for x in ((number.start() as isize - 1).max(0) as usize)
                        ..=number.end().min(line.len() - 1)
                    {
                        for y in ((i as isize - 1).max(0) as usize)..=(i + 1).min(lines.len() - 1) {
                            if (!number.range().contains(&x) || i != y)
                                && line_chars[y][x] == '*'
                            {
                                let index = (x, y);
                                let vec = gears.entry(index).or_insert_with(Vec::new);
                                vec.push(number.as_str());
                                break 'inner;
                            }
                        }
                    }
                }
            }

            let mut sum = 0;
            for gear in gears.keys() {
                if let Some(vec) = gears.get(gear) {
                    if vec.len() == 2 {
                        let first = vec[0].parse::<u32>()?;
                        let second = vec[1].parse::<u32>()?;
                        sum += first * second;
                    }
                }
            }

            Ok(sum.to_string())

        }),
    });
}
