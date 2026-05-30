use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let loword = word.to_lowercase();
    let mut soword = loword.chars().collect::<Vec<_>>();
    soword.sort_unstable();
    println!("{:?}", soword);
    let mut answer = HashSet::<&'a str>::new();
    for anagram in possible_anagrams {
        let mut lanagram = anagram.to_lowercase().chars().collect::<Vec<_>>();
        if lanagram.len() != soword.len() {
            continue;
        }
        if lanagram == loword.chars().collect::<Vec<_>>() {
            continue;
        }
        lanagram.sort_unstable();
        if soword == lanagram {
            answer.insert(*anagram);
        }
        println!("{:?}", lanagram);
    }
    println!("answer: {:?}", answer);
    answer
}
