use std::{collections::btree_map::Range, io::{self, BufRead}, u128};

fn main() {
    let stdin = io::stdin();
    let input: Vec<String> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut input_iter = input.iter();
    let no_of_cases: u128 = input_iter.next().unwrap().parse().unwrap();
    let test_cases: Vec<&str> = input_iter.map(|s| s.as_str()).collect();

    solve(test_cases, no_of_cases);
}

fn solve(mut input: Vec<&str>, amount: u128) {


    for i in 0..amount {

      let current_case: Vec<&str> = input.drain(0..2).collect();

        let phrase_1: Vec<&str> = current_case[0].split_whitespace().collect();
        let phrase_2: Vec<&str> = current_case[1].split_whitespace().collect();

        if phrase_1.len() != phrase_2.len() {
            println!("-");
            continue;
        }
        
        if find_unequal_words(&phrase_1, &phrase_2){
            println!("-");
            continue;
        }

        let paired_word_placeholders = replace_placeholder_word_pairs(phrase_1, phrase_2);

        let paired_placeholders = resolve_unmatched_placeholders(paired_word_placeholders[0].to_vec(), paired_word_placeholders[1].to_vec());

        if paired_placeholders[0].to_vec() == paired_placeholders[1].to_vec()
        {
            let combined = paired_placeholders[0].join(" ");
            println!("{}", combined)
        } else {
            println!("-");
            
        }
    }
}

fn is_placeholder(word: &str) -> bool{
    return word.starts_with('<') && word.ends_with('>');
}

fn find_unequal_words(phrase_1: &Vec<&str>, phrase_2: &Vec<&str>) -> bool {
    for (word_1, word_2) in phrase_1.iter().zip(phrase_2.iter()) {
        if !is_placeholder(word_1) && !is_placeholder(word_2) && word_1 != word_2 {
            return true;
        }
    }
    return false;
}

fn replace_placeholder_word_pairs<'a>(
    phrase_1: Vec<&'a str>, 
    phrase_2: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    for i in 0..phrase_1.len() {
        let word_1 = phrase_1[i];
        let word_2 = phrase_2[i];
        if is_placeholder(word_1) && !is_placeholder(word_2) {
            let updated_phrase = replace_placeholder(phrase_1, word_1, word_2);
            return replace_placeholder_word_pairs(updated_phrase, phrase_2);
        } else if !is_placeholder(word_1) && is_placeholder(word_2) {
            let updated_phrase = replace_placeholder(phrase_2, word_2, word_1);
            return replace_placeholder_word_pairs(phrase_1, updated_phrase);
        }
    }
    return vec![phrase_1, phrase_2]
}

fn resolve_unmatched_placeholders<'a>(
    phrase_1: Vec<&'a str>, 
    phrase_2: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    for i in 0..phrase_1.len() {
        let word_1 = phrase_1[i];
        let word_2 = phrase_2[i];

        if is_placeholder(word_1) && is_placeholder(word_2) {

            let updated_phrase_1 = replace_placeholder(phrase_1, word_1, "dingdong");
            let updated_phrase_2 = replace_placeholder(phrase_2, word_2, "dingdong");
            return resolve_unmatched_placeholders(updated_phrase_1, updated_phrase_2);
        }
    }

    return vec![phrase_1, phrase_2] 
}

fn replace_placeholder<'a>(mut phrase: Vec<&'a str>, old_word: &'a str, new_word: &'a str) -> Vec<&'a str>{
    for i in 0..phrase.len() {
        if phrase[i] == old_word {
            phrase[i] = new_word;
        }
    }
    return phrase;
}
