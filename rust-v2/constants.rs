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

pub fn split_by_5(binary_string: &str) -> String {
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return serde_json::json!([]).to_string();
    }
    let chunks: Vec<String> = binary_string
        .chars()
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect())
        .collect();

    serde_json::json!(chunks).to_string()
}
