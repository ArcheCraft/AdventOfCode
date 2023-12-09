use rayon::prelude::*;
use std::collections::HashMap;

use crate::util::Puzzles;

fn lcm(numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let a = numbers[0];
    let b = lcm(&numbers[1..]);
    a * b / gcd2(a, b)
}

pub fn gcd(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = gcd(&nums[1..]);
    gcd2(a, b)
}

fn gcd2(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd2(b, a % b)
}

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Haunted Wasteland".to_string(),
        year: 2023,
        day: 8,
        solver: Box::new(|input| {
            let text = input.text();
            let (path, nodes) = text.split_once("\n\n").unwrap();
            let path = path.chars().collect::<Vec<_>>();
            let nodes = nodes
                .lines()
                .map(|l| {
                    let (name, targets) = l.split_once(" = (").unwrap();
                    let (left, right) = targets.split_once(", ").unwrap();
                    let right = &right[0..3];
                    (name, (left, right))
                })
                .collect::<HashMap<_, _>>();

            let mut current_node = "AAA";
            let mut num = 0;
            for move_dir in path.iter().cycle() {
                if current_node == "ZZZ" {
                    break;
                }

                num += 1;

                current_node = nodes
                    .get(current_node)
                    .map(|n| if move_dir == &'L' { n.0 } else { n.1 })
                    .unwrap();
            }
            Ok(num.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 8,
        solver: Box::new(|input| {
            let text = input.text();
            let (path, nodes) = text.split_once("\n\n").unwrap();
            let path = path.chars().collect::<Vec<_>>();
            let nodes = nodes
                .lines()
                .map(|l| {
                    let (name, targets) = l.split_once(" = (").unwrap();
                    let (left, right) = targets.split_once(", ").unwrap();
                    let right = &right[0..3];
                    (name, (left, right))
                })
                .collect::<HashMap<_, _>>();

            let start_nodes = nodes
                .keys()
                .filter(|k| k.ends_with('A'))
                .copied()
                .collect::<Vec<_>>();
            let times = start_nodes
                .par_iter()
                .map(|sn| {
                    let mut current_node = sn.to_string();
                    let mut num = 0;
                    for move_dir in path.iter().cycle() {
                        if current_node.ends_with('Z') {
                            break;
                        }

                        num += 1;

                        current_node = nodes
                            .get(&current_node.as_ref())
                            .map(|n| if move_dir == &'L' { n.0 } else { n.1 })
                            .unwrap().to_string()
                    }
                    num
                })
                .collect::<Vec<_>>();
            let num = lcm(&times);

            Ok(num.to_string())
        }),
    });
}
