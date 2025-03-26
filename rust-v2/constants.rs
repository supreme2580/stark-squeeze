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
    // Handle the edge case of an empty string
    if binary_string.is_empty() {
        return serde_json::json!([]).to_string();
    }

    // Validate input: ensure it only contains '0' and '1'
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return serde_json::json!([]).to_string(); // Return an empty JSON array for invalid input
    }

    // Split the string into chunks of 5 characters using as_bytes for efficiency
    let chunks: Vec<String> = binary_string
        .as_bytes()
        .chunks(5)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect();

    // Convert the chunks into a JSON array
    serde_json::json!(chunks).to_string()
}