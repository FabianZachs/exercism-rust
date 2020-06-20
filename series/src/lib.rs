pub fn series(digits: &str, len: usize) -> Vec<String> {
    // doesnt work if digits.len() + 1 - len < 0 since len is usize
    (0..digits.len() + 1 - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
