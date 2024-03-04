use std::io::{self, BufRead};
use std::fs::read_to_string;

fn read_dictionary(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let english_dictionary = read_dictionary("res/Oxford 3000 Word List.txt");
    dbg!(english_dictionary.len());
    let mut name = String::new();
    let stdin = io::stdin();
    println!("Please input your name: ");
    stdin.lock().read_line(&mut name).expect("could not read name");

    println!("Name: {name}");
}
