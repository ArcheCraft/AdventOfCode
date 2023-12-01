use std::{collections::HashMap, path::PathBuf};

use super::PuzzleInput;

pub struct InputLoader {
    cache: HashMap<u32, HashMap<u8, PuzzleInput>>,
    client: reqwest::blocking::Client,
}

impl InputLoader {
    const HEADER_SESSION: &'static str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/session.txt"));

    pub fn new() -> eyre::Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        println!("{}", Self::HEADER_SESSION);
        headers.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_str(Self::HEADER_SESSION.trim())?,
        );
        return Ok(Self {
            cache: HashMap::new(),
            client: reqwest::blocking::Client::builder()
                .user_agent("github.com/ArcheCraft/AdventOfCode using reqwest/0.11 by archecraft1@gmail.com")
                .default_headers(headers)
                .build()?,
        });
    }

    pub fn get_input(&mut self, year: u32, day: u8) -> eyre::Result<&mut PuzzleInput> {
        let year_map = self.cache.entry(year).or_insert_with(HashMap::new);

        if !year_map.contains_key(&day) {
            let path = Self::cached_file_or_download(&mut self.client, year, day)?;
            let input = PuzzleInput::open_file(&path)?;
            year_map.insert(day, input);
        }

        return year_map
            .get_mut(&day)
            .ok_or_else(|| eyre::eyre!("Just inserted into the map if it didn't exist..."));
    }

    fn cached_file_or_download(
        client: &mut reqwest::blocking::Client,
        year: u32,
        day: u8,
    ) -> eyre::Result<PathBuf> {
        let path = PathBuf::from(format!("inputs/{}/{}.txt", year, day));
        if path.exists() {
            return Ok(path);
        }

        println!("Downloading input for {}/{}...", year, day);
        let bytes = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                year, day
            ))
            .send()?
            .bytes()?;
        std::fs::create_dir_all(
            path.parent()
                .ok_or_else(|| eyre::eyre!("Directory of path {} doesn't exist", path.display()))?,
        )?;
        std::fs::write(&path, &bytes)?;
        return Ok(path);
    }
}
