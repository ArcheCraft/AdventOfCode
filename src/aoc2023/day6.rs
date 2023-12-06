use crate::util::Puzzles;

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Wait For It".to_string(),
        year: 2023,
        day: 6,
        solver: Box::new(|input| {
            let times = input.text().lines().nth(0).unwrap().split_ascii_whitespace().skip(1).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            let distances = input.text().lines().nth(1).unwrap().split_ascii_whitespace().skip(1).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

            let mut sum = 1;
            for (time, record_distance) in times.iter().zip(distances.iter()) {
                sum *= (0..=*time).map(|n| (time - n) * n).filter(|d| d > record_distance).count();
            }

            Ok(sum.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 6,
        solver: Box::new(|input| {
            let time = input.text().lines().nth(0).unwrap().split_once(":").unwrap().1.replace(' ', "").parse::<u64>()?;
            let distance = input.text().lines().nth(1).unwrap().split_once(":").unwrap().1.replace(' ', "").parse::<u64>()?;

            let sum = (0..=time).map(|n| (time - n) * n).filter(|d| d > &distance).count();

            Ok(sum.to_string())
        }),
    });
}
