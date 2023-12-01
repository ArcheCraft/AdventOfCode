use crate::util::{Puzzle, Puzzles};

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIGIT_STR: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DIGIT_NAMES: [&'static str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn digit_to_number(text: &str) -> &str {
    for (i, name) in DIGIT_NAMES.iter().enumerate() {
        if name == &text {
            return DIGIT_STR[i];
        }
    }
    text
}

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(Puzzle {
        name: "Trebuchet?!".to_string(),
        year: 2023,
        day: 1,
        solver: Box::new(|input| {
            let mut sum = 0;
            for (i, line) in input.text().lines().enumerate() {
                let left = line
                    .find(DIGITS)
                    .ok_or_else(|| eyre::eyre!("No first digit found on line {}!", i))?;
                let right = line
                    .rfind(DIGITS)
                    .ok_or_else(|| eyre::eyre!("No second digit found on line {}!", i))?;
                let text = format!("{}{}", &line[left..left + 1], &line[right..right + 1]);
                sum += text.parse::<u32>()?;
            }
            Ok(sum.to_string())
        }),
    });
    puzzles.register(Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 1,
        solver: Box::new(|input| {
            let digit_group = format!("({}|{})", DIGIT_STR.join("|"), DIGIT_NAMES.join("|"));
            let first_regex = regex::Regex::new(&format!("^.*?{}.*", &digit_group))?;
            let last_regex = regex::Regex::new(&format!(".*{}.*?$", &digit_group))?;

            let mut sum = 0;
            for (i, line) in input.text().lines().enumerate() {
                let first_captures = first_regex.captures(line).ok_or_else(|| eyre::eyre!("No first digit found on line {}!", i))?;
                let first = first_captures.get(1).unwrap().as_str();
                let last_captures = last_regex.captures(line).ok_or_else(|| eyre::eyre!("No last digit found on line {}!", i))?;
                let last = last_captures.get(1).unwrap().as_str();
                let text = format!("{}{}", digit_to_number(first), digit_to_number(last));
                sum += text.parse::<u32>()?;
            }
            Ok(sum.to_string())
        }),
    });
}
