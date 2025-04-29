pub fn matches_pattern<I>(chars: &mut I, pattern: &str) -> bool 
where
    I: Iterator<Item = char> + Clone,
{
    let mut chars_clone = chars.clone();
    let mut pattern_chars = pattern.chars();
    
    loop {
        match (pattern_chars.next(), chars_clone.next()) {
            (Some(p), Some(c)) => {
                if p != c {
                    return false;
                }
            },
            (None, _) => {
                return true;
            },
            (Some(_), None) => {
                return false;
            }
        }
    }
}
