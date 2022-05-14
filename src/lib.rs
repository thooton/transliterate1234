mod charset;
use charset::CHARSET;

pub fn transliterate(str: &str) -> String {
    let mut result = String::new();
    result.reserve(str.len());
    for char in str.chars() {
        let char_point = char as usize;
        if char_point < CHARSET.len() {
            result.push_str(
                CHARSET[char_point]
            );
        }
    }
    result
}