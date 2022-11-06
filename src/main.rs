use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use regex::Regex;

const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
    'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

#[derive(Default, Debug)]
struct Stats {
    total_words: i32, // number of words in the dictionary
    candidate_words: i32, // number of valid words (i.e. containing today’s letters with no invalid sequences)
    matching_pairs: i32, // number of potential chains, i.e. a tuple of words where the latter starts with the last letter of the prior
    perfect_pairs: i32 // number of word pairs that contain all of today’s letters
}

fn main() {
    let file = File::open("./src/words.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut candidates: Vec<String> = Vec::new();

    let input = env::args().nth(1).unwrap_or("".to_string());
    let re = Regex::new(r"([a-z]{3}/){3}[a-z]{3}").unwrap();

    if !re.is_match(&input) {
        panic!("Invalid input! Expected pattern: abc/def/ghi/jkl")
    }

    let rows = input.split('/').collect::<Vec<_>>();
    let letters = rows.iter().map(|x| x.chars()
        .collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut stats: Stats = Default::default();


    let valid_letters = create_valid_letters(&letters);
    let invalid_letters = create_invalid_letters(&valid_letters);
    let invalid_sequences = create_invalid_sequences(&letters);
    let mut perfect_pairs: Vec<String> = Vec::new();

    for line in lines {
        stats.total_words += 1;

        if let Ok(word) = line {
            if is_candidate(&word, &invalid_letters, &invalid_sequences) {
                candidates.push(word.clone());
                stats.candidate_words += 1;
            }
        }
    }

    for candidate in &candidates {

        // list of potential continuations
        let continuations = create_continuations(&candidate, &candidates);
        stats.matching_pairs += continuations.len() as i32;


        perfect_pairs.extend(create_perfect_pairs(&candidate, &continuations, &valid_letters));
    }

    stats.perfect_pairs = perfect_pairs.len() as i32;

    perfect_pairs.sort_by(|a, b| {
        match (a.len() - b.len()) as i32 {
            diff if diff < 0 => Ordering::Less,
            diff if diff > 0 => Ordering::Greater,
            _ => Ordering::Equal
        }
    });

    for pair in perfect_pairs {
        println!("{}", pair);
    }
}

/// A word is a candidate if it contains only letters from the set of today’s letters,
/// but not consecutive letters from the same “edge”. Also should be >= 4 letters long.
fn is_candidate(word: &String, invalid_letters: &Vec<&char>, invalid_sequences: &Vec<String>) -> bool {

    let mut contains_invalid_letters = false;
    let mut contains_invalid_sequence = false;
    let too_short = word.len() < 4;

    if !too_short {
        for &invalid_letter in invalid_letters {
            if word.contains(*invalid_letter) {
                contains_invalid_letters = true;
                break;
            }
        }

        if !contains_invalid_letters {
            for invalid_sequence in invalid_sequences {
                if word.contains(invalid_sequence) {
                    contains_invalid_sequence = true;
                    break;
                }
            }
        }
    }

    !(contains_invalid_letters || contains_invalid_sequence || too_short)
}

fn create_valid_letters(letters: &Vec<Vec<char>>) -> Vec<&char> {
    let mut valid_letters: Vec<&char> = Vec::new();

    for row in letters {
        for letter in row {
            valid_letters.push(&letter)
        }
    }

    valid_letters
}

fn create_invalid_letters<'a>(valid_letters: &'a Vec<&'a char>) -> Vec<&'a char> {
    let mut invalid_letters: Vec<&char> = Vec::new();

    for letter in &ALPHABET {
        if !valid_letters.contains(&letter) {
            invalid_letters.push(&letter)
        }
    }

    invalid_letters
}

// An invalid sequence is one where two letters come from the same row
fn create_invalid_sequences(letters: &Vec<Vec<char>>) -> Vec<String> {
    let mut invalid_sequences: Vec<String> = Vec::new();

    for row in letters {
        for x in 0..3 {
            for y in 0..3 {
                let mut seq = String::new();
                seq.push(row[x]);
                seq.push(row[y]);
                invalid_sequences.push(seq);
            }
        }
    }

    invalid_sequences
}

fn create_continuations(word: &String, candidates: &Vec<String>) -> Vec<String> {
    let first_word_last_letter = &word[word.len() - 1..];
    let mut continuations: Vec<String> = Vec::new();

    for candidate in candidates {
        if first_word_last_letter == &candidate[0..1] {
            continuations.push(candidate.clone());
        }
    }

    continuations
}

fn create_perfect_pairs(word: &String, continuations: &Vec<String>, letters: &Vec<&char>) -> Vec<String> {

    let mut pairs : Vec<String> = Vec::new();

    for word2 in continuations {
        let teststring = String::from(word) + &word2[1..];
        let mut used_letters : Vec<&char> = Vec::new();

        // check if teststring contains all of today’s letters
        for &letter in letters {
            if teststring.contains(*letter) {
                used_letters.push(&letter);
            }
        }

        if used_letters.len() == letters.len() {
            pairs.push(format!("{}-{}", &word, &word2));
        }
    }

    pairs
}