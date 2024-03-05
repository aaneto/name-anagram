use std::collections::HashMap;
use std::io::{self, BufRead};
use std::fs::read_to_string;

fn main() {
    // let english_dictionary = read_dictionary("res/Oxford 3000 Word List.txt");
    let english_dictionary = read_dictionary("res/ospd.txt");
    let mut name = String::new();
    let stdin = io::stdin();
    println!("Please input your name: ");
    stdin.lock().read_line(&mut name).expect("could not read name");

    let anagrams = get_anagrams(&english_dictionary, &name);

    println!("Name: {name}");

    for anagram in anagrams {
        println!("Anagram: {anagram}");
    }
}

fn read_dictionary(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn get_anagrams(dictionary: &Vec<String>, name: &str) -> Vec<String> {
    let word_count = WordCount::new(name);
    dbg!(&word_count);

    word_count.two_word_anagrams(dictionary)
}

#[derive(Debug, Clone)]
struct WordCount {
    letters: HashMap<char, usize>,
}

impl WordCount {
    fn new(word: &str) -> Self {
        let mut count = WordCount {
            letters: HashMap::new()
        };

        for ch in word.chars() {
            if ch == '\n' || ch == ' ' {
                continue;
            }

            *count.letters.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
        }

        count
    }

    fn match_word_count(&self, word: &str) -> bool {
        let other_word_count = WordCount::new(word);

        if self.letters.len() != other_word_count.letters.len() {
            return false;
        }

        for ch in self.letters.keys() {
            if self.letters.get(ch) != other_word_count.letters.get(ch) {
                return false;
            }
        }

        true
    }

    fn superset_of_word_count(&self, word: &str) -> bool {
        let other_word_count = WordCount::new(word);

        for ch in other_word_count.letters.keys() {
            if self.letters.get(ch) < other_word_count.letters.get(ch) {
                return false;
            }
        }

        true
    }

    fn compute_difference(&self, word: &str) -> WordCount {
        let other_word_count = WordCount::new(word);
        let mut new_letters = HashMap::new();

        for ch in self.letters.keys() {
            let diff = self.letters[ch] - other_word_count.letters.get(ch).unwrap_or(&0);
            if diff != 0 {
                new_letters.insert(ch.clone(), diff);
            }
        }

        WordCount {
            letters: new_letters
        }
    }

    fn one_word_anagrams(&self, dictionary: &Vec<String>) -> Vec<String> {
        let mut anagrams = Vec::new();
        for word in dictionary.iter() {
            if self.match_word_count(word) {
                anagrams.push(word.clone());
            }
        }

        anagrams
    }

    fn two_word_anagrams(&self, dictionary: &Vec<String>) -> Vec<String> {
        let mut anagrams = Vec::new();
        let other_word_count = self.clone();

        for word in dictionary.iter() {
            if self.superset_of_word_count(word) {
                let diff = self.compute_difference(word);
                for second_word in diff.one_word_anagrams(dictionary) {
                    anagrams.push(format!("{word} {second_word}"));
                }
            }
        }

        anagrams
    }
}