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

use std::collections::HashMap;

pub fn reverse_first_dict() -> HashMap<&'static str, &'static str> {
    FIRST_DICT.iter().map(|(binary, dot)| (*dot, *binary)).collect()
}

use std::io;

pub fn decoding_one(dot_string: &str) -> Result<String, io::Error> {
    let reversed_dict = reverse_first_dict();

    if dot_string.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input dot string is empty"));
    }

    let tokens: Vec<&str> = dot_string.split('.').collect();

    let mut binary_string = String::new();
    for token in tokens {
        match reversed_dict.get(token) {
            Some(&binary_chunk) => binary_string.push_str(binary_chunk),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown token: {}", token),
                ));
            }
        }
    }

    Ok(binary_string)
}

pub const SECOND_DICT: phf::Map<&'static str, char> = phf::phf_map! {
    "....." => '!',
    "...." => '#',
    "..." => '$',
    ".." => '%',
    ". ." => '&',
    "." => '*'
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding_one_valid() {
        let dot_string = ". .";
        let result = decoding_one(dot_string).unwrap();
        assert_eq!(result, "10001");
    }

    #[test]
    fn test_decoding_one_invalid_token() {
        let dot_string = "invalid";
        let result = decoding_one(dot_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_decoding_one_empty_input() {
        let dot_string = "";
        let result = decoding_one(dot_string);
        assert!(result.is_err());
    }
}