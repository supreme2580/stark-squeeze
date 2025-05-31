use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub const FIRST_DICT: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "00000" => "",
    "00001" => ".",
    "00010" => ".",
    "00011" => "..",
    "00100" => ".",
    "00101" => ". .",
    "00110" => "..",
    "00111" => "...",
    "01000" => ".",
    "01001" => ". .",
    "01010" => ". .",
    "01011" => ". ..",
    "01100" => "..",
    "01101" => ".. .",
    "01110" => "...",
    "01111" => "....",
    "10000" => ".",
    "10001" => ". .",
    "10010" => ". .",
    "10011" => ". ..",
    "10100" => ". .",
    "10101" => ". . .",
    "10110" => ". ..",
    "10111" => ". ...",
    "11000" => "..",
    "11001" => ".. .",
    "11010" => ".. .",
    "11011" => ".. ..",
    "11100" => "...",
    "11101" => "... .",
    "11110" => "....",
    "11111" => "....."
};

pub const SECOND_DICT: phf::Map<&'static str, char> = phf::phf_map! {
    "....." => '!',
    "...." => '#',
    "..." => '$',
    ".." => '%',
    ". ." => '&',
    "." => '*'
};

#[derive(Debug)]
pub enum DictionaryError {
    IoError(io::Error),
    InvalidFormat(String),
    EmptyDictionary,
}

impl fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DictionaryError::IoError(e) => write!(f, "IO error: {}", e),
            DictionaryError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            DictionaryError::EmptyDictionary => write!(f, "Dictionary is empty"),
        }
    }
}

impl Error for DictionaryError {}

impl From<io::Error> for DictionaryError {
    fn from(err: io::Error) -> DictionaryError {
        DictionaryError::IoError(err)
    }
}

pub trait Dictionary {
    fn get(&self, key: &str) -> Option<&str>;
    fn contains_key(&self, key: &str) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub struct CustomDictionary {
    map: HashMap<String, String>,
}

impl CustomDictionary {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, DictionaryError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut dict = CustomDictionary::new();
        for line in contents.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(DictionaryError::InvalidFormat(
                    "Each line must contain exactly one '=' separator".to_string(),
                ));
            }
            dict.map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        }

        if dict.is_empty() {
            return Err(DictionaryError::EmptyDictionary);
        }

        Ok(dict)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}

impl Dictionary for CustomDictionary {
    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }

    fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Dictionary for phf::Map<&'static str, &'static str> {
    fn get(&self, key: &str) -> Option<&str> {
        self.get(key).copied()
    }

    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Dictionary for phf::Map<&'static str, char> {
    fn get(&self, key: &str) -> Option<&str> {
        self.get(key).map(|c| std::str::from_utf8(&[*c as u8]).unwrap())
    }

    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
