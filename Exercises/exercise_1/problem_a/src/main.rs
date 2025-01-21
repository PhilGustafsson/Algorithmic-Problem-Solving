use std::collections::HashMap;

fn main() {
    let test_input: Vec<&str> = vec![
        "4",
        "how now brown <animal>",
        "<foo> now <color> cow",
        "who are you",
        "<a> <b> <a>",
        "<a> b",
        "c <a>",
        "to be or not to be",
        "<foo> <bar> or not <foo> <bar>"
    ];

    solve(test_input);
}

fn solve(mut input: Vec<&str>) {
    let no_of_cases: u32 = input.remove(0).parse().unwrap();

    let mut output: Vec<String> = Vec::new();

    while (true) {
        if input.len() == 0 {
            break;
        }
        let mut current_case: Vec<&str> = input.drain(0..2).collect();

        let mut conditions_1: HashMap<&str, &str> = HashMap::new();
        let mut conditions_2: HashMap<&str, &str> = HashMap::new();

        let mut phrase_1: Vec<&str> = Vec::new();
        let mut phrase_2: Vec<&str> = Vec::new();

        phrase_1.extend(current_case[0].split_whitespace());
        phrase_2.extend(current_case[1].split_whitespace());

        if (phrase_1.len() != phrase_2.len()) {
            output.push("-".to_owned());
            continue;
        }
        let mut current_phrase: String = String::new();
        for i in 0..phrase_1.len() {
            let mut word_to_add: String = String::new();
            let word_1: &str = phrase_1[i];
            let word_2: &str = phrase_2[i];
            if word_1.chars().nth(0) == Some('<') {
                if let Some(value) = conditions_1.get(word_1) {
                    if *value != word_2 {
                        output.push("-".to_owned());
                        continue;
                    }
                } else {
                    conditions_1.insert(word_1, word_2);
                }
            } else {
                word_to_add += &(format!("{}", word_1));
            }
            if word_2.chars().nth(0) == Some('<') {
                if let Some(value) = conditions_2.get(word_2) {
                    if *value != word_1 {
                        output.push("-".to_owned());
                        continue;
                    }
                } else {
                    conditions_2.insert(word_2, word_1);
                }
            } else {
                if word_to_add.is_empty() {
                    word_to_add += &(format!("{}", word_2));
                }
            }

            current_phrase += " ";
            current_phrase += &word_to_add;
            if i == phrase_1.len() - 1 {
                output.push(current_phrase.to_owned());
            }
        }
    }
    for word in output {
        println!("{}", word);
    }
}
