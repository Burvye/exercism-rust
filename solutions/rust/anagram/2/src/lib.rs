use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lword = word.to_lowercase();
    let mut sword = lword.chars().collect::<Vec<_>>();
    sword.sort_unstable();
    possible_anagrams
        .iter()
        .filter(|anagram| {
            // bogus check
            if anagram.len() != word.len() {
                return false;
            }
            // same word check
            let lanagram = anagram.to_lowercase();
            if lanagram == lword {
                return false;
            }
            // sorted sameness check
            let mut slanagram = lanagram.chars().collect::<Vec<_>>();
            slanagram.sort_unstable();
            if slanagram != sword {
                return false;
            }
            return true;
        })
        .copied()
        .collect()
}
