use std::{collections::HashMap, ops::Range};

use crate::util::Puzzles;

struct Conversion {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

struct ConversionMap {
    source_ns: String,
    dest_ns: String,
    conversions: Vec<Conversion>,
}

struct Input {
    seeds: Vec<u64>,
    conversion_maps: HashMap<String, ConversionMap>,
}

impl ConversionMap {
    fn convert(&self, i: u64) -> u64 {
        for conversion in &self.conversions {
            if (conversion.source_start..conversion.source_start + conversion.length).contains(&i) {
                return i - conversion.source_start + conversion.dest_start;
            }
        }
        return i;
    }
}

fn parse_conversion_map(lines: &[&str]) -> ConversionMap {
    let name = lines[0].split_once(" ").unwrap();
    let (source_ns, dest_ns) = name.0.split_once("-to-").unwrap();

    let mut conversions = vec![];
    for line in lines.iter().skip(1) {
        let nums = line.split_ascii_whitespace().collect::<Vec<_>>();
        conversions.push(Conversion {
            dest_start: nums[0].parse().unwrap(),
            source_start: nums[1].parse().unwrap(),
            length: nums[2].parse().unwrap(),
        });
    }
    return ConversionMap {
        source_ns: source_ns.to_string(),
        dest_ns: dest_ns.to_string(),
        conversions,
    };
}

fn parse_input(input: String) -> Input {
    let lines = input.lines().collect::<Vec<_>>();
    let seeds = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut conversion_maps = HashMap::new();
    let mut start_index = 2;
    let mut index = 2;
    while index < lines.len() {
        while index < lines.len() && !lines[index].is_empty() {
            index += 1;
        }
        let map = parse_conversion_map(&lines[start_index..index]);
        conversion_maps.insert(map.source_ns.clone(), map);
        index += 1;
        start_index = index;
    }
    return Input {
        seeds,
        conversion_maps,
    };
}

fn intersect(range1: Range<u64>, range2: Range<u64>) -> Range<u64> {
    return range1.start.max(range2.start)..range1.end.min(range2.end);
}

fn range_sub(range1: Range<u64>, range2: Range<u64>) -> (Range<u64>, Range<u64>) {
    let left = range1.start..range2.start;
    let right = range2.end..range1.end;
    (left, right)
}

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "".to_string(),
        year: 2023,
        day: 5,
        solver: Box::new(|input| {
            let input = parse_input(input.text());
            let mut results = vec![];
            for mut seed in input.seeds {
                let mut current_ns = "seed";

                while let Some(map) = input.conversion_maps.get(current_ns) {
                    seed = map.convert(seed);
                    current_ns = &map.dest_ns;
                }

                results.push(seed);
            }
            Ok(results.iter().min().unwrap().to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 5,
        solver: Box::new(|input| {
            let input = parse_input(input.text());
            let mut nums = input
                .seeds
                .chunks(2)
                .map(|c| c[0]..c[0] + c[1])
                .collect::<Vec<_>>();
            let mut current_ns = "seed";

            while let Some(map) = input.conversion_maps.get(current_ns) {
                current_ns = &map.dest_ns;

                let mut result = vec![];
                let mut remaining: Vec<Range<u64>> = nums;
                for conversion in &map.conversions {
                    let mut remaining_new = vec![];
                    for rem in &remaining {
                        let inter = intersect(
                            rem.clone(),
                            conversion.source_start..conversion.source_start + conversion.length,
                        );

                        if !inter.is_empty() {
                            // intersection
                            let start =
                                inter.start - conversion.source_start + conversion.dest_start;
                            let end = inter.end - conversion.source_start + conversion.dest_start;
                            result.push(start..end);

                            let (range_rem1, range_rem2) = range_sub(rem.clone(), inter);
                            if !range_rem1.is_empty() {
                                remaining_new.push(range_rem1);
                            }
                            if !range_rem2.is_empty() {
                                remaining_new.push(range_rem2);
                            }
                        } else {
                            remaining_new.push(rem.clone());
                        }
                    }
                    remaining = remaining_new;
                }
                result.extend(remaining.into_iter());
                nums = result;
            }

            Ok(nums.iter().map(|r| r.start).min().unwrap().to_string())
        }),
    });
}
