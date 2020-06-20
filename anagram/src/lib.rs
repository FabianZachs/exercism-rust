use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sort(&word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|possible| {
            let p = possible.to_lowercase();
            sort(&p) == sorted_word && p != word
        })
        .collect()
}

fn sort(word: &str) -> Vec<char> {
    let mut chars: Vec<_> = word.chars().collect();
    chars.sort();
    chars
}
