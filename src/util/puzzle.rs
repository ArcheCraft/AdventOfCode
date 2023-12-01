use std::path::Path;

pub struct Puzzle {
    pub name: String,
    pub year: u32,
    pub day: u8,

    pub solver: Box<dyn FnMut(&mut PuzzleInput) -> eyre::Result<String>>,
}

pub enum PuzzleInput {
    Raw(Vec<u8>),
    Test(String),
    Cached { input: Box<PuzzleInput> },
}

impl PuzzleInput {
    pub fn open_file(path: &Path) -> std::io::Result<Self> {
        return Ok(Self::Raw(std::fs::read(path)?));
    }

    pub fn test<T: ToString>(test_input: T) -> Self {
        return Self::Test(test_input.to_string());
    }

    pub fn cache(self) -> Self {
        return match self {
            Self::Cached { .. } => self,
            _ => Self::Cached {
                input: Box::new(self),
            },
        };
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        match self {
            Self::Raw(bytes) => bytes.clone(),
            Self::Test(text) => text.as_bytes().to_owned(),
            Self::Cached { input } => input.bytes(),
        }
    }

    pub fn text(&mut self) -> String {
        match self {
            Self::Test(text) => text.clone(),
            Self::Raw(bytes) => String::from_utf8_lossy(bytes).to_string(),
            Self::Cached { input } => input.text(),
        }
    }
}
