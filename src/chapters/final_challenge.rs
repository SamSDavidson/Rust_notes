use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;

pub fn main() {
    let mut word_map = HashMap::new();
    let file_path = get_path();
    let content = get_text(file_path);

    for line in content.lines() {
        let words = line.split_whitespace();
        for word in words {
            let new_word = word_map.entry(word.to_lowercase()).or_insert(0);
            *new_word += 1
        }
    }

    let top_items = get_top(word_map);

    println!("{:?}", top_items);
}

fn get_top<K: std::cmp::PartialOrd + Clone, V: std::cmp::PartialOrd + Clone>(words: HashMap<K, V>) -> Option<(K, V)> {
    let mut top_value: Option<(K, V)> = None;
    for (key, value) in words {
        if let Some((_, max_value)) = &top_value {
            if value <= *max_value {
                continue;
            }
        }
        top_value = Some((key.clone(), value.clone()));
    }
    top_value
}

fn get_path() -> String {
    if env::args().len() < 2 {
        println!("Must include a file path");
        std::process::exit(1);
    }
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("Invalid path");
            std::process::exit(1);
        }
    };
    path
}

fn get_text(path: String) -> String {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(error) => {
            format!("Failed to read file {}", error)
        }
    };
    content
}
