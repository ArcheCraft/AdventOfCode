use std::collections::HashMap;

use regex::Regex;

use crate::util::Puzzles;

struct Game {
    id: u32,
    games: Vec<HashMap<String, u32>>,
}

fn parse_game(i: u32, line: &str) -> eyre::Result<Game> {
    let start_regex = Regex::new("^Game (\\d+):")?;
    let color_regex = Regex::new("(\\d+) (\\w+)")?;

    let id = start_regex
        .captures(line)
        .ok_or_else(|| eyre::eyre!("Start didn't match on line {}!", i))?
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()?;
    let rest_index = line.find(':').unwrap();
    let rest = &line[rest_index..];
    let draws = rest.split(';');

    let mut games = vec![];
    for draw in draws {
        let colors = draw.trim().split(',');
        let mut map = HashMap::new();
        for color in colors {
            let captures = color_regex
                .captures(color)
                .ok_or_else(|| eyre::eyre!("Colors didn't match on line {}!", i))?;
            let amount = captures.get(1).unwrap().as_str().parse::<u32>()?;
            let color = captures.get(2).unwrap().as_str();
            map.insert(color.to_string(), amount);
        }
        games.push(map);
    }
    return Ok(Game { id, games });
}

pub fn register_all(puzzles: &mut Puzzles) {
    puzzles.register(crate::util::Puzzle {
        name: "Cube Conundrum".to_string(),
        year: 2023,
        day: 2,
        solver: Box::new(|input| {
            let mut sum = 0;
            for (i, line) in input.text().lines().enumerate() {
                let game = parse_game(i.try_into().unwrap(), line)?;
                let max_red = game
                    .games
                    .iter()
                    .map(|g| g.get("red").unwrap_or(&0))
                    .max()
                    .unwrap_or(&0);
                let max_green = game
                    .games
                    .iter()
                    .map(|g| g.get("green").unwrap_or(&0))
                    .max()
                    .unwrap_or(&0);
                let max_blue = game
                    .games
                    .iter()
                    .map(|g| g.get("blue").unwrap_or(&0))
                    .max()
                    .unwrap_or(&0);
                if max_red <= &12 && max_green <= &13 && max_blue <= &14 {
                    sum += game.id;
                }
            }
            Ok(sum.to_string())
        }),
    });
    puzzles.register(crate::util::Puzzle {
        name: "Part Two".to_string(),
        year: 2023,
        day: 2,
        solver: Box::new(|input| {
            let mut sum = 0;
            for (i, line) in input.text().lines().enumerate() {
                let game = parse_game(i.try_into().unwrap(), line)?;
                let max_red = game
                    .games
                    .iter()
                    .map(|g| g.get("red").unwrap_or(&0))
                    .max()
                    .unwrap_or(&0);
                let max_green = game
                    .games
                    .iter()
                    .map(|g| g.get("green").unwrap_or(&0))
                    .max()
                    .unwrap_or(&0);
                let max_blue = game
                    .games
                    .iter()
                    .map(|g| g.get("blue").unwrap_or(&0))
                    .max()
                    .unwrap_or(&0);
                sum += max_red * max_green * max_blue;
            }
            Ok(sum.to_string())
        }),
    });
}
